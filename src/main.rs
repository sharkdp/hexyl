use std::fs::File;
use std::io::{self, prelude::*, BufWriter, SeekFrom};
use std::num::{NonZeroI64, NonZeroU64};
use std::path::PathBuf;

use clap::builder::styling::{AnsiColor, Effects};
use clap::builder::ArgPredicate;
use clap::builder::Styles;
use clap::{ArgAction, CommandFactory, Parser, ValueEnum};
use clap_complete::aot::{generate, Shell};

use anyhow::{anyhow, bail, Context, Result};

use const_format::formatcp;

use thiserror::Error as ThisError;

use terminal_size::terminal_size;

use hexyl::{
    Base, BorderStyle, CharacterTable, ColorScheme, Endianness, IncludeMode, Input, PrinterBuilder,
};

use hexyl::{
    COLOR_ASCII_OTHER, COLOR_ASCII_PRINTABLE, COLOR_ASCII_WHITESPACE, COLOR_NONASCII, COLOR_NULL,
    COLOR_RESET,
};

#[cfg(test)]
mod tests;

const DEFAULT_BLOCK_SIZE: i64 = 512;

const LENGTH_HELP_TEXT: &str = "Only read N bytes from the input. The N argument can also include \
                                a unit with a decimal prefix (kB, MB, ..) or binary prefix (kiB, \
                                MiB, ..), or can be specified using a hex number. The short \
                                option '-l' can be used as an alias.
Examples: --length=64, --length=4KiB, --length=0xff";

const SKIP_HELP_TEXT: &str = "Skip the first N bytes of the input. The N argument can also \
                              include a unit (see `--length` for details).
A negative value is valid and will seek from the end of the file.";

const BLOCK_SIZE_HELP_TEXT: &str = "Sets the size of the `block` unit to SIZE.
Examples: --block-size=1024, --block-size=4kB";

const DISPLAY_OFFSET_HELP_TEXT: &str = "Add N bytes to the displayed file position. The N \
                                        argument can also include a unit (see `--length` for \
                                        details).
A negative value is valid and calculates an offset relative to the end of the file.";

const TERMINAL_WIDTH_HELP_TEXT: &str = "Sets the number of terminal columns to be displayed.
Since the terminal width may not be an evenly divisible by the width per hex data column, this \
                                        will use the greatest number of hex data panels that can \
                                        fit in the requested width but still leave some space to \
                                        the right.
Cannot be used with other width-setting options.";

const STYLES: Styles = Styles::styled()
    .header(AnsiColor::Green.on_default().effects(Effects::BOLD))
    .usage(AnsiColor::Green.on_default().effects(Effects::BOLD))
    .literal(AnsiColor::Cyan.on_default().effects(Effects::BOLD))
    .placeholder(AnsiColor::Cyan.on_default());

#[derive(Debug, Parser)]
#[command(version, about, max_term_width(90), styles = STYLES)]
struct Opt {
    /// The file to display. If no FILE argument is given, read from STDIN.
    #[arg(value_name("FILE"))]
    file: Option<PathBuf>,

    #[arg(
        help(LENGTH_HELP_TEXT),
        short('n'),
        long,
        visible_short_alias('c'),
        visible_alias("bytes"),
        short_alias('l'),
        value_name("N")
    )]
    length: Option<String>,

    #[arg(help(SKIP_HELP_TEXT), short, long, value_name("N"))]
    skip: Option<String>,

    #[arg(
        help(BLOCK_SIZE_HELP_TEXT),
        long,
        default_value(formatcp!("{DEFAULT_BLOCK_SIZE}")),
        value_name("SIZE")
    )]
    block_size: String,

    /// Displays all input data. Otherwise any number of groups of output lines
    /// which would be identical to the preceding group of lines, are replaced
    /// with a line comprised of a single asterisk.
    #[arg(short('v'), long)]
    no_squeezing: bool,

    /// When to use colors.
    #[arg(
        long,
        value_enum,
        default_value_t,
        value_name("WHEN"),
        default_value_if("plain", ArgPredicate::IsPresent, Some("never"))
    )]
    color: ColorWhen,

    /// Whether to draw a border.
    #[arg(
        long,
        value_enum,
        default_value_t,
        value_name("STYLE"),
        default_value_if("plain", ArgPredicate::IsPresent, Some("none"))
    )]
    border: BorderStyle,

    /// Display output with --no-characters, --no-position, --border=none, and
    /// --color=never.
    #[arg(short, long)]
    plain: bool,

    /// Do not show the character panel on the right.
    #[arg(long)]
    no_characters: bool,

    /// Show the character panel on the right. This is the default, unless
    /// --no-characters has been specified.
    #[arg(
        short('C'),
        long,
        action(ArgAction::SetTrue),
        overrides_with("no_characters")
    )]
    characters: (),

    /// Defines how bytes are mapped to characters.
    #[arg(long, value_enum, default_value_t, value_name("FORMAT"))]
    character_table: CharacterTable,

    /// Defines the color scheme for the characters.
    #[arg(long, value_enum, default_value_t, value_name("FORMAT"))]
    color_scheme: ColorScheme,

    /// Whether to display the position panel on the left.
    #[arg(short('P'), long)]
    no_position: bool,

    #[arg(
        help(DISPLAY_OFFSET_HELP_TEXT),
        short('o'),
        long,
        default_value("0"),
        value_name("N")
    )]
    display_offset: String,

    /// Sets the number of hex data panels to be displayed. `--panels=auto` will
    /// display the maximum number of hex data panels based on the current
    /// terminal width. By default, hexyl will show two panels, unless the
    /// terminal is not wide enough for that.
    #[arg(long, value_name("N"))]
    panels: Option<String>,

    /// Number of bytes/octets that should be grouped together. You can use the
    /// '--endianness' option to control the ordering of the bytes within a
    /// group. '--groupsize' can be used as an alias (xxd-compatibility).
    #[arg(
        short('g'),
        long,
        value_enum,
        default_value_t,
        alias("groupsize"),
        value_name("N")
    )]
    group_size: GroupSize,

    /// Whether to print out groups in little-endian or big-endian format. This
    /// option only has an effect if the '--group-size' is larger than 1. '-e'
    /// can be used as an alias for '--endianness=little'.
    #[arg(long, value_enum, default_value_t, value_name("FORMAT"))]
    endianness: Endianness,

    /// An alias for '--endianness=little'.
    #[arg(short('e'), hide(true), overrides_with("endianness"))]
    little_endian_format: bool,

    /// Sets the base used for the bytes. The possible options are binary,
    /// octal, decimal, and hexadecimal.
    #[arg(short('b'), long, default_value("hexadecimal"), value_name("B"))]
    base: String,

    #[arg(
        help(TERMINAL_WIDTH_HELP_TEXT),
        long,
        value_name("N"),
        conflicts_with("panels")
    )]
    terminal_width: Option<NonZeroU64>,

    /// Print a table showing how different types of bytes are colored.
    #[arg(long)]
    print_color_table: bool,

    /// Output in C include file style (similar to xxd -i).
    #[arg(
        short('i'),
        long("include"),
        help = "Output in C include file style",
        conflicts_with("little_endian_format"),
        conflicts_with("endianness")
    )]
    include_mode: bool,

    /// Show shell completion for a certain shell
    #[arg(long, value_name("SHELL"))]
    completion: Option<Shell>,
}

#[derive(Clone, Debug, Default, ValueEnum)]
enum ColorWhen {
    /// Always use colorized output.
    #[default]
    Always,

    /// Only displays colors if the output goes to an interactive terminal.
    Auto,

    /// Do not use colorized output.
    Never,

    /// Override the NO_COLOR environment variable.
    Force,
}

#[derive(Clone, Debug, Default, ValueEnum)]
enum GroupSize {
    /// Grouped together every byte/octet.
    #[default]
    #[value(name = "1")]
    One,

    /// Grouped together every 2 bytes/octets.
    #[value(name = "2")]
    Two,

    /// Grouped together every 4 bytes/octets.
    #[value(name = "4")]
    Four,

    /// Grouped together every 8 bytes/octets.
    #[value(name = "8")]
    Eight,
}

impl From<GroupSize> for u8 {
    fn from(number: GroupSize) -> Self {
        match number {
            GroupSize::One => 1,
            GroupSize::Two => 2,
            GroupSize::Four => 4,
            GroupSize::Eight => 8,
        }
    }
}

fn run() -> Result<()> {
    let opt = Opt::parse();

    if opt.print_color_table {
        return print_color_table().map_err(|e| anyhow!(e));
    }

    if let Some(sh) = opt.completion {
        let mut cmd = Opt::command();
        let name = cmd.get_name().to_string();
        generate(sh, &mut cmd, name, &mut io::stdout());
        return Ok(());
    }

    let stdin = io::stdin();

    let mut reader = match &opt.file {
        Some(filename) => {
            if filename.as_os_str() == "-" {
                Input::Stdin(stdin.lock())
            } else {
                if filename.is_dir() {
                    bail!("'{}' is a directory.", filename.to_string_lossy());
                }
                let file = File::open(filename)?;

                Input::File(file)
            }
        }
        None => Input::Stdin(stdin.lock()),
    };

    if let Some(hex_number) = try_parse_as_hex_number(&opt.block_size) {
        return hex_number
            .map_err(|e| anyhow!(e))
            .and_then(|x| {
                PositiveI64::new(x).ok_or_else(|| anyhow!("block size argument must be positive"))
            })
            .map(|_| ());
    }
    let (num, unit) = extract_num_and_unit_from(&opt.block_size)?;
    if let Unit::Block { custom_size: _ } = unit {
        return Err(anyhow!(
            "can not use 'block(s)' as a unit to specify block size"
        ));
    };
    let block_size = num
        .checked_mul(unit.get_multiplier())
        .ok_or_else(|| anyhow!(ByteOffsetParseError::UnitMultiplicationOverflow))
        .and_then(|x| {
            PositiveI64::new(x).ok_or_else(|| anyhow!("block size argument must be positive"))
        })?;

    let skip_arg = opt
        .skip
        .as_ref()
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

    let parse_byte_count = |s| -> Result<u64> {
        Ok(parse_byte_offset(s, block_size)?
            .assume_forward_offset_from_start()?
            .into())
    };

    let mut reader = if let Some(ref length) = opt.length {
        let length = parse_byte_count(length).context(anyhow!(
            "failed to parse `--length` arg {:?} as byte count",
            length
        ))?;
        Box::new(reader.take(length))
    } else {
        reader.into_inner()
    };

    let no_color = std::env::var_os("NO_COLOR").is_some();
    let show_color = match opt.color {
        ColorWhen::Never => false,
        ColorWhen::Always => !no_color,
        ColorWhen::Force => true,
        ColorWhen::Auto => {
            if no_color {
                false
            } else {
                supports_color::on(supports_color::Stream::Stdout)
                    .map(|level| level.has_basic)
                    .unwrap_or(false)
            }
        }
    };

    let border_style = opt.border;

    let &squeeze = &!opt.no_squeezing;

    let show_char_panel = !opt.no_characters && !opt.plain;

    let show_position_panel = !opt.no_position && !opt.plain;

    let display_offset: u64 = parse_byte_count(&opt.display_offset).context(anyhow!(
        "failed to parse `--display-offset` arg {:?} as byte count",
        opt.display_offset
    ))?;

    let max_panels_fn = |terminal_width: u64, base_digits: u64, group_size: u64| {
        let offset = if show_position_panel { 10 } else { 1 };
        let col_width = if show_char_panel {
            ((8 / group_size) * (base_digits * group_size + 1)) + 2 + 8
        } else {
            ((8 / group_size) * (base_digits * group_size + 1)) + 2
        };
        if (terminal_width.saturating_sub(offset)) / col_width < 1 {
            1
        } else {
            (terminal_width - offset) / col_width
        }
    };

    let base = if let Ok(base_num) = opt.base.parse::<u8>() {
        match base_num {
            2 => Ok(Base::Binary),
            8 => Ok(Base::Octal),
            10 => Ok(Base::Decimal),
            16 => Ok(Base::Hexadecimal),
            _ => Err(anyhow!(
                "The number provided is not a valid base. Valid bases are 2, 8, 10, and 16."
            )),
        }
    } else {
        match opt.base.as_str() {
            "b" | "bin" | "binary" => Ok(Base::Binary),
            "o" | "oct" | "octal" => Ok(Base::Octal),
            "d" | "dec" | "decimal" => Ok(Base::Decimal),
            "x" | "hex" | "hexadecimal" => Ok(Base::Hexadecimal),
            _ => Err(anyhow!(
                "The base provided is not valid. Valid bases are \"b\", \"o\", \"d\", and \"x\"."
            )),
        }
    }?;

    let base_digits = match base {
        Base::Binary => 8,
        Base::Octal => 3,
        Base::Decimal => 3,
        Base::Hexadecimal => 2,
    };

    let group_size = u8::from(opt.group_size);

    let terminal_width = terminal_size().map(|s| s.0 .0 as u64).unwrap_or(80);

    let panels = if opt.panels.as_deref() == Some("auto") {
        max_panels_fn(terminal_width, base_digits, group_size.into())
    } else if let Some(panels) = opt.panels {
        panels
            .parse::<NonZeroU64>()
            .map(u64::from)
            .context(anyhow!(
                "failed to parse `--panels` arg {:?} as unsigned nonzero integer",
                panels
            ))?
    } else if let Some(terminal_width) = opt.terminal_width {
        max_panels_fn(terminal_width.into(), base_digits, group_size.into())
    } else {
        std::cmp::min(
            2,
            max_panels_fn(terminal_width, base_digits, group_size.into()),
        )
    };

    let endianness = if opt.little_endian_format {
        Endianness::Little
    } else {
        opt.endianness
    };

    let character_table = opt.character_table;

    let color_scheme = opt.color_scheme;

    let mut stdout = BufWriter::new(io::stdout().lock());

    let include_mode = match opt.include_mode {
        // include mode on
        true => {
            if let Some(include_file) = opt.file {
                // input from a file
                if include_file.as_os_str() == "-" {
                    IncludeMode::File("stdin".to_string())
                } else {
                    IncludeMode::File(
                        include_file
                            .file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("file")
                            .to_string(),
                    )
                }
            } else {
                // input from stdin
                IncludeMode::Stdin
            }
        }
        // include mode off
        false => IncludeMode::Off,
    };

    let mut printer = PrinterBuilder::new(&mut stdout)
        .show_color(show_color)
        .show_char_panel(show_char_panel)
        .show_position_panel(show_position_panel)
        .with_border_style(border_style)
        .enable_squeezing(squeeze)
        .num_panels(panels)
        .group_size(group_size)
        .with_base(base)
        .endianness(endianness)
        .character_table(character_table)
        .include_mode(include_mode)
        .color_scheme(color_scheme)
        .build();
    printer.display_offset(skip_offset + display_offset);
    printer.print_all(&mut reader).map_err(|e| anyhow!(e))?;

    Ok(())
}

fn main() {
    let result = run();

    if let Err(err) = result {
        if let Some(io_error) = err.downcast_ref::<io::Error>() {
            if io_error.kind() == ::std::io::ErrorKind::BrokenPipe {
                std::process::exit(0);
            }
        }
        eprintln!("Error: {err:?}");
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

impl From<NonNegativeI64> for u64 {
    fn from(x: NonNegativeI64) -> u64 {
        u64::try_from(x.0)
            .expect("invariant broken: NonNegativeI64 should contain a non-negative i64 value")
    }
}

fn print_color_table() -> io::Result<()> {
    let mut stdout = BufWriter::new(io::stdout().lock());

    writeln!(stdout, "hexyl color reference:\n")?;

    // NULL bytes
    stdout.write_all(COLOR_NULL.as_bytes())?;
    writeln!(stdout, "⋄ NULL bytes (0x00)")?;
    stdout.write_all(COLOR_RESET.as_bytes())?;

    // ASCII printable
    stdout.write_all(COLOR_ASCII_PRINTABLE.as_bytes())?;
    writeln!(stdout, "a ASCII printable characters (0x20 - 0x7E)")?;
    stdout.write_all(COLOR_RESET.as_bytes())?;

    // ASCII whitespace
    stdout.write_all(COLOR_ASCII_WHITESPACE.as_bytes())?;
    writeln!(stdout, "_ ASCII whitespace (0x09 - 0x0D, 0x20)")?;
    stdout.write_all(COLOR_RESET.as_bytes())?;

    // ASCII other
    stdout.write_all(COLOR_ASCII_OTHER.as_bytes())?;
    writeln!(
        stdout,
        "• ASCII control characters (except NULL and whitespace)"
    )?;
    stdout.write_all(COLOR_RESET.as_bytes())?;

    // Non-ASCII
    stdout.write_all(COLOR_NONASCII.as_bytes())?;
    writeln!(stdout, "× Non-ASCII bytes (0x80 - 0xFF)")?;
    stdout.write_all(COLOR_RESET.as_bytes())?;

    Ok(())
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

impl From<PositiveI64> for u64 {
    fn from(x: PositiveI64) -> u64 {
        u64::try_from(x.0)
            .expect("invariant broken: PositiveI64 should contain a positive i64 value")
    }
}

#[derive(Debug, PartialEq)]
enum Unit {
    Byte,
    Kilobyte,
    Megabyte,
    Gigabyte,
    Terabyte,
    Kibibyte,
    Mebibyte,
    Gibibyte,
    Tebibyte,
    /// a customizable amount of bytes
    Block {
        custom_size: Option<NonZeroI64>,
    },
}

impl Unit {
    const fn get_multiplier(self) -> i64 {
        match self {
            Self::Byte => 1,
            Self::Kilobyte => 1000,
            Self::Megabyte => 1_000_000,
            Self::Gigabyte => 1_000_000_000,
            Self::Terabyte => 1_000_000_000_000,
            Self::Kibibyte => 1 << 10,
            Self::Mebibyte => 1 << 20,
            Self::Gibibyte => 1 << 30,
            Self::Tebibyte => 1 << 40,
            Self::Block {
                custom_size: Some(size),
            } => size.get(),
            Self::Block { custom_size: None } => DEFAULT_BLOCK_SIZE,
        }
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
    #[error("count multiplied by the unit overflowed a signed 64-bit integer; are you sure it should be that big?")]
    UnitMultiplicationOverflow,
}

fn parse_byte_offset(n: &str, block_size: PositiveI64) -> Result<ByteOffset, ByteOffsetParseError> {
    use ByteOffsetParseError::*;

    let (n, kind) = process_sign_of(n)?;

    let into_byte_offset = |value| {
        Ok(ByteOffset {
            value: NonNegativeI64::new(value).unwrap(),
            kind,
        })
    };

    if let Some(hex_number) = try_parse_as_hex_number(n) {
        return hex_number.map(into_byte_offset)?;
    }

    let (num, mut unit) = extract_num_and_unit_from(n)?;
    if let Unit::Block { custom_size: None } = unit {
        unit = Unit::Block {
            custom_size: Some(
                NonZeroI64::new(block_size.into_inner()).expect("PositiveI64 was zero"),
            ),
        };
    }

    num.checked_mul(unit.get_multiplier())
        .ok_or(UnitMultiplicationOverflow)
        .and_then(into_byte_offset)
}

/// Takes a string containing a base-10 number and an optional unit, and returns them with their proper types.
/// The unit must directly follow the number (e.g. no whitespace is allowed between them).
/// When no unit is given, [Unit::Byte] is assumed.
/// When the unit is [Unit::Block], it is returned without custom size.
/// No normalization is performed, that is "1024" is extracted to (1024, Byte), not (1, Kibibyte).
fn extract_num_and_unit_from(n: &str) -> Result<(i64, Unit), ByteOffsetParseError> {
    use ByteOffsetParseError::*;
    if n.is_empty() {
        return Err(Empty);
    }
    match n.chars().position(|c| !c.is_ascii_digit()) {
        Some(unit_begin_idx) => {
            let (n, raw_unit) = n.split_at(unit_begin_idx);
            let unit = match raw_unit.to_lowercase().as_str() {
                "" => Unit::Byte, // no "b" => Byte to allow hex nums with units
                "kb" => Unit::Kilobyte,
                "mb" => Unit::Megabyte,
                "gb" => Unit::Gigabyte,
                "tb" => Unit::Terabyte,
                "kib" => Unit::Kibibyte,
                "mib" => Unit::Mebibyte,
                "gib" => Unit::Gibibyte,
                "tib" => Unit::Tebibyte,
                "block" | "blocks" => Unit::Block { custom_size: None },
                _ => {
                    return if n.is_empty() {
                        Err(InvalidNumAndUnit(raw_unit.to_string()))
                    } else {
                        Err(InvalidUnit(raw_unit.to_string()))
                    }
                }
            };
            let num = n.parse::<i64>().map_err(|e| {
                if n.is_empty() {
                    EmptyWithUnit(raw_unit.to_owned())
                } else {
                    ParseNum(e)
                }
            })?;
            Ok((num, unit))
        }
        None => {
            // no unit part
            let num = n.parse::<i64>().map_err(ParseNum)?;
            Ok((num, Unit::Byte))
        }
    }
}

/// Extracts a [ByteOffsetKind] based on the sign at the beginning of the given string.
/// Returns the input string without the sign (or an equal string if there wasn't any sign).
fn process_sign_of(n: &str) -> Result<(&str, ByteOffsetKind), ByteOffsetParseError> {
    use ByteOffsetParseError::*;
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
        Some('+') => Ok((
            check_empty_after_sign()?,
            ByteOffsetKind::ForwardFromLastOffset,
        )),
        Some('-') => Ok((check_empty_after_sign()?, ByteOffsetKind::BackwardFromEnd)),
        None => Err(Empty),
        _ => Ok((n, ByteOffsetKind::ForwardFromBeginning)),
    }
}

/// If `n` starts with a hex prefix, its remaining part is returned as some number (if possible),
/// otherwise None is returned.
fn try_parse_as_hex_number(n: &str) -> Option<Result<i64, ByteOffsetParseError>> {
    use ByteOffsetParseError::*;
    n.strip_prefix(HEX_PREFIX).map(|num| {
        let mut chars = num.chars();
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
        i64::from_str_radix(num, 16).map_err(ParseNum)
    })
}
