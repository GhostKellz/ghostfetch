use std::process::Command;

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
            if !line.starts_with(' ')
                && line.contains(':')
                && let Some(iface) = line.split(':').nth(1)
            {
                current_iface = iface.trim().to_string();
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
        let primary = networks.iter().find(|(i, _)| {
            !i.starts_with("br")
                && !i.starts_with("bond")
                && !i.starts_with("veth")
                && !i.starts_with("docker")
                && !i.starts_with("virbr")
        });

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
        if !iface.starts_with("veth") && !iface.starts_with("docker") && !iface.starts_with("virbr")
        {
            return ip.to_string();
        }
    }

    // Fallback to first
    networks
        .first()
        .map(|(_, ip)| ip.clone())
        .unwrap_or_else(|| "Unknown".to_string())
}
