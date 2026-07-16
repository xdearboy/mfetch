<h1 align="center">📦 mfetch v0.5.0</h1>

<p align="center">
  <a href="https://github.com/org-fe/mfetch/blob/master/LICENSE">
    <img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="MIT License" />
  </a>
  <a href="https://crates.io/crates/mfetch">
    <img src="https://img.shields.io/crates/v/mfetch.svg" alt="mfetch crate version" />
  </a>
  <a href="https://github.com/org-fe/mfetch/stargazers">
    <img src="https://img.shields.io/github/stars/org-fe/mfetch.svg" alt="GitHub stars" />
  </a>
  <a href="https://github.com/org-fe/mfetch/network">
    <img src="https://img.shields.io/github/forks/org-fe/mfetch.svg" alt="GitHub forks" />
  </a>
  <a href="https://aur.archlinux.org/packages/mfetch">
    <img src="https://img.shields.io/badge/AUR-mfetch-green.svg" alt="AUR package" />
  </a>
</p>

## Features

- system info: hostname, os, kernel, uptime
- cpu info: model, physical cores, logical threads
- memory: total, available, usage with progress bar
- swap: total, used, free
- hugepages: size, total, used, free (if hugetlb configured)
- memory pressure (psi): avg10/avg60/avg300 stall percentages
- spd data per dimm: manufacturer, dram maker, timings, ranks, memory type
- nvidia gpu: model, vram total and used
- 8 built-in color themes
- configurable via `~/.config/mfetch/config.toml`
- requires root for dmidecode, decode-dimms, and nvidia-smi

## Usage

```bash
# requires root for detailed module/gpu info
sudo mfetch

# debug mode shows raw keys and diagnostics
sudo mfetch --debug

# print version
mfetch --version
```

## Config

`~/.config/mfetch/config.toml` (auto-created on first run):

```toml
theme = "catppuccin"
show_system = true
show_cpu = true
show_spd_source = true
show_swap = true
show_hugepages = true
show_pressure = true
show_modules = true
show_gpu = true
```

Available themes: `catppuccin`, `tokyonight`, `rosepine`, `gruvbox`, `nord`, `dracula`, `everforest`, `neon`.

## Preview

```bash

  mfetch
  github.com/org-fe/mfetch
  ──────────────────────────────

  system
    host:        workstation
    os:          Arch Linux
    kernel:      Linux version 6.12.10
    uptime:      3d 5h 12m

  cpu
    model:       Intel(R) Core(TM) Ultra 7 155H
    cores:       16
    threads:     22

  memory
    total:       31.14 gb
    available:   18.50 gb
    usage:       |||||.......... 40.6%

  swap
    total:       8.00 gb
    used:        1.23 gb (15.4%)
    free:        6.77 gb

  hugepages
    size:        2048 mb
    total:       16
    used:        0
    free:        16

  mem pressure
    avg10:       0.00%
    avg60:       0.00%
    avg300:      0.00%

  modules

    ○ slot dimm 0 (spd)
      🏭 maker:     crucial technology
      💎 dram:      micron technology
      🏷️  part:      ct16g56c46s5.m8g1
      ⏱️  timings:   46-45-45-90
      ≡ ranks:      1
      ⬡ type:       ddr5 sdram
      📦 size:      16 gb
      ⚡ speed:     5600 mt/s
      🔌 voltage:   1.1 v

  gpu
    name:        NVIDIA GeForce RTX 4090
    vram total:  24564 mib
    vram used:   8192 mib

```

### ⚠️ disclaimer: spd vs real-time timings

`mfetch` displays **spd timings** — the factory profiles burned into the ram chip (not the active bios/overclock timings from the cpu memory controller).

### 🛠 enabling spd support

if the `modules` section is empty:

```bash
sudo modprobe i2c-dev
sudo modprobe ee1004
```

## Installation

```bash
cargo build --release
```

Or via AUR:

```bash
yay -S mfetch
```

## License

MIT License. See LICENSE file for details.
