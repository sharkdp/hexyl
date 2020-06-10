#[macro_use]
extern crate clap;

use std::convert::TryFrom;
use std::fs::File;
use std::io::{self, prelude::*, SeekFrom};

use clap::{App, AppSettings, Arg};

use atty::Stream;

use anyhow::{anyhow, Error as AnyhowError};

use hexyl::{BorderStyle, Input, Printer};

fn run() -> Result<(), AnyhowError> {
    let app = App::new(crate_name!())
        .setting(AppSettings::ColorAuto)
        .setting(AppSettings::ColoredHelp)
        .setting(AppSettings::DeriveDisplayOrder)
        .setting(AppSettings::UnifiedHelpMessage)
        .max_term_width(90)
        .version(crate_version!())
        .about(crate_description!())
        .arg(
            Arg::with_name("FILE")
                .help("The file to display. If no FILE argument is given, read from STDIN."),
        )
        .arg(
            Arg::with_name("length")
                .short("n")
                .long("length")
                .takes_value(true)
                .value_name("N")
                .help(
                    "Only read N bytes from the input. The N argument can also include a \
                      unit with a decimal prefix (kB, MB, ..) or binary prefix (kiB, MiB, ..).\n\
                      Examples: --length=64, --length=4KiB",
                ),
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
                .help(
                    "Skip the first N bytes of the input. The N argument can also include \
                      a unit (see `--length` for details)",
                ),
        )
        .arg(
            Arg::with_name("block_size")
                .long("block-size")
                .takes_value(true)
                .value_name("SIZE")
                .help(
                    "Sets the size of the `block` unit to SIZE.\n\
                    Examples: --block-size=1024, --block-size=4kB",
                ),
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
                .value_name("WHEN")
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
                .value_name("STYLE")
                .possible_values(&["unicode", "ascii", "none"])
                .default_value("unicode")
                .help("Whether to draw a border with Unicode characters, ASCII characters, or none at all"),
        )
        .arg(
            Arg::with_name("display_offset")
                .short("o")
                .long("display-offset")
                .takes_value(true)
                .value_name("N")
                .help("Add N bytes to the displayed file position. The N argument can also include \
                a unit (see `--length` for details)"),
        );

    let matches = app.get_matches_safe()?;

    let stdin = io::stdin();

    let mut reader = match matches.value_of("FILE") {
        Some(filename) => Input::File(File::open(filename)?),
        None => Input::Stdin(stdin.lock()),
    };

    let block_size = matches
        .value_of("block_size")
        .and_then(|bs| bs.parse().ok().and_then(PositiveI64::new))
        .unwrap_or_else(|| PositiveI64::new(512).unwrap());

    let skip_arg = matches
        .value_of("skip")
        .and_then(|s| parse_byte_count(s, block_size));

    if let Some(skip) = skip_arg {
        reader.seek(SeekFrom::Current(skip.into_inner()))?;
    }

    let length_arg = matches
        .value_of("length")
        .or_else(|| matches.value_of("bytes"));

    let mut reader = if let Some(length) = length_arg.and_then(|s| parse_byte_count(s, block_size))
    {
        Box::new(reader.take(length.into()))
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
        .and_then(|s| parse_byte_count(s, block_size))
        .or(skip_arg)
        .unwrap_or_default()
        .into();

    let stdout = io::stdout();
    let mut stdout_lock = stdout.lock();

    let mut printer = Printer::new(&mut stdout_lock, show_color, border_style, squeeze);
    printer.display_offset(display_offset);
    printer.print_all(&mut reader).map_err(|e| anyhow!(e))?;

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
            eprintln!("Error: {:?}", err);
        }
        std::process::exit(1);
    }
}

#[derive(Clone, Copy, Debug, Default, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub struct PositiveI64(i64);

impl PositiveI64 {
    pub fn new(x: i64) -> Option<Self> {
        if x.is_negative() {
            None
        } else {
            Some(Self(x))
        }
    }

    pub fn into_inner(self) -> i64 {
        self.0
    }
}

impl Into<u64> for PositiveI64 {
    fn into(self) -> u64 {
        u64::try_from(self.0)
            .expect("invariant broken: PositiveI64 should contain a positive i64 value")
    }
}

fn parse_byte_count(n: &str, block_size: PositiveI64) -> Option<PositiveI64> {
    const HEX_PREFIX: &str = "0x";

    let n = {
        let mut chars = n.chars();
        match chars.next()? {
            '+' => chars.as_str(),
            '-' => return None,
            _ => n,
        }
    };

    if n.starts_with(HEX_PREFIX) {
        let n = &n[HEX_PREFIX.len()..];
        match n.chars().next() {
            Some('+') | Some('-') => return None,
            _ => (),
        }
        return i64::from_str_radix(n, 16).ok().and_then(PositiveI64::new);
    }

    let (n, unit_multiplier) = match n.chars().position(|c| !c.is_ascii_digit()) {
        Some(unit_begin_idx) => {
            let (n, raw_unit) = n.split_at(unit_begin_idx);
            let raw_unit = raw_unit.to_lowercase();
            (
                n,
                [
                    ("b", 1),
                    ("kb", 1000i64.pow(1)),
                    ("mb", 1000i64.pow(2)),
                    ("gb", 1000i64.pow(3)),
                    ("tb", 1000i64.pow(4)),
                    ("kib", 1024i64.pow(1)),
                    ("mib", 1024i64.pow(2)),
                    ("gib", 1024i64.pow(3)),
                    ("tib", 1024i64.pow(4)),
                    ("block", block_size.into_inner()),
                ]
                .iter()
                .cloned()
                .find_map(|(unit, multiplier)| {
                    if unit == raw_unit {
                        Some(multiplier)
                    } else {
                        None
                    }
                })?,
            )
        }
        None => (n, 1),
    };

    n.parse::<i64>()
        .ok()?
        .checked_mul(unit_multiplier)
        .and_then(PositiveI64::new)
}

#[test]
fn test_parse_byte_count() {
    macro_rules! success {
        ($input: expr, $expected: expr) => {
            success!($input, 512, $expected)
        };
        ($input: expr, $block_size: expr, $expected: expr) => {
            assert_eq!(
                parse_byte_count($input, PositiveI64::new($block_size).unwrap()),
                Some(PositiveI64::new($expected).unwrap())
            );
        };
    }

    macro_rules! error {
        ($input: expr) => {
            assert_eq!(
                parse_byte_count($input, PositiveI64::new(512).unwrap()),
                None
            );
        };
    }

    success!("0", 0);
    success!("1", 1);
    success!("1", 1);
    success!("100", 100);
    success!("+100", 100);

    success!("1KB", 1000);
    success!("2MB", 2000000);
    success!("3GB", 3000000000);
    success!("4TB", 4000000000000);
    success!("+4TB", 4000000000000);

    success!("1GiB", 1073741824);
    success!("2TiB", 2199023255552);
    success!("+2TiB", 2199023255552);

    success!("0xff", 255);
    success!("0xEE", 238);
    success!("+0xFF", 255);

    success!("1block", 512, 512);
    success!("2block", 512, 1024);
    success!("1block", 4, 4);
    success!("2block", 4, 8);

    // empty string is invalid
    error!("");
    // These are also bad.
    error!("+");
    error!("-");
    error!("K");
    error!("k");
    error!("m");
    error!("block");
    // leading/trailing space is invalid
    error!(" 0");
    error!("0 ");
    // Negatives make no sense for byte counts
    error!("-1");
    error!("0x-12");
    // This was previously accepted but shouldn't be.
    error!("0x+12");
    // invalid suffix
    error!("1234asdf");
    // bad numbers
    error!("asdf1234");
    error!("a1s2d3f4");
    // multiplication overflows u64
    error!("20000000TiB");
}
