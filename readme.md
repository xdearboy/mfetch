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

- total and available memory  
- RAM module speed (MHz)  
- type (DDR3 / DDR4 / DDR5)  
- channel count  
- manufacturer and model  
- latency timings (CL, tRCD, etc. if available)  
- ECC status  
- voltage

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
mfetch
```

## License
MIT License. See LICENSE file for details.
