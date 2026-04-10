use std::fs;
use std::process::Command;

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
                if part
                    .chars()
                    .next()
                    .map(|c| c.is_ascii_digit())
                    .unwrap_or(false)
                {
                    return format!("{} {}", shell, part);
                }
            }
        }
    }
    shell
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
    if counts.is_empty()
        && let Ok(output) = Command::new("dpkg-query")
            .args(["-f", ".\n", "-W"])
            .output()
    {
        let count = String::from_utf8_lossy(&output.stdout).lines().count();
        if count > 0 {
            counts.push(format!("{} (dpkg)", count));
        }
    }

    // rpm (fedora/rhel)
    if counts.is_empty()
        && let Ok(output) = Command::new("rpm").args(["-qa"]).output()
    {
        let count = String::from_utf8_lossy(&output.stdout).lines().count();
        if count > 0 {
            counts.push(format!("{} (rpm)", count));
        }
    }

    if counts.is_empty() {
        "Unknown".to_string()
    } else {
        counts.join(", ")
    }
}

pub fn get_shell_theme() -> Option<String> {
    let home = std::env::var("HOME").ok()?;
    let shell = std::env::var("SHELL").ok()?;

    // Read shell rc file once
    let zshrc = fs::read_to_string(format!("{}/.zshrc", home)).ok();
    let bashrc = fs::read_to_string(format!("{}/.bashrc", home)).ok();

    // Check what's actually initialized (order matters - last one wins)
    let has_starship_init = zshrc
        .as_ref()
        .map(|rc| rc.contains("starship init"))
        .unwrap_or(false)
        || bashrc
            .as_ref()
            .map(|rc| rc.contains("starship init"))
            .unwrap_or(false);

    let has_p10k = fs::metadata(format!("{}/.p10k.zsh", home)).is_ok();
    let has_p10k_source = zshrc
        .as_ref()
        .map(|rc| rc.contains("source ~/.p10k.zsh") || rc.contains("source $HOME/.p10k.zsh"))
        .unwrap_or(false);

    // If both are present, show both with indicator of which is active
    if has_starship_init && has_p10k {
        if let Some(ref rc) = zshrc {
            let starship_pos = rc.find("starship init");
            let p10k_pos = rc
                .find("source ~/.p10k.zsh")
                .or_else(|| rc.find("source $HOME/.p10k.zsh"));

            match (starship_pos, p10k_pos) {
                (Some(s), Some(p)) if p > s => {
                    return Some("Powerlevel10k, Starship".to_string());
                }
                (Some(s), Some(p)) if s > p => {
                    return Some("Starship, Powerlevel10k".to_string());
                }
                _ => {}
            }
        }
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
    if shell.contains("zsh")
        && let Ok(zshrc) = fs::read_to_string(format!("{}/.zshrc", home))
    {
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

    // Check for bash-it
    if shell.contains("bash") && fs::metadata(format!("{}/.bash_it", home)).is_ok() {
        return Some("Bash-it".to_string());
    }

    // Check for Fish
    if shell.contains("fish") {
        if fs::metadata(format!("{}/.config/fish/functions/fish_prompt.fish", home)).is_ok() {
            return Some("Fish (custom)".to_string());
        }
        if let Ok(home) = std::env::var("HOME")
            && fs::metadata(format!("{}/.local/share/omf", home)).is_ok()
        {
            return Some("Oh My Fish".to_string());
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
                return Some(format!(
                    "Neovim {}",
                    ver.split_whitespace().next().unwrap_or(ver)
                ));
            }
        } else if display_name == "Vim" {
            // VIM - Vi IMproved 9.0
            if let Some(pos) = first_line.find("Vi IMproved ") {
                let ver = &first_line[pos + 12..];
                return Some(format!(
                    "Vim {}",
                    ver.split_whitespace().next().unwrap_or(ver)
                ));
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
                return Some(format!(
                    "Nano {}",
                    ver.split_whitespace().next().unwrap_or(ver)
                ));
            }
        } else if display_name == "Emacs" {
            // GNU Emacs 29.1
            if let Some(pos) = first_line.find("Emacs ") {
                let ver = &first_line[pos + 6..];
                return Some(format!(
                    "Emacs {}",
                    ver.split_whitespace().next().unwrap_or(ver)
                ));
            }
        }
    }

    Some(display_name.to_string())
}
