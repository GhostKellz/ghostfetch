use std::fs;
use std::process::Command;
use sysinfo::{Disks, System};

pub fn get_os_info() -> (String, String) {
    let content = fs::read_to_string("/etc/os-release").unwrap_or_default();
    let mut pretty_name = String::from("Linux");
    let mut id = String::from("linux");

    for line in content.lines() {
        if line.starts_with("PRETTY_NAME=") {
            pretty_name = line
                .trim_start_matches("PRETTY_NAME=")
                .trim_matches('"')
                .to_string();
        } else if line.starts_with("ID=") {
            id = line.trim_start_matches("ID=").trim_matches('"').to_string();
        }
    }
    (pretty_name, id)
}

pub fn get_kernel() -> String {
    fs::read_to_string("/proc/sys/kernel/osrelease")
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|_| "Unknown".to_string())
}

pub fn get_hostname() -> String {
    whoami::fallible::hostname().unwrap_or_else(|_| "Unknown".to_string())
}

pub fn get_username() -> String {
    whoami::username()
}

pub fn get_shell() -> String {
    let shell = std::env::var("SHELL")
        .map(|s| s.rsplit('/').next().unwrap_or("Unknown").to_string())
        .unwrap_or_else(|_| "Unknown".to_string());

    // Try to get version
    if let Ok(output) = Command::new(&shell).arg("--version").output() {
        let version_str = String::from_utf8_lossy(&output.stdout);
        if let Some(first_line) = version_str.lines().next() {
            // Extract version number
            for part in first_line.split_whitespace() {
                if part.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false) {
                    return format!("{} {}", shell, part);
                }
            }
        }
    }
    shell
}

pub fn get_uptime() -> String {
    fs::read_to_string("/proc/uptime")
        .ok()
        .and_then(|content| content.split_whitespace().next().map(String::from))
        .and_then(|secs_str| secs_str.parse::<f64>().ok())
        .map(|secs| {
            let total_secs = secs as u64;
            let days = total_secs / 86400;
            let hours = (total_secs % 86400) / 3600;
            let mins = (total_secs % 3600) / 60;

            if days > 0 {
                format!("{} days, {} hours, {} mins", days, hours, mins)
            } else if hours > 0 {
                format!("{} hours, {} mins", hours, mins)
            } else {
                format!("{} mins", mins)
            }
        })
        .unwrap_or_else(|| "Unknown".to_string())
}

pub fn get_packages() -> String {
    let mut counts = Vec::new();

    // pacman
    if let Ok(output) = Command::new("pacman").args(["-Qq"]).output() {
        let count = String::from_utf8_lossy(&output.stdout).lines().count();
        if count > 0 {
            counts.push(format!("{} (pacman)", count));
        }
    }

    // flatpak
    if let Ok(output) = Command::new("flatpak").args(["list", "--app"]).output() {
        let count = String::from_utf8_lossy(&output.stdout).lines().count();
        if count > 0 {
            counts.push(format!("{} (flatpak)", count));
        }
    }

    // snap
    if let Ok(output) = Command::new("snap").args(["list"]).output() {
        let count = String::from_utf8_lossy(&output.stdout)
            .lines()
            .skip(1)
            .count();
        if count > 0 {
            counts.push(format!("{} (snap)", count));
        }
    }

    // dpkg (debian/ubuntu)
    if counts.is_empty() {
        if let Ok(output) = Command::new("dpkg-query").args(["-f", ".\n", "-W"]).output() {
            let count = String::from_utf8_lossy(&output.stdout).lines().count();
            if count > 0 {
                counts.push(format!("{} (dpkg)", count));
            }
        }
    }

    // rpm (fedora/rhel)
    if counts.is_empty() {
        if let Ok(output) = Command::new("rpm").args(["-qa"]).output() {
            let count = String::from_utf8_lossy(&output.stdout).lines().count();
            if count > 0 {
                counts.push(format!("{} (rpm)", count));
            }
        }
    }

    if counts.is_empty() {
        "Unknown".to_string()
    } else {
        counts.join(", ")
    }
}

pub fn get_de() -> String {
    let de = std::env::var("XDG_CURRENT_DESKTOP")
        .or_else(|_| std::env::var("DESKTOP_SESSION"))
        .unwrap_or_else(|_| "Unknown".to_string());

    // Try to get version for KDE Plasma
    if de.to_lowercase().contains("kde") || de.to_lowercase().contains("plasma") {
        if let Ok(output) = Command::new("plasmashell").arg("--version").output() {
            let version = String::from_utf8_lossy(&output.stdout);
            for line in version.lines() {
                if line.contains("plasmashell") {
                    if let Some(ver) = line.split_whitespace().last() {
                        return format!("KDE Plasma {}", ver);
                    }
                }
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
                    "hyprland", "sway", "i3", "bspwm", "dwm", "awesome", "openbox",
                    "fluxbox", "herbstluftwm", "qtile", "xmonad", "spectrwm",
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
                let comm = parts[1].trim_matches(|c| c == '(' || c == ')').to_lowercase();
                for (proc_name, display_name) in &terminals {
                    if comm == *proc_name || comm.contains(proc_name) {
                        // Try to get version
                        if let Ok(output) = Command::new(display_name).arg("--version").output() {
                            let version = String::from_utf8_lossy(&output.stdout);
                            if let Some(line) = version.lines().next() {
                                // Extract version from line
                                let line_parts: Vec<&str> = line.split_whitespace().collect();
                                for part in line_parts {
                                    if part.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false)
                                       || part.starts_with('v') {
                                        return format!("{} {}", display_name, part.trim_start_matches('v'));
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

pub fn get_memory(sys: &System) -> String {
    let used = sys.used_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
    let total = sys.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
    let percent = (used / total * 100.0) as u64;

    // Try to get RAM speed
    if let Some(speed) = get_ram_speed() {
        format!("{:.2} GiB / {:.2} GiB ({}%) @ {} MT/s", used, total, percent, speed)
    } else {
        format!("{:.2} GiB / {:.2} GiB ({}%)", used, total, percent)
    }
}

fn get_ram_speed() -> Option<u32> {
    // Try dmidecode first (requires root, but might be cached)
    if let Ok(output) = Command::new("dmidecode")
        .args(["-t", "memory"])
        .output()
    {
        let dmi = String::from_utf8_lossy(&output.stdout);
        let mut configured_speed: Option<u32> = None;
        let mut base_speed: Option<u32> = None;

        for line in dmi.lines() {
            let line = line.trim();
            // Prefer "Configured Memory Speed:" (actual running speed with XMP/EXPO)
            if line.starts_with("Configured Memory Speed:") {
                if let Some(speed_str) = line.split(':').nth(1) {
                    let speed_str = speed_str.trim().replace(" MT/s", "").replace(" MHz", "");
                    if let Ok(speed) = speed_str.parse::<u32>() {
                        if speed > 0 && speed < 100000 {
                            configured_speed = Some(speed);
                        }
                    }
                }
            // Fallback to "Speed:" (JEDEC base speed)
            } else if line.starts_with("Speed:") && line.contains("MT/s") && base_speed.is_none() {
                if let Some(speed_str) = line.split(':').nth(1) {
                    let speed_str = speed_str.trim().replace(" MT/s", "").replace(" MHz", "");
                    if let Ok(speed) = speed_str.parse::<u32>() {
                        if speed > 0 && speed < 100000 {
                            base_speed = Some(speed);
                        }
                    }
                }
            }
        }

        // Return configured speed if available (XMP/EXPO), otherwise base speed
        if configured_speed.is_some() {
            return configured_speed;
        }
        if base_speed.is_some() {
            return base_speed;
        }
    }

    // Try reading from sysfs (works without root on some systems)
    if let Ok(entries) = fs::read_dir("/sys/devices/system/edac/mc") {
        for entry in entries.flatten() {
            let path = entry.path();
            // Try to find memory controller info
            if let Ok(content) = fs::read_to_string(path.join("dimm0/dimm_mem_type")) {
                if content.contains("DDR") {
                    // Found DDR info
                }
            }
        }
    }

    // Try lshw as fallback
    if let Ok(output) = Command::new("lshw")
        .args(["-C", "memory", "-short"])
        .output()
    {
        let lshw = String::from_utf8_lossy(&output.stdout);
        for line in lshw.lines() {
            if line.contains("System Memory") || line.contains("DIMM") {
                // Parse speed if available
            }
        }
    }

    // Try reading from /proc/meminfo for type hints
    // Unfortunately this doesn't contain speed info

    None
}

pub fn get_swap(sys: &System) -> Option<String> {
    let total = sys.total_swap();
    if total == 0 {
        return None;
    }
    let used = sys.used_swap() as f64 / 1024.0 / 1024.0 / 1024.0;
    let total = total as f64 / 1024.0 / 1024.0 / 1024.0;
    let percent = if total > 0.0 { (used / total * 100.0) as u64 } else { 0 };
    Some(format!("{:.2} GiB / {:.2} GiB ({}%)", used, total, percent))
}

pub fn get_cpu(sys: &System) -> String {
    sys.cpus()
        .first()
        .map(|cpu| {
            let brand = cpu.brand().to_string();
            let cores = sys.cpus().len();

            // Try to get max boost frequency from cpufreq (more accurate with PBO)
            let freq = if let Ok(max_freq) = fs::read_to_string("/sys/devices/system/cpu/cpu0/cpufreq/cpuinfo_max_freq") {
                // cpuinfo_max_freq is in kHz
                max_freq.trim().parse::<f64>().unwrap_or(0.0) / 1_000_000.0
            } else {
                // Fallback to sysinfo
                cpu.frequency() as f64 / 1000.0
            };

            // Clean up CPU name
            let clean_brand = brand
                .replace("(R)", "")
                .replace("(TM)", "")
                .replace("CPU ", "")
                .replace("  ", " ")
                .trim()
                .to_string();

            format!("{} ({}) @ {:.2} GHz", clean_brand, cores, freq)
        })
        .unwrap_or_else(|| "Unknown".to_string())
}

pub fn get_gpu() -> Vec<String> {
    let mut gpus = Vec::new();

    if let Ok(output) = Command::new("lspci").args(["-mm"]).output() {
        let lspci = String::from_utf8_lossy(&output.stdout);
        for line in lspci.lines() {
            if line.contains("VGA") || line.contains("3D") || line.contains("Display") {
                // Parse the quoted fields
                let fields: Vec<&str> = line.split('"').collect();
                if fields.len() >= 6 {
                    let vendor = fields[3];
                    let device = fields[5];

                    // Determine if discrete or integrated
                    let is_integrated = device.to_lowercase().contains("radeon graphics")
                        || device.to_lowercase().contains("intel") && device.to_lowercase().contains("graphics")
                        || device.to_lowercase().contains("integrated")
                        || device.to_lowercase().contains("granite ridge");

                    // Clean up vendor name
                    let clean_vendor = vendor
                        .replace("NVIDIA Corporation", "NVIDIA")
                        .replace("Advanced Micro Devices, Inc. [AMD/ATI]", "AMD")
                        .replace("Intel Corporation", "Intel");

                    // Parse device: "GB202 [GeForce RTX 5090]" -> "GeForce RTX 5090 [GB202]"
                    let gpu_name = if device.contains('[') && device.contains(']') {
                        // Format: "CHIPNAME [Product Name]"
                        let bracket_start = device.find('[').unwrap();
                        let bracket_end = device.find(']').unwrap();
                        let chip = device[..bracket_start].trim();
                        let product = &device[bracket_start + 1..bracket_end];

                        if is_integrated {
                            // For iGPU: "AMD Radeon [Granite Ridge]"
                            format!("{} {} [{}]", clean_vendor, product.replace(" Graphics", ""), chip)
                        } else {
                            // For dGPU: "NVIDIA GeForce RTX 5090 [GB202]"
                            format!("{} {} [{}]", clean_vendor, product, chip)
                        }
                    } else {
                        format!("{} {}", clean_vendor, device)
                    };

                    let gpu_type = if is_integrated { "[iGPU]" } else { "[dGPU]" };
                    gpus.push(format!("{} {}", gpu_name, gpu_type));
                }
            }
        }
    }

    if gpus.is_empty() {
        gpus.push("Unknown".to_string());
    }

    gpus
}

pub struct MonitorInfo {
    pub name: String,
    pub resolution: String,
    pub refresh_rate: String,
    pub size: Option<String>,
    pub hdr: bool,
}

pub fn get_monitors() -> Vec<MonitorInfo> {
    let mut monitors = Vec::new();

    // First try to get monitor names from EDID
    let mut edid_names: std::collections::HashMap<String, String> = std::collections::HashMap::new();

    if let Ok(entries) = fs::read_dir("/sys/class/drm") {
        for entry in entries.flatten() {
            let path = entry.path();
            let dir_name = path.file_name().unwrap_or_default().to_string_lossy().to_string();

            if dir_name.starts_with("card") && dir_name.contains('-') {
                let edid_path = path.join("edid");
                if edid_path.exists() {
                    if let Ok(edid_bytes) = fs::read(&edid_path) {
                        if edid_bytes.len() >= 128 {
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
                    let name = edid_names.get(&current_output)
                        .cloned()
                        .unwrap_or(current_output.clone());
                    monitors.push(MonitorInfo {
                        name,
                        resolution: current_res.clone(),
                        refresh_rate: current_rate.clone(),
                        size: None,
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
            let name = edid_names.get(&current_output)
                .cloned()
                .unwrap_or(current_output.clone());
            monitors.push(MonitorInfo {
                name,
                resolution: current_res,
                refresh_rate: current_rate,
                size: None,
                hdr: has_hdr,
            });
        }
    }

    // Fallback to xrandr if kscreen-doctor didn't work
    if monitors.is_empty() && std::env::var("DISPLAY").is_ok() {
        if let Ok(output) = Command::new("xrandr").args(["--query"]).output() {
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

                        let name = edid_names.get(&current_output)
                            .cloned()
                            .unwrap_or(current_output.clone());

                        monitors.push(MonitorInfo {
                            name,
                            resolution: res.to_string(),
                            refresh_rate: format!("{} Hz", rate),
                            size: None,
                            hdr: false,
                        });
                        current_output.clear();
                    }
                }
            }
        }
    }

    monitors
}

fn strip_ansi_codes(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '\x1b' {
            // Skip escape sequence
            if chars.peek() == Some(&'[') {
                chars.next();
                // Skip until we hit a letter
                while let Some(&next) = chars.peek() {
                    chars.next();
                    if next.is_ascii_alphabetic() {
                        break;
                    }
                }
            }
        } else {
            result.push(c);
        }
    }
    result
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
                .filter(|&&b| b >= 0x20 && b < 0x7F)
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

pub fn get_disks() -> Vec<String> {
    let disks = Disks::new_with_refreshed_list();
    let mut disk_info = Vec::new();

    for disk in disks.list() {
        let mount = disk.mount_point().to_string_lossy();

        // Only show important mount points
        if mount == "/" || mount.starts_with("/home") || mount.starts_with("/data")
           || mount.starts_with("/mnt") || mount.starts_with("/media") {
            let total = disk.total_space() as f64 / 1024.0 / 1024.0 / 1024.0;
            let available = disk.available_space() as f64 / 1024.0 / 1024.0 / 1024.0;
            let used = total - available;
            let percent = if total > 0.0 { (used / total * 100.0) as u64 } else { 0 };

            let fs = disk.file_system().to_string_lossy();

            // Use TiB for large disks
            if total >= 1024.0 {
                let used_tib = used / 1024.0;
                let total_tib = total / 1024.0;
                disk_info.push(format!(
                    "({}) {:.2} TiB / {:.2} TiB ({}%) - {}",
                    mount, used_tib, total_tib, percent, fs
                ));
            } else {
                disk_info.push(format!(
                    "({}) {:.2} GiB / {:.2} GiB ({}%) - {}",
                    mount, used, total, percent, fs
                ));
            }
        }
    }

    disk_info
}

pub fn get_init_system() -> String {
    if fs::metadata("/run/systemd/system").is_ok() {
        "systemd".to_string()
    } else if fs::metadata("/run/openrc").is_ok() {
        "OpenRC".to_string()
    } else if fs::metadata("/run/runit").is_ok() {
        "runit".to_string()
    } else if fs::metadata("/run/s6").is_ok() {
        "s6".to_string()
    } else if fs::metadata("/sbin/dinit").is_ok() {
        "dinit".to_string()
    } else {
        "Unknown".to_string()
    }
}

pub fn get_network_info() -> Vec<(String, String)> {
    let mut networks = Vec::new();

    if let Ok(output) = Command::new("ip")
        .args(["-4", "addr", "show", "scope", "global"])
        .output()
    {
        let ip_output = String::from_utf8_lossy(&output.stdout);
        let mut current_iface = String::new();

        for line in ip_output.lines() {
            // Interface line: "2: enp6s0: <BROADCAST..."
            if !line.starts_with(' ') && line.contains(':') {
                if let Some(iface) = line.split(':').nth(1) {
                    current_iface = iface.trim().to_string();
                }
            }
            // IP line: "    inet 10.0.0.21/24 ..."
            if line.trim().starts_with("inet ") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    let ip = parts[1].to_string();
                    networks.push((current_iface.clone(), ip));
                }
            }
        }
    }

    networks
}

pub fn get_local_ip() -> String {
    let networks = get_network_info();

    if networks.is_empty() {
        return "Unknown".to_string();
    }

    // Prioritize bridges (br0, br1, etc.) and bonds, then regular interfaces
    let bridge = networks.iter().find(|(iface, _)| iface.starts_with("br"));
    let bond = networks.iter().find(|(iface, _)| iface.starts_with("bond"));

    // If we have a bridge, show it prominently
    if let Some((iface, ip)) = bridge {
        // Also find primary non-bridge interface
        let primary = networks.iter()
            .find(|(i, _)| !i.starts_with("br") && !i.starts_with("bond") && !i.starts_with("veth") && !i.starts_with("docker") && !i.starts_with("virbr"));

        if let Some((p_iface, p_ip)) = primary {
            return format!("{} ({}), {} ({})", ip, iface, p_ip, p_iface);
        }
        return format!("{} ({})", ip, iface);
    }

    // If we have a bond, show it
    if let Some((iface, ip)) = bond {
        return format!("{} ({})", ip, iface);
    }

    // Regular interface - skip virtual ones
    for (iface, ip) in &networks {
        if !iface.starts_with("veth") && !iface.starts_with("docker") && !iface.starts_with("virbr") {
            return format!("{}", ip);
        }
    }

    // Fallback to first
    networks.first().map(|(_, ip)| ip.clone()).unwrap_or_else(|| "Unknown".to_string())
}

pub fn get_locale() -> String {
    std::env::var("LANG").unwrap_or_else(|_| "Unknown".to_string())
}

pub fn get_terminal_font() -> Option<String> {
    // Try ghostty config
    if let Ok(home) = std::env::var("HOME") {
        let ghostty_config = format!("{}/.config/ghostty/config", home);
        if let Ok(content) = fs::read_to_string(&ghostty_config) {
            for line in content.lines() {
                let line = line.trim();
                if line.starts_with("font-family") {
                    if let Some(font) = line.split('=').nth(1) {
                        return Some(font.trim().trim_matches('"').to_string());
                    }
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
                if line.starts_with("font_family") {
                    if let Some(font) = line.split_whitespace().skip(1).next() {
                        return Some(font.to_string());
                    }
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
                    if line.contains("family") && !line.trim().starts_with('#') {
                        if let Some(font) = line.split(['=', ':']).nth(1) {
                            let font = font.trim().trim_matches('"').trim_matches('\'');
                            if !font.is_empty() {
                                return Some(font.to_string());
                            }
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
                if path.extension().map(|e| e == "profile").unwrap_or(false) {
                    if let Ok(content) = fs::read_to_string(&path) {
                        for line in content.lines() {
                            if line.starts_with("Font=") {
                                if let Some(font) = line.strip_prefix("Font=") {
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
        }
    }

    None
}

pub fn get_editor() -> Option<String> {
    // Check EDITOR and VISUAL env vars
    let editor_var = std::env::var("EDITOR")
        .or_else(|_| std::env::var("VISUAL"))
        .ok()?;

    let editor_name = std::path::Path::new(&editor_var)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or(&editor_var);

    // Get version for known editors
    let (cmd, display_name) = match editor_name {
        "nvim" | "neovim" => ("nvim", "Neovim"),
        "vim" => ("vim", "Vim"),
        "nano" => ("nano", "Nano"),
        "emacs" => ("emacs", "Emacs"),
        "code" | "code-oss" => ("code", "VS Code"),
        "hx" | "helix" => ("hx", "Helix"),
        "micro" => ("micro", "Micro"),
        "kate" => ("kate", "Kate"),
        "gedit" => ("gedit", "gedit"),
        "sublime_text" | "subl" => ("subl", "Sublime Text"),
        _ => (editor_name, editor_name),
    };

    // Try to get version
    if let Ok(output) = Command::new(cmd).arg("--version").output() {
        let version_out = String::from_utf8_lossy(&output.stdout);
        let first_line = version_out.lines().next().unwrap_or("");

        // Parse version from output
        if display_name == "Neovim" {
            // NVIM v0.10.0
            if let Some(ver) = first_line.strip_prefix("NVIM v") {
                return Some(format!("Neovim {}", ver.split_whitespace().next().unwrap_or(ver)));
            }
        } else if display_name == "Vim" {
            // VIM - Vi IMproved 9.0
            if let Some(pos) = first_line.find("Vi IMproved ") {
                let ver = &first_line[pos + 12..];
                return Some(format!("Vim {}", ver.split_whitespace().next().unwrap_or(ver)));
            }
        } else if display_name == "Helix" {
            // helix 24.07
            if let Some(ver) = first_line.strip_prefix("helix ") {
                return Some(format!("Helix {}", ver));
            }
        } else if display_name == "Nano" {
            // GNU nano, version 7.2
            if let Some(pos) = first_line.find("version ") {
                let ver = &first_line[pos + 8..];
                return Some(format!("Nano {}", ver.split_whitespace().next().unwrap_or(ver)));
            }
        } else if display_name == "Emacs" {
            // GNU Emacs 29.1
            if let Some(pos) = first_line.find("Emacs ") {
                let ver = &first_line[pos + 6..];
                return Some(format!("Emacs {}", ver.split_whitespace().next().unwrap_or(ver)));
            }
        }
    }

    Some(display_name.to_string())
}

pub fn get_shell_theme() -> Option<String> {
    let home = std::env::var("HOME").ok()?;
    let shell = std::env::var("SHELL").ok()?;

    // Read shell rc file once
    let zshrc = fs::read_to_string(format!("{}/.zshrc", home)).ok();
    let bashrc = fs::read_to_string(format!("{}/.bashrc", home)).ok();

    // Check what's actually initialized (order matters - last one wins)
    let has_starship_init = zshrc.as_ref().map(|rc| rc.contains("starship init")).unwrap_or(false)
        || bashrc.as_ref().map(|rc| rc.contains("starship init")).unwrap_or(false);

    let has_p10k = fs::metadata(format!("{}/.p10k.zsh", home)).is_ok();
    let has_p10k_source = zshrc.as_ref().map(|rc| rc.contains("source ~/.p10k.zsh") || rc.contains("source $HOME/.p10k.zsh")).unwrap_or(false);

    // If both are present, show both with indicator of which is active
    if has_starship_init && has_p10k {
        if let Some(ref rc) = zshrc {
            let starship_pos = rc.find("starship init");
            let p10k_pos = rc.find("source ~/.p10k.zsh")
                .or_else(|| rc.find("source $HOME/.p10k.zsh"));

            match (starship_pos, p10k_pos) {
                (Some(s), Some(p)) if p > s => {
                    // p10k comes after starship, p10k is active
                    return Some("Powerlevel10k, Starship".to_string());
                }
                (Some(s), Some(p)) if s > p => {
                    // starship comes after p10k, starship is active
                    return Some("Starship, Powerlevel10k".to_string());
                }
                _ => {}
            }
        }
        // Both present but can't determine order
        return Some("Starship, Powerlevel10k".to_string());
    }

    // Check for Starship
    if has_starship_init {
        return Some("Starship".to_string());
    }

    // Check for Powerlevel10k
    if has_p10k && has_p10k_source {
        return Some("Powerlevel10k".to_string());
    }
    if has_p10k {
        return Some("Powerlevel10k".to_string());
    }

    // Check zshrc for theme indicators
    if shell.contains("zsh") {
        if let Ok(zshrc) = fs::read_to_string(format!("{}/.zshrc", home)) {
            // Check for Oh My Zsh
            if zshrc.contains("oh-my-zsh") || zshrc.contains("ohmyzsh") {
                // Try to find the theme
                for line in zshrc.lines() {
                    let line = line.trim();
                    if line.starts_with("ZSH_THEME=") {
                        let theme = line
                            .strip_prefix("ZSH_THEME=")
                            .unwrap_or("")
                            .trim_matches('"')
                            .trim_matches('\'');
                        if theme == "powerlevel10k/powerlevel10k" {
                            return Some("Powerlevel10k (OMZ)".to_string());
                        }
                        return Some(format!("Oh My Zsh ({})", theme));
                    }
                }
                return Some("Oh My Zsh".to_string());
            }

            // Check for other frameworks
            if zshrc.contains("zinit") {
                return Some("Zinit".to_string());
            }
            if zshrc.contains("antigen") {
                return Some("Antigen".to_string());
            }
            if zshrc.contains("zplug") {
                return Some("Zplug".to_string());
            }
        }
    }

    // Check for bash-it
    if shell.contains("bash") {
        if fs::metadata(format!("{}/.bash_it", home)).is_ok() {
            return Some("Bash-it".to_string());
        }
    }

    // Check for Fish
    if shell.contains("fish") {
        if fs::metadata(format!("{}/.config/fish/functions/fish_prompt.fish", home)).is_ok() {
            return Some("Fish (custom)".to_string());
        }
        if let Ok(home) = std::env::var("HOME") {
            if fs::metadata(format!("{}/.local/share/omf", home)).is_ok() {
                return Some("Oh My Fish".to_string());
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

pub fn get_host() -> Option<String> {
    // Try to get product name (laptop/desktop model)
    if let Ok(product) = fs::read_to_string("/sys/devices/virtual/dmi/id/product_name") {
        let product = product.trim();
        if !product.is_empty() && product != "System Product Name" && product != "To Be Filled By O.E.M." {
            // Also try to get version for some systems
            if let Ok(version) = fs::read_to_string("/sys/devices/virtual/dmi/id/product_version") {
                let version = version.trim();
                if !version.is_empty() && version != "System Version" && version != "To Be Filled By O.E.M." {
                    return Some(format!("{} {}", product, version));
                }
            }
            return Some(product.to_string());
        }
    }

    // Try board name as fallback (for desktops)
    if let Ok(board) = fs::read_to_string("/sys/devices/virtual/dmi/id/board_name") {
        let board = board.trim();
        if !board.is_empty() && board != "To Be Filled By O.E.M." {
            if let Ok(vendor) = fs::read_to_string("/sys/devices/virtual/dmi/id/board_vendor") {
                let vendor = vendor.trim();
                return Some(format!("{} {}", vendor, board));
            }
            return Some(board.to_string());
        }
    }

    None
}
