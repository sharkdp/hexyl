> **语言**：本文档为简体中文翻译，[点此查看英文原版 README.md](README.md)。

![](doc/logo.svg)

[![CICD](https://github.com/sharkdp/hexyl/actions/workflows/CICD.yml/badge.svg)](https://github.com/sharkdp/hexyl/actions/workflows/CICD.yml)
[![](https://img.shields.io/crates/l/hexyl.svg?colorB=22ba4c)](https://crates.io/crates/hexyl)
![](https://img.shields.io/crates/v/hexyl.svg?colorB=00aa88)

`hexyl` 是一款终端十六进制（hex）查看器。它通过彩色输出来区分不同类别的
字节（NULL 字节、可打印 ASCII 字符、ASCII 空白字符、其他 ASCII 字符以及非 ASCII 字符）。

## 预览

![](https://i.imgur.com/MWO9uSL.png)

![](https://i.imgur.com/Dp7Wncz.png)

![](https://i.imgur.com/ln3TniI.png)

![](https://i.imgur.com/f8nm8g6.png)


## 安装

### 在 Ubuntu 上

*……以及其他基于 Debian 的 Linux 发行版。*

如果你运行的是 Ubuntu 19.10（Eoan Ermine）或更新版本，可以安装[官方维护的软件包](https://packages.ubuntu.com/search?keywords=hexyl)：
```bash
sudo apt install hexyl
```
如果你使用的是更早版本的 Ubuntu，可以从发布页面下载
最新的 `.deb` 软件包，并通过以下命令安装：

``` bash
sudo dpkg -i hexyl_0.15.0_amd64.deb  # adapt version number and architecture
```

### 在 Debian 上

如果你运行的是 Debian Buster 或更新版本，可以安装[官方维护的 Debian 软件包](https://packages.debian.org/search?searchon=names&keywords=hexyl)：
```bash
sudo apt-get install hexyl
```

如果你运行的是更早版本的 Debian，请参考上文了解如何
手动安装 `hexyl`。

### 在 Fedora 上

如果你运行的是 Fedora 35 或更新版本，可以安装[官方维护的 Fedora 软件包](https://packages.fedoraproject.org/pkgs/rust-hexyl/hexyl)：

```bash
sudo dnf install hexyl
```

### 在 Arch Linux 上

你可以从[官方软件包仓库](https://archlinux.org/packages/extra/x86_64/hexyl/)安装 `hexyl`：

```
pacman -S hexyl
```

### 在 Void Linux 上

```
xbps-install hexyl
```

### 在 Gentoo Linux 上

可在 [dm9pZCAq overlay](https://github.com/gentoo-mirror/dm9pZCAq) 中获取

```
sudo eselect repository enable dm9pZCAq
sudo emerge --sync dm9pZCAq
sudo emerge sys-apps/hexyl::dm9pZCAq
```

### 在 macOS 上

通过 [Homebrew](https://brew.sh)：

```
brew install hexyl
```

……或通过 [MacPorts](https://www.macports.org)：

```
sudo port install hexyl
```

### 在 FreeBSD 上

```
pkg install hexyl
```

### 在 NetBSD 上

```
pkgin install hexyl
```

### 在 OpenBSD 上

```
doas pkg_add hexyl
```

### 在 Termux 上
```
pkg install hexyl
```
或
```
apt install hexyl
```

### 通过 Nix

```
nix-env -i hexyl
```

### 通过 Guix

```
guix package -i hexyl
```

或者在你的系统配置文件（例如 `/etc/config.scm`）中，把 `hexyl` 软件包加入待安装软件包列表。

### 在其他发行版上

请查看[发布页面](https://github.com/sharkdp/hexyl/releases)获取二进制构建。

### 在 Windows 上

请查看[发布页面](https://github.com/sharkdp/hexyl/releases)获取二进制构建。
也可以通过 `cargo`、`snap` 或 `scoop` 从源码安装（见下文）。
请确保你使用的终端支持 ANSI 转义序列（例如 Windows 10 1703 起自带的 ConHost v2
或 Windows 10 1903 起推出的 Windows Terminal）。

### 通过 cargo

如果你有 Rust 1.56 或更高版本，可以通过 `cargo` 从源码安装 `hexyl`：
```
cargo install hexyl
```

或者，你也可以直接从仓库安装 `hexyl`：
```
git clone https://github.com/sharkdp/hexyl
cargo install --path ./hexyl
```

注意：要转换手册页，你需要 [Pandoc](https://pandoc.org/)。

可以在项目根目录下，将 Markdown 转换为手册页：
```
pandoc -s -f markdown -t man -o ./doc/hexyl.1 ./doc/hexyl.1.md
```

### 通过 snap 软件包

```
sudo snap install hexyl
```
[从 Snap Store 获取](https://snapcraft.io/hexyl)


### 通过 [Scoop](https://scoop.sh)
```
scoop install hexyl
```

### 通过 [X-CMD](https://x-cmd.com)
```
x env use hexyl
```

## 配置

`hexyl` 的颜色可以通过环境变量配置。所用变量如下：

 * `HEXYL_COLOR_ASCII_PRINTABLE`：任意非空白的可打印 ASCII 字符
 * `HEXYL_COLOR_ASCII_WHITESPACE`：空格或换行符之类的空白字符（仅在显示字节值的中间面板中可见）
 * `HEXYL_COLOR_ASCII_OTHER`：除 null 之外的任意其他 ASCII 字符（< `0x80`）
 * `HEXYL_COLOR_NULL`：null 字节（`0x00`）
 * `HEXYL_COLOR_NONASCII`：任意非 ASCII 字节（> `0x7F`）
 * `HEXYL_COLOR_OFFSET`：左侧的文件偏移量

颜色可以是 8 种标准终端颜色中的任意一种：`black`、`blue`、`cyan`、`green`、`magenta`、`red`、
`yellow` 和 `white`。也支持 "bright" 变体（例如 `bright blue`）。此外，你还可以使用
RGB 十六进制格式 `#abcdef`。例如，`HEXYL_COLOR_ASCII_PRINTABLE=blue HEXYL_COLOR_ASCII_WHITESPACE="bright green"
HEXYL_COLOR_ASCII_OTHER="#ff7f99"`。

## 许可

本项目以下列许可证之一授权：

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

你可自行选择适用哪一种。
