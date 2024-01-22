use clap::builder::ArgPredicate;
use clap::{crate_name, crate_version, Arg, ArgAction, ColorChoice, Command};

use const_format::formatcp;

pub const DEFAULT_BLOCK_SIZE: i64 = 512;

pub fn build_cli() -> Command {
    Command::new(crate_name!())
        .color(ColorChoice::Auto)
    .max_term_width(90)
    .version(crate_version!())
    .about(crate_description!())
    .arg(
        Arg::new("FILE")
            .help("The file to display. If no FILE argument is given, read from STDIN."),
    )
    .arg(
        Arg::new("length")
            .short('n')
            .long("length")
            .num_args(1)
            .value_name("N")
            .help(
                "Only read N bytes from the input. The N argument can also include a \
                 unit with a decimal prefix (kB, MB, ..) or binary prefix (kiB, MiB, ..), \
                 or can be specified using a hex number. \
                 The short option '-l' can be used as an alias.\n\
                 Examples: --length=64, --length=4KiB, --length=0xff",
            ),
    )
    .arg(
        Arg::new("bytes")
            .short('c')
            .long("bytes")
            .num_args(1)
            .value_name("N")
            .conflicts_with("length")
            .help("An alias for -n/--length"),
    )
    .arg(
        Arg::new("count")
            .short('l')
            .num_args(1)
            .value_name("N")
            .conflicts_with_all(["length", "bytes"])
            .hide(true)
            .help("Yet another alias for -n/--length"),
    )
    .arg(
        Arg::new("skip")
            .short('s')
            .long("skip")
            .num_args(1)
            .value_name("N")
            .help(
                "Skip the first N bytes of the input. The N argument can also include \
                 a unit (see `--length` for details)\n\
                 A negative value is valid and will seek from the end of the file.",
            ),
    )
    .arg(
        Arg::new("block_size")
            .long("block-size")
            .num_args(1)
            .value_name("SIZE")
            .help(formatcp!(
                "Sets the size of the `block` unit to SIZE (default is {}).\n\
                 Examples: --block-size=1024, --block-size=4kB",
                DEFAULT_BLOCK_SIZE
            )),
    )
    .arg(
        Arg::new("nosqueezing")
            .short('v')
            .long("no-squeezing")
            .action(ArgAction::SetFalse)
            .help(
                "Displays all input data. Otherwise any number of groups of output \
                 lines which would be identical to the preceding group of lines, are \
                 replaced with a line comprised of a single asterisk.",
            ),
    )
    .arg(
        Arg::new("color")
            .long("color")
            .num_args(1)
            .value_name("WHEN")
            .value_parser(["always", "auto", "never", "force"])
            .default_value_if("plain", ArgPredicate::IsPresent, Some("never"))
            .default_value("always")
            .help(
                "When to use colors. The 'auto' mode only displays colors if the output \
                 goes to an interactive terminal. 'force' can be used to override the \
                 NO_COLOR environment variable.",
            ),
    )
    .arg(
        Arg::new("border")
            .long("border")
            .num_args(1)
            .value_name("STYLE")
            .value_parser(["unicode", "ascii", "none"])
            .default_value_if("plain", ArgPredicate::IsPresent, Some("none"))
            .default_value("unicode")
            .help(
                "Whether to draw a border with Unicode characters, ASCII characters, \
                or none at all",
            ),
    )
    .arg(Arg::new("plain").short('p').long("plain").action(ArgAction::SetTrue).help(
        "Display output with --no-characters, --no-position, --border=none, and --color=never.",
    ))
    .arg(
        Arg::new("no_chars")
            .long("no-characters")
            .action(ArgAction::SetFalse)
            .help("Do not show the character panel on the right."),
    )
    .arg(
        Arg::new("chars")
            .short('C')
            .long("characters")
            .overrides_with("no_chars")
            .action(ArgAction::SetTrue)
            .help("Show the character panel on the right. This is the default, unless --no-characters has been specified."),
    )
    .arg(
        Arg::new("character-table")
            .long("character-table")
            .value_name("FORMAT")
            .value_parser(["default", "ascii", "codepage-437"])
            .default_value("default")
            .help(
                "Defines how bytes are mapped to characters:\n  \
                \"default\": show printable ASCII characters as-is, '⋄' for NULL bytes, \
                ' ' for space, '_' for other ASCII whitespace, \
                '•' for other ASCII characters, and '×' for non-ASCII bytes.\n  \
                \"ascii\": show printable ASCII as-is, ' ' for space, '.' for everything else.\n  \
                \"codepage-437\": uses code page 437 (for non-ASCII bytes).\n"
            ),
    )
    .arg(
        Arg::new("no_position")
            .short('P')
            .long("no-position")
            .action(ArgAction::SetFalse)
            .help("Whether to display the position panel on the left."),
    )
    .arg(
        Arg::new("display_offset")
            .short('o')
            .long("display-offset")
            .num_args(1)
            .value_name("N")
            .help(
                "Add N bytes to the displayed file position. The N argument can also \
                include a unit (see `--length` for details)\n\
                A negative value is valid and calculates an offset relative to the \
                end of the file.",
            ),
    )
    .arg(
        Arg::new("panels")
            .long("panels")
            .num_args(1)
            .value_name("N")
            .help(
                "Sets the number of hex data panels to be displayed. \
                `--panels=auto` will display the maximum number of hex data panels \
                based on the current terminal width. By default, hexyl will show \
                two panels, unless the terminal is not wide enough for that.",
            ),
    )
    .arg(
        Arg::new("group_size")
            .short('g')
            .long("group-size")
            .alias("groupsize")
            .num_args(1)
            .value_name("N")
            .help(
                "Number of bytes/octets that should be grouped together. \
                Possible group sizes are 1, 2, 4, 8. The default is 1. You \
                can use the '--endianness' option to control the ordering of \
                the bytes within a group. '--groupsize' can be used as an \
                alias (xxd-compatibility).",
            ),
    )
    .arg(
        Arg::new("endianness")
            .long("endianness")
            .num_args(1)
            .value_name("FORMAT")
            .value_parser(["big", "little"])
            .default_value("big")
            .help(
                "Whether to print out groups in little-endian or big-endian \
                 format. This option only has an effect if the '--group-size' \
                 is larger than 1. '-e' can be used as an alias for \
                 '--endianness=little'.",
            ),
    )
    .arg(
        Arg::new("little_endian_format")
            .short('e')
            .action(ArgAction::SetTrue)
            .overrides_with("endianness")
            .hide(true)
            .help("An alias for '--endianness=little'."),
    )
    .arg(
        Arg::new("base")
            .short('b')
            .long("base")
            .num_args(1)
            .value_name("B")
            .help(
                "Sets the base used for the bytes. The possible options are \
                binary, octal, decimal, and hexadecimal. The default base \
                is hexadecimal."
            )
    )
    .arg(
        Arg::new("terminal_width")
            .long("terminal-width")
            .num_args(1)
            .value_name("N")
            .conflicts_with("panels")
            .help(
                "Sets the number of terminal columns to be displayed.\nSince the terminal \
                width may not be an evenly divisible by the width per hex data column, this \
                will use the greatest number of hex data panels that can fit in the requested \
                width but still leave some space to the right.\nCannot be used with other \
                width-setting options.",
            ),
    )
}
