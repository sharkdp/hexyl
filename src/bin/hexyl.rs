// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate clap;

use atty;
use ctrlc;

use std::fs::File;
use std::io::{self, prelude::*};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use clap::{App, AppSettings, Arg};

use atty::Stream;

use hexyl::{BorderStyle, Printer};

mod errors {
    error_chain! {
        foreign_links {
            Clap(::clap::Error);
            Io(::std::io::Error);
            ParseIntError(::std::num::ParseIntError);
        }
    }
}

use crate::errors::*;

fn run() -> Result<()> {
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
            Arg::with_name("range")
                .short("r")
                .long("range")
                .takes_value(true)
                .value_name("N:M")
                .conflicts_with("length")
                .conflicts_with("bytes")
                .help("Read only bytes N through M from the input")
                .long_help(
                    "Only print the specified range of bytes. \
                     For example:\n  \
                     '--range 512:1024' prints bytes 512 to 1024\n  \
                     '--range 512:+512' skips 512 bytes and prints the next 512 bytes (equivalent to 512:1024)\n  \
                     '--range :512' prints the first 512 bytes\n  \
                     '--range 512:' skips 512 bytes and prints the rest of the input")
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

    let mut reader: Box<dyn Read> = match matches.value_of("file") {
        Some(filename) => Box::new(File::open(filename)?),
        None => Box::new(stdin.lock()),
    };

    let length_arg = matches.value_of("length").or(matches.value_of("bytes"));

    if let Some(length) = length_arg.and_then(parse_hex_or_int) {
        reader = Box::new(reader.take(length));
    }

    let byterange_arg = matches.value_of("range");
    let mut range_offset = 0;
    if let Some(range) = byterange_arg {
        if let Ok((offset, num_bytes)) = parse_range(range) {
            range_offset = offset;
            let mut discard = vec![0u8; offset as usize];
            reader
                .read_exact(&mut discard)
                .map_err(|_| format!("Unable to start reading at {}, input too small", offset))?;
            reader = Box::new(reader.take(num_bytes));
        }
    }

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
        .unwrap_or(range_offset);

    // Set up Ctrl-C handler
    let cancelled = Arc::new(AtomicBool::new(false));
    let c = cancelled.clone();

    ctrlc::set_handler(move || {
        c.store(true, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    let stdout = io::stdout();
    let mut stdout_lock = stdout.lock();

    let mut printer = Printer::new(&mut stdout_lock, show_color, border_style, squeeze);
    printer.display_offset(display_offset as usize);
    printer
        .print_all(&mut reader, Some(cancelled))
        .map_err(|err| format!("{}", err))?;

    Ok(())
}

fn main() {
    // Enable ANSI support for Windows
    #[cfg(windows)]
    let _ = ansi_term::enable_ansi_support();

    let result = run();

    if let Err(err) = result {
        match err {
            Error(ErrorKind::Clap(ref clap_error), _) => {
                eprint!("{}", clap_error); // Clap errors already have newlines

                match clap_error.kind {
                    // The exit code should not indicate an error for --help / --version
                    clap::ErrorKind::HelpDisplayed | clap::ErrorKind::VersionDisplayed => {
                        std::process::exit(0)
                    }
                    _ => (),
                }
            }
            Error(err, _) => eprintln!("Error: {}", err),
        }
        std::process::exit(1);
    }
}

fn parse_hex_or_int(n: &str) -> Option<u64> {
    let n = n.trim_start_matches('+');
    if n.starts_with("0x") {
        u64::from_str_radix(n.trim_start_matches("0x"), 16).ok()
    } else {
        n.parse::<u64>().ok()
    }
}

fn parse_range(range_raw: &str) -> Result<(u64, u64)> {
    match range_raw.split(':').collect::<Vec<&str>>()[..] {
        [offset_raw, bytes_to_read_raw] => {
            let offset = parse_hex_or_int(&offset_raw).unwrap_or_else(u64::min_value);
            let bytes_to_read = match parse_hex_or_int(bytes_to_read_raw) {
                Some(num) if bytes_to_read_raw.starts_with('+') => num,
                Some(num) if offset <= num => num - offset,
                Some(num) => {
                    return Err(format!(
                        "cannot start reading at {} and stop reading at {}",
                        offset, num
                    )
                    .into())
                }
                None if bytes_to_read_raw != "" => return Err("unable to parse range".into()),
                None => u64::max_value(),
            };
            Ok((offset, bytes_to_read))
        }
        [offset_raw] => {
            let offset = parse_hex_or_int(&offset_raw).unwrap_or_else(u64::min_value);
            let bytes_to_read = u64::max_value();
            Ok((offset, bytes_to_read))
        }
        _ => Err("expected single ':' character".into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_empty_range() {
        let (offset, bytes_to_read) = parse_range(":").expect("Not allowed to fail test.");
        assert_eq!(offset, u64::min_value());
        assert_eq!(bytes_to_read, u64::max_value());
    }

    #[test]
    fn parse_starting_offset() {
        let (offset, bytes_to_read) = parse_range("0x200:").expect("Not allowed to fail test.");
        assert_eq!(offset, 512);
        assert_eq!(bytes_to_read, u64::max_value());

        let (offset, bytes_to_read) = parse_range("0x200").expect("Not allowed to fail test.");
        assert_eq!(offset, 512);
        assert_eq!(bytes_to_read, u64::max_value());
    }

    #[test]
    fn parse_ending_offset() {
        let (offset, bytes_to_read) = parse_range(":0x200").expect("Not allowed to fail test.");
        assert_eq!(offset, u64::min_value());
        assert_eq!(bytes_to_read, 512);

        let (offset, bytes_to_read) = parse_range(":+512").expect("Not allowed to fail test.");
        assert_eq!(offset, u64::min_value());
        assert_eq!(bytes_to_read, 512);

        let (offset, bytes_to_read) = parse_range("512:512").expect("Not allowed to fail test.");
        assert_eq!(offset, 512);
        assert_eq!(bytes_to_read, 0);

        let (offset, bytes_to_read) = parse_range("512:+512").expect("Not allowed to fail test.");
        assert_eq!(offset, 512);
        assert_eq!(bytes_to_read, 512);
    }

    #[test]
    fn parse_bad_input() {
        let result = parse_range("1024:512");
        assert!(result.is_err());

        let result = parse_range("512:-512");
        assert!(result.is_err());
    }
}
