//! Keyboard shortcut handling for Asteroid Browser.
//!
//! Maps keyboard combinations to browser actions.
//! Supports vim-style link hints when enabled.

/// Browser actions that can be triggered by keyboard shortcuts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BrowserAction {
    /// Focus the address bar (Ctrl+L)
    FocusAddressBar,
    /// Create a new tab (Ctrl+T)
    NewTab,
    /// Close current tab (Ctrl+W)
    CloseTab,
    /// Show tab switcher overlay (Ctrl+Tab)
    TabSwitcher,
    /// Open find-in-page (Ctrl+F)
    FindInPage,
    /// Show history (Ctrl+Shift+H)
    ShowHistory,
    /// Toggle fullscreen (F11)
    ToggleFullscreen,
    /// Quick find mode (/)
    QuickFind,
    /// Toggle tab sidebar (F1)
    ToggleSidebar,
    /// Reload page (F5)
    Reload,
    /// Navigate back (Alt+Left)
    GoBack,
    /// Navigate forward (Alt+Right)
    GoForward,
    /// Close window (Ctrl+Q)
    CloseWindow,
    /// Show bookmarks (Ctrl+B)
    ShowBookmarks,
    /// Open settings
    OpenSettings,
    /// Show vim-style link hints (f)
    VimHints,
    /// Switch to tab by number (Ctrl+1-9)
    SwitchToTab(u8),
    /// Zoom in (Ctrl++)
    ZoomIn,
    /// Zoom out (Ctrl+-)
    ZoomOut,
    /// Reset zoom (Ctrl+0)
    ZoomReset,
    /// Open downloads (Ctrl+J)
    OpenDownloads,
    /// Print page (Ctrl+P)
    PrintPage,
    /// View page source (Ctrl+U)
    ViewSource,
}

/// Shortcut definition mapping a key combination to an action.
#[derive(Debug, Clone)]
pub struct Shortcut {
    pub key: String,
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
    pub action: BrowserAction,
}

/// Get the default keyboard shortcuts.
pub fn default_shortcuts() -> Vec<Shortcut> {
    vec![
        Shortcut { key: "l".into(), ctrl: true, alt: false, shift: false, action: BrowserAction::FocusAddressBar },
        Shortcut { key: "t".into(), ctrl: true, alt: false, shift: false, action: BrowserAction::NewTab },
        Shortcut { key: "w".into(), ctrl: true, alt: false, shift: false, action: BrowserAction::CloseTab },
        Shortcut { key: "Tab".into(), ctrl: true, alt: false, shift: false, action: BrowserAction::TabSwitcher },
        Shortcut { key: "f".into(), ctrl: true, alt: false, shift: false, action: BrowserAction::FindInPage },
        Shortcut { key: "h".into(), ctrl: true, alt: false, shift: true, action: BrowserAction::ShowHistory },
        Shortcut { key: "F11".into(), ctrl: false, alt: false, shift: false, action: BrowserAction::ToggleFullscreen },
        Shortcut { key: "slash".into(), ctrl: false, alt: false, shift: false, action: BrowserAction::QuickFind },
        Shortcut { key: "F1".into(), ctrl: false, alt: false, shift: false, action: BrowserAction::ToggleSidebar },
        Shortcut { key: "F5".into(), ctrl: false, alt: false, shift: false, action: BrowserAction::Reload },
        Shortcut { key: "Left".into(), ctrl: false, alt: true, shift: false, action: BrowserAction::GoBack },
        Shortcut { key: "Right".into(), ctrl: false, alt: true, shift: false, action: BrowserAction::GoForward },
        Shortcut { key: "q".into(), ctrl: true, alt: false, shift: false, action: BrowserAction::CloseWindow },
        Shortcut { key: "b".into(), ctrl: true, alt: false, shift: false, action: BrowserAction::ShowBookmarks },
        Shortcut { key: "f".into(), ctrl: false, alt: false, shift: false, action: BrowserAction::VimHints },
        Shortcut { key: "plus".into(), ctrl: true, alt: false, shift: false, action: BrowserAction::ZoomIn },
        Shortcut { key: "minus".into(), ctrl: true, alt: false, shift: false, action: BrowserAction::ZoomOut },
        Shortcut { key: "0".into(), ctrl: true, alt: false, shift: false, action: BrowserAction::ZoomReset },
        Shortcut { key: "j".into(), ctrl: true, alt: false, shift: false, action: BrowserAction::OpenDownloads },
        Shortcut { key: "p".into(), ctrl: true, alt: false, shift: false, action: BrowserAction::PrintPage },
        Shortcut { key: "u".into(), ctrl: true, alt: false, shift: false, action: BrowserAction::ViewSource },
        // Tab switching: Ctrl+1 through Ctrl+9
        Shortcut { key: "1".into(), ctrl: true, alt: false, shift: false, action: BrowserAction::SwitchToTab(1) },
        Shortcut { key: "2".into(), ctrl: true, alt: false, shift: false, action: BrowserAction::SwitchToTab(2) },
        Shortcut { key: "3".into(), ctrl: true, alt: false, shift: false, action: BrowserAction::SwitchToTab(3) },
        Shortcut { key: "4".into(), ctrl: true, alt: false, shift: false, action: BrowserAction::SwitchToTab(4) },
        Shortcut { key: "5".into(), ctrl: true, alt: false, shift: false, action: BrowserAction::SwitchToTab(5) },
        Shortcut { key: "6".into(), ctrl: true, alt: false, shift: false, action: BrowserAction::SwitchToTab(6) },
        Shortcut { key: "7".into(), ctrl: true, alt: false, shift: false, action: BrowserAction::SwitchToTab(7) },
        Shortcut { key: "8".into(), ctrl: true, alt: false, shift: false, action: BrowserAction::SwitchToTab(8) },
        Shortcut { key: "9".into(), ctrl: true, alt: false, shift: false, action: BrowserAction::SwitchToTab(9) },
    ]
}

/// Vim-style link hint characters.
pub const HINT_CHARS: &str = "asdfghjklqwertyuiopzxcvbnm";

/// Generate JavaScript for vim-style link hints.
pub fn vim_hints_js() -> &'static str {
    r#"
(function() {
    'use strict';

    const HINT_CHARS = 'asdfghjklqwertyuiopzxcvbnm';
    let hints = [];
    let active = false;
    let input = '';

    function generateLabels(count) {
        const labels = [];
        const len = Math.ceil(Math.log(count) / Math.log(HINT_CHARS.length)) || 1;
        for (let i = 0; i < count; i++) {
            let label = '';
            let n = i;
            for (let j = 0; j < len; j++) {
                label = HINT_CHARS[n % HINT_CHARS.length] + label;
                n = Math.floor(n / HINT_CHARS.length);
            }
            labels.push(label);
        }
        return labels;
    }

    function showHints() {
        if (active) { removeHints(); return; }
        active = true;
        input = '';

        const clickable = document.querySelectorAll(
            'a, button, input, select, textarea, [onclick], [role="button"], [tabindex]'
        );

        const labels = generateLabels(clickable.length);

        clickable.forEach((el, i) => {
            const rect = el.getBoundingClientRect();
            if (rect.width === 0 || rect.height === 0) return;
            if (rect.top < 0 || rect.left < 0) return;

            const hint = document.createElement('div');
            hint.className = 'asteroid-hint';
            hint.textContent = labels[i];
            hint.style.cssText = `
                position: fixed;
                top: ${rect.top}px;
                left: ${rect.left}px;
                background: #7DC6DA;
                color: #0a0e1a;
                font-size: 12px;
                font-weight: bold;
                padding: 1px 4px;
                border-radius: 3px;
                z-index: 999999;
                font-family: monospace;
                pointer-events: none;
            `;
            hint.dataset.label = labels[i];
            hint.dataset.index = i;
            document.body.appendChild(hint);
            hints.push({ element: el, hint: hint, label: labels[i] });
        });
    }

    function removeHints() {
        hints.forEach(h => h.hint.remove());
        hints = [];
        active = false;
        input = '';
    }

    function handleInput(char) {
        input += char;
        const match = hints.find(h => h.label === input);
        if (match) {
            match.element.click();
            match.element.focus();
            removeHints();
        } else {
            // Filter visible hints
            hints.forEach(h => {
                h.hint.style.display = h.label.startsWith(input) ? '' : 'none';
            });
            // If no hints match, cancel
            if (!hints.some(h => h.label.startsWith(input))) {
                removeHints();
            }
        }
    }

    window.__asteroidHints = { show: showHints, remove: removeHints, handle: handleInput };
})();
"#
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_shortcuts() {
        let shortcuts = default_shortcuts();
        assert!(!shortcuts.is_empty());

        // Check that essential shortcuts exist
        let has_new_tab = shortcuts.iter().any(|s| s.action == BrowserAction::NewTab);
        let has_close = shortcuts.iter().any(|s| s.action == BrowserAction::CloseTab);
        let has_address = shortcuts.iter().any(|s| s.action == BrowserAction::FocusAddressBar);
        assert!(has_new_tab);
        assert!(has_close);
        assert!(has_address);
    }

    #[test]
    fn test_vim_hints_js() {
        let js = vim_hints_js();
        assert!(js.contains("asteroid-hint"));
        assert!(js.contains("showHints"));
    }
}
