//! GTK4 UI layer for Asteroid Browser.
//!
//! Provides the minimal-chrome user interface including:
//! - Window management
//! - Address bar (omnibox)
//! - Navigation buttons (back, forward, reload)
//! - Tab management (horizontal/vertical)
//! - Keyboard shortcuts
//! - Settings page
//! - Status overlay

pub mod window;
pub mod toolbar;
pub mod tab_bar;
pub mod settings;
pub mod shortcuts;

/// Keyboard shortcuts configuration.
pub struct KeyboardShortcuts {
    pub focus_address_bar: &'static str,
    pub new_tab: &'static str,
    pub close_tab: &'static str,
    pub tab_switcher: &'static str,
    pub find_in_page: &'static str,
    pub history: &'static str,
    pub fullscreen: &'static str,
    pub quick_find: &'static str,
    pub toggle_sidebar: &'static str,
    pub reload: &'static str,
    pub go_back: &'static str,
    pub go_forward: &'static str,
}

impl Default for KeyboardShortcuts {
    fn default() -> Self {
        Self {
            focus_address_bar: "<Ctrl>l",
            new_tab: "<Ctrl>t",
            close_tab: "<Ctrl>w",
            tab_switcher: "<Ctrl>Tab",
            find_in_page: "<Ctrl>f",
            history: "<Ctrl><Shift>h",
            fullscreen: "F11",
            quick_find: "slash",
            toggle_sidebar: "F1",
            reload: "F5",
            go_back: "<Alt>Left",
            go_forward: "<Alt>Right",
        }
    }
}
