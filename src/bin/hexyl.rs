#[macro_use]
extern crate clap;

use std::fs::File;
use std::io::{self, prelude::*, SeekFrom};

use clap::{App, AppSettings, Arg};

use atty::Stream;

use hexyl::{BorderStyle, Input, Printer};

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let app = App::new(crate_name!())
        .setting(AppSettings::ColorAuto)
        .setting(AppSettings::ColoredHelp)
        .setting(AppSettings::DeriveDisplayOrder)
        .setting(AppSettings::UnifiedHelpMessage)
        .version(crate_version!())
        .about(crate_description!())
        .arg(Arg::with_name("file").help("File to display"))
        .arg(
            Arg::with_name("length")
                .short("n")
                .long("length")
                .takes_value(true)
                .value_name("N")
                .help("Read only N bytes from the input"),
        )
        .arg(
            Arg::with_name("bytes")
                .short("c")
                .long("bytes")
                .takes_value(true)
                .value_name("N")
                .help("An alias for -n/--length"),
        )
        .arg(
            Arg::with_name("skip")
                .short("s")
                .long("skip")
                .takes_value(true)
                .value_name("N")
                .help("Skip first N bytes"),
        )
        .arg(
            Arg::with_name("nosqueezing")
                .short("v")
                .long("no-squeezing")
                .help(
                    "Displays all input data. Otherwise any number of groups of output \
                     lines which would be identical to the preceding group of lines, are \
                     replaced with a line comprised of a single asterisk.",
                ),
        )
        .arg(
            Arg::with_name("color")
                .long("color")
                .takes_value(true)
                .value_name("when")
                .possible_values(&["always", "auto", "never"])
                .default_value("always")
                .help(
                    "When to use colors. The auto-mode only displays colors if the output \
                     goes to an interactive terminal",
                ),
        )
        .arg(
            Arg::with_name("border")
                .long("border")
                .takes_value(true)
                .possible_values(&["unicode", "ascii", "none"])
                .default_value("unicode")
                .help("Whether to draw a border with unicode or ASCII characters, or none at all"),
        )
        .arg(
            Arg::with_name("display_offset")
                .short("o")
                .long("display-offset")
                .takes_value(true)
                .value_name("OFFSET")
                .help("Add OFFSET to the displayed file position."),
        );

    let matches = app.get_matches_safe()?;

    let stdin = io::stdin();

    let mut reader: Input = match matches.value_of("file") {
        Some(filename) => Input::File(File::open(filename)?),
        None => Input::Stdin(stdin.lock()),
    };

    let skip_arg = matches.value_of("skip").and_then(parse_hex_or_int);

    if let Some(skip) = skip_arg {
        reader.seek(SeekFrom::Start(skip))?;
    }

    let length_arg = matches
        .value_of("length")
        .or_else(|| matches.value_of("bytes"));

    let mut reader = if let Some(length) = length_arg.and_then(parse_hex_or_int) {
        Box::new(reader.take(length))
    } else {
        reader.into_inner()
    };

    let show_color = match matches.value_of("color") {
        Some("never") => false,
        Some("auto") => atty::is(Stream::Stdout),
        _ => true,
    };

    let border_style = match matches.value_of("border") {
        Some("unicode") => BorderStyle::Unicode,
        Some("ascii") => BorderStyle::Ascii,
        _ => BorderStyle::None,
    };

    let squeeze = !matches.is_present("nosqueezing");

    let display_offset = matches
        .value_of("display_offset")
        .and_then(parse_hex_or_int)
        .unwrap_or_else(|| skip_arg.unwrap_or(0));

    let stdout = io::stdout();
    let mut stdout_lock = stdout.lock();

    let mut printer = Printer::new(&mut stdout_lock, show_color, border_style, squeeze);
    printer.display_offset(display_offset as usize);
    printer.print_all(&mut reader)?;

    Ok(())
}

fn main() {
    // Enable ANSI support for Windows
    #[cfg(windows)]
    let _ = ansi_term::enable_ansi_support();

    let result = run();

    if let Err(err) = result {
        if let Some(clap_err) = err.downcast_ref::<clap::Error>() {
            eprint!("{}", clap_err); // Clap errors already have newlines

            match clap_err.kind {
                // The exit code should not indicate an error for --help / --version
                clap::ErrorKind::HelpDisplayed | clap::ErrorKind::VersionDisplayed => {
                    std::process::exit(0)
                }
                _ => (),
            }
        } else {
            eprintln!("Error: {}", err);
        }
        std::process::exit(1);
    }
}

fn parse_hex_or_int(n: &str) -> Option<u64> {
    if n.starts_with("0x") {
        let n = n.trim_start_matches("0x");
        if n.chars().next() == Some('+') {
            return None;
        }
        u64::from_str_radix(n, 16).ok()
    } else {
        n.parse::<u64>().ok()
    }
}
