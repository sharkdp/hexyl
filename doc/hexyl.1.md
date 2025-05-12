% HEXYL(1) hexyl 0.12.0 | General Commands Manual
%
% 2022-12-05

# NAME

hexyl - a command-line hex viewer

# SYNOPSIS

**hexyl** [_OPTIONS_] [_FILE_]

# DESCRIPTION

**hexyl** is a simple hex viewer for the terminal.
It uses a colored output to distinguish different categories of bytes (NULL
bytes, printable ASCII characters, ASCII whitespace characters, other ASCII
characters and non-ASCII).

# POSITIONAL ARGUMENTS

_FILE_
:   The file to display.
    If no _FILE_ argument is given, read from STDIN.

# OPTIONS

**-n**, **\--length** _N_
:   Only read _N_ bytes from the input.
    The _N_ argument can also include a unit with a decimal prefix (kB, MB, ..)
    or binary prefix (kiB, MiB, ..), or can be specified using a hex number.

    Examples:

    :   

        Read the first 64 bytes:
        :   $ **hexyl \--length=64**

        Read the first 4 kibibytes:
        :   $ **hexyl \--length=4KiB**

        Read the first 255 bytes (specified using a hex number):
        :   $ **hexyl \--length=0xff**

**-c**, **\--bytes** _N_
:   An alias for **-n**/**\--length**.

**-l** _N_
:   Yet another alias for **-n**/**\--length**.

**-s**, **\--skip** _N_
:   Skip the first _N_ bytes of the input.
    The _N_ argument can also include a unit (see **\--length** for details).
    A negative value is valid and will seek from the end of the file.

**\--block-size** _SIZE_
:   Sets the size of the block unit to _SIZE_ (default is 512).

    Examples:

    :   

        Sets the block size to 1024 bytes:
        :   $ **hexyl \--block-size=1024 \--length=5block**

        Sets the block size to 4 kilobytes:
        :   $ **hexyl \--block-size=4kB \--length=2block**

**-v**, **\--no-squeezing**
:   Displays all input data.
    Otherwise any number of groups of output lines which would be identical to
    the preceding group of lines, are replaced with a line comprised of a
    single asterisk.

**\--color** _WHEN_
:   When to use colors.
    The auto-mode only displays colors if the output goes to an interactive
    terminal.

    Possible values:

    :   - **always** (default)
        - **auto**
        - **never**

**\--border** _STYLE_
:   Whether to draw a border with Unicode characters, ASCII characters, or none
    at all.

    Possible values:

    :   - **unicode** (default)
        - **ascii**
        - **none**

**-o**, **\--display-offset** _N_
:   Add _N_ bytes to the displayed file position.
    The _N_ argument can also include a unit (see **\--length** for details).
    A negative value is valid and calculates an offset relative to the end of
    the file.

**-h**, **\--help**
:   Prints help information.

**-V**, **\--version**
:   Prints version information.

# ENVIRONMENT VARIABLES

**hexyl** colors can be configured via environment variables. The variables used are as follows:

:   - **HEXYL_ASCII_PRINTABLE**: Any non-whitespace printable ASCII character
    - **HEXYL_ASCII_WHITESPACE**: Whitespace such as space or newline (only visible in middle panel with byte values)
    - **HEXYL_ASCII_OTHER**: Any other ASCII character (< **0x80**) besides null
    - **HEXYL_NULL**: The null byte (**0x00**)
    - **HEXYL_NONASCII**: Any non-ASCII byte (> **0x7F**)
    - **HEXYL_OFFSET**: The lefthand file offset

The colors can be any of the 8 standard terminal colors: **black**, **blue**, **cyan**, **green**, **magenta**, **red**,
**yellow** and **white**. The "bright" variants are also supported (e.g., **bright blue**). Additionally, you can use
the RGB hex format, **#abcdef**. For example, **HEXYL_ASCII_PRINTABLE=blue HEXYL_ASCII_WHITESPACE="bright green" HEXYL_ASCII_OTHER="#ff7f99"**.

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
