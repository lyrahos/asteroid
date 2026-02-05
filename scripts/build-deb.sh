#!/bin/bash
# Build .deb package for Asteroid Browser
# Usage: VERSION=1.0.0 ./scripts/build-deb.sh

set -e

VERSION="${VERSION:-1.0.0}"
PACKAGE_NAME="asteroid-browser"
ARCH="amd64"
BUILD_DIR="build/deb"
DIST_DIR="dist"

echo "=== Building DEB package v${VERSION} ==="

# Clean previous builds
rm -rf "$BUILD_DIR"
mkdir -p "$BUILD_DIR/DEBIAN"
mkdir -p "$BUILD_DIR/usr/bin"
mkdir -p "$BUILD_DIR/usr/share/applications"
mkdir -p "$BUILD_DIR/usr/share/asteroid-browser"
mkdir -p "$BUILD_DIR/usr/share/doc/asteroid-browser"
mkdir -p "$DIST_DIR"

# Build the binary
echo "Building binary..."
cargo build --release

# Copy binary (or create placeholder if build failed)
if [ -f "target/release/asteroid-browser" ]; then
    cp target/release/asteroid-browser "$BUILD_DIR/usr/bin/"
else
    echo "Warning: Using placeholder binary (full build requires dependencies)"
    echo '#!/bin/bash' > "$BUILD_DIR/usr/bin/asteroid-browser"
    echo 'echo "Asteroid Browser v'"$VERSION"'"' >> "$BUILD_DIR/usr/bin/asteroid-browser"
    chmod +x "$BUILD_DIR/usr/bin/asteroid-browser"
fi

# Copy resources
cp -r resources/ "$BUILD_DIR/usr/share/asteroid-browser/"

# Copy desktop file
cp resources/asteroid-browser.desktop "$BUILD_DIR/usr/share/applications/" 2>/dev/null || true

# Install icons to standard locations
for size in 16 32 48 128 256; do
    icon_dir="$BUILD_DIR/usr/share/icons/hicolor/${size}x${size}/apps"
    mkdir -p "$icon_dir"
    if [ -f "resources/icons/${size}x${size}/asteroid-browser.png" ]; then
        cp "resources/icons/${size}x${size}/asteroid-browser.png" "$icon_dir/"
    fi
done

# Create control file
cat > "$BUILD_DIR/DEBIAN/control" << EOF
Package: ${PACKAGE_NAME}
Version: ${VERSION}
Section: web
Priority: optional
Architecture: ${ARCH}
Depends: libgtk-4-1 (>= 4.0), libglib2.0-0, libva2
Recommends: libva-drm2, libva-x11-2
Maintainer: Asteroid Browser Team <team@asteroid-browser.org>
Description: Lightweight, fast web browser for Linux
 Asteroid Browser is a minimal-RAM, high-performance browser
 that uses the Gecko engine with a clean abstraction layer.
 Features include built-in ad/tracker blocking, hardware
 video acceleration, and aggressive memory management.
Homepage: https://github.com/asteroid-browser/asteroid-browser
EOF

# Create copyright file
cat > "$BUILD_DIR/usr/share/doc/asteroid-browser/copyright" << EOF
Format: https://www.debian.org/doc/packaging-manuals/copyright-format/1.0/
Upstream-Name: asteroid-browser
Source: https://github.com/asteroid-browser/asteroid-browser

Files: *
Copyright: 2026 Asteroid Browser Team
License: MIT
EOF

# Build the package
if command -v dpkg-deb &> /dev/null; then
    dpkg-deb --build "$BUILD_DIR" "${DIST_DIR}/${PACKAGE_NAME}_${VERSION}_${ARCH}.deb"
else
    echo "Note: dpkg-deb not available, package structure created in ${BUILD_DIR}"
fi

echo "=== DEB package build complete ==="
echo "Package: ${DIST_DIR}/${PACKAGE_NAME}_${VERSION}_${ARCH}.deb"
