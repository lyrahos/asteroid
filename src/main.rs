//! Asteroid Browser - Lightweight, Fast, Independent
//!
//! A minimal-RAM, high-performance browser for Linux that maintains
//! independence from corporate control while supporting modern web standards.
//!
//! Uses Gecko (Firefox engine) initially with a clean abstraction layer
//! enabling future migration to Servo or other engines.

mod core;
mod engines;
mod ui;

use crate::core::blocker::{ContentBlocker, DEFAULT_FILTERS};
use crate::core::config::Config;
use crate::core::engine::ViewId;
use crate::core::memory::{
    handle_memory_pressure, monitor_memory_pressure_loop, MemoryMonitorConfig, MemoryPressure,
};
use crate::core::tab::{SuspensionConfig, TabManager};
use crate::core::updater;

use gtk4::prelude::*;
use gtk4::Application;
use std::time::Duration;

const APP_ID: &str = "com.asteroid.browser";

fn main() {
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

    // Start the GTK4 application
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(move |app| {
        let window = ui::window::build_window(app);
        ui::window::load_css();
        window.present();
    });

    // Set up async runtime for background tasks
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build();

    if let Ok(rt) = rt {
        rt.spawn(async move {
            // Start memory pressure monitor
            let mem_config = MemoryMonitorConfig::default();
            let (pressure_tx, mut pressure_rx) =
                tokio::sync::mpsc::channel::<MemoryPressure>(10);

            tokio::spawn(monitor_memory_pressure_loop(mem_config, pressure_tx));

            // Start update checker
            if config.general.auto_update_check {
                let (update_tx, mut update_rx) =
                    tokio::sync::mpsc::channel(1);
                updater::start_update_checker(update_tx);

                tokio::spawn(async move {
                    while let Some(info) = update_rx.recv().await {
                        log::info!(
                            "Update available: v{} - {}",
                            info.version,
                            info.release_url
                        );
                    }
                });
            }

            // Handle memory pressure events
            while let Some(pressure) = pressure_rx.recv().await {
                log::warn!("Memory pressure: {:?}", pressure);
            }
        });
    }

    // Run the GTK application
    let exit_code = app.run();

    // Cleanup
    if let Err(e) = engine.shutdown() {
        log::error!("Engine shutdown error: {}", e);
    }

    log::info!("Asteroid Browser exited with code: {}", exit_code);
}
