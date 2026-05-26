complete -c hexyl -s n -s c -l length -l bytes -d 'Only read N bytes from the input. The N argument can also include a unit with a decimal prefix (kB, MB, ..) or binary prefix (kiB, MiB, ..), or can be specified using a hex number. The short option \'-l\' can be used as an alias. Examples: --length=64, --length=4KiB, --length=0xff' -r
complete -c hexyl -s s -l skip -d 'Skip the first N bytes of the input. The N argument can also include a unit (see `--length` for details). A negative value is valid and will seek from the end of the file.' -r
complete -c hexyl -l block-size -d 'Sets the size of the `block` unit to SIZE. Examples: --block-size=1024, --block-size=4kB' -r
complete -c hexyl -l color -d 'When to use colors' -r -f -a "always\t'Always use colorized output'
auto\t'Only displays colors if the output goes to an interactive terminal'
never\t'Do not use colorized output'
force\t'Override the NO_COLOR environment variable'"
complete -c hexyl -l border -d 'Whether to draw a border' -r -f -a "unicode\t'Draw a border with Unicode characters'
ascii\t'Draw a border with ASCII characters'
none\t'Do not draw a border at all'"
complete -c hexyl -l character-table -d 'Defines how bytes are mapped to characters' -r -f -a "default\t'Show printable ASCII characters as-is, \'⋄\' for NULL bytes, \' \' for space, \'_\' for other ASCII whitespace, \'•\' for other ASCII characters, and \'×\' for non-ASCII bytes'
ascii\t'Show printable ASCII as-is, \' \' for space, \'.\' for everything else'
codepage-1047\t'Show printable EBCDIC as-is, \' \' for space, \'.\' for everything else'
codepage-437\t'Uses code page 437 (for non-ASCII bytes)'
braille\t'Uses braille characters for non-printable bytes'"
complete -c hexyl -l color-scheme -d 'Defines the color scheme for the characters' -r -f -a "default\t'Show the default colors: bright black for NULL bytes, green for ASCII space characters and non-printable ASCII, cyan for printable ASCII characters, and yellow for non-ASCII bytes'
gradient\t'Show bright black for NULL bytes, cyan for printable ASCII characters, a gradient from pink to violet for non-printable ASCII characters and a heatmap-like gradient from red to yellow to white for non-ASCII bytes'"
complete -c hexyl -s o -l display-offset -d 'Add N bytes to the displayed file position. The N argument can also include a unit (see `--length` for details). A negative value is valid and calculates an offset relative to the end of the file.' -r
complete -c hexyl -l panels -d 'Sets the number of hex data panels to be displayed. `--panels=auto` will display the maximum number of hex data panels based on the current terminal width. By default, hexyl will show two panels, unless the terminal is not wide enough for that' -r
complete -c hexyl -s g -l group-size -d 'Number of bytes/octets that should be grouped together. You can use the \'--endianness\' option to control the ordering of the bytes within a group. \'--groupsize\' can be used as an alias (xxd-compatibility)' -r -f -a "1\t'Grouped together every byte/octet'
2\t'Grouped together every 2 bytes/octets'
4\t'Grouped together every 4 bytes/octets'
8\t'Grouped together every 8 bytes/octets'"
complete -c hexyl -l endianness -d 'Whether to print out groups in little-endian or big-endian format. This option only has an effect if the \'--group-size\' is larger than 1. \'-e\' can be used as an alias for \'--endianness=little\'' -r -f -a "little\t'Print out groups in little-endian format'
big\t'Print out groups in big-endian format'"
complete -c hexyl -s b -l base -d 'Sets the base used for the bytes. The possible options are binary, octal, decimal, and hexadecimal' -r
complete -c hexyl -l terminal-width -d 'Sets the number of terminal columns to be displayed. Since the terminal width may not be an evenly divisible by the width per hex data column, this will use the greatest number of hex data panels that can fit in the requested width but still leave some space to the right. Cannot be used with other width-setting options.' -r
complete -c hexyl -l completion -d 'Show shell completion for a certain shell' -r -f -a "bash\t''
elvish\t''
fish\t''
powershell\t''
zsh\t''"
complete -c hexyl -s v -l no-squeezing -d 'Displays all input data. Otherwise any number of groups of output lines which would be identical to the preceding group of lines, are replaced with a line comprised of a single asterisk'
complete -c hexyl -s p -l plain -d 'Display output with --no-characters, --no-position, --border=none, and --color=never'
complete -c hexyl -l no-characters -d 'Do not show the character panel on the right'
complete -c hexyl -s C -l characters -d 'Show the character panel on the right. This is the default, unless --no-characters has been specified'
complete -c hexyl -s P -l no-position -d 'Whether to display the position panel on the left'
complete -c hexyl -s e -d 'An alias for \'--endianness=little\''
complete -c hexyl -l print-color-table -d 'Print a table showing how different types of bytes are colored'
complete -c hexyl -s i -l include -d 'Output in C include file style'
complete -c hexyl -s h -l help -d 'Print help (see more with \'--help\')'
complete -c hexyl -s V -l version -d 'Print version'
