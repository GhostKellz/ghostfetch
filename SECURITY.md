# Security Policy

## Supported Versions

Security fixes are applied to the latest development branch and the most recent tagged release.

Older releases should be treated as unsupported unless explicitly noted otherwise.

## Reporting a Vulnerability

Do not report undisclosed security issues in public GitHub issues.

Use GitHub's private vulnerability reporting for this repository if it is available. If private reporting is unavailable, contact the maintainer through GitHub and include:

- A description of the issue and its impact
- Steps to reproduce or a proof of concept
- Any affected versions or environments
- Suggested mitigations, if known

## Response Expectations

Reports will be triaged as quickly as possible. Valid issues will be acknowledged, reproduced when possible, fixed in a supported version, and disclosed responsibly after a patch is available.

## Dependency Hygiene

This project uses `cargo audit` to check the lockfile against RustSec advisories and should keep dependencies updated within compatible release ranges when security or maintenance fixes are available.

## Threat Model

### Scope
ghostfetch is a local-only system information tool. It:
- Reads system files (/proc, /sys, /etc)
- Reads user config files (~/.config, ~/.zshrc, etc.)
- Executes external commands (lspci, ip, pacman, etc.)
- Outputs to terminal with ANSI color codes

### Data Collection
All data is collected locally and displayed locally. No network requests are made. No data is logged or stored.

### External Command Execution
ghostfetch executes external commands for hardware/package detection:
- Commands use standard PATH resolution
- All outputs are parsed, not executed
- Command failures result in "Unknown" fallbacks, not errors

### Custom ASCII Files (--ascii)
Custom ASCII files are validated:
- Maximum size: 50KB
- Rejected sequences: OSC escapes (\x1b]), BEL (\x07)
- Basic ANSI color codes (\x1b[...m) are allowed

### Privileged Execution
Running with sudo may be needed for dmidecode (RAM speed). This does not grant ghostfetch any additional capabilities beyond what the user already has.

### Known Limitations
- PATH spoofing: If a malicious binary shadows a standard tool name in PATH, ghostfetch will execute it. This is a general system concern, not specific to ghostfetch.
- File content injection: Custom ASCII files could contain misleading text, but terminal control sequences are filtered.
