//! Engine abstraction layer for Asteroid Browser.
//!
//! Provides a stable, engine-agnostic interface that allows swapping
//! between rendering engines (Gecko, Servo) without changing the UI layer.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

/// Unique identifier for a browser view/tab.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ViewId(pub u64);

impl fmt::Display for ViewId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "View({})", self.0)
    }
}

/// Video decoder backends available for hardware acceleration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VideoDecoder {
    /// VA-API hardware decoder (Linux)
    VAAPI,
    /// Optimized FFmpeg software decoder (fallback)
    FFmpegOptimized,
    /// Default software decoder
    Software,
}

/// Memory trim aggressiveness levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrimLevel {
    /// Light trim - free unused caches
    Light,
    /// Moderate trim - reduce caches and free pooled memory
    Moderate,
    /// Aggressive trim - minimize memory footprint
    Aggressive,
}

/// Statistics about current memory usage.
#[derive(Debug, Clone, Default)]
pub struct MemoryStats {
    /// Total memory used by the engine in bytes
    pub total_bytes: u64,
    /// Memory used by JavaScript heaps
    pub js_heap_bytes: u64,
    /// Memory used by image caches
    pub image_cache_bytes: u64,
    /// Memory used by DOM trees
    pub dom_bytes: u64,
    /// Memory used by layout data
    pub layout_bytes: u64,
    /// Memory used by network caches
    pub network_cache_bytes: u64,
}

impl MemoryStats {
    /// Returns total memory in megabytes.
    pub fn total_mb(&self) -> f64 {
        self.total_bytes as f64 / (1024.0 * 1024.0)
    }
}

/// Navigation state for a view.
#[derive(Debug, Clone, Default)]
pub struct NavigationState {
    pub can_go_back: bool,
    pub can_go_forward: bool,
    pub is_loading: bool,
    pub url: String,
    pub title: String,
    pub progress: f64,
}

/// Events emitted by the engine to the UI layer.
#[derive(Debug, Clone)]
pub enum EngineEvent {
    /// Page title changed
    TitleChanged(ViewId, String),
    /// URL changed (navigation)
    UrlChanged(ViewId, String),
    /// Page load progress updated (0.0 - 1.0)
    LoadProgress(ViewId, f64),
    /// Page finished loading
    LoadFinished(ViewId),
    /// Page started loading
    LoadStarted(ViewId),
    /// Favicon available
    FaviconReady(ViewId, Vec<u8>),
    /// Navigation state changed
    NavigationStateChanged(ViewId, NavigationState),
    /// Console message from page
    ConsoleMessage(ViewId, String),
    /// Certificate error
    CertificateError(ViewId, String),
}

/// Result type for engine operations.
pub type EngineResult<T> = Result<T, EngineError>;

/// Errors that can occur during engine operations.
#[derive(Debug)]
pub enum EngineError {
    /// View not found
    ViewNotFound(ViewId),
    /// Engine initialization failed
    InitializationFailed(String),
    /// Navigation error
    NavigationError(String),
    /// Script execution error
    ScriptError(String),
    /// Memory operation error
    MemoryError(String),
    /// Video decoder error
    VideoError(String),
    /// Generic engine error
    Other(String),
}

impl fmt::Display for EngineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ViewNotFound(id) => write!(f, "View not found: {}", id),
            Self::InitializationFailed(msg) => write!(f, "Engine init failed: {}", msg),
            Self::NavigationError(msg) => write!(f, "Navigation error: {}", msg),
            Self::ScriptError(msg) => write!(f, "Script error: {}", msg),
            Self::MemoryError(msg) => write!(f, "Memory error: {}", msg),
            Self::VideoError(msg) => write!(f, "Video error: {}", msg),
            Self::Other(msg) => write!(f, "Engine error: {}", msg),
        }
    }
}

impl std::error::Error for EngineError {}

/// Core browser engine trait.
///
/// This trait defines the engine-agnostic interface that all rendering
/// engines must implement. The UI layer communicates exclusively through
/// this trait, enabling clean engine swaps.
pub trait BrowserEngine: Send {
    /// Initialize the engine. Must be called before any other operations.
    fn initialize(&mut self) -> EngineResult<()>;

    /// Shut down the engine and release all resources.
    fn shutdown(&mut self) -> EngineResult<()>;

    /// Create a new browser view (tab).
    fn create_view(&mut self, view_id: ViewId) -> EngineResult<()>;

    /// Load a URL in the specified view.
    fn load_url(&mut self, view_id: ViewId, url: &str) -> EngineResult<()>;

    /// Load raw HTML content into the specified view.
    fn load_html(&mut self, view_id: ViewId, html: &str, base_url: &str) -> EngineResult<()>;

    /// Navigate back in the specified view.
    fn go_back(&mut self, view_id: ViewId) -> EngineResult<()>;

    /// Navigate forward in the specified view.
    fn go_forward(&mut self, view_id: ViewId) -> EngineResult<()>;

    /// Reload the current page in the specified view.
    fn reload(&mut self, view_id: ViewId) -> EngineResult<()>;

    /// Stop loading the current page.
    fn stop(&mut self, view_id: ViewId) -> EngineResult<()>;

    /// Execute JavaScript in the specified view and return the result.
    fn execute_script(&mut self, view_id: ViewId, script: &str) -> EngineResult<serde_json::Value>;

    /// Suspend a view to save memory (serialize state, release resources).
    fn suspend_view(&mut self, view_id: ViewId) -> EngineResult<()>;

    /// Resume a previously suspended view.
    fn resume_view(&mut self, view_id: ViewId) -> EngineResult<()>;

    /// Destroy a view and release all its resources.
    fn destroy_view(&mut self, view_id: ViewId) -> EngineResult<()>;

    /// Set the video decoder backend.
    fn set_video_decoder(&mut self, decoder: VideoDecoder) -> EngineResult<()>;

    /// Enable or disable hardware acceleration.
    fn enable_hardware_acceleration(&mut self, enabled: bool) -> EngineResult<()>;

    /// Get current memory usage statistics.
    fn get_memory_usage(&self) -> MemoryStats;

    /// Trim memory according to the specified aggressiveness level.
    fn trim_memory(&mut self, level: TrimLevel) -> EngineResult<()>;

    /// Get navigation state for a view.
    fn get_navigation_state(&self, view_id: ViewId) -> EngineResult<NavigationState>;

    /// Find text in the current page.
    fn find_in_page(&mut self, view_id: ViewId, query: &str, forward: bool) -> EngineResult<()>;

    /// Clear the find-in-page highlight.
    fn clear_find(&mut self, view_id: ViewId) -> EngineResult<()>;

    /// Get the engine name and version.
    fn engine_info(&self) -> (String, String);

    /// Poll for pending events from the engine.
    fn poll_events(&mut self) -> Vec<EngineEvent>;
}

/// Factory function type for creating engine instances.
pub type EngineFactory = fn() -> Box<dyn BrowserEngine>;

/// Registry of available engines.
pub struct EngineRegistry {
    engines: HashMap<String, EngineFactory>,
    default_engine: String,
}

impl EngineRegistry {
    pub fn new() -> Self {
        Self {
            engines: HashMap::new(),
            default_engine: String::new(),
        }
    }

    /// Register an engine factory.
    pub fn register(&mut self, name: &str, factory: EngineFactory) {
        self.engines.insert(name.to_string(), factory);
        if self.default_engine.is_empty() {
            self.default_engine = name.to_string();
        }
    }

    /// Set the default engine.
    pub fn set_default(&mut self, name: &str) {
        if self.engines.contains_key(name) {
            self.default_engine = name.to_string();
        }
    }

    /// Create an instance of the default engine.
    pub fn create_default(&self) -> Option<Box<dyn BrowserEngine>> {
        self.engines.get(&self.default_engine).map(|f| f())
    }

    /// Create an instance of a named engine.
    pub fn create(&self, name: &str) -> Option<Box<dyn BrowserEngine>> {
        self.engines.get(name).map(|f| f())
    }

    /// List available engine names.
    pub fn available_engines(&self) -> Vec<&str> {
        self.engines.keys().map(|s| s.as_str()).collect()
    }
}

impl Default for EngineRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_view_id_display() {
        let id = ViewId(42);
        assert_eq!(format!("{}", id), "View(42)");
    }

    #[test]
    fn test_memory_stats_total_mb() {
        let stats = MemoryStats {
            total_bytes: 104_857_600, // 100 MB
            ..Default::default()
        };
        assert!((stats.total_mb() - 100.0).abs() < 0.01);
    }

    #[test]
    fn test_engine_registry() {
        let registry = EngineRegistry::new();
        assert!(registry.available_engines().is_empty());
        assert!(registry.create_default().is_none());
    }
}
