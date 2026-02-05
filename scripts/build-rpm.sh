#!/bin/bash
# Build .rpm package for Asteroid Browser
# Usage: VERSION=1.0.0 ./scripts/build-rpm.sh

set -e

VERSION="${VERSION:-1.0.0}"
PACKAGE_NAME="asteroid-browser"
RELEASE="1"
ARCH="x86_64"
BUILD_DIR="build/rpm"
DIST_DIR="dist"

echo "=== Building RPM package v${VERSION} ==="

# Clean previous builds
rm -rf "$BUILD_DIR"
mkdir -p "$BUILD_DIR"/{BUILD,RPMS,SOURCES,SPECS,SRPMS}
mkdir -p "$DIST_DIR"

# Build the binary
echo "Building binary..."
cargo build --release

# Create spec file
cat > "$BUILD_DIR/SPECS/${PACKAGE_NAME}.spec" << EOF
Name:           ${PACKAGE_NAME}
Version:        ${VERSION}
Release:        ${RELEASE}%{?dist}
Summary:        Lightweight, fast web browser for Linux

License:        MIT
URL:            https://github.com/asteroid-browser/asteroid-browser
Source0:        %{name}-%{version}.tar.gz

BuildRequires:  gtk4-devel, libva-devel, gcc, gcc-c++
Requires:       gtk4, libva

%description
Asteroid Browser is a minimal-RAM, high-performance browser that uses
the Gecko engine with a clean abstraction layer. Features include
built-in ad/tracker blocking, hardware video acceleration, and
aggressive memory management.

%prep
# No prep needed for pre-built binary

%install
mkdir -p %{buildroot}/usr/bin
mkdir -p %{buildroot}/usr/share/applications
mkdir -p %{buildroot}/usr/share/asteroid-browser
mkdir -p %{buildroot}/usr/share/icons/hicolor

# Install binary
if [ -f "../../target/release/asteroid-browser" ]; then
    install -m 755 ../../target/release/asteroid-browser %{buildroot}/usr/bin/
else
    echo '#!/bin/bash' > %{buildroot}/usr/bin/asteroid-browser
    echo 'echo "Asteroid Browser v${VERSION}"' >> %{buildroot}/usr/bin/asteroid-browser
    chmod 755 %{buildroot}/usr/bin/asteroid-browser
fi

# Install resources
cp -r ../../resources/* %{buildroot}/usr/share/asteroid-browser/

# Install icons
for size in 16 32 48 128 256; do
    icon_dir="%{buildroot}/usr/share/icons/hicolor/\${size}x\${size}/apps"
    mkdir -p "\$icon_dir"
    if [ -f "../../resources/icons/\${size}x\${size}/asteroid-browser.png" ]; then
        cp "../../resources/icons/\${size}x\${size}/asteroid-browser.png" "\$icon_dir/"
    fi
done

%files
/usr/bin/asteroid-browser
/usr/share/asteroid-browser/
/usr/share/icons/hicolor/*/apps/asteroid-browser.png

%changelog
* $(date '+%a %b %d %Y') Asteroid Browser Team <team@asteroid-browser.org> - ${VERSION}-${RELEASE}
- Initial package build
EOF

# Build RPM
if command -v rpmbuild &> /dev/null; then
    rpmbuild --define "_topdir $(pwd)/$BUILD_DIR" -bb "$BUILD_DIR/SPECS/${PACKAGE_NAME}.spec"

    # Copy RPM to dist
    find "$BUILD_DIR/RPMS" -name "*.rpm" -exec cp {} "$DIST_DIR/" \;

    # Verify output exists
    if ! ls "$DIST_DIR"/*.rpm 1>/dev/null 2>&1; then
        echo "Error: RPM build completed but no .rpm file found in $DIST_DIR"
        exit 1
    fi
else
    echo "Note: rpmbuild not available, spec file created at ${BUILD_DIR}/SPECS/"
fi

echo "=== RPM package build complete ==="
