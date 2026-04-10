mod display;
mod hardware;
pub mod helpers;
mod host;
mod network;
mod software;

// Re-export public functions used by main.rs
pub use display::{
    MonitorInfo, get_de, get_monitors, get_multiplexer, get_terminal, get_terminal_font, get_wm,
};
pub use hardware::{get_cpu, get_disks, get_gpu, get_memory, get_swap};
pub use host::{get_host, get_hostname, get_locale, get_uptime, get_username};
pub use network::get_local_ip;
pub use software::{get_editor, get_kernel, get_os_info, get_packages, get_shell, get_shell_theme};
