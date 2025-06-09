![](doc/logo.svg)

[![CICD](https://github.com/sharkdp/hexyl/actions/workflows/CICD.yml/badge.svg)](https://github.com/sharkdp/hexyl/actions/workflows/CICD.yml)
[![](https://img.shields.io/crates/l/hexyl.svg?colorB=22ba4c)](https://crates.io/crates/hexyl)
![](https://img.shields.io/crates/v/hexyl.svg?colorB=00aa88)

`hexyl` is a hex viewer for the terminal. It uses a colored output to distinguish different categories
of bytes (NULL bytes, printable ASCII characters, ASCII whitespace characters, other ASCII characters and non-ASCII).

### Sponsors

A special *thank you* goes to our biggest <a href="doc/sponsors.md">sponsor</a>:<br>

<a href="https://www.warp.dev/hexyl">
  <img src="doc/sponsors/warp-logo.png" width="200" alt="Warp">
  <br>
  <strong>Warp, the intelligent terminal</strong>
  <br>
  <sub>Available on MacOS, Linux, Windows</sub>
</a>

## Preview

![](https://i.imgur.com/MWO9uSL.png)

![](https://i.imgur.com/Dp7Wncz.png)

![](https://i.imgur.com/ln3TniI.png)

![](https://i.imgur.com/f8nm8g6.png)


## Installation

### On Ubuntu

*... and other Debian-based Linux distributions.*

If you run Ubuntu 19.10 (Eoan Ermine) or newer, you can install the [officially maintained package](https://packages.ubuntu.com/search?keywords=hexyl):
```bash
sudo apt install hexyl
```
If you use an older version of Ubuntu, you can download
the latest `.deb` package from the release page and install it via:

``` bash
sudo dpkg -i hexyl_0.15.0_amd64.deb  # adapt version number and architecture
```

### On Debian

If you run Debian Buster or newer, you can install the [officially maintained Debian package](https://packages.debian.org/search?searchon=names&keywords=hexyl):
```bash
sudo apt-get install hexyl
```

If you run an older version of Debian, see above for instructions on how to
manually install `hexyl`.

### On Fedora

If you run Fedora 35 or newer, you can install the [officially maintained Fedora package](https://packages.fedoraproject.org/pkgs/rust-hexyl/hexyl):

```bash
sudo dnf install hexyl
```

### On Arch Linux

You can install `hexyl` from [the official package repository](https://archlinux.org/packages/extra/x86_64/hexyl/):

```
pacman -S hexyl
```

### On Void Linux

```
xbps-install hexyl
```

### On Gentoo Linux

Available in [dm9pZCAq overlay](https://github.com/gentoo-mirror/dm9pZCAq)

```
sudo eselect repository enable dm9pZCAq
sudo emerge --sync dm9pZCAq
sudo emerge sys-apps/hexyl::dm9pZCAq
```

### On macOS

Via [Homebrew](https://brew.sh):

```
brew install hexyl
```

...or via [MacPorts](https://www.macports.org):

```
sudo port install hexyl
```

### On FreeBSD

```
pkg install hexyl
```

### On NetBSD

```
pkgin install hexyl
```

### On OpenBSD

```
doas pkg_add hexyl
```

### on Termux
```
pkg install hexyl
```
or
```
apt install hexyl
```

### Via Nix

```
nix-env -i hexyl
```

### Via Guix

```
guix package -i hexyl
```

Or add the `hexyl` package in the list of packages to be installed in your system configuration (e.g., `/etc/config.scm`).

### On other distributions

Check out the [release page](https://github.com/sharkdp/hexyl/releases) for binary builds.

### On Windows

Check out the [release page](https://github.com/sharkdp/hexyl/releases) for binary builds.
Alternatively, install from source via `cargo`, `snap` or `scoop` (see below).
Make sure that you use a terminal that supports ANSI escape sequences (like ConHost v2 since Windows 10 1703
or Windows Terminal since Windows 10 1903).

### Via cargo

If you have Rust 1.56 or higher, you can install `hexyl` from source via `cargo`:
```
cargo install hexyl
```

Alternatively, you can install `hexyl` directly from the repository by using:
```
git clone https://github.com/sharkdp/hexyl
cargo install --path ./hexyl
```

Note: To convert the man page, you will need [Pandoc](https://pandoc.org/).

You can convert from Markdown by using (in the project root):
```
pandoc -s -f markdown -t man -o ./doc/hexyl.1 ./doc/hexyl.1.md
```

### Via snap package

```
sudo snap install hexyl
```
[Get it from the Snap Store](https://snapcraft.io/hexyl)


### Via [Scoop](https://scoop.sh)
```
scoop install hexyl
```

### Via [X-CMD](https://x-cmd.com)
```
x env use hexyl
```

## Configuration

`hexyl` colors can be configured via environment variables. The variables used are as follows:

 * `HEXYL_COLOR_ASCII_PRINTABLE`: Any non-whitespace printable ASCII character
 * `HEXYL_COLOR_ASCII_WHITESPACE`: Whitespace such as space or newline (only visible in middle panel with byte values)
 * `HEXYL_COLOR_ASCII_OTHER`: Any other ASCII character (< `0x80`) besides null
 * `HEXYL_COLOR_NULL`: The null byte (`0x00`)
 * `HEXYL_COLOR_NONASCII`: Any non-ASCII byte (> `0x7F`)
 * `HEXYL_COLOR_OFFSET`: The lefthand file offset

The colors can be any of the 8 standard terminal colors: `black`, `blue`, `cyan`, `green`, `magenta`, `red`,
`yellow` and `white`. The "bright" variants are also supported (e.g., `bright blue`). Additionally, you can use
the RGB hex format, `#abcdef`. For example, `HEXYL_COLOR_ASCII_PRINTABLE=blue HEXYL_COLOR_ASCII_WHITESPACE="bright green"
HEXYL_COLOR_ASCII_OTHER="#ff7f99"`.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.
