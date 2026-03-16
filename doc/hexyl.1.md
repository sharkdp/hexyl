% HEXYL(1) hexyl 0.17.0 | General Commands Manual
%
% 2026-03-16

# NAME

hexyl - a command-line hex viewer

# SYNOPSIS

**hexyl** [_OPTIONS_] [_FILE_]

# DESCRIPTION

**hexyl** is a simple hex viewer for the terminal.
It uses colored output to distinguish different categories of bytes (NULL
bytes, printable ASCII characters, ASCII whitespace characters, other ASCII
characters, and non-ASCII bytes).

# POSITIONAL ARGUMENTS

_FILE_
:   The file to display.
    If no _FILE_ argument is given, read from STDIN.

# OPTIONS

**-n**, **--length** _N_
:   Only read _N_ bytes from the input.
    The _N_ argument can also include a unit with a decimal prefix (kB, MB, ..)
    or binary prefix (kiB, MiB, ..), or can be specified using a hex number.

    Examples:

    :

        Read the first 64 bytes:
        :   $ **hexyl --length=64**

        Read the first 4 kibibytes:
        :   $ **hexyl --length=4KiB**

        Read the first 255 bytes (specified using a hex number):
        :   $ **hexyl --length=0xff**

**-c**, **--bytes** _N_
:   An alias for **-n**/**--length**.

**-l** _N_
:   Yet another alias for **-n**/**--length**.

**-s**, **--skip** _N_
:   Skip the first _N_ bytes of the input.
    The _N_ argument can also include a unit (see **--length** for details).
    A negative value is valid and will seek from the end of the file.

**--block-size** _SIZE_
:   Sets the size of the **block** unit to _SIZE_ (default is 512).

    Examples:

    :

        Set the block size to 1024 bytes:
        :   $ **hexyl --block-size=1024 --length=5block**

        Set the block size to 4 kilobytes:
        :   $ **hexyl --block-size=4kB --length=2block**

**-v**, **--no-squeezing**
:   Display all input data.
    Otherwise any number of groups of output lines that would be identical to
    the preceding group of lines are replaced with a line comprised of a
    single asterisk.

**--color** _WHEN_
:   When to use colors.
    The **auto** mode only displays colors if the output goes to an interactive
    terminal.
    **force** overrides the **NO_COLOR** environment variable.

    Possible values:

    :   - **always** (default)
        - **auto**
        - **never**
        - **force**

**--border** _STYLE_
:   Whether to draw a border.

    Possible values:

    :   - **unicode** (default)
        - **ascii**
        - **none**

**-p**, **--plain**
:   Display output with **--no-characters**, **--no-position**,
    **--border=none**, and **--color=never**.

**--no-characters**
:   Do not show the character panel on the right.

**-C**, **--characters**
:   Show the character panel on the right.
    This is the default unless **--no-characters** has been specified.

**--character-table** _FORMAT_
:   Define how bytes are mapped to characters.

    Possible values:

    :   - **default** (default)
        - **ascii**
        - **codepage-1047**
        - **codepage-437**
        - **braille**

**--color-scheme** _FORMAT_
:   Define the color scheme for the characters.

    Possible values:

    :   - **default** (default)
        - **gradient**

**-P**, **--no-position**
:   Do not display the position panel on the left.

**-o**, **--display-offset** _N_
:   Add _N_ bytes to the displayed file position.
    The _N_ argument can also include a unit (see **--length** for details).
    A negative value is valid and calculates an offset relative to the end of
    the file.

**--panels** _N_
:   Set the number of hex data panels to be displayed.
    **--panels=auto** displays the maximum number of hex data panels based on
    the current terminal width.
    By default, hexyl shows two panels unless the terminal is not wide enough.

**-g**, **--group-size**, **--groupsize** _N_
:   Number of bytes/octets that should be grouped together.
    You can use **--endianness** to control the ordering of the bytes within a
    group.

    Possible values:

    :   - **1** (default)
        - **2**
        - **4**
        - **8**

**--endianness** _FORMAT_
:   Whether to print groups in little-endian or big-endian format.
    This option only has an effect if **--group-size** is larger than 1.
    **-e** can be used as an alias for **--endianness=little**.

    Possible values:

    :   - **big** (default)
        - **little**

**-e**
:   An alias for **--endianness=little**.

**-b**, **--base** _B_
:   Set the base used for the bytes.
    Possible options are binary, octal, decimal, and hexadecimal.

**--terminal-width** _N_
:   Set the number of terminal columns to be displayed.
    Since the terminal width may not be evenly divisible by the width per hex
    data column, hexyl will use the greatest number of hex data panels that can
    fit in the requested width while still leaving some space to the right.
    Cannot be used with **--panels**.

**--print-color-table**
:   Print a table showing how different types of bytes are colored.

**-i**, **--include**
:   Output in C include file style (similar to **xxd -i**).

**--completion** _SHELL_
:   Show shell completion for a certain shell.

    Possible values:

    :   - **bash**
        - **elvish**
        - **fish**
        - **powershell**
        - **zsh**

**-h**, **--help**
:   Print help information.

**-V**, **--version**
:   Print version information.

# ENVIRONMENT VARIABLES

**hexyl** colors can be configured via environment variables. The variables used are as follows:

:   - **HEXYL_COLOR_ASCII_PRINTABLE**: Any non-whitespace printable ASCII character
    - **HEXYL_COLOR_ASCII_WHITESPACE**: Whitespace such as space or newline (only visible in middle panel with byte values)
    - **HEXYL_COLOR_ASCII_OTHER**: Any other ASCII character (< **0x80**) besides null
    - **HEXYL_COLOR_NULL**: The null byte (**0x00**)
    - **HEXYL_COLOR_NONASCII**: Any non-ASCII byte (> **0x7F**)
    - **HEXYL_COLOR_OFFSET**: The left-hand file offset

The colors can be any of the 8 standard terminal colors: **black**, **blue**, **cyan**, **green**, **magenta**, **red**,
**yellow**, and **white**. The "bright" variants are also supported (for example, **bright blue**). Additionally, you can use
the RGB hex format, **#abcdef**. For example, **HEXYL_COLOR_ASCII_PRINTABLE=blue HEXYL_COLOR_ASCII_WHITESPACE="bright green"
HEXYL_COLOR_ASCII_OTHER="#ff7f99"**.

# NOTES

Source repository:
:   <https://github.com/sharkdp/hexyl>

# EXAMPLES

Print a given file:
:   $ **hexyl small.png**

Print and view a given file in the terminal pager:
:   $ **hexyl big.png | less -r**

Print the first 256 bytes of a given special file:
:   $ **hexyl -n 256 /dev/urandom**

Print output without borders, positions, characters, or colors:
:   $ **hexyl --plain small.png**

Emit C include file style output:
:   $ **hexyl --include small.png**

Generate shell completion for zsh:
:   $ **hexyl --completion zsh**

# AUTHORS

**hexyl** was written by David Peter <mail@david-peter.de>.

# REPORTING BUGS

Bugs can be reported on GitHub at:
:   <https://github.com/sharkdp/hexyl/issues>

# COPYRIGHT

**hexyl** is dual-licensed under:

:   - Apache License 2.0 (<https://www.apache.org/licenses/LICENSE-2.0>)
    - MIT License (<https://opensource.org/licenses/MIT>)

# SEE ALSO

**hexdump**(1), **xxd**(1)
