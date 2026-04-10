# External Tools

ghostfetch uses external command-line tools for some detection features. These are optional - missing tools result in "Unknown" or omitted fields rather than errors.

## Required Tools

None. ghostfetch has no required external dependencies.

## Optional Tools

### Hardware Detection

| Tool | Package | Used For | Notes |
|------|---------|----------|-------|
| `lspci` | pciutils | GPU detection | Parses -mm output for vendor/device info |
| `dmidecode` | dmidecode | RAM speed | Requires root or cached data |

### Display Detection

| Tool | Package | Used For | Notes |
|------|---------|----------|-------|
| `kscreen-doctor` | kscreen | KDE monitor info | Best source for resolution, refresh, HDR |
| `xrandr` | xorg-xrandr | X11 monitor info | Fallback for non-KDE environments |
| `plasmashell` | plasma-desktop | KDE Plasma version | Version detection only |

### Package Counting

| Tool | Package | Used For |
|------|---------|----------|
| `pacman` | pacman | Arch-based package count |
| `flatpak` | flatpak | Flatpak application count |
| `snap` | snapd | Snap package count |
| `dpkg-query` | dpkg | Debian-based package count |
| `rpm` | rpm | RPM-based package count |

### Network Detection

| Tool | Package | Used For |
|------|---------|----------|
| `ip` | iproute2 | Local IP addresses |

### Terminal/Shell Detection

| Tool | Package | Used For |
|------|---------|----------|
| Various terminals | - | Version detection via --version |
| Various shells | - | Version detection via --version |

## Installation

### Arch Linux
```bash
sudo pacman -S pciutils dmidecode kscreen xorg-xrandr iproute2
```

### Debian/Ubuntu
```bash
sudo apt install pciutils dmidecode kscreen xrandr iproute2
```

### Fedora
```bash
sudo dnf install pciutils dmidecode kscreen xrandr iproute
```

## Permissions

### dmidecode
RAM speed detection requires either:
- Running ghostfetch with sudo
- Configuring dmidecode capabilities
- Having cached dmidecode data available

### EDID Reading
Monitor names from EDID require read access to /sys/class/drm/*/edid files, which is typically available to all users.
