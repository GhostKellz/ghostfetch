use std::fs;

pub fn get_hostname() -> String {
    whoami::hostname().unwrap_or_else(|_| "Unknown".to_string())
}

pub fn get_username() -> String {
    whoami::username().unwrap_or_else(|_| "Unknown".to_string())
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

pub fn get_locale() -> String {
    std::env::var("LANG").unwrap_or_else(|_| "Unknown".to_string())
}

pub fn get_host() -> Option<String> {
    // Try to get product name (laptop/desktop model)
    if let Ok(product) = fs::read_to_string("/sys/devices/virtual/dmi/id/product_name") {
        let product = product.trim();
        if !product.is_empty()
            && product != "System Product Name"
            && product != "To Be Filled By O.E.M."
        {
            // Also try to get version for some systems
            if let Ok(version) = fs::read_to_string("/sys/devices/virtual/dmi/id/product_version") {
                let version = version.trim();
                if !version.is_empty()
                    && version != "System Version"
                    && version != "To Be Filled By O.E.M."
                {
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
