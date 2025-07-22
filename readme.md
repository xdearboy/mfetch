<h1 align="center">📦 mfetch</h1>

<p align="center">
  <a href="https://github.com/xdearboy/mfetch/blob/master/LICENSE">
    <img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="MIT License" />
  </a>
  <a href="https://crates.io/crates/mfetch">
    <img src="https://img.shields.io/crates/v/mfetch.svg" alt="mfetch crate version" />
  </a>
  <a href="https://github.com/xdearboy/mfetch/stargazers">
    <img src="https://img.shields.io/github/stars/xdearboy/mfetch.svg" alt="GitHub stars" />
  </a>
  <a href="https://github.com/xdearboy/mfetch/network">
    <img src="https://img.shields.io/github/forks/xdearboy/mfetch.svg" alt="GitHub forks" />
  </a>
  <a href="https://aur.archlinux.org/packages/mfetch">
    <img src="https://img.shields.io/badge/AUR-mfetch-green.svg" alt="AUR package" />
  </a>
</p>

## Features

- lightweight and fast memory information tool written in rust
- colorful and easy-to-read terminal output using the colored crate
- displays total and available system memory from /proc/meminfo
- detailed info about memory modules (dimms) via dmidecode
- shows slot, size, speed, type, bank locator, ecc status, and configured voltage
- minimal dependencies and straightforward to build
- requires root privileges to access detailed hardware info via dmidecode


## Installation

Build from source:

```bash
cargo build --release
```
Or install via AUR (once package is added):

```bash
yay -S mfetch
```

## Usage
```bash
sudo mfetch
```

## License
MIT License. See LICENSE file for details.
