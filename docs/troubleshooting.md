# Troubleshooting

## Common Issues

### "Unknown" GPU
**Cause**: `lspci` not installed or not finding GPU devices.

**Solution**:
```bash
# Install pciutils
sudo pacman -S pciutils  # Arch
sudo apt install pciutils  # Debian/Ubuntu

# Verify GPU is detected
lspci | grep -i vga
```

### No RAM Speed Shown
**Cause**: `dmidecode` not installed or requires root privileges.

**Solution**:
```bash
# Install dmidecode
sudo pacman -S dmidecode  # Arch

# Run with sudo to get RAM speed
sudo ghostfetch
```

### "Unknown" Monitor Name
**Cause**: EDID data not available or kscreen-doctor/xrandr not working.

**Solution**:
```bash
# Check EDID availability
ls /sys/class/drm/*/edid

# For KDE, install kscreen
sudo pacman -S kscreen

# For X11, ensure xrandr is available
xrandr --query
```

### Wrong Terminal Detected
**Cause**: Running inside tmux/zellij/screen obscures the actual terminal.

**Solution**: ghostfetch walks the process tree to find terminals. If detection fails, it falls back to the TERM environment variable. The actual terminal should still be detected via environment variables like GHOSTTY_BIN_DIR.

### No Shell Theme/Prompt Detected
**Cause**: Shell config files not in expected locations or using non-standard framework.

**Checks**:
- ~/.zshrc should contain "starship init" for Starship
- ~/.p10k.zsh should exist for Powerlevel10k
- ~/.zshrc should source p10k config

### Wrong Package Count
**Cause**: Package managers not in PATH or using non-standard installations.

**Solution**: Ensure package managers are accessible:
```bash
which pacman flatpak snap dpkg-query rpm
```

## Environment Issues

### Running Under sudo
Running with sudo may change environment-based detection results:
- TERM, SHELL, HOME may differ
- Terminal detection via process tree still works
- Shell theme detection may fail if HOME points to /root

### Headless/Container Systems
In containers or headless environments:
- Display detection will show "TTY"
- Monitor detection will be empty
- Desktop environment shows "Unknown"

## Reporting Issues

When reporting bugs, include:
1. ghostfetch version: `ghostfetch --version`
2. Distribution: `cat /etc/os-release`
3. Kernel: `uname -r`
4. Display server: echo $WAYLAND_DISPLAY or $DISPLAY
5. Relevant tool availability: `which lspci dmidecode kscreen-doctor xrandr`
