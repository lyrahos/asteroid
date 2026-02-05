# Asteroid Browser

A lightweight, high-performance web browser for Linux built with Rust and GTK4. Designed for low-resource systems while supporting modern web standards.

## Features

- **Minimal RAM Usage** - Target: <150MB idle, <300MB for 5 tabs
- **Gecko Engine** - Mozilla's rendering engine with performance optimizations
- **Engine Abstraction Layer** - Clean trait-based abstraction enabling future Servo migration
- **Built-in Content Blocking** - Ad and tracker blocking with EasyList/AdBlock Plus filter support
- **Hardware Video Acceleration** - VA-API integration with automatic fallback
- **Tab Suspension** - Automatic suspension of inactive tabs to reclaim memory
- **Memory Pressure Monitoring** - Reads `/proc/meminfo` and responds to low-memory conditions
- **Keyboard-First Design** - Vim-style link hints, comprehensive keyboard shortcuts
- **Auto-Update** - Checks GitHub Releases API for new versions
- **Vertical Tab Sidebar** - Optional vertical tab bar

## Architecture

The browser follows a three-layer architecture:

```
┌─────────────────────────────┐
│         UI Layer (GTK4)     │
├─────────────────────────────┤
│    Abstraction Layer        │
│   (BrowserEngine trait)     │
├─────────────────────────────┤
│     Engine Layer            │
│   (Gecko / Servo stub)      │
└─────────────────────────────┘
```

Engine selection is done at compile time via Cargo feature flags:
- `gecko-engine` (default)
- `servo-engine` (stub, for future development)

## Building

### Prerequisites

- Rust nightly toolchain
- GTK4 development libraries
- Gecko engine source (for full build)

### Ubuntu/Debian

```bash
sudo apt-get install libgtk-4-dev libva-dev build-essential \
  libglib2.0-dev libcairo2-dev libpango1.0-dev libgdk-pixbuf2.0-dev
```

### Fedora

```bash
dnf install gtk4-devel libva-devel gcc gcc-c++ \
  glib2-devel cairo-devel pango-devel gdk-pixbuf2-devel
```

### Compile

```bash
cargo build --release
```

To build with the Servo engine stub instead:

```bash
cargo build --release --no-default-features --features servo-engine
```

## Packaging

Build scripts are provided for multiple package formats:

```bash
# Debian/Ubuntu .deb package
./scripts/build-deb.sh

# Fedora/RHEL .rpm package
./scripts/build-rpm.sh

# Flatpak
./scripts/build-flatpak.sh
```

### Icon Generation

Generate PNG icons from the SVG logo:

```bash
./scripts/generate-icons.sh
```

Requires `inkscape` and `imagemagick`.

## GitHub Actions Workflows

All workflows are **manual only** (`workflow_dispatch`) and will not run automatically on push or pull requests.

- **Generate Icons** - Generate PNG icons from SVG logo files. Choose target environment (main/test/dev).
- **Build Installation Package** - Build .deb, .rpm, and/or Flatpak packages. Choose environment, package type, and optional version string.

Trigger workflows from the GitHub Actions tab.

## Benchmarks

Built-in benchmarking tools:

```bash
cargo run --bin bench-memory    # Memory usage benchmarks
cargo run --bin bench-video     # Video acceleration benchmarks
cargo run --bin bench-pageload  # Page load time benchmarks
```

Test pages are provided in `test-pages/` for benchmarking scenarios.

## Configuration

Configuration is stored in `~/.config/asteroid-browser/config.toml` and includes settings for:

- General (homepage, search engine, download directory)
- Performance (max tabs, suspension threshold, memory limits)
- Privacy (content blocking, DNT, cookie policy)
- UI (theme, tab position, toolbar visibility)
- Keyboard shortcuts

## Project Structure

```
asteroid-browser/
├── .github/workflows/       # Manual CI/CD workflows
├── gecko-config/             # Gecko engine configuration
│   ├── .mozconfig
│   └── prefs.js
├── resources/
│   ├── icons/                # Generated PNG icons
│   ├── filters/default.txt   # Default ad/tracker filter list
│   ├── ui/settings.css       # Settings page styles
│   ├── logo-static.svg       # Static app icon
│   ├── logo.svg              # Animated logo
│   └── asteroid-browser.desktop
├── scripts/                  # Build and icon generation scripts
├── src/
│   ├── core/                 # Core browser logic
│   │   ├── engine.rs         # BrowserEngine trait
│   │   ├── tab.rs            # Tab management
│   │   ├── memory.rs         # Memory monitoring
│   │   ├── blocker.rs        # Content blocking
│   │   ├── config.rs         # Configuration
│   │   └── updater.rs        # Auto-update
│   ├── engines/
│   │   ├── gecko/            # Gecko engine implementation
│   │   └── servo/            # Servo engine stub
│   ├── ui/                   # GTK4 user interface
│   │   ├── window.rs         # Main window
│   │   ├── toolbar.rs        # Navigation toolbar
│   │   ├── tab_bar.rs        # Tab sidebar
│   │   ├── settings.rs       # Settings page
│   │   └── shortcuts.rs      # Keyboard shortcuts
│   ├── bench/                # Benchmark binaries
│   └── main.rs               # Entry point
├── test-pages/               # HTML test pages
├── Cargo.toml
├── LICENSE
└── README.md
```

## Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Ctrl+T` | New tab |
| `Ctrl+W` | Close tab |
| `Ctrl+L` | Focus address bar |
| `Ctrl+R` | Reload |
| `Ctrl+Shift+R` | Hard reload |
| `Ctrl+Tab` | Next tab |
| `Ctrl+Shift+Tab` | Previous tab |
| `Ctrl+1-8` | Switch to tab N |
| `Ctrl+9` | Switch to last tab |
| `Ctrl+F` | Find in page |
| `F11` | Toggle fullscreen |
| `F` | Toggle link hints (vim-style) |
| `Ctrl+Shift+P` | New private window |

## License

MIT License - see [LICENSE](LICENSE) for details.
