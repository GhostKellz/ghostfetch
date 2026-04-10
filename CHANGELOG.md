# Changelog

All notable changes to ghostfetch will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.1] - 2026-04-10

### Fixed
- Memory leak in custom ASCII loading (replaced `Box::leak` with `Cow<'static, str>`)
- GPU parsing safety (replaced `unwrap()` with proper error handling)
- Terminal size detection bounds validation

### Added
- Input validation for custom ASCII files (size limit, escape sequence filtering)
- 22 unit tests covering ANSI handling, truncation, logo selection
- Documentation under `docs/` (architecture, external-tools, troubleshooting)
- Threat model section in SECURITY.md
- Binary download option in install.sh (`--binary` flag)

### Changed
- Refactored monolithic `info.rs` into modular structure (`src/info/`)
- Removed dead code paths in RAM speed detection

## [0.1.0] - 2026-01-07

### Added
- Initial release
- Distro-specific ASCII logos with proper colors
  - Arch Linux, CachyOS, EndeavourOS, Bazzite, Nobara
  - Ubuntu, Debian, Fedora, Pop!_OS, Manjaro, Linux Mint
  - openSUSE, Gentoo, NixOS, Void Linux, Alpine, Proxmox
- Monitor detection with actual model names (e.g., PG32UCDM, AW2725DF)
- Resolution and refresh rate display
- HDR status detection
- Display server detection (Wayland/X11) shown with window manager
- Shell prompt detection (Starship, Powerlevel10k, Oh My Zsh, Zinit, etc.)
- Terminal font detection from ghostty, kitty, alacritty, konsole configs
- Multiplexer detection (tmux, zellij, GNU Screen) with versions
- Host/motherboard detection (e.g., ROG CROSSHAIR X670E HERO)
- Multiple GPU support with discrete/integrated labels
- RAM usage with speed detection (DDR4/DDR5 MT/s when running as root)
- Multiple package manager support (pacman, flatpak, snap, dpkg, rpm)
- Disk usage with mount points and filesystem types
- Local IP address display
- CLI options: `--off`, `--logo`, `--ascii`, `--no-color`, `--all`

### Technical
- Written in Rust for speed and reliability
- Single static binary with no runtime dependencies
- LTO and strip enabled for minimal binary size
