<h1 align="center">📦 mfetch v0.0.3</h1>

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

## Preview

```bash

  mfetch
  github.com/xdearboy/mfetch
  ──────────────────────────────

  memory
    total:       15.54 gb
    available:   9.02 gb
    usage:       ||||||......... 41.9%

  modules

    ○ slot dimm 1 (spd)
      🏭 maker:     crucial technology
      💎 dram:      micron technology
      🏷️  part:      bl8g32c16u4bl.m8fe
      ⏱️  timings:   19-19-19-43
      📦 size:      8 gb
      ⚡ speed:     3733 mt/s
      🔌 voltage:   1.2 v

    ○ slot dimm 1 (spd)
      🏭 maker:     crucial technology
      💎 dram:      micron technology
      🏷️  part:      bl8g32c16u4bl.m8fe
      ⏱️  timings:   19-19-19-43
      📦 size:      8 gb
      ⚡ speed:     3733 mt/s
      🔌 voltage:   1.2 v

```

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
# Requires root privileges to see detailed memory module info
sudo mfetch
```

## License
MIT License. See LICENSE file for details.
