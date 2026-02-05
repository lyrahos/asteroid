//! Configuration management for Asteroid Browser.
//!
//! Handles loading, saving, and providing access to user preferences.
//! Config file location: ~/.config/asteroid-browser/config.toml

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Main configuration structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub general: GeneralConfig,
    #[serde(default)]
    pub performance: PerformanceConfig,
    #[serde(default)]
    pub privacy: PrivacyConfig,
    #[serde(default)]
    pub engine: EngineConfig,
    #[serde(default)]
    pub ui: UiConfig,
    #[serde(default)]
    pub keybindings: KeybindingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
    /// Enable tab suspension after inactivity
    pub tab_suspension_enabled: bool,
    /// Seconds of inactivity before tab suspension
    pub tab_suspension_delay: u64,
    /// Show vertical tab sidebar
    pub vertical_tabs: bool,
    /// Home page URL
    pub home_page: String,
    /// Search engine URL template ({} = query)
    pub search_engine: String,
    /// Enable vim-style link hints
    pub vim_hints: bool,
    /// Download directory
    pub download_dir: String,
    /// Check for updates automatically
    pub auto_update_check: bool,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            tab_suspension_enabled: true,
            tab_suspension_delay: 300,
            vertical_tabs: false,
            home_page: "about:blank".to_string(),
            search_engine: "https://duckduckgo.com/?q={}".to_string(),
            vim_hints: false,
            download_dir: "~/Downloads".to_string(),
            auto_update_check: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    /// Enable hardware video acceleration
    pub hardware_acceleration: bool,
    /// Memory trim level: "off", "moderate", "aggressive"
    pub memory_trim_level: String,
    /// Disk cache size in megabytes
    pub cache_size_mb: u64,
    /// Memory cache size in megabytes
    pub memory_cache_mb: u64,
    /// Maximum number of active (non-suspended) tabs
    pub max_active_tabs: usize,
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            hardware_acceleration: true,
            memory_trim_level: "moderate".to_string(),
            cache_size_mb: 100,
            memory_cache_mb: 50,
            max_active_tabs: 10,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyConfig {
    /// Block ads
    pub block_ads: bool,
    /// Block trackers
    pub block_trackers: bool,
    /// Send Do Not Track header
    pub send_dnt: bool,
    /// Clear cookies on browser close
    pub clear_cookies_on_close: bool,
    /// HTTPS-only mode
    pub https_only: bool,
}

impl Default for PrivacyConfig {
    fn default() -> Self {
        Self {
            block_ads: true,
            block_trackers: true,
            send_dnt: false,
            clear_cookies_on_close: false,
            https_only: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineConfig {
    /// Current engine: "gecko" or "servo"
    pub current: String,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            current: "gecko".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiConfig {
    /// Auto-hide toolbar on idle
    pub auto_hide_toolbar: bool,
    /// Show status bar overlay
    pub show_status_overlay: bool,
    /// Default window width
    pub window_width: u32,
    /// Default window height
    pub window_height: u32,
    /// Enable developer tools
    pub developer_tools: bool,
}

impl Default for UiConfig {
    fn default() -> Self {
        Self {
            auto_hide_toolbar: false,
            show_status_overlay: true,
            window_width: 1280,
            window_height: 800,
            developer_tools: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeybindingConfig {
    pub focus_address_bar: String,
    pub new_tab: String,
    pub close_tab: String,
    pub tab_switcher: String,
    pub find_in_page: String,
    pub history: String,
    pub fullscreen: String,
    pub quick_find: String,
    pub toggle_sidebar: String,
    pub reload: String,
    pub back: String,
    pub forward: String,
}

impl Default for KeybindingConfig {
    fn default() -> Self {
        Self {
            focus_address_bar: "<Ctrl>l".to_string(),
            new_tab: "<Ctrl>t".to_string(),
            close_tab: "<Ctrl>w".to_string(),
            tab_switcher: "<Ctrl>Tab".to_string(),
            find_in_page: "<Ctrl>f".to_string(),
            history: "<Ctrl><Shift>h".to_string(),
            fullscreen: "F11".to_string(),
            quick_find: "slash".to_string(),
            toggle_sidebar: "F1".to_string(),
            reload: "F5".to_string(),
            back: "<Alt>Left".to_string(),
            forward: "<Alt>Right".to_string(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            general: GeneralConfig::default(),
            performance: PerformanceConfig::default(),
            privacy: PrivacyConfig::default(),
            engine: EngineConfig::default(),
            ui: UiConfig::default(),
            keybindings: KeybindingConfig::default(),
        }
    }
}

impl Config {
    /// Get the config file path.
    pub fn config_path() -> PathBuf {
        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("~/.config"));
        config_dir.join("asteroid-browser").join("config.toml")
    }

    /// Load configuration from disk, or return defaults.
    pub fn load() -> Self {
        let path = Self::config_path();

        if path.exists() {
            match std::fs::read_to_string(&path) {
                Ok(content) => match toml::from_str(&content) {
                    Ok(config) => {
                        log::info!("Loaded config from {}", path.display());
                        return config;
                    }
                    Err(e) => {
                        log::error!("Failed to parse config: {}", e);
                    }
                },
                Err(e) => {
                    log::error!("Failed to read config file: {}", e);
                }
            }
        }

        log::info!("Using default configuration");
        Self::default()
    }

    /// Save configuration to disk.
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let path = Self::config_path();

        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let content = toml::to_string_pretty(self)?;
        std::fs::write(&path, content)?;

        log::info!("Saved config to {}", path.display());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert!(config.general.tab_suspension_enabled);
        assert_eq!(config.general.tab_suspension_delay, 300);
        assert!(config.privacy.block_ads);
        assert!(config.privacy.block_trackers);
        assert_eq!(config.engine.current, "gecko");
    }

    #[test]
    fn test_config_serialization() {
        let config = Config::default();
        let serialized = toml::to_string_pretty(&config).unwrap();
        let deserialized: Config = toml::from_str(&serialized).unwrap();
        assert_eq!(config.general.tab_suspension_delay, deserialized.general.tab_suspension_delay);
        assert_eq!(config.engine.current, deserialized.engine.current);
    }

    #[test]
    fn test_config_path() {
        let path = Config::config_path();
        assert!(path.to_string_lossy().contains("asteroid-browser"));
        assert!(path.to_string_lossy().contains("config.toml"));
    }
}
