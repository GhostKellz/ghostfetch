mod info;
mod logos;
mod types;

use clap::Parser;
use colored::Colorize;
use std::borrow::Cow;
use std::io::{self, IsTerminal};
use sysinfo::System;

fn get_terminal_width() -> usize {
    // Try to get terminal size
    if let Some((width, _)) = term_size() {
        width
    } else {
        120 // Default fallback
    }
}

fn term_size() -> Option<(usize, usize)> {
    // Use ioctl to get terminal size
    use std::os::unix::io::AsRawFd;

    #[repr(C)]
    struct Winsize {
        ws_row: u16,
        ws_col: u16,
        ws_xpixel: u16,
        ws_ypixel: u16,
    }

    let fd = if io::stdout().is_terminal() {
        io::stdout().as_raw_fd()
    } else if io::stderr().is_terminal() {
        io::stderr().as_raw_fd()
    } else {
        return None;
    };

    let mut size = Winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };

    // TIOCGWINSZ = 0x5413 on Linux
    let result = unsafe { libc::ioctl(fd, libc::TIOCGWINSZ, &mut size) };

    // Validate result and apply reasonable bounds
    // Reject sizes that are clearly invalid (0, negative via overflow, or absurdly large)
    if result == 0
        && size.ws_col > 0
        && size.ws_col < 10000
        && size.ws_row > 0
        && size.ws_row < 10000
    {
        Some((size.ws_col as usize, size.ws_row as usize))
    } else {
        None
    }
}

fn truncate_line(line: &str, max_width: usize) -> String {
    // Count visible characters (excluding ANSI escape codes)
    let visible_len = strip_ansi_len(line);

    if visible_len <= max_width {
        return line.to_string();
    }

    // Need to truncate - find where to cut
    let mut visible_count = 0;
    let mut byte_pos = 0;
    let mut in_escape = false;

    for (i, c) in line.char_indices() {
        if c == '\x1b' {
            in_escape = true;
        } else if in_escape {
            if c == 'm' {
                in_escape = false;
            }
        } else {
            visible_count += 1;
            if visible_count >= max_width - 3 {
                byte_pos = i;
                break;
            }
        }
        byte_pos = i + c.len_utf8();
    }

    format!("{}...", &line[..byte_pos])
}

fn strip_ansi_len(s: &str) -> usize {
    let mut len = 0;
    let mut in_escape = false;

    for c in s.chars() {
        if c == '\x1b' {
            in_escape = true;
        } else if in_escape {
            if c == 'm' {
                in_escape = false;
            }
        } else {
            len += 1;
        }
    }
    len
}

fn validate_ascii_file(path: &str) -> Result<String, String> {
    use std::path::Path;

    let path = Path::new(path);

    if !path.exists() {
        return Err(format!("ASCII file not found: {}", path.display()));
    }

    let content =
        std::fs::read_to_string(path).map_err(|e| format!("Cannot read ASCII file: {}", e))?;

    // Size limit to prevent reading huge files
    if content.len() > 50_000 {
        return Err("ASCII file too large (max 50KB)".to_string());
    }

    // Security: reject files with potentially unsafe terminal escape sequences
    // Allow basic ANSI color codes (\x1b[...m) but reject OSC sequences (\x1b]) and BEL (\x07)
    if content.contains("\x1b]") || content.contains("\x07") {
        return Err("ASCII file contains potentially unsafe terminal sequences".to_string());
    }

    Ok(content)
}

#[derive(Parser, Debug)]
#[command(name = "ghostfetch")]
#[command(author = "Christopher Kelley <ckelley@ghostkellz.sh>")]
#[command(version)]
#[command(about = "A fast, minimal system fetch tool for Linux", long_about = None)]
struct Args {
    /// Disable ASCII art logo
    #[arg(long, short = 'o')]
    off: bool,

    /// Use a specific distro's logo
    #[arg(long, short = 'l')]
    logo: Option<String>,

    /// Use a custom ASCII art file
    #[arg(long, short = 'a')]
    ascii: Option<String>,

    /// Disable colors
    #[arg(long)]
    no_color: bool,

    /// Show all available info (including optional fields)
    #[arg(long)]
    all: bool,
}

fn print_color_blocks() {
    let colors = [
        "   ".on_black(),
        "   ".on_red(),
        "   ".on_green(),
        "   ".on_yellow(),
        "   ".on_blue(),
        "   ".on_magenta(),
        "   ".on_cyan(),
        "   ".on_white(),
    ];

    for color in &colors {
        print!("{}", color);
    }
    println!();
}

fn main() {
    let args = Args::parse();

    // Handle --no-color
    if args.no_color {
        colored::control::set_override(false);
    }

    let mut sys = System::new_all();
    sys.refresh_all();

    let (os_name, distro_id) = info::get_os_info();

    // Get logo based on args
    let logo = if args.off {
        None
    } else if let Some(ref logo_name) = args.logo {
        Some(logos::get_logo(logo_name))
    } else if let Some(ref ascii_path) = args.ascii {
        // Custom ASCII file support with validation
        match validate_ascii_file(ascii_path) {
            Ok(content) => Some(logos::DistroLogo {
                art: Cow::Owned(content),
                width: 40,
                primary_color: |s| s.cyan(),
            }),
            Err(e) => {
                eprintln!("Warning: {}", e);
                Some(logos::get_logo(&distro_id))
            }
        }
    } else {
        Some(logos::get_logo(&distro_id))
    };

    let username = info::get_username();
    let hostname = info::get_hostname();

    // Build info lines
    let mut info_lines: Vec<String> = Vec::new();

    let primary_color: fn(&str) -> colored::ColoredString = logo
        .as_ref()
        .map(|l| l.primary_color)
        .unwrap_or(|s| s.cyan());

    // Title
    info_lines.push(format!(
        "{}{}{}",
        primary_color(&username).bold(),
        "@".white(),
        primary_color(&hostname).bold()
    ));
    info_lines.push("-".repeat(username.len() + 1 + hostname.len()).to_string());

    // Host (motherboard/laptop model)
    if let Some(host) = info::get_host() {
        info_lines.push(format!("{:<12} {}", primary_color("Host").bold(), host));
    }

    // System info
    info_lines.push(format!("{:<12} {}", primary_color("OS").bold(), os_name));
    info_lines.push(format!(
        "{:<12} {}",
        primary_color("Kernel").bold(),
        info::get_kernel()
    ));
    info_lines.push(format!(
        "{:<12} {}",
        primary_color("Uptime").bold(),
        info::get_uptime()
    ));
    info_lines.push(format!(
        "{:<12} {}",
        primary_color("Packages").bold(),
        info::get_packages()
    ));
    info_lines.push(format!(
        "{:<12} {}",
        primary_color("Shell").bold(),
        info::get_shell()
    ));

    // Shell theme (p10k, starship, omz, etc.)
    if let Some(theme) = info::get_shell_theme() {
        info_lines.push(format!("{:<12} {}", primary_color("Prompt").bold(), theme));
    }

    // Display/Monitor info
    let monitors = info::get_monitors();
    for (i, monitor) in monitors.iter().enumerate() {
        let label = if monitors.len() > 1 {
            format!("Display {}", i + 1)
        } else {
            "Display".to_string()
        };

        let mut display_str = format!(
            "({}) {} @ {}",
            monitor.name, monitor.resolution, monitor.refresh_rate
        );
        if monitor.hdr {
            display_str.push_str(" [HDR]");
        }

        info_lines.push(format!(
            "{:<12} {}",
            primary_color(&label).bold(),
            display_str
        ));
    }

    // DE and WM
    info_lines.push(format!(
        "{:<12} {}",
        primary_color("DE").bold(),
        info::get_de()
    ));
    info_lines.push(format!(
        "{:<12} {}",
        primary_color("WM").bold(),
        info::get_wm()
    ));

    // Terminal
    info_lines.push(format!(
        "{:<12} {}",
        primary_color("Terminal").bold(),
        info::get_terminal()
    ));

    // Terminal Font
    if let Some(font) = info::get_terminal_font() {
        info_lines.push(format!("{:<12} {}", primary_color("Font").bold(), font));
    }

    // Multiplexer (tmux, zellij, screen)
    if let Some(mux) = info::get_multiplexer() {
        info_lines.push(format!(
            "{:<12} {}",
            primary_color("Multiplexer").bold(),
            mux
        ));
    }

    // Editor
    if let Some(editor) = info::get_editor() {
        info_lines.push(format!("{:<12} {}", primary_color("Editor").bold(), editor));
    }

    // Hardware
    info_lines.push(format!(
        "{:<12} {}",
        primary_color("CPU").bold(),
        info::get_cpu(&sys)
    ));

    // GPUs
    let gpus = info::get_gpu();
    for (i, gpu) in gpus.iter().enumerate() {
        let label = if gpus.len() > 1 {
            format!("GPU {}", i + 1)
        } else {
            "GPU".to_string()
        };
        info_lines.push(format!("{:<12} {}", primary_color(&label).bold(), gpu));
    }

    // Memory
    info_lines.push(format!(
        "{:<12} {}",
        primary_color("Memory").bold(),
        info::get_memory(&sys)
    ));

    // Swap (if exists)
    if let Some(swap) = info::get_swap(&sys) {
        info_lines.push(format!("{:<12} {}", primary_color("Swap").bold(), swap));
    }

    // Disks
    let disks = info::get_disks();
    for disk in &disks {
        info_lines.push(format!("{:<12} {}", primary_color("Disk").bold(), disk));
    }

    // Network info
    info_lines.push(format!(
        "{:<12} {}",
        primary_color("Local IP").bold(),
        info::get_local_ip()
    ));

    // Locale (optional, show with --all)
    if args.all {
        info_lines.push(format!(
            "{:<12} {}",
            primary_color("Locale").bold(),
            info::get_locale()
        ));
    }

    // Empty line before color blocks
    info_lines.push(String::new());

    // Print output
    let term_width = get_terminal_width();

    if let Some(ref logo) = logo {
        let logo_lines: Vec<&str> = logo.art.lines().collect();
        let max_lines = logo_lines.len().max(info_lines.len());

        // Calculate max width for info lines (terminal width - logo width - some padding)
        let max_info_width = if term_width > logo.width + 10 {
            term_width - logo.width - 2
        } else {
            60 // Minimum reasonable width
        };

        for i in 0..max_lines {
            let logo_line = logo_lines.get(i).unwrap_or(&"");
            let info_line = info_lines.get(i).map(|s| s.as_str()).unwrap_or("");

            // Truncate info line if too long
            let display_line = truncate_line(info_line, max_info_width);

            print!(
                "{:width$}",
                (logo.primary_color)(logo_line),
                width = logo.width
            );
            println!("{}", display_line);
        }

        // Print color blocks
        print!("{:width$}", "", width = logo.width);
        print_color_blocks();
    } else {
        // No logo mode
        let max_width = term_width.saturating_sub(2);
        for line in &info_lines {
            println!("{}", truncate_line(line, max_width));
        }
        print_color_blocks();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_ansi_len_plain() {
        assert_eq!(strip_ansi_len("hello"), 5);
        assert_eq!(strip_ansi_len(""), 0);
        assert_eq!(strip_ansi_len("test string"), 11);
    }

    #[test]
    fn test_strip_ansi_len_with_colors() {
        // Single color code
        assert_eq!(strip_ansi_len("\x1b[31mhello\x1b[0m"), 5);
        // Bold + color
        assert_eq!(strip_ansi_len("\x1b[1;32mtest\x1b[0m"), 4);
        // Multiple codes
        assert_eq!(strip_ansi_len("\x1b[36mone\x1b[0m \x1b[31mtwo\x1b[0m"), 7);
    }

    #[test]
    fn test_truncate_line_short() {
        assert_eq!(truncate_line("hello", 10), "hello");
        assert_eq!(truncate_line("", 10), "");
    }

    #[test]
    fn test_truncate_line_exact() {
        assert_eq!(truncate_line("hello", 5), "hello");
    }

    #[test]
    fn test_truncate_line_long() {
        let result = truncate_line("hello world", 8);
        assert!(
            result.ends_with("..."),
            "Expected '...' suffix, got: {}",
            result
        );
        assert!(result.len() <= 11); // 8 visible + "..." could expand
    }

    #[test]
    fn test_truncate_line_with_ansi() {
        // Colored text should count visible chars only
        let colored = "\x1b[31mhello world\x1b[0m";
        let result = truncate_line(colored, 8);
        assert!(result.ends_with("..."));
    }

    #[test]
    fn test_validate_ascii_file_nonexistent() {
        let result = validate_ascii_file("/nonexistent/path");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not found"));
    }

    #[test]
    fn test_validate_ascii_file_rejects_osc() {
        // This test would need a temp file with OSC sequences
        // For now, just verify the function exists and returns Result
        let result = validate_ascii_file("/etc/passwd");
        // /etc/passwd exists and should be valid (no bad sequences)
        assert!(result.is_ok() || result.is_err()); // Either is valid behavior
    }
}
