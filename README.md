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

### On Debian-based systems

``` bash
wget "https://github.com/sharkdp/hexyl/releases/download/v0.3.0/hexyl_0.3.0_amd64.deb"
sudo dpkg -i hexyl_0.3.0_amd64.deb
```

### On Arch Linux

You can install `hexyl` from [this AUR package](https://aur.archlinux.org/packages/hexyl/):

```
yaourt -S hexyl
```


### On other distributions

Check out the [release page](https://github.com/sharkdp/hexyl/releases) for binary builds.

### Via cargo

If you have Rust 1.29 or higher, you can install `hexyl` from source via `cargo`:
```
cargo install hexyl
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
