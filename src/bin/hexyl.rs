#[macro_use]
extern crate clap;

use std::convert::TryFrom;
use std::fs::File;
use std::io::{self, prelude::*, SeekFrom};

use clap::{App, AppSettings, Arg};

use atty::Stream;

use anyhow::{anyhow, Context, Error as AnyhowError};

use thiserror::Error as ThisError;

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
                     unit with a decimal prefix (kB, MB, ..) or binary prefix (kiB, MiB, ..). \
                     The short option '-l' can be used as an alias.\n\
                     Examples: --length=64, --length=4KiB",
                ),
        )
        .arg(
            Arg::with_name("bytes")
                .short("c")
                .long("bytes")
                .takes_value(true)
                .value_name("N")
                .conflicts_with("length")
                .help("An alias for -n/--length"),
        )
        .arg(
            Arg::with_name("count")
                .short("l")
                .takes_value(true)
                .value_name("N")
                .hidden(true)
                .help("Yet another alias for -n/--length"),
        )
        .arg(
            Arg::with_name("skip")
                .short("s")
                .long("skip")
                .takes_value(true)
                .value_name("N")
                .help(
                    "Skip the first N bytes of the input. The N argument can also include \
                     a unit (see `--length` for details)\n\
                     A negative value is valid and will seek from the end of the file.",
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
                .help(
                    "Whether to draw a border with Unicode characters, ASCII characters, \
                    or none at all",
                ),
        )
        .arg(
            Arg::with_name("display_offset")
                .short("o")
                .long("display-offset")
                .takes_value(true)
                .value_name("N")
                .help(
                    "Add N bytes to the displayed file position. The N argument can also \
                    include a unit (see `--length` for details)\n\
                    A negative value is valid and calculates an offset relative to the \
                    end of the file.",
                ),
        );

    let matches = app.get_matches_safe()?;

    let stdin = io::stdin();

    let mut reader = match matches.value_of("FILE") {
        Some(filename) => Input::File(File::open(filename)?),
        None => Input::Stdin(stdin.lock()),
    };

    let block_size = matches
        .value_of("block_size")
        .map(|bs| {
            bs.parse::<i64>().map_err(|e| anyhow!(e)).and_then(|x| {
                PositiveI64::new(x).ok_or_else(|| anyhow!("block size argument must be positive"))
            })
        })
        .transpose()?
        .unwrap_or_else(|| PositiveI64::new(512).unwrap());

    let skip_arg = matches
        .value_of("skip")
        .map(|s| {
            parse_byte_offset(s, block_size).context(anyhow!(
                "failed to parse `--skip` arg {:?} as byte count",
                s
            ))
        })
        .transpose()?;

    let skip_offset = if let Some(ByteOffset { kind, value }) = skip_arg {
        let value = value.into_inner();
        reader
            .seek(match kind {
                ByteOffsetKind::ForwardFromBeginning | ByteOffsetKind::ForwardFromLastOffset => {
                    SeekFrom::Current(value)
                }
                ByteOffsetKind::BackwardFromEnd => SeekFrom::End(value.checked_neg().unwrap()),
            })
            .map_err(|_| {
                anyhow!(
                    "Failed to jump to the desired input position. \
                     This could be caused by a negative offset that is too large or by \
                     an input that is not seek-able (e.g. if the input comes from a pipe)."
                )
            })?
    } else {
        0
    };

    let parse_byte_count = |s| -> Result<u64, AnyhowError> {
        Ok(parse_byte_offset(s, block_size)?
            .assume_forward_offset_from_start()?
            .into())
    };

    let mut reader = if let Some(length) = matches
        .value_of("length")
        .or_else(|| matches.value_of("bytes"))
        .or_else(|| matches.value_of("count"))
        .map(|s| {
            parse_byte_count(s).context(anyhow!(
                "failed to parse `--length` arg {:?} as byte count",
                s
            ))
        })
        .transpose()?
    {
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

    let display_offset: u64 = matches
        .value_of("display_offset")
        .map(|s| {
            parse_byte_count(s).context(anyhow!(
                "failed to parse `--display-offset` arg {:?} as byte count",
                s
            ))
        })
        .transpose()?
        .unwrap_or(0)
        .into();

    let stdout = io::stdout();
    let mut stdout_lock = stdout.lock();

    let mut printer = Printer::new(&mut stdout_lock, show_color, border_style, squeeze);
    printer.display_offset(skip_offset + display_offset);
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
            match clap_err.kind {
                // The exit code should not indicate an error for --help / --version
                clap::ErrorKind::HelpDisplayed => {
                    eprint!("{}", clap_err); // Clap errors already have newlines
                    std::process::exit(0)
                },
                clap::ErrorKind::VersionDisplayed => {
                    // Version output in clap 2.33.1 (dep as of now) doesn't have a newline
                    // and the fix is not included even in the latest stable release
                    println!("");
                    std::process::exit(0)
                },
                _ => (),
            }
        } else {
            eprintln!("Error: {:?}", err);
        }
        std::process::exit(1);
    }
}

#[derive(Clone, Copy, Debug, Default, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub struct NonNegativeI64(i64);

impl NonNegativeI64 {
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

impl Into<u64> for NonNegativeI64 {
    fn into(self) -> u64 {
        u64::try_from(self.0)
            .expect("invariant broken: NonNegativeI64 should contain a non-negative i64 value")
    }
}

#[derive(Clone, Copy, Debug, Default, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub struct PositiveI64(i64);

impl PositiveI64 {
    pub fn new(x: i64) -> Option<Self> {
        if x < 1 {
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

const HEX_PREFIX: &str = "0x";

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum ByteOffsetKind {
    ForwardFromBeginning,
    ForwardFromLastOffset,
    BackwardFromEnd,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct ByteOffset {
    value: NonNegativeI64,
    kind: ByteOffsetKind,
}

#[derive(Clone, Debug, ThisError)]
#[error(
    "negative offset specified, but only positive offsets (counts) are accepted in this context"
)]
struct NegativeOffsetSpecifiedError;

impl ByteOffset {
    fn assume_forward_offset_from_start(
        &self,
    ) -> Result<NonNegativeI64, NegativeOffsetSpecifiedError> {
        let &Self { value, kind } = self;
        match kind {
            ByteOffsetKind::ForwardFromBeginning | ByteOffsetKind::ForwardFromLastOffset => {
                Ok(value)
            }
            ByteOffsetKind::BackwardFromEnd => Err(NegativeOffsetSpecifiedError),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, ThisError)]
enum ByteOffsetParseError {
    #[error("no character data found, did you forget to write it?")]
    Empty,
    #[error("no digits found after sign, did you forget to write them?")]
    EmptyAfterSign,
    #[error(
        "found {0:?} sign after hex prefix ({:?}); signs should go before it",
        HEX_PREFIX
    )]
    SignFoundAfterHexPrefix(char),
    #[error("{0:?} is not of the expected form <pos-integer>[<unit>]")]
    InvalidNumAndUnit(String),
    #[error("{0:?} is a valid unit, but an integer should come before it")]
    EmptyWithUnit(String),
    #[error("invalid unit {0:?}")]
    InvalidUnit(String),
    #[error("failed to parse integer part")]
    ParseNum(#[source] std::num::ParseIntError),
    #[error("count multipled by the unit overflowed a signed 64-bit integer; are you sure it should be that big?")]
    UnitMultiplicationOverflow,
}

fn parse_byte_offset(n: &str, block_size: PositiveI64) -> Result<ByteOffset, ByteOffsetParseError> {
    use ByteOffsetParseError::*;

    let (n, kind) = {
        let mut chars = n.chars();
        let next_char = chars.next();
        let check_empty_after_sign = || {
            if chars.clone().next().is_none() {
                Err(EmptyAfterSign)
            } else {
                Ok(chars.as_str())
            }
        };
        match next_char {
            Some('+') => (
                check_empty_after_sign()?,
                ByteOffsetKind::ForwardFromLastOffset,
            ),
            Some('-') => (check_empty_after_sign()?, ByteOffsetKind::BackwardFromEnd),
            None => return Err(Empty),
            _ => (n, ByteOffsetKind::ForwardFromBeginning),
        }
    };

    let into_byte_offset = |value| {
        Ok(ByteOffset {
            value: NonNegativeI64::new(value).unwrap(),
            kind,
        })
    };

    if n.starts_with(HEX_PREFIX) {
        let n = &n[HEX_PREFIX.len()..];
        let mut chars = n.chars();
        match chars.next() {
            Some(c @ '+') | Some(c @ '-') => {
                return if chars.next().is_none() {
                    Err(EmptyAfterSign)
                } else {
                    Err(SignFoundAfterHexPrefix(c))
                }
            }
            _ => (),
        }
        return i64::from_str_radix(n, 16)
            .map(into_byte_offset)
            .map_err(ParseNum)?;
    }

    let (num, unit) = match n.chars().position(|c| !c.is_ascii_digit()) {
        Some(unit_begin_idx) => {
            let (n, raw_unit) = n.split_at(unit_begin_idx);
            let raw_unit_lower = raw_unit.to_lowercase();
            let multiplier = [
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
                if unit == raw_unit_lower {
                    Some(multiplier)
                } else {
                    None
                }
            })
            .ok_or_else(|| InvalidUnit(raw_unit.to_owned()));
            (n, multiplier.map(|m| (Some(raw_unit), m)))
        }
        None => (n, Ok((None, 1))),
    };

    match (num.parse::<i64>(), unit) {
        (Ok(num), Ok((_raw_unit, unit_multiplier))) => num
            .checked_mul(unit_multiplier)
            .ok_or_else(|| UnitMultiplicationOverflow)
            .and_then(into_byte_offset),
        (Ok(_), Err(e)) => Err(e),
        (Err(e), Ok((raw_unit, _unit_multiplier))) => match raw_unit {
            Some(raw_unit) if num.is_empty() => Err(EmptyWithUnit(raw_unit.to_owned())),
            _ => Err(ParseNum(e)),
        },
        (Err(_), Err(_)) => Err(InvalidNumAndUnit(n.to_owned())),
    }
}

#[test]
fn test_parse_byte_offset() {
    use ByteOffsetParseError::*;

    macro_rules! success {
        ($input: expr, $expected_kind: ident $expected_value: expr) => {
            success!($input, $expected_kind $expected_value; block_size: 512)
        };
        ($input: expr, $expected_kind: ident $expected_value: expr; block_size: $block_size: expr) => {
            assert_eq!(
                parse_byte_offset($input, PositiveI64::new($block_size).unwrap()),
                Ok(
                    ByteOffset {
                        value: NonNegativeI64::new($expected_value).unwrap(),
                        kind: ByteOffsetKind::$expected_kind,
                    }
                ),
            );
        };
    }

    macro_rules! error {
        ($input: expr, $expected_err: expr) => {
            assert_eq!(
                parse_byte_offset($input, PositiveI64::new(512).unwrap()),
                Err($expected_err),
            );
        };
    }

    success!("0", ForwardFromBeginning 0);
    success!("1", ForwardFromBeginning 1);
    success!("1", ForwardFromBeginning 1);
    success!("100", ForwardFromBeginning 100);
    success!("+100", ForwardFromLastOffset 100);

    success!("0x0", ForwardFromBeginning 0);
    success!("0xf", ForwardFromBeginning 15);
    success!("0xdeadbeef", ForwardFromBeginning 3_735_928_559);

    success!("1KB", ForwardFromBeginning 1000);
    success!("2MB", ForwardFromBeginning 2000000);
    success!("3GB", ForwardFromBeginning 3000000000);
    success!("4TB", ForwardFromBeginning 4000000000000);
    success!("+4TB", ForwardFromLastOffset 4000000000000);

    success!("1GiB", ForwardFromBeginning 1073741824);
    success!("2TiB", ForwardFromBeginning 2199023255552);
    success!("+2TiB", ForwardFromLastOffset 2199023255552);

    success!("0xff", ForwardFromBeginning 255);
    success!("0xEE", ForwardFromBeginning 238);
    success!("+0xFF", ForwardFromLastOffset 255);

    success!("1block", ForwardFromBeginning 512; block_size: 512);
    success!("2block", ForwardFromBeginning 1024; block_size: 512);
    success!("1block", ForwardFromBeginning 4; block_size: 4);
    success!("2block", ForwardFromBeginning 8; block_size: 4);

    // empty string is invalid
    error!("", Empty);
    // These are also bad.
    error!("+", EmptyAfterSign);
    error!("-", EmptyAfterSign);
    error!("K", InvalidNumAndUnit("K".to_owned()));
    error!("k", InvalidNumAndUnit("k".to_owned()));
    error!("m", InvalidNumAndUnit("m".to_owned()));
    error!("block", EmptyWithUnit("block".to_owned()));
    // leading/trailing space is invalid
    error!(" 0", InvalidNumAndUnit(" 0".to_owned()));
    error!("0 ", InvalidUnit(" ".to_owned()));
    // Signs after the hex prefix make no sense
    error!("0x-12", SignFoundAfterHexPrefix('-'));
    // This was previously accepted but shouldn't be.
    error!("0x+12", SignFoundAfterHexPrefix('+'));
    // invalid suffix
    error!("1234asdf", InvalidUnit("asdf".to_owned()));
    // bad numbers
    error!("asdf1234", InvalidNumAndUnit("asdf1234".to_owned()));
    error!("a1s2d3f4", InvalidNumAndUnit("a1s2d3f4".to_owned()));
    // multiplication overflows u64
    error!("20000000TiB", UnitMultiplicationOverflow);

    assert!(
        match parse_byte_offset("99999999999999999999", PositiveI64::new(512).unwrap()) {
            // We can't check against the kind of the `ParseIntError`, so we'll just make sure it's the
            // same as trying to do the parse directly.
            Err(ParseNum(e)) => e == "99999999999999999999".parse::<i64>().unwrap_err(),
            _ => false,
        }
    );
}
