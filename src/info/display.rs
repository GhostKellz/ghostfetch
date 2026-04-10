use super::helpers::strip_ansi_codes;
use std::fs;
use std::process::Command;

/// Monitor display information.
#[derive(Clone)]
pub struct MonitorInfo {
    pub name: String,
    pub resolution: String,
    pub refresh_rate: String,
    pub hdr: bool,
}

pub fn get_de() -> String {
    let de = std::env::var("XDG_CURRENT_DESKTOP")
        .or_else(|_| std::env::var("DESKTOP_SESSION"))
        .unwrap_or_else(|_| "Unknown".to_string());

    // Try to get version for KDE Plasma
    if (de.to_lowercase().contains("kde") || de.to_lowercase().contains("plasma"))
        && let Ok(output) = Command::new("plasmashell").arg("--version").output()
    {
        let version = String::from_utf8_lossy(&output.stdout);
        for line in version.lines() {
            if line.contains("plasmashell")
                && let Some(ver) = line.split_whitespace().last()
            {
                return format!("KDE Plasma {}", ver);
            }
        }
    }

    de
}

pub fn get_display_server() -> String {
    if std::env::var("WAYLAND_DISPLAY").is_ok() {
        "Wayland".to_string()
    } else if std::env::var("DISPLAY").is_ok() {
        "X11".to_string()
    } else {
        "TTY".to_string()
    }
}

pub fn get_wm() -> String {
    let display_server = get_display_server();

    // Try to detect WM more accurately
    let wm = if let Ok(desktop) = std::env::var("XDG_CURRENT_DESKTOP") {
        match desktop.to_lowercase().as_str() {
            "kde" => "KWin".to_string(),
            "gnome" => "Mutter".to_string(),
            "xfce" => "Xfwm4".to_string(),
            "cinnamon" => "Muffin".to_string(),
            "mate" => "Marco".to_string(),
            "lxqt" => "Openbox".to_string(),
            "hyprland" => "Hyprland".to_string(),
            "sway" => "Sway".to_string(),
            "i3" => "i3".to_string(),
            _ => {
                // Check for standalone WMs via process
                let wms = [
                    "hyprland",
                    "sway",
                    "i3",
                    "bspwm",
                    "dwm",
                    "awesome",
                    "openbox",
                    "fluxbox",
                    "herbstluftwm",
                    "qtile",
                    "xmonad",
                    "spectrwm",
                ];
                if let Ok(output) = Command::new("ps").args(["-e", "-o", "comm="]).output() {
                    let procs = String::from_utf8_lossy(&output.stdout).to_lowercase();
                    for wm in &wms {
                        if procs.contains(wm) {
                            return format!("{} ({})", wm, display_server);
                        }
                    }
                }
                desktop
            }
        }
    } else {
        "Unknown".to_string()
    };

    format!("{} ({})", wm, display_server)
}

pub fn get_terminal() -> String {
    // Check for ghostty-specific environment variables (works even inside tmux)
    if std::env::var("GHOSTTY_BIN_DIR").is_ok() || std::env::var("GHOSTTY_RESOURCES_DIR").is_ok() {
        if let Ok(output) = Command::new("ghostty").arg("--version").output() {
            let version = String::from_utf8_lossy(&output.stdout);
            if let Some(line) = version.lines().next() {
                return format!("ghostty {}", line.split_whitespace().last().unwrap_or(""));
            }
        }
        return "ghostty".to_string();
    }

    // Check TERM_PROGRAM (set by many modern terminals)
    if let Ok(term_prog) = std::env::var("TERM_PROGRAM") {
        let term = term_prog.to_lowercase();
        if term.contains("ghostty") {
            if let Ok(output) = Command::new("ghostty").arg("--version").output() {
                let version = String::from_utf8_lossy(&output.stdout);
                if let Some(line) = version.lines().next() {
                    return format!("ghostty {}", line.split_whitespace().last().unwrap_or(""));
                }
            }
            return "ghostty".to_string();
        }
    }

    let terminals = [
        ("ghostty", "ghostty"),
        ("alacritty", "alacritty"),
        ("kitty", "kitty"),
        ("konsole", "konsole"),
        ("gnome-terminal-server", "gnome-terminal"),
        ("xfce4-terminal", "xfce4-terminal"),
        ("terminator", "terminator"),
        ("tilix", "tilix"),
        ("urxvt", "urxvt"),
        ("foot", "foot"),
        ("wezterm-gui", "wezterm"),
        ("xterm", "xterm"),
        ("lxterminal", "lxterminal"),
        ("mate-terminal", "mate-terminal"),
        ("yakuake", "yakuake"),
        ("guake", "guake"),
        ("st", "st"),
    ];

    // Walk up process tree to find terminal
    let mut ppid = std::process::id();
    for _ in 0..20 {
        if let Ok(stat) = fs::read_to_string(format!("/proc/{}/stat", ppid)) {
            let parts: Vec<&str> = stat.split_whitespace().collect();
            if parts.len() > 3 {
                let comm = parts[1]
                    .trim_matches(|c| c == '(' || c == ')')
                    .to_lowercase();
                for (proc_name, display_name) in &terminals {
                    if comm == *proc_name || comm.contains(proc_name) {
                        // Try to get version
                        if let Ok(output) = Command::new(display_name).arg("--version").output() {
                            let version = String::from_utf8_lossy(&output.stdout);
                            if let Some(line) = version.lines().next() {
                                // Extract version from line
                                let line_parts: Vec<&str> = line.split_whitespace().collect();
                                for part in line_parts {
                                    if part
                                        .chars()
                                        .next()
                                        .map(|c| c.is_ascii_digit())
                                        .unwrap_or(false)
                                        || part.starts_with('v')
                                    {
                                        return format!(
                                            "{} {}",
                                            display_name,
                                            part.trim_start_matches('v')
                                        );
                                    }
                                }
                            }
                        }
                        return display_name.to_string();
                    }
                }
                ppid = parts[3].parse().unwrap_or(1);
                if ppid <= 1 {
                    break;
                }
            }
        } else {
            break;
        }
    }

    std::env::var("TERM").unwrap_or_else(|_| "Unknown".to_string())
}

pub fn get_terminal_font() -> Option<String> {
    // Try ghostty config
    if let Ok(home) = std::env::var("HOME") {
        let ghostty_config = format!("{}/.config/ghostty/config", home);
        if let Ok(content) = fs::read_to_string(&ghostty_config) {
            for line in content.lines() {
                let line = line.trim();
                if line.starts_with("font-family")
                    && let Some(font) = line.split('=').nth(1)
                {
                    return Some(font.trim().trim_matches('"').to_string());
                }
            }
        }
    }

    // Try kitty config
    if let Ok(home) = std::env::var("HOME") {
        let kitty_config = format!("{}/.config/kitty/kitty.conf", home);
        if let Ok(content) = fs::read_to_string(&kitty_config) {
            for line in content.lines() {
                let line = line.trim();
                if line.starts_with("font_family")
                    && let Some(font) = line.split_whitespace().nth(1)
                {
                    return Some(font.to_string());
                }
            }
        }
    }

    // Try alacritty config
    if let Ok(home) = std::env::var("HOME") {
        for config_path in &[
            format!("{}/.config/alacritty/alacritty.toml", home),
            format!("{}/.config/alacritty/alacritty.yml", home),
        ] {
            if let Ok(content) = fs::read_to_string(config_path) {
                for line in content.lines() {
                    if line.contains("family")
                        && !line.trim().starts_with('#')
                        && let Some(font) = line.split(['=', ':']).nth(1)
                    {
                        let font = font.trim().trim_matches('"').trim_matches('\'');
                        if !font.is_empty() {
                            return Some(font.to_string());
                        }
                    }
                }
            }
        }
    }

    // Try konsole (KDE)
    if let Ok(home) = std::env::var("HOME") {
        let konsole_dir = format!("{}/.local/share/konsole", home);
        if let Ok(entries) = fs::read_dir(&konsole_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().map(|e| e == "profile").unwrap_or(false)
                    && let Ok(content) = fs::read_to_string(&path)
                {
                    for line in content.lines() {
                        if line.starts_with("Font=")
                            && let Some(font) = line.strip_prefix("Font=")
                        {
                            // Format is usually "FontName,size,-1,..."
                            if let Some(name) = font.split(',').next() {
                                return Some(name.to_string());
                            }
                        }
                    }
                }
            }
        }
    }

    None
}

pub fn get_multiplexer() -> Option<String> {
    // Check TMUX
    if std::env::var("TMUX").is_ok() {
        if let Ok(output) = Command::new("tmux").args(["-V"]).output() {
            let version = String::from_utf8_lossy(&output.stdout);
            let version = version.trim();
            if !version.is_empty() {
                return Some(version.to_string());
            }
        }
        return Some("tmux".to_string());
    }

    // Check Zellij
    if std::env::var("ZELLIJ").is_ok() || std::env::var("ZELLIJ_SESSION_NAME").is_ok() {
        if let Ok(output) = Command::new("zellij").args(["--version"]).output() {
            let version = String::from_utf8_lossy(&output.stdout);
            let version = version.trim().replace("zellij ", "");
            if !version.is_empty() {
                return Some(format!("Zellij {}", version));
            }
        }
        return Some("Zellij".to_string());
    }

    // Check Screen
    if std::env::var("STY").is_ok() {
        return Some("GNU Screen".to_string());
    }

    None
}

pub fn get_monitors() -> Vec<MonitorInfo> {
    let mut monitors = Vec::new();

    // First try to get monitor names from EDID
    let mut edid_names: std::collections::HashMap<String, String> =
        std::collections::HashMap::new();

    if let Ok(entries) = fs::read_dir("/sys/class/drm") {
        for entry in entries.flatten() {
            let path = entry.path();
            let dir_name = path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();

            if dir_name.starts_with("card") && dir_name.contains('-') {
                let edid_path = path.join("edid");
                if edid_path.exists()
                    && let Ok(edid_bytes) = fs::read(&edid_path)
                    && edid_bytes.len() >= 128
                {
                    // Extract monitor name from EDID (descriptor blocks start at byte 54)
                    if let Some(name) = extract_edid_name(&edid_bytes) {
                        // Map card1-DP-2 -> DP-2
                        let connector = dir_name.split('-').skip(1).collect::<Vec<_>>().join("-");
                        edid_names.insert(connector, name);
                    }
                }
            }
        }
    }

    // Use kscreen-doctor for KDE Plasma (best source)
    if let Ok(output) = Command::new("kscreen-doctor").arg("-o").output() {
        // Strip ANSI color codes
        let raw = String::from_utf8_lossy(&output.stdout);
        let kscreen = strip_ansi_codes(&raw);
        let mut current_output = String::new();
        let mut current_res = String::new();
        let mut current_rate = String::new();
        let mut has_hdr = false;

        for line in kscreen.lines() {
            let line = line.trim();

            if line.starts_with("Output:") {
                // Save previous if exists
                if !current_output.is_empty() && !current_res.is_empty() {
                    let name = edid_names
                        .get(&current_output)
                        .cloned()
                        .unwrap_or(current_output.clone());
                    monitors.push(MonitorInfo {
                        name,
                        resolution: current_res.clone(),
                        refresh_rate: current_rate.clone(),
                        hdr: has_hdr,
                    });
                }

                // Parse: "Output: 1 DP-2 uuid..."
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 3 {
                    current_output = parts[2].to_string();
                }
                current_res.clear();
                current_rate.clear();
                has_hdr = false;
            } else if line.starts_with("Modes:") {
                // Find the mode with * (current)
                // Format: 2:3840x2160@240.02*
                for part in line.split_whitespace() {
                    if part.contains('*') {
                        if let Some(mode) = part.split(':').nth(1) {
                            let mode = mode.trim_end_matches('*');
                            if let Some((res, rate)) = mode.split_once('@') {
                                current_res = res.to_string();
                                // Round refresh rate (remove trailing *)
                                let rate_clean = rate.trim_end_matches('*');
                                if let Ok(rate_f) = rate_clean.parse::<f64>() {
                                    current_rate = format!("{:.0} Hz", rate_f);
                                }
                            }
                        }
                        break;
                    }
                }
            } else if line.contains("HDR:") && line.contains("enabled") {
                has_hdr = true;
            }
        }

        // Don't forget the last one
        if !current_output.is_empty() && !current_res.is_empty() {
            let name = edid_names
                .get(&current_output)
                .cloned()
                .unwrap_or(current_output.clone());
            monitors.push(MonitorInfo {
                name,
                resolution: current_res,
                refresh_rate: current_rate,
                hdr: has_hdr,
            });
        }
    }

    // Fallback to xrandr if kscreen-doctor didn't work
    if monitors.is_empty()
        && std::env::var("DISPLAY").is_ok()
        && let Ok(output) = Command::new("xrandr").args(["--query"]).output()
    {
        let xrandr = String::from_utf8_lossy(&output.stdout);
        let mut current_output = String::new();

        for line in xrandr.lines() {
            if line.contains(" connected") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if !parts.is_empty() {
                    current_output = parts[0].to_string();
                }
            } else if line.contains('*') && !current_output.is_empty() {
                // Current resolution line
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    let res = parts[0];
                    let rate = parts[1].trim_end_matches('*').trim_end_matches('+');

                    let name = edid_names
                        .get(&current_output)
                        .cloned()
                        .unwrap_or(current_output.clone());

                    monitors.push(MonitorInfo {
                        name,
                        resolution: res.to_string(),
                        refresh_rate: format!("{} Hz", rate),
                        hdr: false,
                    });
                    current_output.clear();
                }
            }
        }
    }

    monitors
}

fn extract_edid_name(edid: &[u8]) -> Option<String> {
    // EDID descriptor blocks are at bytes 54-125 (4 blocks of 18 bytes each)
    for i in 0..4 {
        let offset = 54 + i * 18;
        if offset + 18 > edid.len() {
            break;
        }

        // Monitor name descriptor has tag 0xFC
        if edid[offset] == 0 && edid[offset + 1] == 0 && edid[offset + 3] == 0xFC {
            let name_bytes = &edid[offset + 5..offset + 18];
            let name: String = name_bytes
                .iter()
                .take_while(|&&b| b != 0x0A && b != 0x00)
                .filter(|&&b| (0x20..0x7F).contains(&b))
                .map(|&b| b as char)
                .collect();
            let name = name.trim().to_string();
            if !name.is_empty() {
                return Some(name);
            }
        }
    }
    None
}
