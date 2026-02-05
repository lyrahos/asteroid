//! Tab management system for Asteroid Browser.
//!
//! Handles tab lifecycle including creation, suspension after inactivity,
//! restoration, and memory-pressure-driven unloading.

use crate::core::engine::{BrowserEngine, EngineResult, ViewId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// State of a suspended tab (serialized for memory savings).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuspendedState {
    /// URL of the page when suspended
    pub url: String,
    /// Page title when suspended
    pub title: String,
    /// Scroll position (x, y)
    pub scroll_position: (f64, f64),
    /// Timestamp of suspension
    pub suspended_at: u64,
    /// Favicon data (optional, compressed)
    pub favicon: Option<Vec<u8>>,
}

/// Current state of a tab.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TabState {
    /// Tab is actively loaded and rendered
    Active,
    /// Tab is loaded but not currently visible
    Background,
    /// Tab has been suspended to save memory
    Suspended,
    /// Tab is being loaded
    Loading,
    /// Tab encountered an error
    Error,
}

/// Represents a single browser tab.
#[derive(Debug)]
pub struct Tab {
    /// Unique view identifier
    pub view_id: ViewId,
    /// Current tab state
    pub state: TabState,
    /// Current URL
    pub url: String,
    /// Page title
    pub title: String,
    /// Last time the tab was active/focused
    pub last_active: Instant,
    /// Creation time
    pub created_at: Instant,
    /// Suspended state data (when tab is suspended)
    pub suspended_data: Option<SuspendedState>,
    /// Whether this tab is pinned
    pub pinned: bool,
    /// Favicon data
    pub favicon: Option<Vec<u8>>,
}

impl Tab {
    pub fn new(view_id: ViewId) -> Self {
        let now = Instant::now();
        Self {
            view_id,
            state: TabState::Loading,
            url: String::from("about:blank"),
            title: String::from("New Tab"),
            last_active: now,
            created_at: now,
            suspended_data: None,
            pinned: false,
            favicon: None,
        }
    }

    /// Duration since this tab was last active.
    pub fn inactive_duration(&self) -> Duration {
        self.last_active.elapsed()
    }

    /// Mark this tab as currently active.
    pub fn mark_active(&mut self) {
        self.last_active = Instant::now();
        if self.state == TabState::Background {
            self.state = TabState::Active;
        }
    }

    /// Mark this tab as background (not focused).
    pub fn mark_background(&mut self) {
        if self.state == TabState::Active {
            self.state = TabState::Background;
        }
    }
}

/// Configuration for tab suspension behavior.
#[derive(Debug, Clone)]
pub struct SuspensionConfig {
    /// Duration of inactivity before a tab is suspended
    pub inactive_threshold: Duration,
    /// Whether tab suspension is enabled
    pub enabled: bool,
    /// Maximum number of active (non-suspended) tabs
    pub max_active_tabs: usize,
    /// Whether to suspend pinned tabs
    pub suspend_pinned: bool,
}

impl Default for SuspensionConfig {
    fn default() -> Self {
        Self {
            inactive_threshold: Duration::from_secs(300), // 5 minutes
            enabled: true,
            max_active_tabs: 10,
            suspend_pinned: false,
        }
    }
}

/// Manages all browser tabs and their lifecycle.
pub struct TabManager {
    /// All managed tabs
    tabs: HashMap<ViewId, Tab>,
    /// Currently active (focused) tab
    active_tab: Option<ViewId>,
    /// Tab ordering (for display)
    tab_order: Vec<ViewId>,
    /// Next view ID to assign
    next_id: u64,
    /// Suspension configuration
    pub suspension_config: SuspensionConfig,
}

impl TabManager {
    pub fn new(config: SuspensionConfig) -> Self {
        Self {
            tabs: HashMap::new(),
            active_tab: None,
            tab_order: Vec::new(),
            next_id: 1,
            suspension_config: config,
        }
    }

    /// Create a new tab and return its ViewId.
    pub fn create_tab(&mut self, engine: &mut dyn BrowserEngine) -> EngineResult<ViewId> {
        let view_id = ViewId(self.next_id);
        self.next_id += 1;

        engine.create_view(view_id)?;

        let tab = Tab::new(view_id);
        self.tabs.insert(view_id, tab);
        self.tab_order.push(view_id);

        // If this is the first tab, make it active
        if self.active_tab.is_none() {
            self.active_tab = Some(view_id);
        }

        Ok(view_id)
    }

    /// Close a tab and release its resources.
    pub fn close_tab(
        &mut self,
        view_id: ViewId,
        engine: &mut dyn BrowserEngine,
    ) -> EngineResult<()> {
        if let Some(tab) = self.tabs.get(&view_id) {
            if tab.state != TabState::Suspended {
                engine.destroy_view(view_id)?;
            }
        }

        self.tabs.remove(&view_id);
        self.tab_order.retain(|&id| id != view_id);

        // If we closed the active tab, activate the nearest tab
        if self.active_tab == Some(view_id) {
            self.active_tab = self.tab_order.last().copied();
        }

        Ok(())
    }

    /// Switch to a different tab.
    pub fn switch_to_tab(
        &mut self,
        view_id: ViewId,
        engine: &mut dyn BrowserEngine,
    ) -> EngineResult<()> {
        // Background the current active tab
        if let Some(current_id) = self.active_tab {
            if let Some(tab) = self.tabs.get_mut(&current_id) {
                tab.mark_background();
            }
        }

        // Resume tab if suspended
        if let Some(tab) = self.tabs.get(&view_id) {
            if tab.state == TabState::Suspended {
                self.resume_tab(view_id, engine)?;
            }
        }

        // Activate the new tab
        if let Some(tab) = self.tabs.get_mut(&view_id) {
            tab.mark_active();
        }

        self.active_tab = Some(view_id);
        Ok(())
    }

    /// Suspend a tab to save memory.
    pub fn suspend_tab(
        &mut self,
        view_id: ViewId,
        engine: &mut dyn BrowserEngine,
    ) -> EngineResult<()> {
        let tab = self
            .tabs
            .get_mut(&view_id)
            .ok_or(crate::core::engine::EngineError::ViewNotFound(view_id))?;

        // Don't suspend already-suspended tabs or the active tab
        if tab.state == TabState::Suspended || self.active_tab == Some(view_id) {
            return Ok(());
        }

        // Don't suspend pinned tabs if configured
        if tab.pinned && !self.suspension_config.suspend_pinned {
            return Ok(());
        }

        // Save state
        let suspended_state = SuspendedState {
            url: tab.url.clone(),
            title: tab.title.clone(),
            scroll_position: (0.0, 0.0),
            suspended_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            favicon: tab.favicon.clone(),
        };

        // Release engine resources
        engine.suspend_view(view_id)?;

        tab.suspended_data = Some(suspended_state);
        tab.state = TabState::Suspended;

        log::info!("Suspended tab {}: {}", view_id, tab.title);
        Ok(())
    }

    /// Resume a previously suspended tab.
    pub fn resume_tab(
        &mut self,
        view_id: ViewId,
        engine: &mut dyn BrowserEngine,
    ) -> EngineResult<()> {
        let tab = self
            .tabs
            .get_mut(&view_id)
            .ok_or(crate::core::engine::EngineError::ViewNotFound(view_id))?;

        if tab.state != TabState::Suspended {
            return Ok(());
        }

        let url = if let Some(ref data) = tab.suspended_data {
            data.url.clone()
        } else {
            tab.url.clone()
        };

        // Resume in engine
        engine.resume_view(view_id)?;

        // Reload the page
        engine.load_url(view_id, &url)?;

        tab.suspended_data = None;
        tab.state = TabState::Loading;

        log::info!("Resumed tab {}: {}", view_id, tab.title);
        Ok(())
    }

    /// Check for tabs that should be suspended based on inactivity.
    pub fn check_suspensions(&mut self, engine: &mut dyn BrowserEngine) {
        if !self.suspension_config.enabled {
            return;
        }

        let threshold = self.suspension_config.inactive_threshold;
        let active_tab = self.active_tab;

        let tabs_to_suspend: Vec<ViewId> = self
            .tabs
            .iter()
            .filter(|(id, tab)| {
                tab.state == TabState::Background
                    && tab.inactive_duration() > threshold
                    && active_tab != Some(**id)
                    && (!tab.pinned || self.suspension_config.suspend_pinned)
            })
            .map(|(id, _)| *id)
            .collect();

        for view_id in tabs_to_suspend {
            if let Err(e) = self.suspend_tab(view_id, engine) {
                log::error!("Failed to suspend tab {}: {}", view_id, e);
            }
        }
    }

    /// Suspend all inactive tabs immediately (memory pressure response).
    pub fn suspend_all_inactive(&mut self, engine: &mut dyn BrowserEngine) {
        let active_tab = self.active_tab;

        let tabs_to_suspend: Vec<ViewId> = self
            .tabs
            .iter()
            .filter(|(id, tab)| {
                tab.state != TabState::Suspended && active_tab != Some(**id)
            })
            .map(|(id, _)| *id)
            .collect();

        for view_id in tabs_to_suspend {
            if let Err(e) = self.suspend_tab(view_id, engine) {
                log::error!("Failed to suspend tab {}: {}", view_id, e);
            }
        }
    }

    /// Suspend the oldest N inactive tabs.
    pub fn suspend_oldest_inactive(&mut self, count: usize, engine: &mut dyn BrowserEngine) {
        let active_tab = self.active_tab;

        let mut inactive_tabs: Vec<(ViewId, Instant)> = self
            .tabs
            .iter()
            .filter(|(id, tab)| {
                tab.state == TabState::Background && active_tab != Some(**id)
            })
            .map(|(id, tab)| (*id, tab.last_active))
            .collect();

        // Sort by last active time (oldest first)
        inactive_tabs.sort_by_key(|(_, last_active)| *last_active);

        for (view_id, _) in inactive_tabs.into_iter().take(count) {
            if let Err(e) = self.suspend_tab(view_id, engine) {
                log::error!("Failed to suspend tab {}: {}", view_id, e);
            }
        }
    }

    /// Get the currently active tab.
    pub fn active_tab(&self) -> Option<&Tab> {
        self.active_tab
            .and_then(|id| self.tabs.get(&id))
    }

    /// Get a tab by its ViewId.
    pub fn get_tab(&self, view_id: ViewId) -> Option<&Tab> {
        self.tabs.get(&view_id)
    }

    /// Get a mutable reference to a tab.
    pub fn get_tab_mut(&mut self, view_id: ViewId) -> Option<&mut Tab> {
        self.tabs.get_mut(&view_id)
    }

    /// Get all tabs in display order.
    pub fn tabs_in_order(&self) -> Vec<&Tab> {
        self.tab_order
            .iter()
            .filter_map(|id| self.tabs.get(id))
            .collect()
    }

    /// Get the number of open tabs.
    pub fn tab_count(&self) -> usize {
        self.tabs.len()
    }

    /// Get the number of suspended tabs.
    pub fn suspended_count(&self) -> usize {
        self.tabs
            .values()
            .filter(|t| t.state == TabState::Suspended)
            .count()
    }

    /// Get the active tab's ViewId.
    pub fn active_tab_id(&self) -> Option<ViewId> {
        self.active_tab
    }

    /// Move a tab to a new position in the tab order.
    pub fn move_tab(&mut self, view_id: ViewId, new_index: usize) {
        if let Some(pos) = self.tab_order.iter().position(|&id| id == view_id) {
            self.tab_order.remove(pos);
            let insert_at = new_index.min(self.tab_order.len());
            self.tab_order.insert(insert_at, view_id);
        }
    }

    /// Pin or unpin a tab.
    pub fn set_pinned(&mut self, view_id: ViewId, pinned: bool) {
        if let Some(tab) = self.tabs.get_mut(&view_id) {
            tab.pinned = pinned;
        }
    }

    /// Update tab metadata from engine events.
    pub fn update_tab_url(&mut self, view_id: ViewId, url: String) {
        if let Some(tab) = self.tabs.get_mut(&view_id) {
            tab.url = url;
        }
    }

    /// Update tab title.
    pub fn update_tab_title(&mut self, view_id: ViewId, title: String) {
        if let Some(tab) = self.tabs.get_mut(&view_id) {
            tab.title = title;
        }
    }

    /// Update tab favicon.
    pub fn update_tab_favicon(&mut self, view_id: ViewId, favicon: Vec<u8>) {
        if let Some(tab) = self.tabs.get_mut(&view_id) {
            tab.favicon = Some(favicon);
        }
    }

    /// Mark a tab as finished loading.
    pub fn mark_loaded(&mut self, view_id: ViewId) {
        if let Some(tab) = self.tabs.get_mut(&view_id) {
            if tab.state == TabState::Loading {
                if self.active_tab == Some(view_id) {
                    tab.state = TabState::Active;
                } else {
                    tab.state = TabState::Background;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tab_creation() {
        let tab = Tab::new(ViewId(1));
        assert_eq!(tab.state, TabState::Loading);
        assert_eq!(tab.url, "about:blank");
        assert_eq!(tab.title, "New Tab");
        assert!(!tab.pinned);
    }

    #[test]
    fn test_tab_state_transitions() {
        let mut tab = Tab::new(ViewId(1));
        tab.state = TabState::Active;
        tab.mark_background();
        assert_eq!(tab.state, TabState::Background);
        tab.mark_active();
        assert_eq!(tab.state, TabState::Active);
    }

    #[test]
    fn test_suspension_config_default() {
        let config = SuspensionConfig::default();
        assert!(config.enabled);
        assert_eq!(config.inactive_threshold, Duration::from_secs(300));
        assert_eq!(config.max_active_tabs, 10);
        assert!(!config.suspend_pinned);
    }
}
