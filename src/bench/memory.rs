//! Memory benchmark for Asteroid Browser.
//!
//! Measures memory usage across different scenarios:
//! - Idle (1 tab, blank)
//! - Simple page (Wikipedia-like)
//! - Multiple tabs (5 news-like sites)
//! - Heavy use (10 tabs + video)
//!
//! Targets:
//! - Idle: <150MB
//! - 5 tabs: <300MB
//! - 10 tabs + video: <600MB

mod core {
    pub mod engine;
    pub mod tab;
    pub mod memory;
    pub mod updater;
    pub mod blocker;
    pub mod config;
}
mod engines {
    pub mod gecko;
    pub mod servo;
    pub use gecko::GeckoEngine as DefaultEngine;
    pub fn create_default_engine() -> Box<dyn crate::core::engine::BrowserEngine> {
        Box::new(gecko::GeckoEngine::new())
    }
}

use crate::core::engine::{BrowserEngine, ViewId};
use crate::core::memory::get_system_memory;
use crate::core::tab::{SuspensionConfig, TabManager};
use std::time::Duration;

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    println!("=== Asteroid Browser Memory Benchmark ===\n");

    // System info
    let sys_mem = get_system_memory();
    println!(
        "System memory: {:.0}MB total, {:.0}MB available\n",
        sys_mem.total_bytes as f64 / (1024.0 * 1024.0),
        sys_mem.available_bytes as f64 / (1024.0 * 1024.0)
    );

    let mut engine = engines::create_default_engine();
    engine.initialize().expect("Failed to initialize engine");

    let config = SuspensionConfig::default();
    let mut tab_manager = TabManager::new(config);

    // Scenario 1: Idle (1 tab, blank)
    println!("--- Scenario 1: Idle (1 blank tab) ---");
    let view_id = tab_manager.create_tab(engine.as_mut()).unwrap();
    let stats = engine.get_memory_usage();
    let target = 150;
    println!(
        "  Memory: {:.1}MB (target: <{}MB) {}",
        stats.total_mb(),
        target,
        if stats.total_mb() < target as f64 { "PASS" } else { "FAIL" }
    );
    println!();

    // Scenario 2: Simple page
    println!("--- Scenario 2: Simple page ---");
    engine
        .load_url(view_id, "https://en.wikipedia.org")
        .unwrap();
    let stats = engine.get_memory_usage();
    let target = 180;
    println!(
        "  Memory: {:.1}MB (target: <{}MB) {}",
        stats.total_mb(),
        target,
        if stats.total_mb() < target as f64 { "PASS" } else { "FAIL" }
    );
    println!();

    // Scenario 3: 5 tabs (news sites)
    println!("--- Scenario 3: 5 tabs (news sites) ---");
    let urls = [
        "https://news.ycombinator.com",
        "https://www.bbc.com/news",
        "https://arstechnica.com",
        "https://www.theverge.com",
    ];
    for url in &urls {
        let vid = tab_manager.create_tab(engine.as_mut()).unwrap();
        engine.load_url(vid, url).unwrap();
    }
    let stats = engine.get_memory_usage();
    let target = 300;
    println!(
        "  Memory: {:.1}MB (target: <{}MB) {}",
        stats.total_mb(),
        target,
        if stats.total_mb() < target as f64 { "PASS" } else { "FAIL" }
    );
    println!("  Tabs: {} total, {} suspended", tab_manager.tab_count(), tab_manager.suspended_count());
    println!();

    // Scenario 4: 10 tabs + video
    println!("--- Scenario 4: 10 tabs + video ---");
    let more_urls = [
        "https://www.youtube.com/watch?v=test",
        "https://github.com",
        "https://stackoverflow.com",
        "https://reddit.com",
        "https://docs.rs",
    ];
    for url in &more_urls {
        let vid = tab_manager.create_tab(engine.as_mut()).unwrap();
        engine.load_url(vid, url).unwrap();
    }
    let stats = engine.get_memory_usage();
    let target = 600;
    println!(
        "  Memory: {:.1}MB (target: <{}MB) {}",
        stats.total_mb(),
        target,
        if stats.total_mb() < target as f64 { "PASS" } else { "FAIL" }
    );
    println!("  Tabs: {} total, {} suspended", tab_manager.tab_count(), tab_manager.suspended_count());
    println!();

    // Memory breakdown
    println!("--- Memory Breakdown ---");
    let stats = engine.get_memory_usage();
    println!("  Total:         {:.1}MB", stats.total_bytes as f64 / (1024.0 * 1024.0));
    println!("  JS Heap:       {:.1}MB", stats.js_heap_bytes as f64 / (1024.0 * 1024.0));
    println!("  Image Cache:   {:.1}MB", stats.image_cache_bytes as f64 / (1024.0 * 1024.0));
    println!("  DOM:           {:.1}MB", stats.dom_bytes as f64 / (1024.0 * 1024.0));
    println!("  Layout:        {:.1}MB", stats.layout_bytes as f64 / (1024.0 * 1024.0));
    println!("  Network Cache: {:.1}MB", stats.network_cache_bytes as f64 / (1024.0 * 1024.0));

    // Cleanup
    engine.shutdown().ok();

    println!("\n=== Benchmark Complete ===");
}
