#!/bin/bash
# Install Asteroid Browser with dependency checking.
# Detects the distro, checks for missing dependencies, and offers to install them.
#
# Usage:
#   ./scripts/install.sh          # Install from source build
#   ./scripts/install.sh --check  # Just check dependencies, don't install

set -e

PACKAGE_NAME="asteroid-browser"
BINARY_PATH="target/release/asteroid-browser"
INSTALL_PREFIX="${INSTALL_PREFIX:-/usr/local}"

# ---------- Colors ----------
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

info()  { echo -e "${CYAN}[*]${NC} $1"; }
ok()    { echo -e "${GREEN}[+]${NC} $1"; }
warn()  { echo -e "${YELLOW}[!]${NC} $1"; }
error() { echo -e "${RED}[-]${NC} $1"; }

# ---------- Detect distro ----------
detect_distro() {
    if [ -f /etc/os-release ]; then
        . /etc/os-release
        DISTRO_ID="$ID"
        DISTRO_NAME="$PRETTY_NAME"
    elif command -v lsb_release &>/dev/null; then
        DISTRO_ID="$(lsb_release -si | tr '[:upper:]' '[:lower:]')"
        DISTRO_NAME="$(lsb_release -sd)"
    else
        DISTRO_ID="unknown"
        DISTRO_NAME="Unknown Linux"
    fi

    # Normalize family
    case "$DISTRO_ID" in
        ubuntu|debian|linuxmint|pop|elementary|zorin|kali)
            DISTRO_FAMILY="debian"
            PKG_MGR="apt"
            ;;
        fedora|rhel|centos|rocky|alma|nobara)
            DISTRO_FAMILY="redhat"
            PKG_MGR="dnf"
            ;;
        arch|manjaro|endeavouros|garuda)
            DISTRO_FAMILY="arch"
            PKG_MGR="pacman"
            ;;
        opensuse*|suse*)
            DISTRO_FAMILY="suse"
            PKG_MGR="zypper"
            ;;
        *)
            DISTRO_FAMILY="unknown"
            PKG_MGR=""
            ;;
    esac
}

# ---------- Define dependencies per distro family ----------
# Each entry: "package_name:check_command_or_lib"
# check_command_or_lib is used to test if the dependency is already satisfied

get_runtime_deps() {
    case "$DISTRO_FAMILY" in
        debian)
            DEPS=(
                "libgtk-4-1:libgtk-4-1"
                "libglib2.0-0:libglib2.0-0"
                "libwebkitgtk-6.0-4:libwebkitgtk-6.0-4"
                "glib-networking:glib-networking"
                "libva2:libva2"
                "gstreamer1.0-plugins-bad:gstreamer1.0-plugins-bad"
                "gstreamer1.0-plugins-good:gstreamer1.0-plugins-good"
                "gstreamer1.0-vaapi:gstreamer1.0-vaapi"
            )
            ;;
        redhat)
            DEPS=(
                "gtk4:gtk4"
                "webkitgtk6.0:webkitgtk6.0"
                "glib-networking:glib-networking"
                "libva:libva"
                "gstreamer1-plugins-bad-free:gstreamer1-plugins-bad-free"
                "gstreamer1-plugins-good:gstreamer1-plugins-good"
            )
            ;;
        arch)
            DEPS=(
                "gtk4:gtk4"
                "webkitgtk-6.0:webkitgtk-6.0"
                "glib-networking:glib-networking"
                "libva:libva"
                "gst-plugins-bad:gst-plugins-bad"
                "gst-plugins-good:gst-plugins-good"
                "gstreamer-vaapi:gstreamer-vaapi"
            )
            ;;
        suse)
            DEPS=(
                "gtk4:libgtk-4-1"
                "webkitgtk6.0:libwebkitgtk-6_0-4"
                "glib-networking:glib-networking"
                "libva:libva2"
                "gstreamer-plugins-bad:gstreamer-plugins-bad"
                "gstreamer-plugins-good:gstreamer-plugins-good"
            )
            ;;
        *)
            DEPS=()
            ;;
    esac
}

get_build_deps() {
    case "$DISTRO_FAMILY" in
        debian)
            BUILD_DEPS=(
                "libgtk-4-dev:libgtk-4-dev"
                "libwebkitgtk-6.0-dev:libwebkitgtk-6.0-dev"
                "libva-dev:libva-dev"
                "gcc:gcc"
                "pkg-config:pkg-config"
            )
            ;;
        redhat)
            BUILD_DEPS=(
                "gtk4-devel:gtk4-devel"
                "webkitgtk6.0-devel:webkitgtk6.0-devel"
                "libva-devel:libva-devel"
                "gcc:gcc"
                "gcc-c++:gcc-c++"
            )
            ;;
        arch)
            BUILD_DEPS=(
                "gtk4:gtk4"
                "webkitgtk-6.0:webkitgtk-6.0"
                "libva:libva"
                "gcc:gcc"
                "pkgconf:pkgconf"
            )
            ;;
        *)
            BUILD_DEPS=()
            ;;
    esac
}

# ---------- Check if a package is installed ----------
is_pkg_installed() {
    local pkg_check="$1"
    case "$DISTRO_FAMILY" in
        debian)
            dpkg -s "$pkg_check" &>/dev/null 2>&1
            ;;
        redhat)
            rpm -q "$pkg_check" &>/dev/null 2>&1
            ;;
        arch)
            pacman -Qi "$pkg_check" &>/dev/null 2>&1
            ;;
        suse)
            rpm -q "$pkg_check" &>/dev/null 2>&1
            ;;
        *)
            return 1
            ;;
    esac
}

# ---------- Check dependencies and collect missing ones ----------
check_deps() {
    local dep_type="$1"
    shift
    local -a dep_list=("$@")
    local -a missing=()

    for entry in "${dep_list[@]}"; do
        local pkg="${entry%%:*}"
        local check="${entry##*:}"
        if ! is_pkg_installed "$check"; then
            missing+=("$pkg")
        fi
    done

    echo "${missing[@]}"
}

# ---------- Install packages ----------
install_packages() {
    local -a packages=("$@")
    if [ ${#packages[@]} -eq 0 ]; then
        return 0
    fi

    info "The following packages will be installed:"
    for pkg in "${packages[@]}"; do
        echo "    - $pkg"
    done
    echo ""

    read -rp "$(echo -e "${YELLOW}Install these packages? [Y/n]${NC} ")" answer
    case "$answer" in
        [nN]|[nN][oO])
            warn "Skipping dependency installation."
            warn "Asteroid Browser may not work correctly without these packages."
            return 1
            ;;
    esac

    info "Installing packages..."
    case "$PKG_MGR" in
        apt)
            sudo apt update -qq && sudo apt install -y "${packages[@]}"
            ;;
        dnf)
            sudo dnf install -y "${packages[@]}"
            ;;
        pacman)
            sudo pacman -S --needed --noconfirm "${packages[@]}"
            ;;
        zypper)
            sudo zypper install -y "${packages[@]}"
            ;;
    esac
}

# ---------- Main ----------
main() {
    echo ""
    echo "========================================="
    echo "  Asteroid Browser Installer"
    echo "========================================="
    echo ""

    detect_distro
    info "Detected: $DISTRO_NAME ($DISTRO_FAMILY)"

    if [ "$DISTRO_FAMILY" = "unknown" ]; then
        error "Unsupported distribution. Please install dependencies manually:"
        echo "  Runtime: GTK4, WebKitGTK 6.0, glib-networking, libva,"
        echo "           GStreamer plugins (bad, good), GStreamer VA-API"
        echo "  Build:   GTK4-dev, WebKitGTK 6.0-dev, libva-dev, gcc, pkg-config"
        exit 1
    fi

    # Check mode: just report, don't install
    if [ "${1:-}" = "--check" ]; then
        info "Checking dependencies..."
        echo ""

        get_runtime_deps
        local missing_rt
        missing_rt=$(check_deps "runtime" "${DEPS[@]}")

        get_build_deps
        local missing_build
        missing_build=$(check_deps "build" "${BUILD_DEPS[@]}")

        if [ -z "$missing_rt" ] && [ -z "$missing_build" ]; then
            ok "All dependencies are installed!"
            return 0
        fi

        if [ -n "$missing_rt" ]; then
            warn "Missing runtime dependencies:"
            for pkg in $missing_rt; do
                echo "    - $pkg"
            done
        else
            ok "All runtime dependencies installed."
        fi
        echo ""

        if [ -n "$missing_build" ]; then
            warn "Missing build dependencies:"
            for pkg in $missing_build; do
                echo "    - $pkg"
            done
        else
            ok "All build dependencies installed."
        fi
        return 0
    fi

    # Full install mode
    get_runtime_deps
    get_build_deps

    # Combine and check all deps
    local -a all_deps=("${DEPS[@]}" "${BUILD_DEPS[@]}")
    local missing
    missing=$(check_deps "all" "${all_deps[@]}")

    if [ -n "$missing" ]; then
        warn "Missing dependencies detected."
        echo ""
        # shellcheck disable=SC2086
        install_packages $missing || true
        echo ""
    else
        ok "All dependencies already installed."
    fi

    # Build if binary doesn't exist
    if [ ! -f "$BINARY_PATH" ]; then
        info "Building Asteroid Browser..."
        if command -v cargo &>/dev/null; then
            cargo build --release
        else
            error "Rust/Cargo not found. Install Rust first: https://rustup.rs"
            exit 1
        fi
    fi

    # Install binary and resources
    if [ -f "$BINARY_PATH" ]; then
        info "Installing to ${INSTALL_PREFIX}..."
        sudo install -Dm755 "$BINARY_PATH" "${INSTALL_PREFIX}/bin/asteroid-browser"

        # Install resources
        if [ -d "resources" ]; then
            sudo mkdir -p "/usr/share/asteroid-browser"
            sudo cp -r resources/* "/usr/share/asteroid-browser/"
        fi

        # Install desktop file
        if [ -f "resources/asteroid-browser.desktop" ]; then
            sudo install -Dm644 "resources/asteroid-browser.desktop" \
                "/usr/share/applications/asteroid-browser.desktop"
        fi

        # Install icons
        for size in 16 32 48 128 256; do
            local icon="resources/icons/${size}x${size}/asteroid-browser.png"
            if [ -f "$icon" ]; then
                sudo install -Dm644 "$icon" \
                    "/usr/share/icons/hicolor/${size}x${size}/apps/asteroid-browser.png"
            fi
        done

        echo ""
        ok "Asteroid Browser installed successfully!"
        info "Run with: asteroid-browser"
    else
        error "Binary not found at $BINARY_PATH. Build may have failed."
        exit 1
    fi
}

main "$@"
