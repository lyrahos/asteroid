#!/bin/bash
# Build Flatpak package for Asteroid Browser
# Usage: VERSION=1.0.0 ./scripts/build-flatpak.sh

set -e

VERSION="${VERSION:-1.0.0}"
APP_ID="com.asteroid.browser"
BUILD_DIR="build/flatpak"
DIST_DIR="dist"

echo "=== Building Flatpak package v${VERSION} ==="

# Clean previous builds
rm -rf "$BUILD_DIR"
mkdir -p "$BUILD_DIR"
mkdir -p "$DIST_DIR"

# Create Flatpak manifest
cat > "$BUILD_DIR/${APP_ID}.yml" << EOF
app-id: ${APP_ID}
runtime: org.gnome.Platform
runtime-version: '46'
sdk: org.gnome.Sdk
sdk-extensions:
  - org.freedesktop.Sdk.Extension.rust-stable

command: asteroid-browser

finish-args:
  - --share=ipc
  - --socket=fallback-x11
  - --socket=wayland
  - --device=dri
  - --share=network
  - --socket=pulseaudio
  - --filesystem=xdg-download

modules:
  - name: asteroid-browser
    buildsystem: simple
    build-options:
      append-path: /usr/lib/sdk/rust-stable/bin
      env:
        CARGO_HOME: /run/build/asteroid-browser/cargo
    build-commands:
      - cargo build --release
      - install -Dm755 target/release/asteroid-browser /app/bin/asteroid-browser
      - install -Dm644 resources/asteroid-browser.desktop /app/share/applications/${APP_ID}.desktop
      - |
        for size in 16 32 48 128 256; do
          install -Dm644 "resources/icons/\${size}x\${size}/asteroid-browser.png" \
            "/app/share/icons/hicolor/\${size}x\${size}/apps/${APP_ID}.png" 2>/dev/null || true
        done
      - cp -r resources/ /app/share/asteroid-browser/
    sources:
      - type: dir
        path: ../../
EOF

# Build Flatpak
if command -v flatpak-builder &> /dev/null; then
    flatpak-builder --force-clean "$BUILD_DIR/build" "$BUILD_DIR/${APP_ID}.yml"

    # Bundle into .flatpak file
    flatpak build-bundle "$BUILD_DIR/repo" "$DIST_DIR/${APP_ID}-${VERSION}.flatpak" "$APP_ID"
else
    echo "Note: flatpak-builder not installed, manifest created at ${BUILD_DIR}/${APP_ID}.yml"
fi

echo "=== Flatpak package build complete ==="
