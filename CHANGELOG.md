# v0.13.0

## Features

- Support both little and big Endian dumps using `--endianness={little,big}`, see #189 and #104 (@RinHizakura)

## Changes

- **Breaking**: Changed the meaning of the short flag `-C` to be consistent with `hexdump -C`. Previously, this would *hide* the character panel, but now `-C` *shows* the character panel, in case it has been previously (e.g. in an `alias`) disabled with `--no-characters`, see #187 (@sharkdp)

## `hexyl` as a library

- New `endianness` method for `PrinterBuilder`


# v0.12.0

## Features

- Only show one panel by default if the terminal width is not wide enough for two panels, see #182 (@sharkdp)
- Respect the `NO_COLOR` environment variable, see #179 (@sharifhsn)

## Bugfixes

- Do not fail with an error if `--panels=auto` is used and the output is piped, see #184 (@sharkdp)

## Changes

- Breaking: For `xxd`-compatibility reasons, `--group_bytes` has been renamed to `--group-size` (with an `--groupsize` alias), see #121 (@sharkdp)

## `hexyl` as a library

- Breaking: `num_group_bytes` has been renamed to `group_size`.


# v0.11.0

## Features

- Significantly improved performance, see #173 and #176 (@sharifhsn)
- Added variable panels through the `--panels` and `--terminal-width` flags, see [#13](https://github.com/sharkdp/hexyl/issues/13) and [#164](https://github.com/sharkdp/hexyl/pull/164) (@sharifhsn)
- Added new `--group-bytes`/`-g` option, see #104 and #170 (@RinHizakura)
- Added new `--base B` option (where `B` can be `binary`, `octal`, `decimal` or `hexadecimal`), see #147 and #178 (@sharifhsn)
- Show actual zero bytes as `⋄` in the character panel (previously: `0`), in order not to confuse them with ASCII
  `0` bytes if colors are deactivated. Closes #166 (@sharkdp)

## `hexyl` as a library

- Breaking change: `Printer::new` is deprecated as a part of the public API. Alternatively, you can now construct a `Printer` using the `PrinterBuilder` builder API, see [#168](https://github.com/sharkdp/hexyl/pull/168). (@sharifhsn)

## Other

- More tests for the squeezing feature, see #177 (@mkatychev)

## Thank you

Special thanks go to @sharifhsn, not just for the new features,
bugfixes and performance improvements. But also for many internal
improvements of the code base and other maintenance tasks.


# v0.10.0

## Features

- Added new `--plain`, `--no-characters`, and `--no-position` flags, see #154 (@mkatychev)
- Allow hex numbers and units for `--block-size` argument, see #111 and #144 (@merkrafter)

## Other

- Added a man page, see #151 (@sorairolake)
- Mention ability to specify length in hex, see #143 (@merkrafter)
- `--length` and `--bytes` are now marked as conflicting command-line options, see #152 (@sorairolake)


# v0.9.0

## Changes

- Breaking change (binary): setting the `-o/--display-offset` flag no longer overrides the value set by `--skip` [#115](https://github.com/sharkdp/hexyl/issues/115). The first displayed address is now the sum of the two values - this matches the behaviour of `xxd`.

## Features

- Allow relative and negative byte offsets (e.g. `hexyl --skip=-1block`), see #99 (@ErichDonGubler)
- Added `-l` as another alias for '-n/--length' (`xxd` compatibility), see #121 and #135 (@TheDoctor314)

## Bugfixes

- Argument `--length` silently takes precedence over `--bytes`, see #105
- Print warning on empty content, see #107 and #108
- Disallow block sizes of zero, see #110
- Fix newline appearing in `--version` output, see #131 and #133 (@scimas)

## Other

- Better diagnostic messages, see #98 (@ErichDonGubler)

## Packaging

- `hexyl` is now available on snapstore, see #116 (@purveshpatel511)


# v0.8.0

## Features

- A new `--skip <N>` / `-s <N>` option can be used to skip the first `N` bytes of the input, see #16, #88 (@Tarnadas, @MaxJohansen, @ErichDonGubler)
- The `--length`/`--bytes`/`--skip`/`--display-offset` options can now take units for their value argument, for example:
  ``` bash
  hexyl /dev/random --length=1KiB
  hexyl $(which hexyl) --skip=1MiB --length=10KiB
  ```
  Both decimal SI prefixes (kB, MB, …) as well as binary IEC prefixes (KiB, MiB, …) are supported.
  In addition, there is a new `--block-size <SIZE>` option that can be used to control the size of the `block`
  unit:
  ``` bash
  hexyl /dev/random --block-size=4kB --length=2block
  ```
  See: #44 (@ErichDonGubler and @aswild)

## Other

- Various improvements throughout the code base by @ErichDonGubler

## Packaging

- `hexyl` is now available on Void Linux, see #91 (@notramo)


# v0.7.0

## Bugfixes

- hexyl can now be closed with `Ctrl-C` when reading input from STDIN, see #84

## Changes

- Breaking change (library): [`Printer::print_all`](https://docs.rs/hexyl/latest/hexyl/struct.Printer.html#method.print_all) does not take a second argument anymore.
- Added an example on how to use `hexyl` as a library: https://github.com/sharkdp/hexyl/blob/v0.7.0/examples/simple.rs


# v0.6.0

## Features

- `hexyl` can now be used as a library, see #67 (@tommilligan)

- Added a new `-o`/`--display-offset` option to add a certain offset to the
  reported file positions, see #57 (@tommilligan)

## Bugfixes

- Remove additional space on short input, see #69 (@nalshihabi)

## Other

- Performance improvements, see #73 and #66


# v0.5.1

## Bugfixes

- A bug in the squeezing logic caused a wrong hexdump, see #62 (@awidegreen)
- Some colors are printed even if they're disabled, see #64 (@awidegreen)
- Fixed build failure on OpenBSD 6.5, see #61


# v0.5.0

## Features

- Added support for squeezing where reoccurring lines are squashed together and visualized with an asterisk. A new `-v`/`--no-squeezing` option can be used to disable the feature. For details, see #59 (@awidegreen)
- Added a new `--border` option with support for various styles (Unicode, ASCII, None), see #54 (@dmke)
- The `--length`/`-n` argument can be passed as a hexadecimal number (`hexyl -n 0xff /dev/urandom`), see #45 (@Qyriad)
- Added `--bytes`/`-c` as an alias for `--length`/`-n`, see #48 (@selfup)

## Changes

- Print header immediately before the first line, see #51 (@mziter)


# v0.4.0

## Features

- Added a new `--color=always/auto/never` option which can be used
  to control `hexyl`s color output, see #30 (@bennetthardwick)
- Use 16 colors instead of 256, see #38

## Changes

- Various speed improvements, see #33 (@kballard)

## Bugfixes

- Proper Ctrl-C handling, see #35
- Proper handling of broken pipes (`hexyl … | head`)


# v0.3.1

- Various (huge) performance improvements, see #23 and #24 (@kballard)
- Replaced 24-bit truecolor ANSI codes by 8-bit codes to support
  more terminal emulators, fixes #9


# v0.3.0

Windows support


# v0.2.0

Initial release
