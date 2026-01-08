Name:           ghostfetch
Version:        0.1.0
Release:        1%{?dist}
Summary:        A fast, minimal system fetch tool for Linux

License:        MIT
URL:            https://github.com/ghostkellz/ghostfetch
Source0:        %{url}/archive/v%{version}/%{name}-%{version}.tar.gz

BuildRequires:  rust >= 1.70
BuildRequires:  cargo
BuildRequires:  gcc

Recommends:     dmidecode

%description
ghostfetch is a modern system information tool written in Rust,
inspired by neofetch, fastfetch, and betterfetch.

Features include:
- Distro-specific ASCII logos with colors
- Monitor detection with model names, resolution, refresh rate, HDR
- Display server detection (Wayland/X11)
- Shell prompt detection (Starship, Powerlevel10k, Oh My Zsh)
- Terminal font detection
- Multiple GPU support with discrete/integrated labels
- RAM speed detection (requires dmidecode)

%prep
%autosetup -n %{name}-%{version}

%build
cargo build --release %{?_smp_mflags}

%install
# Install binary
install -Dm755 target/release/%{name} %{buildroot}%{_bindir}/%{name}

# Install desktop file
install -Dm644 %{name}.desktop %{buildroot}%{_datadir}/applications/%{name}.desktop

# Install icons
for size in 16 22 24 32 48 64 128 256 512; do
    install -Dm644 assets/icons/hicolor/${size}x${size}/apps/%{name}.png \
        %{buildroot}%{_datadir}/icons/hicolor/${size}x${size}/apps/%{name}.png
done

# Install license
install -Dm644 LICENSE %{buildroot}%{_datadir}/licenses/%{name}/LICENSE

# Install docs
install -Dm644 README.md %{buildroot}%{_docdir}/%{name}/README.md

%files
%license LICENSE
%doc README.md
%{_bindir}/%{name}
%{_datadir}/applications/%{name}.desktop
%{_datadir}/icons/hicolor/*/apps/%{name}.png

%changelog
* Tue Jan 07 2026 Christopher Kelley <ckelley@ghostkellz.sh> - 0.1.0-1
- Initial package release
- Distro-specific ASCII logos with colors
- Monitor detection with model names, resolution, refresh rate, HDR
- Display server detection (Wayland/X11)
- Shell prompt detection (Starship, Powerlevel10k, Oh My Zsh)
- Terminal font detection
- Multiple GPU support
- RAM speed detection
