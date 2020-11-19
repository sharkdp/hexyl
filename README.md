![](doc/logo.svg)

[![Build Status](https://travis-ci.org/sharkdp/hexyl.svg?branch=master)](https://travis-ci.org/sharkdp/hexyl)
[![](https://img.shields.io/crates/l/hexyl.svg?colorB=22ba4c)](https://crates.io/crates/hexyl)
![](https://img.shields.io/crates/v/hexyl.svg?colorB=00aa88)

`hexyl` is a simple hex viewer for the terminal. It uses a colored output to distinguish different categories
of bytes (NULL bytes, printable ASCII characters, ASCII whitespace characters, other ASCII characters and non-ASCII).

## Preview

![](https://i.imgur.com/MWO9uSL.png)

![](https://i.imgur.com/Dp7Wncz.png)

![](https://i.imgur.com/ln3TniI.png)

![](https://i.imgur.com/f8nm8g6.png)

## Installation

### On Ubuntu

*... and other Debian-based Linux distributions.*

If you run Ubuntu 19.10 (Eoan Ermine) or newer, you can install the [officially maintained package](https://packages.ubuntu.com/eoan/hexyl):
```bash
sudo apt install hexyl
```
If you use an older version of Ubuntu, you can download
the latest `.deb` package from the release page and install it via:

``` bash
sudo dpkg -i hexyl_0.8.0_amd64.deb  # adapt version number and architecture
```

### On Debian

If you run Debian Buster or newer, you can install the [officially maintained Debian package](https://packages.debian.org/buster/hexyl):
```bash
sudo apt-get install hexyl
```

If you run an older version of Debian, see above for instructions on how to
manually install `hexyl`.

### On Arch Linux

You can install `hexyl` from [the official package repository](https://www.archlinux.org/packages/community/x86_64/hexyl/):

```
pacman -S hexyl
```

### On Void Linux
```
xbps-install hexyl
```

### On macOS

```
brew install hexyl
```

### On FreeBSD

```
pkg install hexyl
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

For now, you will have to install from source via `cargo` (see below). Make sure that you
use a terminal that supports ANSI escape sequences (like ConHost v2 since Windows 10 1703
or Windows Terminal since Windows 10 1903).

### Via cargo

If you have Rust 1.39 or higher, you can install `hexyl` from source via `cargo`:
```
cargo install hexyl
```

### Via snap package

```
sudo snap install hexyl
```

[Get it from the Snap Store](https://snapcraft.io/hexyl)

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
