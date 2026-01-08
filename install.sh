#!/usr/bin/env bash
#
# ghostfetch installer
# Copyright (c) 2026 CK Technology LLC
# Author: Christopher Kelley <ckelley@ghostkellz.sh>
# Licensed under MIT
#

set -e

VERSION="0.1.0"
REPO_URL="https://github.com/ghostkellz/ghostfetch"
INSTALL_DIR="/usr/local/bin"
ICON_DIR="/usr/share/icons/hicolor"
APP_DIR="/usr/share/applications"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

print_banner() {
    echo -e "${CYAN}"
    echo "   _____ _               _    __     _       _     "
    echo "  / ____| |             | |  / _|   | |     | |    "
    echo " | |  __| |__   ___  ___| |_| |_ ___| |_ ___| |__  "
    echo " | | |_ | '_ \\ / _ \\/ __| __|  _/ _ \\ __/ __| '_ \\ "
    echo " | |__| | | | | (_) \\__ \\ |_| ||  __/ || (__| | | |"
    echo "  \\_____|_| |_|\\___/|___/\\__|_| \\___|\\__\\___|_| |_|"
    echo -e "${NC}"
    echo -e "${GREEN}Fast, minimal system fetch tool for Linux${NC}"
    echo ""
}

info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

error() {
    echo -e "${RED}[ERROR]${NC} $1"
    exit 1
}

check_root() {
    if [[ $EUID -ne 0 ]]; then
        error "This script must be run as root (use sudo)"
    fi
}

detect_os() {
    if [[ -f /etc/os-release ]]; then
        . /etc/os-release
        OS_ID="${ID}"
        OS_ID_LIKE="${ID_LIKE:-}"
        OS_NAME="${PRETTY_NAME}"
    elif [[ -f /etc/lsb-release ]]; then
        . /etc/lsb-release
        OS_ID="${DISTRIB_ID,,}"
        OS_NAME="${DISTRIB_DESCRIPTION}"
    else
        OS_ID="unknown"
        OS_NAME="Unknown Linux"
    fi

    # Detect package manager and distro family
    if command -v pacman &> /dev/null; then
        PKG_MANAGER="pacman"
        DISTRO_FAMILY="arch"
    elif command -v dnf &> /dev/null; then
        PKG_MANAGER="dnf"
        DISTRO_FAMILY="fedora"
    elif command -v apt &> /dev/null; then
        PKG_MANAGER="apt"
        DISTRO_FAMILY="debian"
    elif command -v zypper &> /dev/null; then
        PKG_MANAGER="zypper"
        DISTRO_FAMILY="suse"
    elif command -v apk &> /dev/null; then
        PKG_MANAGER="apk"
        DISTRO_FAMILY="alpine"
    else
        PKG_MANAGER="unknown"
        DISTRO_FAMILY="unknown"
    fi

    info "Detected: ${OS_NAME}"
    info "Package manager: ${PKG_MANAGER}"
}

check_dependencies() {
    local missing=()

    # Check for Rust/Cargo (needed for building from source)
    if ! command -v cargo &> /dev/null; then
        missing+=("rust/cargo")
    fi

    if [[ ${#missing[@]} -gt 0 ]]; then
        warn "Missing dependencies: ${missing[*]}"
        echo ""
        read -p "Install missing dependencies? [y/N] " -n 1 -r
        echo
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            install_dependencies
        else
            error "Cannot continue without dependencies"
        fi
    fi
}

install_dependencies() {
    case $DISTRO_FAMILY in
        arch)
            pacman -S --needed --noconfirm rust
            ;;
        fedora)
            dnf install -y rust cargo
            ;;
        debian)
            apt update
            apt install -y rustc cargo
            ;;
        suse)
            zypper install -y rust cargo
            ;;
        alpine)
            apk add rust cargo
            ;;
        *)
            error "Unsupported distribution for automatic dependency installation"
            ;;
    esac
}

build_from_source() {
    info "Building ghostfetch from source..."

    # Check if we're in the repo directory
    if [[ -f "Cargo.toml" ]] && grep -q "ghostfetch" Cargo.toml; then
        info "Building in current directory..."
        cargo build --release
        BINARY_PATH="target/release/ghostfetch"

        # Check alternate target path
        if [[ ! -f "$BINARY_PATH" ]]; then
            BINARY_PATH=$(find target -name "ghostfetch" -type f -executable 2>/dev/null | head -1)
        fi
    else
        # Clone and build
        TEMP_DIR=$(mktemp -d)
        info "Cloning repository to ${TEMP_DIR}..."
        git clone --depth 1 "${REPO_URL}" "${TEMP_DIR}/ghostfetch"
        cd "${TEMP_DIR}/ghostfetch"
        cargo build --release
        BINARY_PATH="target/release/ghostfetch"

        if [[ ! -f "$BINARY_PATH" ]]; then
            BINARY_PATH=$(find target -name "ghostfetch" -type f -executable 2>/dev/null | head -1)
        fi
    fi

    if [[ ! -f "$BINARY_PATH" ]]; then
        error "Build failed - binary not found"
    fi

    info "Build successful!"
}

install_binary() {
    info "Installing binary to ${INSTALL_DIR}..."
    install -Dm755 "$BINARY_PATH" "${INSTALL_DIR}/ghostfetch"
}

install_icons() {
    info "Installing icons..."

    local icon_src=""
    if [[ -d "assets/icons/hicolor" ]]; then
        icon_src="assets/icons/hicolor"
    elif [[ -d "${TEMP_DIR}/ghostfetch/assets/icons/hicolor" ]]; then
        icon_src="${TEMP_DIR}/ghostfetch/assets/icons/hicolor"
    fi

    if [[ -n "$icon_src" ]]; then
        for size in 16x16 22x22 24x24 32x32 48x48 64x64 128x128 256x256 512x512; do
            if [[ -f "${icon_src}/${size}/apps/ghostfetch.png" ]]; then
                install -Dm644 "${icon_src}/${size}/apps/ghostfetch.png" \
                    "${ICON_DIR}/${size}/apps/ghostfetch.png"
            fi
        done
        info "Icons installed"
    else
        warn "Icon directory not found, skipping icon installation"
    fi
}

install_desktop_file() {
    info "Installing desktop file..."

    local desktop_src=""
    if [[ -f "ghostfetch.desktop" ]]; then
        desktop_src="ghostfetch.desktop"
    elif [[ -f "${TEMP_DIR}/ghostfetch/ghostfetch.desktop" ]]; then
        desktop_src="${TEMP_DIR}/ghostfetch/ghostfetch.desktop"
    fi

    if [[ -n "$desktop_src" ]]; then
        install -Dm644 "$desktop_src" "${APP_DIR}/ghostfetch.desktop"
        info "Desktop file installed"
    else
        warn "Desktop file not found, skipping"
    fi
}

update_caches() {
    info "Updating icon cache..."
    if command -v gtk-update-icon-cache &> /dev/null; then
        gtk-update-icon-cache -f -t "${ICON_DIR}" 2>/dev/null || true
    fi

    if command -v update-desktop-database &> /dev/null; then
        update-desktop-database "${APP_DIR}" 2>/dev/null || true
    fi
}

cleanup() {
    if [[ -n "${TEMP_DIR:-}" ]] && [[ -d "${TEMP_DIR}" ]]; then
        info "Cleaning up..."
        rm -rf "${TEMP_DIR}"
    fi
}

uninstall() {
    check_root
    info "Uninstalling ghostfetch..."

    rm -f "${INSTALL_DIR}/ghostfetch"
    rm -f "${APP_DIR}/ghostfetch.desktop"

    for size in 16x16 22x22 24x24 32x32 48x48 64x64 128x128 256x256 512x512; do
        rm -f "${ICON_DIR}/${size}/apps/ghostfetch.png"
    done

    update_caches
    info "ghostfetch has been uninstalled"
}

show_help() {
    echo "Usage: $0 [OPTIONS]"
    echo ""
    echo "Options:"
    echo "  --help, -h        Show this help message"
    echo "  --uninstall, -u   Uninstall ghostfetch"
    echo "  --version, -v     Show version"
    echo ""
    echo "This script will:"
    echo "  1. Detect your Linux distribution"
    echo "  2. Install required dependencies (Rust)"
    echo "  3. Build ghostfetch from source"
    echo "  4. Install the binary, icons, and desktop file"
}

main() {
    print_banner

    case "${1:-}" in
        --help|-h)
            show_help
            exit 0
            ;;
        --uninstall|-u)
            uninstall
            exit 0
            ;;
        --version|-v)
            echo "ghostfetch installer v${VERSION}"
            exit 0
            ;;
    esac

    check_root
    detect_os
    check_dependencies
    build_from_source
    install_binary
    install_icons
    install_desktop_file
    update_caches
    cleanup

    echo ""
    echo -e "${GREEN}============================================${NC}"
    echo -e "${GREEN}  ghostfetch installed successfully!${NC}"
    echo -e "${GREEN}============================================${NC}"
    echo ""
    echo "Run 'ghostfetch' to see your system info"
    echo "Run 'ghostfetch --help' for more options"
    echo ""
}

trap cleanup EXIT
main "$@"
