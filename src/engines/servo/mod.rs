//! Servo engine stub for Asteroid Browser.
//!
//! This module provides a placeholder implementation for the Servo
//! rendering engine. It implements the BrowserEngine trait but returns
//! errors indicating that Servo is not yet available.
//!
//! Servo integration will be completed when Servo reaches production
//! readiness. Readiness criteria:
//! - [ ] CSS Grid support
//! - [ ] WebGL support
//! - [ ] Service Workers
//! - [ ] 95%+ Web Platform Tests pass rate
//! - [ ] Production-ready stability

use crate::core::engine::{
    BrowserEngine, EngineError, EngineEvent, EngineResult, MemoryStats,
    NavigationState, TrimLevel, VideoDecoder, ViewId,
};

/// Servo engine implementation (stub).
///
/// This will be fully implemented when Servo reaches production readiness.
/// For now, all operations return appropriate error messages.
pub struct ServoEngine {
    initialized: bool,
}

impl ServoEngine {
    pub fn new() -> Self {
        Self { initialized: false }
    }
}

impl Default for ServoEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl BrowserEngine for ServoEngine {
    fn initialize(&mut self) -> EngineResult<()> {
        self.initialized = true;
        log::warn!("Servo engine initialized (stub - not fully functional)");
        Err(EngineError::InitializationFailed(
            "Servo engine is not yet available. Use Gecko engine instead.".to_string(),
        ))
    }

    fn shutdown(&mut self) -> EngineResult<()> {
        self.initialized = false;
        Ok(())
    }

    fn create_view(&mut self, _view_id: ViewId) -> EngineResult<()> {
        Err(EngineError::Other(
            "Servo engine not available".to_string(),
        ))
    }

    fn load_url(&mut self, _view_id: ViewId, _url: &str) -> EngineResult<()> {
        Err(EngineError::Other(
            "Servo engine not available".to_string(),
        ))
    }

    fn load_html(&mut self, _view_id: ViewId, _html: &str, _base_url: &str) -> EngineResult<()> {
        Err(EngineError::Other(
            "Servo engine not available".to_string(),
        ))
    }

    fn go_back(&mut self, _view_id: ViewId) -> EngineResult<()> {
        Err(EngineError::Other(
            "Servo engine not available".to_string(),
        ))
    }

    fn go_forward(&mut self, _view_id: ViewId) -> EngineResult<()> {
        Err(EngineError::Other(
            "Servo engine not available".to_string(),
        ))
    }

    fn reload(&mut self, _view_id: ViewId) -> EngineResult<()> {
        Err(EngineError::Other(
            "Servo engine not available".to_string(),
        ))
    }

    fn stop(&mut self, _view_id: ViewId) -> EngineResult<()> {
        Err(EngineError::Other(
            "Servo engine not available".to_string(),
        ))
    }

    fn execute_script(
        &mut self,
        _view_id: ViewId,
        _script: &str,
    ) -> EngineResult<serde_json::Value> {
        Err(EngineError::Other(
            "Servo engine not available".to_string(),
        ))
    }

    fn suspend_view(&mut self, _view_id: ViewId) -> EngineResult<()> {
        Err(EngineError::Other(
            "Servo engine not available".to_string(),
        ))
    }

    fn resume_view(&mut self, _view_id: ViewId) -> EngineResult<()> {
        Err(EngineError::Other(
            "Servo engine not available".to_string(),
        ))
    }

    fn destroy_view(&mut self, _view_id: ViewId) -> EngineResult<()> {
        Err(EngineError::Other(
            "Servo engine not available".to_string(),
        ))
    }

    fn set_video_decoder(&mut self, _decoder: VideoDecoder) -> EngineResult<()> {
        Err(EngineError::Other(
            "Servo engine not available".to_string(),
        ))
    }

    fn enable_hardware_acceleration(&mut self, _enabled: bool) -> EngineResult<()> {
        Err(EngineError::Other(
            "Servo engine not available".to_string(),
        ))
    }

    fn get_memory_usage(&self) -> MemoryStats {
        MemoryStats::default()
    }

    fn trim_memory(&mut self, _level: TrimLevel) -> EngineResult<()> {
        Err(EngineError::Other(
            "Servo engine not available".to_string(),
        ))
    }

    fn get_navigation_state(&self, _view_id: ViewId) -> EngineResult<NavigationState> {
        Err(EngineError::Other(
            "Servo engine not available".to_string(),
        ))
    }

    fn find_in_page(&mut self, _view_id: ViewId, _query: &str, _forward: bool) -> EngineResult<()> {
        Err(EngineError::Other(
            "Servo engine not available".to_string(),
        ))
    }

    fn clear_find(&mut self, _view_id: ViewId) -> EngineResult<()> {
        Err(EngineError::Other(
            "Servo engine not available".to_string(),
        ))
    }

    fn engine_info(&self) -> (String, String) {
        ("Servo".to_string(), "0.0.0-stub".to_string())
    }

    fn poll_events(&mut self) -> Vec<EngineEvent> {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_servo_stub_creation() {
        let engine = ServoEngine::new();
        assert!(!engine.initialized);
    }

    #[test]
    fn test_servo_stub_returns_errors() {
        let mut engine = ServoEngine::new();
        assert!(engine.create_view(ViewId(1)).is_err());
        assert!(engine.load_url(ViewId(1), "https://example.com").is_err());
    }

    #[test]
    fn test_servo_engine_info() {
        let engine = ServoEngine::new();
        let (name, version) = engine.engine_info();
        assert_eq!(name, "Servo");
        assert!(version.contains("stub"));
    }
}
