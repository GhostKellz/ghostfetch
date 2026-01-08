# ghostfetch

[![Built with Rust](https://img.shields.io/badge/Built%20with-Rust-b7410e?style=flat-square&logo=rust)](https://www.rust-lang.org/)
[![Arch Linux](https://img.shields.io/badge/Arch%20Linux-1793D1?style=flat-square&logo=arch-linux&logoColor=white)](https://archlinux.org/)
[![Fedora](https://img.shields.io/badge/Fedora-51A2DA?style=flat-square&logo=fedora&logoColor=white)](https://fedoraproject.org/)
[![Debian](https://img.shields.io/badge/Debian-A81D33?style=flat-square&logo=debian&logoColor=white)](https://www.debian.org/)
[![Pop!_OS](https://img.shields.io/badge/Pop!__OS-48B9C7?style=flat-square&logo=pop!_os&logoColor=white)](https://pop.system76.com/)
[![Wayland](https://img.shields.io/badge/Wayland-FFBC00?style=flat-square&logo=wayland&logoColor=black)](https://wayland.freedesktop.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg?style=flat-square)](LICENSE)

A fast, minimal system information tool for Linux written in Rust.

<p align="center">
  <img src="assets/logo/logo.png" alt="ghostfetch logo" width="400">
</p>

## Features

- **Distro-specific ASCII logos** with proper colors (Arch, CachyOS, EndeavourOS, Bazzite, Nobara, Ubuntu, Debian, Fedora, Pop!_OS, Manjaro, Mint, openSUSE, Gentoo, NixOS, Void, Alpine, Proxmox, and more)
- **Monitor detection** with actual model names (e.g., PG32UCDM, AW2725DF), resolution, refresh rate, and HDR status
- **Display server detection** (Wayland/X11) shown alongside window manager
- **Shell prompt detection** (Starship, Powerlevel10k, Oh My Zsh, Zinit, etc.)
- **Terminal font detection** from ghostty, kitty, alacritty, konsole configs
- **Multiplexer detection** (tmux, zellij, GNU Screen) with versions
- **Host/motherboard detection** (e.g., ROG CROSSHAIR X670E HERO)
- **Multiple GPU support** with discrete/integrated labels
- **RAM speed detection** (DDR4/DDR5 MT/s when available)
- **Multiple package managers** (pacman, flatpak, snap, dpkg, rpm)
- **Disk usage** with mount points and filesystem types
- **Fast** - single static binary, no runtime dependencies

## Installation

### From source (requires Rust 1.70+)

```bash
git clone https://github.com/yourusername/ghostfetch.git
cd ghostfetch
cargo build --release
sudo cp target/release/ghostfetch /usr/local/bin/
```

### Arch Linux (AUR)

```bash
# Coming soon
yay -S ghostfetch
```

## Usage

```bash
ghostfetch              # Normal output
ghostfetch --off        # No ASCII logo
ghostfetch --logo arch  # Force specific distro logo
ghostfetch --no-color   # Disable colors
ghostfetch --all        # Show all info (including locale)
ghostfetch --help       # Show help
```

### CLI Options

| Flag | Short | Description |
|------|-------|-------------|
| `--off` | `-o` | Disable ASCII art logo |
| `--logo <DISTRO>` | `-l` | Use a specific distro's logo |
| `--ascii <FILE>` | `-a` | Use a custom ASCII art file |
| `--no-color` | | Disable colors |
| `--all` | | Show all available info |
| `--help` | `-h` | Print help |
| `--version` | `-V` | Print version |

## Example Output

```
                   -`                   chris@arch
                  .o+`                  ----------
                 `ooo/                  Host         ASUSTeK ROG CROSSHAIR X670E HERO
                `+oooo:                 OS           Arch Linux
               `+oooooo:                Kernel       6.18.3-273-tkg-linux-ghost
               -+oooooo+:               Uptime       3 hours, 17 mins
             `/:-:++oooo+:              Packages     2361 (pacman), 4 (flatpak)
            `/++++/+++++++:             Shell        zsh 5.9
           `/++++++++++++++:            Prompt       Starship
          `/+++ooooooooooooo/`          Display 1    (PG32UCDM) 3840x2160 @ 240 Hz [HDR]
         ./ooosssso++osssssso+`         Display 2    (AW2725DF) 2560x1440 @ 360 Hz [HDR]
        .oossssso-````/ossssss+`        DE           KDE Plasma 6.5.4
       -osssssso.      :ssssssso.       WM           KWin (Wayland)
      :osssssss/        osssso+++.      Terminal     ghostty 1.1.4
     /ossssssss/        +ssssooo/-      Font         CaskaydiaCove NFM SemiBold
   `/ossssso+/:-        -:/+osssso+-    Multiplexer  tmux 3.6a
  `+sso+:-`                 `.-/+oso:   CPU          AMD Ryzen 9 9950X3D (32) @ 5.54 GHz
 `++:.                           `-/+/  GPU 1        NVIDIA GeForce RTX 5090 [Discrete]
 .`                                 `/  GPU 2        AMD Radeon Graphics [Integrated]
                                        Memory       18.06 GiB / 60.46 GiB (29%) @ 6000 MT/s
                                        Swap         0.20 GiB / 30.23 GiB (0%)
                                        Disk         (/) 1.38 TiB / 1.82 TiB (75%) - btrfs
                                        Local IP     10.0.0.21/24
```

## Supported Distributions

| Distribution | Logo | Status |
|--------------|------|--------|
| Arch Linux | Yes | Full support |
| CachyOS | Yes | Full support |
| EndeavourOS | Yes | Full support |
| Bazzite | Yes | Full support |
| Nobara | Yes | Full support |
| Ubuntu | Yes | Full support |
| Debian | Yes | Full support |
| Fedora | Yes | Full support |
| Pop!_OS | Yes | Full support |
| Manjaro | Yes | Full support |
| Linux Mint | Yes | Full support |
| openSUSE | Yes | Full support |
| Gentoo | Yes | Full support |
| NixOS | Yes | Full support |
| Void Linux | Yes | Full support |
| Alpine | Yes | Full support |
| Proxmox | Yes | Full support |
| Other | Generic | Basic support |

## Shell Prompt Detection

ghostfetch detects these shell prompts/frameworks:

- **Starship** - Cross-shell prompt
- **Powerlevel10k** - Zsh theme
- **Oh My Zsh** - Zsh framework (with theme name)
- **Zinit** - Zsh plugin manager
- **Antigen** - Zsh plugin manager
- **Zplug** - Zsh plugin manager
- **Bash-it** - Bash framework
- **Oh My Fish** - Fish framework

## Notes

### RAM Speed
RAM speed detection requires `dmidecode` which typically needs root privileges. To show RAM speed:
```bash
sudo ghostfetch  # Run with sudo
# Or set up dmidecode permissions
```

### Terminal Font
Terminal font is detected from config files for: ghostty, kitty, alacritty, konsole.

## Why ghostfetch?

With [neofetch](https://github.com/dylanaraps/neofetch) no longer maintained, the Linux community needed modern alternatives. ghostfetch aims to combine the best aspects of existing fetch tools:

- The **extensive distro support** and aesthetic of neofetch
- The **simplicity and speed** of betterfetch
- The **detailed hardware detection** of fastfetch
- **Shell/terminal rice detection** for the dotfiles community

All wrapped in a fast, single-binary Rust application.

## Acknowledgments

ghostfetch stands on the shoulders of giants. Special thanks to:

- **[neofetch](https://github.com/dylanaraps/neofetch)** by Dylan Araps - The original system fetch tool that started it all. ASCII art and distro detection patterns referenced from this project. (No longer maintained)
- **[fastfetch](https://github.com/fastfetch-cli/fastfetch)** - A blazingly fast system information tool written in C. Inspired our detailed hardware detection approach (monitor names, GPU classification, etc.)
- **[betterfetch](https://codeberg.org/sctech/betterfetch)** - A simple, clean fetch script. Inspired our minimal approach and output formatting.

## Contributing

Contributions are welcome! Feel free to:

- Add support for new distributions
- Improve hardware detection
- Fix bugs or improve performance
- Add new features

## License

MIT License - see [LICENSE](LICENSE) for details.

---

*ghostfetch - Because your terminal deserves to look good.*
