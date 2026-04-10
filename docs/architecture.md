# Architecture

## Module Structure

```
src/
├── main.rs          # CLI entry point, argument parsing, output rendering
├── logos.rs         # Distro ASCII art and color schemes
├── types.rs         # SystemInfo data structure for collected info
└── info/
    ├── mod.rs       # Module re-exports
    ├── helpers.rs   # Shared utilities (ANSI stripping, version parsing)
    ├── host.rs      # Username, hostname, uptime, locale, host model
    ├── software.rs  # OS, kernel, packages, shell, shell theme, editor
    ├── hardware.rs  # CPU, GPU, memory, swap, disks
    ├── display.rs   # DE, WM, terminal, monitors, font, multiplexer
    └── network.rs   # Network interfaces, local IP
```

## Data Flow

1. **Initialization**: main.rs parses CLI arguments via clap
2. **Logo Selection**: Based on distro ID from /etc/os-release or --logo flag
3. **Data Collection**: info module functions gather system information
4. **Rendering**: main.rs formats info lines and renders alongside ASCII logo
5. **Output**: Side-by-side display with ANSI color codes

## Key Design Decisions

### Cow<'static, str> for Logo Art
Static logos use `Cow::Borrowed` (zero-cost), while custom ASCII files use `Cow::Owned`. This avoids memory leaks from Box::leak.

### Modular Info Collection
The info module is split by category (hardware, software, display, network) for maintainability. Each function is independent and returns String or Option<String>.

### External Command Handling
Uses std::process::Command for external tools (lspci, ip, kscreen-doctor, etc.). All commands use Ok pattern matching - failures fall back to "Unknown" rather than panicking.

### Terminal Size Detection
Direct libc::ioctl call for TIOCGWINSZ with bounds validation (0 < size < 10000) to handle edge cases.
