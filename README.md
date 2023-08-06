![](doc/logo.svg)

[![CICD](https://github.com/sharkdp/hexyl/actions/workflows/CICD.yml/badge.svg)](https://github.com/sharkdp/hexyl/actions/workflows/CICD.yml)
[![](https://img.shields.io/crates/l/hexyl.svg?colorB=22ba4c)](https://crates.io/crates/hexyl)
![](https://img.shields.io/crates/v/hexyl.svg?colorB=00aa88)

`hexyl` is a simple hex viewer for the terminal. It uses a colored output to distinguish different categories
of bytes (NULL bytes, printable ASCII characters, ASCII whitespace characters, other ASCII characters and non-ASCII).

## Preview

![](https://i.imgur.com/MWO9uSL.png)

![](https://i.imgur.com/Dp7Wncz.png)

![](https://i.imgur.com/ln3TniI.png)

![](https://i.imgur.com/f8nm8g6.png)

## Color Reference

|Type of Byte|Color|ANSI Code |
|---|---|---|
|NULL|![#555753](https://placehold.co/10x10/555753/555753.png) Bright Black|90|
|OFFSET|![#555753](https://placehold.co/10x10/555753/555753.png) Bright Black|90|
|ASCII Printable|![#06989a](https://placehold.co/10x10/06989a/06989a.png) Cyan|36|
|ASCII Whitespace|![#4e9a06](https://placehold.co/10x10/4e9a06/4e9a06.png) Green|32|
|ASCII Other|![#4e9a06](https://placehold.co/10x10/4e9a06/4e9a06.png) Green|32|
|Non-ASCII|![#c4a000](https://placehold.co/10x10/c4a000/c4a000.png) Yellow|33|

*Colors taken from the Ubuntu terminal color scheme, they could look different in your terminal*


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
sudo dpkg -i hexyl_0.13.1_amd64.deb  # adapt version number and architecture
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

You can install `hexyl` from [the official package repository](https://www.archlinux.org/packages/community/x86_64/hexyl/):

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

### On OpenBSD

```
doas pkg_add install hexyl
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


## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.
