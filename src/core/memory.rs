//! Memory pressure monitoring system for Asteroid Browser.
//!
//! Monitors system memory and triggers tab suspension and memory trimming
//! when available memory drops below configurable thresholds.

use crate::core::engine::{BrowserEngine, TrimLevel};
use crate::core::tab::TabManager;
use std::time::Duration;

/// Memory pressure levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryPressure {
    /// Normal memory availability
    Normal,
    /// Low memory - start conserving
    Low,
    /// Critical memory - aggressive conservation
    Critical,
}

/// System memory information.
#[derive(Debug, Clone, Default)]
pub struct SystemMemoryInfo {
    /// Total system memory in bytes
    pub total_bytes: u64,
    /// Available (free + reclaimable) memory in bytes
    pub available_bytes: u64,
    /// Used memory in bytes
    pub used_bytes: u64,
    /// Swap total in bytes
    pub swap_total_bytes: u64,
    /// Swap used in bytes
    pub swap_used_bytes: u64,
}

impl SystemMemoryInfo {
    /// Get available memory in megabytes.
    pub fn available_mb(&self) -> f64 {
        self.available_bytes as f64 / (1024.0 * 1024.0)
    }

    /// Get the percentage of memory used.
    pub fn usage_percent(&self) -> f64 {
        if self.total_bytes == 0 {
            return 0.0;
        }
        (self.used_bytes as f64 / self.total_bytes as f64) * 100.0
    }
}

/// Configuration for memory monitoring.
#[derive(Debug, Clone)]
pub struct MemoryMonitorConfig {
    /// Check interval
    pub check_interval: Duration,
    /// Available memory threshold for "low" state (bytes)
    pub low_threshold_bytes: u64,
    /// Available memory threshold for "critical" state (bytes)
    pub critical_threshold_bytes: u64,
    /// Whether monitoring is enabled
    pub enabled: bool,
}

impl Default for MemoryMonitorConfig {
    fn default() -> Self {
        Self {
            check_interval: Duration::from_secs(10),
            low_threshold_bytes: 512 * 1024 * 1024,      // 512 MB
            critical_threshold_bytes: 256 * 1024 * 1024,  // 256 MB
            enabled: true,
        }
    }
}

/// Reads system memory information from /proc/meminfo on Linux.
pub fn get_system_memory() -> SystemMemoryInfo {
    let mut info = SystemMemoryInfo::default();

    if let Ok(contents) = std::fs::read_to_string("/proc/meminfo") {
        for line in contents.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                let value_kb: u64 = parts[1].parse().unwrap_or(0);
                let value_bytes = value_kb * 1024;

                match parts[0] {
                    "MemTotal:" => info.total_bytes = value_bytes,
                    "MemAvailable:" => info.available_bytes = value_bytes,
                    "SwapTotal:" => info.swap_total_bytes = value_bytes,
                    "SwapFree:" => {
                        info.swap_used_bytes =
                            info.swap_total_bytes.saturating_sub(value_bytes);
                    }
                    _ => {}
                }
            }
        }
        info.used_bytes = info.total_bytes.saturating_sub(info.available_bytes);
    }

    info
}

/// Determine the current memory pressure level.
pub fn assess_memory_pressure(config: &MemoryMonitorConfig) -> MemoryPressure {
    let mem_info = get_system_memory();

    if mem_info.available_bytes < config.critical_threshold_bytes {
        MemoryPressure::Critical
    } else if mem_info.available_bytes < config.low_threshold_bytes {
        MemoryPressure::Low
    } else {
        MemoryPressure::Normal
    }
}

/// Respond to memory pressure by suspending tabs and trimming memory.
pub fn handle_memory_pressure(
    pressure: MemoryPressure,
    tab_manager: &mut TabManager,
    engine: &mut dyn BrowserEngine,
) {
    match pressure {
        MemoryPressure::Critical => {
            log::warn!("Critical memory pressure - suspending all inactive tabs");
            tab_manager.suspend_all_inactive(engine);
            if let Err(e) = engine.trim_memory(TrimLevel::Aggressive) {
                log::error!("Failed to trim memory: {}", e);
            }
        }
        MemoryPressure::Low => {
            log::info!("Low memory pressure - suspending oldest inactive tabs");
            tab_manager.suspend_oldest_inactive(3, engine);
            if let Err(e) = engine.trim_memory(TrimLevel::Moderate) {
                log::error!("Failed to trim memory: {}", e);
            }
        }
        MemoryPressure::Normal => {
            // Normal operation - just check for timed-out tabs
            tab_manager.check_suspensions(engine);
        }
    }
}

/// Monitor memory pressure in a loop (designed to run as an async task).
pub async fn monitor_memory_pressure_loop(
    config: MemoryMonitorConfig,
    pressure_tx: tokio::sync::mpsc::Sender<MemoryPressure>,
) {
    if !config.enabled {
        log::info!("Memory monitoring disabled");
        return;
    }

    log::info!(
        "Memory monitor started (check interval: {:?}, low: {}MB, critical: {}MB)",
        config.check_interval,
        config.low_threshold_bytes / (1024 * 1024),
        config.critical_threshold_bytes / (1024 * 1024)
    );

    loop {
        let pressure = assess_memory_pressure(&config);

        if pressure != MemoryPressure::Normal {
            if let Err(e) = pressure_tx.send(pressure).await {
                log::error!("Failed to send memory pressure event: {}", e);
                break;
            }
        }

        tokio::time::sleep(config.check_interval).await;
    }
}

/// Cache configuration for memory-efficient browsing.
#[derive(Debug, Clone)]
pub struct CacheConfig {
    /// Maximum disk cache size in bytes
    pub disk_cache_max_bytes: u64,
    /// Maximum memory cache size in bytes
    pub memory_cache_max_bytes: u64,
    /// Maximum image cache size in bytes
    pub image_cache_max_bytes: u64,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            disk_cache_max_bytes: 100 * 1024 * 1024,    // 100 MB
            memory_cache_max_bytes: 50 * 1024 * 1024,   // 50 MB
            image_cache_max_bytes: 30 * 1024 * 1024,     // 30 MB
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_memory_info() {
        let info = SystemMemoryInfo {
            total_bytes: 8 * 1024 * 1024 * 1024, // 8 GB
            available_bytes: 4 * 1024 * 1024 * 1024,
            used_bytes: 4 * 1024 * 1024 * 1024,
            ..Default::default()
        };
        assert!((info.available_mb() - 4096.0).abs() < 1.0);
        assert!((info.usage_percent() - 50.0).abs() < 0.1);
    }

    #[test]
    fn test_memory_monitor_config_default() {
        let config = MemoryMonitorConfig::default();
        assert!(config.enabled);
        assert_eq!(config.check_interval, Duration::from_secs(10));
        assert_eq!(config.low_threshold_bytes, 512 * 1024 * 1024);
        assert_eq!(config.critical_threshold_bytes, 256 * 1024 * 1024);
    }

    #[test]
    fn test_cache_config_default() {
        let config = CacheConfig::default();
        assert_eq!(config.disk_cache_max_bytes, 100 * 1024 * 1024);
        assert_eq!(config.memory_cache_max_bytes, 50 * 1024 * 1024);
    }
}
