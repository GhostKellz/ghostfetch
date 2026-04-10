use crate::info::MonitorInfo;

/// Collected system information data structure.
/// Enables separation of data collection from rendering,
/// and future JSON output support.
#[allow(dead_code)]
pub struct SystemInfo {
    pub username: String,
    pub hostname: String,
    pub os_name: String,
    pub distro_id: String,
    pub kernel: String,
    pub uptime: String,
    pub packages: String,
    pub shell: String,
    pub shell_theme: Option<String>,
    pub de: String,
    pub wm: String,
    pub terminal: String,
    pub terminal_font: Option<String>,
    pub multiplexer: Option<String>,
    pub editor: Option<String>,
    pub cpu: String,
    pub gpus: Vec<String>,
    pub memory: String,
    pub swap: Option<String>,
    pub disks: Vec<String>,
    pub monitors: Vec<MonitorInfo>,
    pub local_ip: String,
    pub locale: String,
    pub host: Option<String>,
}
