//! Tab bar component for Asteroid Browser.
//!
//! Provides both horizontal tab strip and vertical tab sidebar.
//! Supports tab switching, closing, drag-to-reorder, and visual
//! indicators for suspended/loading tabs.

use crate::core::engine::ViewId;
use crate::core::tab::{Tab, TabState};

/// Visual representation of a tab in the sidebar/strip.
#[derive(Debug, Clone)]
pub struct TabEntry {
    pub view_id: ViewId,
    pub title: String,
    pub url: String,
    pub is_active: bool,
    pub is_loading: bool,
    pub is_suspended: bool,
    pub is_pinned: bool,
    pub favicon: Option<Vec<u8>>,
}

impl TabEntry {
    pub fn from_tab(tab: &Tab, is_active: bool) -> Self {
        Self {
            view_id: tab.view_id,
            title: if tab.title.is_empty() {
                tab.url.clone()
            } else {
                tab.title.clone()
            },
            url: tab.url.clone(),
            is_active,
            is_loading: tab.state == TabState::Loading,
            is_suspended: tab.state == TabState::Suspended,
            is_pinned: tab.pinned,
            favicon: tab.favicon.clone(),
        }
    }

    /// Get a shortened title for display.
    pub fn display_title(&self, max_len: usize) -> String {
        if self.title.len() <= max_len {
            self.title.clone()
        } else {
            format!("{}...", &self.title[..max_len.saturating_sub(3)])
        }
    }

    /// Get a status indicator character.
    pub fn status_indicator(&self) -> &str {
        if self.is_loading {
            "\u{27F3}" // ⟳ loading
        } else if self.is_suspended {
            "\u{23F8}" // ⏸ suspended
        } else if self.is_pinned {
            "\u{1F4CC}" // 📌 pinned
        } else {
            ""
        }
    }
}

/// Tab bar action events.
#[derive(Debug, Clone)]
pub enum TabBarAction {
    /// Switch to a tab
    SwitchTab(ViewId),
    /// Close a tab
    CloseTab(ViewId),
    /// Create a new tab
    NewTab,
    /// Pin/unpin a tab
    TogglePin(ViewId),
    /// Move tab to new position
    MoveTab(ViewId, usize),
    /// Toggle sidebar visibility
    ToggleSidebar,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tab_entry_display_title() {
        let entry = TabEntry {
            view_id: ViewId(1),
            title: "A Very Long Page Title That Should Be Truncated".to_string(),
            url: "https://example.com".to_string(),
            is_active: false,
            is_loading: false,
            is_suspended: false,
            is_pinned: false,
            favicon: None,
        };

        let short = entry.display_title(20);
        assert!(short.len() <= 20);
        assert!(short.ends_with("..."));
    }

    #[test]
    fn test_status_indicator() {
        let mut entry = TabEntry {
            view_id: ViewId(1),
            title: "Test".to_string(),
            url: "https://example.com".to_string(),
            is_active: false,
            is_loading: true,
            is_suspended: false,
            is_pinned: false,
            favicon: None,
        };

        assert!(!entry.status_indicator().is_empty()); // loading indicator
        entry.is_loading = false;
        entry.is_suspended = true;
        assert!(!entry.status_indicator().is_empty()); // suspended indicator
    }
}
