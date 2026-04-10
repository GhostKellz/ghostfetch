use std::fs;
use std::process::Command;
use sysinfo::{Disks, System};

pub fn get_cpu(sys: &System) -> String {
    sys.cpus()
        .first()
        .map(|cpu| {
            let brand = cpu.brand().to_string();
            let cores = sys.cpus().len();

            // Try to get max boost frequency from cpufreq (more accurate with PBO)
            let freq = if let Ok(max_freq) =
                fs::read_to_string("/sys/devices/system/cpu/cpu0/cpufreq/cpuinfo_max_freq")
            {
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
                        || device.to_lowercase().contains("intel")
                            && device.to_lowercase().contains("graphics")
                        || device.to_lowercase().contains("integrated")
                        || device.to_lowercase().contains("granite ridge");

                    // Clean up vendor name
                    let clean_vendor = vendor
                        .replace("NVIDIA Corporation", "NVIDIA")
                        .replace("Advanced Micro Devices, Inc. [AMD/ATI]", "AMD")
                        .replace("Intel Corporation", "Intel");

                    // Parse device: "GB202 [GeForce RTX 5090]" -> "GeForce RTX 5090 [GB202]"
                    // Fixed: use if-let instead of unwrap() for safety
                    let gpu_name = if let (Some(bracket_start), Some(bracket_end)) =
                        (device.find('['), device.find(']'))
                    {
                        let chip = device[..bracket_start].trim();
                        let product = &device[bracket_start + 1..bracket_end];

                        if is_integrated {
                            format!(
                                "{} {} [{}]",
                                clean_vendor,
                                product.replace(" Graphics", ""),
                                chip
                            )
                        } else {
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

pub fn get_memory(sys: &System) -> String {
    let used = sys.used_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
    let total = sys.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
    let percent = (used / total * 100.0) as u64;

    // Try to get RAM speed
    if let Some(speed) = get_ram_speed() {
        format!(
            "{:.2} GiB / {:.2} GiB ({}%) @ {} MT/s",
            used, total, percent, speed
        )
    } else {
        format!("{:.2} GiB / {:.2} GiB ({}%)", used, total, percent)
    }
}

fn get_ram_speed() -> Option<u32> {
    // Try dmidecode (requires root, but might be cached)
    if let Ok(output) = Command::new("dmidecode").args(["-t", "memory"]).output() {
        let dmi = String::from_utf8_lossy(&output.stdout);
        let mut configured_speed: Option<u32> = None;
        let mut base_speed: Option<u32> = None;

        for line in dmi.lines() {
            let line = line.trim();
            // Prefer "Configured Memory Speed:" (actual running speed with XMP/EXPO)
            if line.starts_with("Configured Memory Speed:") {
                if let Some(speed_str) = line.split(':').nth(1) {
                    let speed_str = speed_str.trim().replace(" MT/s", "").replace(" MHz", "");
                    if let Ok(speed) = speed_str.parse::<u32>()
                        && speed > 0
                        && speed < 100000
                    {
                        configured_speed = Some(speed);
                    }
                }
            // Fallback to "Speed:" (JEDEC base speed)
            } else if line.starts_with("Speed:")
                && line.contains("MT/s")
                && base_speed.is_none()
                && let Some(speed_str) = line.split(':').nth(1)
            {
                let speed_str = speed_str.trim().replace(" MT/s", "").replace(" MHz", "");
                if let Ok(speed) = speed_str.parse::<u32>()
                    && speed > 0
                    && speed < 100000
                {
                    base_speed = Some(speed);
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

    // Note: Removed dead code branches that never returned useful data:
    // - /sys/devices/system/edac/mc reads (incomplete implementation)
    // - lshw fallback (incomplete implementation)

    None
}

pub fn get_swap(sys: &System) -> Option<String> {
    let total = sys.total_swap();
    if total == 0 {
        return None;
    }
    let used = sys.used_swap() as f64 / 1024.0 / 1024.0 / 1024.0;
    let total = total as f64 / 1024.0 / 1024.0 / 1024.0;
    let percent = if total > 0.0 {
        (used / total * 100.0) as u64
    } else {
        0
    };
    Some(format!("{:.2} GiB / {:.2} GiB ({}%)", used, total, percent))
}

pub fn get_disks() -> Vec<String> {
    let disks = Disks::new_with_refreshed_list();
    let mut disk_info = Vec::new();

    for disk in disks.list() {
        let mount = disk.mount_point().to_string_lossy();

        // Only show important mount points
        if mount == "/"
            || mount.starts_with("/home")
            || mount.starts_with("/data")
            || mount.starts_with("/mnt")
            || mount.starts_with("/media")
        {
            let total = disk.total_space() as f64 / 1024.0 / 1024.0 / 1024.0;
            let available = disk.available_space() as f64 / 1024.0 / 1024.0 / 1024.0;
            let used = total - available;
            let percent = if total > 0.0 {
                (used / total * 100.0) as u64
            } else {
                0
            };

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
