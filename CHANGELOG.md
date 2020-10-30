# unreleased

## Features

- Allow relative and negative byte offsets (e.g. `hexyl --skip=-1block`), see #99 (@ErichDonGubler)

## Bugfixes

- Argument `--length` silently takes precedence over `--bytes`, see #105
- Print warning on empty content, see #107 and #108

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

- Added support for squeezing where reoccuring lines are squashed together and visualized with an asterisk. A new `-v`/`--no-squeezing` option can be used to disable the feature. For details, see #59 (@awidegreen)
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
