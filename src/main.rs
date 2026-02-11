//! Asteroid Browser - Lightweight, Fast, Independent
//!
//! A minimal-RAM, high-performance browser for Linux that maintains
//! independence from corporate control while supporting modern web standards.
//!
//! Uses WebKitGTK (community-maintained) for rendering with a clean
//! abstraction layer enabling future engine migration.

mod core;
mod engines;
mod ui;

use crate::core::blocker::{ContentBlocker, DEFAULT_FILTERS};
use crate::core::config::Config;
use crate::core::tab::{SuspensionConfig, TabManager};
use crate::ui::window::BrowserState;

use gtk4::prelude::*;
use gtk4::Application;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

const APP_ID: &str = "com.asteroid.browser";

fn main() {
    // Raise file descriptor limit — WebKit subprocesses open many fds for
    // media-heavy sites (Twitch, YouTube). Default soft limit (1024) is too low.
    #[cfg(unix)]
    unsafe {
        let mut rlim = libc::rlimit {
            rlim_cur: 0,
            rlim_max: 0,
        };
        if libc::getrlimit(libc::RLIMIT_NOFILE, &mut rlim) == 0 {
            let target = rlim.rlim_max.min(65536);
            if rlim.rlim_cur < target {
                rlim.rlim_cur = target;
                libc::setrlimit(libc::RLIMIT_NOFILE, &rlim);
            }
        }
    }

    // Force GStreamer to prefer VA-API hardware video decoders.
    // Without this, GStreamer defaults to software decode (avdec_h264 via FFmpeg)
    // which uses ~15% CPU vs ~5% with hardware decode. WebKitGTK delegates all
    // media playback to GStreamer, and VA-API decoders ship with rank NONE so
    // they're never auto-selected unless we boost their rank.
    if std::env::var("GST_PLUGIN_FEATURE_RANK").is_err() {
        std::env::set_var(
            "GST_PLUGIN_FEATURE_RANK",
            "vah264dec:MAX,vah265dec:MAX,vavp9dec:MAX,vaav1dec:MAX",
        );
    }

    // Set process name so system monitor shows "Asteroid Browser" in Applications
    gtk4::glib::set_prgname(Some("asteroid-browser"));
    gtk4::glib::set_application_name("Asteroid Browser");

    // Initialize logging
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format_timestamp_millis()
        .init();

    log::info!(
        "Asteroid Browser v{} starting...",
        env!("CARGO_PKG_VERSION")
    );

    // Load configuration
    let config = Config::load();
    log::info!("Engine: {}", config.engine.current);

    // Initialize content blocker
    let mut blocker = ContentBlocker::new();
    if config.privacy.block_ads || config.privacy.block_trackers {
        blocker.add_filter_list(DEFAULT_FILTERS);
        log::info!(
            "Content blocker enabled ({} rules loaded)",
            blocker.stats().filter_count
        );
    } else {
        blocker.set_enabled(false);
        log::info!("Content blocker disabled");
    }

    // Create the engine
    let mut engine = engines::create_default_engine();
    if let Err(e) = engine.initialize() {
        log::error!("Failed to initialize engine: {}", e);
    }

    // Configure hardware acceleration
    if config.performance.hardware_acceleration {
        if let Err(e) = engine.enable_hardware_acceleration(true) {
            log::warn!("Failed to enable hardware acceleration: {}", e);
        }
    }

    // Set up tab manager
    let suspension_config = SuspensionConfig {
        enabled: config.general.tab_suspension_enabled,
        inactive_threshold: Duration::from_secs(config.general.tab_suspension_delay),
        max_active_tabs: config.performance.max_active_tabs,
        suspend_pinned: false,
    };
    let mut tab_manager = TabManager::new(suspension_config);

    // Create initial tab
    match tab_manager.create_tab(engine.as_mut()) {
        Ok(view_id) => {
            let home = &config.general.home_page;
            if let Err(e) = engine.load_url(view_id, home) {
                log::error!("Failed to load home page: {}", e);
            }
        }
        Err(e) => {
            log::error!("Failed to create initial tab: {}", e);
        }
    }

    // Wrap engine + tab manager in shared state for UI callbacks
    let state = Rc::new(RefCell::new(BrowserState {
        engine,
        tab_manager,
    }));

    // Start the GTK4 application
    let app = Application::builder().application_id(APP_ID).build();

    let state_for_window = state.clone();
    app.connect_activate(move |app| {
        let window = ui::window::build_window(app, state_for_window.clone());
        ui::window::load_css();
        window.present();
    });

    // Run the GTK application
    let exit_code = app.run();

    // Cleanup
    if let Ok(mut s) = state.try_borrow_mut() {
        if let Err(e) = s.engine.shutdown() {
            log::error!("Engine shutdown error: {}", e);
        }
    }

    log::info!("Asteroid Browser exited with code: {:?}", exit_code);
}
