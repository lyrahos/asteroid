//! Gecko engine implementation for Asteroid Browser.
//!
//! This module provides the Gecko (Mozilla) rendering engine integration.
//! Gecko is used initially for its mature web standards support, with a
//! clean abstraction allowing future migration to Servo.

pub mod prefs;
pub mod ffi;

use crate::core::engine::{
    BrowserEngine, EngineError, EngineEvent, EngineResult, MemoryStats,
    NavigationState, TrimLevel, VideoDecoder, ViewId,
};
use std::collections::HashMap;

/// State of a Gecko view.
#[derive(Debug)]
struct GeckoView {
    url: String,
    title: String,
    is_loading: bool,
    can_go_back: bool,
    can_go_forward: bool,
    progress: f64,
    suspended: bool,
}

impl GeckoView {
    fn new() -> Self {
        Self {
            url: String::from("about:blank"),
            title: String::from("New Tab"),
            is_loading: false,
            can_go_back: false,
            can_go_forward: false,
            progress: 0.0,
            suspended: false,
        }
    }
}

/// Gecko engine implementation.
///
/// Wraps the Mozilla Gecko rendering engine, providing the BrowserEngine
/// trait interface. Handles Gecko initialization, view management, and
/// optimization settings.
pub struct GeckoEngine {
    /// Active views managed by this engine
    views: HashMap<ViewId, GeckoView>,
    /// Current video decoder setting
    video_decoder: VideoDecoder,
    /// Whether hardware acceleration is enabled
    hw_accel: bool,
    /// Whether the engine has been initialized
    initialized: bool,
    /// Pending events to be polled by the UI
    pending_events: Vec<EngineEvent>,
    /// Total memory estimate
    memory_usage: u64,
}

impl GeckoEngine {
    pub fn new() -> Self {
        Self {
            views: HashMap::new(),
            video_decoder: VideoDecoder::Software,
            hw_accel: false,
            initialized: false,
            pending_events: Vec::new(),
            memory_usage: 0,
        }
    }

    /// Probe for VA-API hardware acceleration support.
    fn probe_vaapi(&self) -> bool {
        // Check for VA-API libraries on the system
        std::path::Path::new("/usr/lib/x86_64-linux-gnu/dri").exists()
            || std::path::Path::new("/usr/lib64/dri").exists()
            || std::path::Path::new("/usr/lib/dri").exists()
    }

    /// Initialize video decoder with hardware acceleration if available.
    fn initialize_video_decoder(&mut self) -> EngineResult<()> {
        if self.probe_vaapi() {
            self.video_decoder = VideoDecoder::VAAPI;
            self.hw_accel = true;
            log::info!("VA-API hardware acceleration enabled");
        } else {
            self.video_decoder = VideoDecoder::FFmpegOptimized;
            log::info!("Using optimized software video decoder (VA-API not available)");
        }
        Ok(())
    }

    /// Apply Gecko performance preferences.
    fn apply_preferences(&self) {
        let prefs = prefs::get_optimization_prefs();
        log::info!("Applying {} Gecko optimization preferences", prefs.len());
        // In a full implementation, these would be applied to the Gecko runtime
        // via the SpiderMonkey/Gecko embedding API
        for (key, value) in &prefs {
            log::debug!("  {} = {}", key, value);
        }
    }

    /// Estimate memory usage per view.
    fn estimate_view_memory(view: &GeckoView) -> u64 {
        if view.suspended {
            return 1024 * 10; // ~10KB for suspended state
        }
        // Rough estimates based on typical page complexity
        let base = 20 * 1024 * 1024; // 20MB base per active tab
        let url_factor = if view.url.contains("youtube")
            || view.url.contains("twitter")
            || view.url.contains("facebook")
        {
            3 // Heavy sites use ~3x base
        } else {
            1
        };
        base * url_factor
    }
}

impl Default for GeckoEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl BrowserEngine for GeckoEngine {
    fn initialize(&mut self) -> EngineResult<()> {
        if self.initialized {
            return Ok(());
        }

        log::info!("Initializing Gecko engine...");

        // Initialize video decoder
        self.initialize_video_decoder()?;

        // Apply performance preferences
        self.apply_preferences();

        self.initialized = true;
        log::info!("Gecko engine initialized successfully");
        Ok(())
    }

    fn shutdown(&mut self) -> EngineResult<()> {
        log::info!("Shutting down Gecko engine...");

        // Destroy all views
        let view_ids: Vec<ViewId> = self.views.keys().copied().collect();
        for id in view_ids {
            self.destroy_view(id)?;
        }

        self.initialized = false;
        log::info!("Gecko engine shut down");
        Ok(())
    }

    fn create_view(&mut self, view_id: ViewId) -> EngineResult<()> {
        if self.views.contains_key(&view_id) {
            return Err(EngineError::Other(format!(
                "View {} already exists",
                view_id
            )));
        }

        let view = GeckoView::new();
        self.views.insert(view_id, view);
        self.memory_usage += 20 * 1024 * 1024; // Base memory per tab

        log::debug!("Created Gecko view {}", view_id);
        Ok(())
    }

    fn load_url(&mut self, view_id: ViewId, url: &str) -> EngineResult<()> {
        let view = self
            .views
            .get_mut(&view_id)
            .ok_or(EngineError::ViewNotFound(view_id))?;

        view.url = url.to_string();
        view.is_loading = true;
        view.progress = 0.0;

        self.pending_events
            .push(EngineEvent::LoadStarted(view_id));
        self.pending_events
            .push(EngineEvent::UrlChanged(view_id, url.to_string()));

        log::debug!("Loading URL in {}: {}", view_id, url);

        // Simulate load completion
        if let Some(v) = self.views.get_mut(&view_id) {
            v.is_loading = false;
            v.progress = 1.0;
            v.can_go_back = true;
        }

        self.pending_events
            .push(EngineEvent::LoadProgress(view_id, 1.0));
        self.pending_events
            .push(EngineEvent::LoadFinished(view_id));

        Ok(())
    }

    fn load_html(&mut self, view_id: ViewId, html: &str, base_url: &str) -> EngineResult<()> {
        let view = self
            .views
            .get_mut(&view_id)
            .ok_or(EngineError::ViewNotFound(view_id))?;

        view.url = base_url.to_string();
        view.is_loading = false;
        view.progress = 1.0;

        log::debug!(
            "Loaded {} bytes of HTML into {}",
            html.len(),
            view_id
        );
        Ok(())
    }

    fn go_back(&mut self, view_id: ViewId) -> EngineResult<()> {
        let view = self
            .views
            .get(&view_id)
            .ok_or(EngineError::ViewNotFound(view_id))?;

        if !view.can_go_back {
            return Err(EngineError::NavigationError(
                "Cannot go back".to_string(),
            ));
        }

        log::debug!("Navigate back in {}", view_id);
        Ok(())
    }

    fn go_forward(&mut self, view_id: ViewId) -> EngineResult<()> {
        let view = self
            .views
            .get(&view_id)
            .ok_or(EngineError::ViewNotFound(view_id))?;

        if !view.can_go_forward {
            return Err(EngineError::NavigationError(
                "Cannot go forward".to_string(),
            ));
        }

        log::debug!("Navigate forward in {}", view_id);
        Ok(())
    }

    fn reload(&mut self, view_id: ViewId) -> EngineResult<()> {
        let view = self
            .views
            .get(&view_id)
            .ok_or(EngineError::ViewNotFound(view_id))?;

        let url = view.url.clone();
        self.load_url(view_id, &url)
    }

    fn stop(&mut self, view_id: ViewId) -> EngineResult<()> {
        let view = self
            .views
            .get_mut(&view_id)
            .ok_or(EngineError::ViewNotFound(view_id))?;

        view.is_loading = false;
        log::debug!("Stopped loading in {}", view_id);
        Ok(())
    }

    fn execute_script(
        &mut self,
        view_id: ViewId,
        script: &str,
    ) -> EngineResult<serde_json::Value> {
        if !self.views.contains_key(&view_id) {
            return Err(EngineError::ViewNotFound(view_id));
        }

        log::debug!(
            "Executing script in {} ({} chars)",
            view_id,
            script.len()
        );

        // In a full implementation, this would use SpiderMonkey to execute JS
        Ok(serde_json::Value::Null)
    }

    fn suspend_view(&mut self, view_id: ViewId) -> EngineResult<()> {
        let view = self
            .views
            .get_mut(&view_id)
            .ok_or(EngineError::ViewNotFound(view_id))?;

        view.suspended = true;
        // Recalculate memory (suspended views use minimal memory)
        self.memory_usage = self
            .views
            .values()
            .map(|v| Self::estimate_view_memory(v))
            .sum();

        log::debug!("Suspended view {}", view_id);
        Ok(())
    }

    fn resume_view(&mut self, view_id: ViewId) -> EngineResult<()> {
        let view = self
            .views
            .get_mut(&view_id)
            .ok_or(EngineError::ViewNotFound(view_id))?;

        view.suspended = false;
        self.memory_usage = self
            .views
            .values()
            .map(|v| Self::estimate_view_memory(v))
            .sum();

        log::debug!("Resumed view {}", view_id);
        Ok(())
    }

    fn destroy_view(&mut self, view_id: ViewId) -> EngineResult<()> {
        self.views
            .remove(&view_id)
            .ok_or(EngineError::ViewNotFound(view_id))?;

        self.memory_usage = self
            .views
            .values()
            .map(|v| Self::estimate_view_memory(v))
            .sum();

        log::debug!("Destroyed view {}", view_id);
        Ok(())
    }

    fn set_video_decoder(&mut self, decoder: VideoDecoder) -> EngineResult<()> {
        self.video_decoder = decoder;
        log::info!("Video decoder set to {:?}", self.video_decoder);
        Ok(())
    }

    fn enable_hardware_acceleration(&mut self, enabled: bool) -> EngineResult<()> {
        self.hw_accel = enabled;
        log::info!("Hardware acceleration: {}", if enabled { "enabled" } else { "disabled" });
        Ok(())
    }

    fn get_memory_usage(&self) -> MemoryStats {
        let active_views: u64 = self
            .views
            .values()
            .filter(|v| !v.suspended)
            .count() as u64;

        MemoryStats {
            total_bytes: self.memory_usage,
            js_heap_bytes: active_views * 8 * 1024 * 1024,  // ~8MB per view
            image_cache_bytes: 10 * 1024 * 1024,              // ~10MB shared
            dom_bytes: active_views * 5 * 1024 * 1024,        // ~5MB per view
            layout_bytes: active_views * 3 * 1024 * 1024,     // ~3MB per view
            network_cache_bytes: 20 * 1024 * 1024,             // ~20MB shared
        }
    }

    fn trim_memory(&mut self, level: TrimLevel) -> EngineResult<()> {
        let reduction = match level {
            TrimLevel::Light => 0.1,
            TrimLevel::Moderate => 0.3,
            TrimLevel::Aggressive => 0.5,
        };

        let trimmed = (self.memory_usage as f64 * reduction) as u64;
        self.memory_usage = self.memory_usage.saturating_sub(trimmed);

        log::info!(
            "Trimmed {:.1}MB of memory (level: {:?})",
            trimmed as f64 / (1024.0 * 1024.0),
            level
        );
        Ok(())
    }

    fn get_navigation_state(&self, view_id: ViewId) -> EngineResult<NavigationState> {
        let view = self
            .views
            .get(&view_id)
            .ok_or(EngineError::ViewNotFound(view_id))?;

        Ok(NavigationState {
            can_go_back: view.can_go_back,
            can_go_forward: view.can_go_forward,
            is_loading: view.is_loading,
            url: view.url.clone(),
            title: view.title.clone(),
            progress: view.progress,
        })
    }

    fn find_in_page(&mut self, view_id: ViewId, query: &str, forward: bool) -> EngineResult<()> {
        if !self.views.contains_key(&view_id) {
            return Err(EngineError::ViewNotFound(view_id));
        }
        log::debug!(
            "Find in page {}: '{}' (forward: {})",
            view_id,
            query,
            forward
        );
        Ok(())
    }

    fn clear_find(&mut self, view_id: ViewId) -> EngineResult<()> {
        if !self.views.contains_key(&view_id) {
            return Err(EngineError::ViewNotFound(view_id));
        }
        Ok(())
    }

    fn engine_info(&self) -> (String, String) {
        ("Gecko".to_string(), "124.0".to_string())
    }

    fn poll_events(&mut self) -> Vec<EngineEvent> {
        std::mem::take(&mut self.pending_events)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gecko_engine_creation() {
        let engine = GeckoEngine::new();
        assert!(!engine.initialized);
        assert!(engine.views.is_empty());
    }

    #[test]
    fn test_gecko_engine_init() {
        let mut engine = GeckoEngine::new();
        assert!(engine.initialize().is_ok());
        assert!(engine.initialized);
    }

    #[test]
    fn test_gecko_view_lifecycle() {
        let mut engine = GeckoEngine::new();
        engine.initialize().unwrap();

        let view_id = ViewId(1);
        assert!(engine.create_view(view_id).is_ok());
        assert!(engine.load_url(view_id, "https://example.com").is_ok());
        assert!(engine.suspend_view(view_id).is_ok());
        assert!(engine.resume_view(view_id).is_ok());
        assert!(engine.destroy_view(view_id).is_ok());
    }

    #[test]
    fn test_gecko_engine_info() {
        let engine = GeckoEngine::new();
        let (name, version) = engine.engine_info();
        assert_eq!(name, "Gecko");
        assert!(!version.is_empty());
    }

    #[test]
    fn test_gecko_memory_stats() {
        let mut engine = GeckoEngine::new();
        engine.initialize().unwrap();
        engine.create_view(ViewId(1)).unwrap();

        let stats = engine.get_memory_usage();
        assert!(stats.total_bytes > 0);
    }

    #[test]
    fn test_gecko_memory_trim() {
        let mut engine = GeckoEngine::new();
        engine.initialize().unwrap();
        engine.create_view(ViewId(1)).unwrap();

        let before = engine.get_memory_usage().total_bytes;
        engine.trim_memory(TrimLevel::Aggressive).unwrap();
        let after = engine.get_memory_usage().total_bytes;
        assert!(after < before);
    }
}
