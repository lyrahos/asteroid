//! Main browser window implementation.
//!
//! Creates the primary GTK4 application window with minimal chrome:
//! - Navigation toolbar (back, forward, reload, address bar, menu)
//! - Optional vertical tab sidebar
//! - Web content area
//! - Status overlay (bottom-left, appears on hover/activity)

use std::cell::RefCell;
use std::rc::Rc;

use gtk4::glib;
use gtk4::prelude::*;
use gtk4::{
    Application, ApplicationWindow, Box as GtkBox, Button, Entry,
    Label, Orientation, Paned, ScrolledWindow, Separator,
};

use crate::core::engine::{BrowserEngine, EngineEvent};
use crate::core::tab::TabManager;
use crate::ui::toolbar::parse_address_input;

/// Shared browser state accessible from UI callbacks.
/// Uses Rc<RefCell<>> for zero-overhead single-threaded sharing.
pub struct BrowserState {
    pub engine: Box<dyn BrowserEngine>,
    pub tab_manager: TabManager,
}

/// Toolbar widget handles needed for signal wiring.
struct ToolbarWidgets {
    container: GtkBox,
    back_btn: Button,
    forward_btn: Button,
    reload_btn: Button,
    address_bar: Entry,
}

/// Build the main browser window with navigation wiring.
pub fn build_window(app: &Application, state: Rc<RefCell<BrowserState>>) -> ApplicationWindow {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Asteroid Browser")
        .default_width(1280)
        .default_height(800)
        .build();

    let main_box = GtkBox::new(Orientation::Vertical, 0);

    // Toolbar
    let tw = build_toolbar();
    main_box.append(&tw.container);

    // Sidebar + content
    let content_paned = Paned::new(Orientation::Horizontal);

    let sidebar = build_tab_sidebar();
    sidebar.set_visible(false);
    content_paned.set_start_child(Some(&sidebar));
    content_paned.set_position(200);

    // Content area with label (placeholder until real web view)
    let content_area = GtkBox::new(Orientation::Vertical, 0);
    content_area.set_hexpand(true);
    content_area.set_vexpand(true);
    content_area.add_css_class("content-area");

    let content_label = Label::new(Some("Asteroid Browser\n\nLightweight. Fast. Independent."));
    content_label.set_vexpand(true);
    content_label.set_hexpand(true);
    content_label.set_valign(gtk4::Align::Center);
    content_label.set_halign(gtk4::Align::Center);
    content_label.add_css_class("welcome-text");
    content_area.append(&content_label);

    content_paned.set_end_child(Some(&content_area));
    main_box.append(&content_paned);

    // Status bar
    let status_label = Label::new(Some("Ready"));
    status_label.set_halign(gtk4::Align::Start);
    status_label.set_margin_start(8);
    status_label.set_margin_bottom(4);
    status_label.add_css_class("status-overlay");
    main_box.append(&status_label);

    window.set_child(Some(&main_box));

    // --- Wire signals ---

    // Address bar: Enter key navigates
    {
        let state = state.clone();
        let content = content_label.clone();
        let status = status_label.clone();
        let win = window.clone();
        tw.address_bar.connect_activate(move |entry| {
            let text = entry.text().to_string();
            if text.trim().is_empty() {
                return;
            }
            let url = parse_address_input(&text);
            entry.set_text(&url);

            let mut s = state.borrow_mut();
            if let Some(view_id) = s.tab_manager.active_tab_id() {
                match s.engine.load_url(view_id, &url) {
                    Ok(()) => {
                        content.set_text(&url);
                        if let Ok(nav) = s.engine.get_navigation_state(view_id) {
                            let title = if nav.title.is_empty() { &url } else { &nav.title };
                            win.set_title(Some(&format!("{} - Asteroid Browser", title)));
                        }
                        status.set_text("Done");
                    }
                    Err(e) => {
                        status.set_text(&format!("Error: {}", e));
                    }
                }
            }
        });
    }

    // Back button
    {
        let state = state.clone();
        let addr = tw.address_bar.clone();
        let status = status_label.clone();
        tw.back_btn.connect_clicked(move |_| {
            let mut s = state.borrow_mut();
            if let Some(view_id) = s.tab_manager.active_tab_id() {
                if let Err(e) = s.engine.go_back(view_id) {
                    status.set_text(&format!("{}", e));
                } else if let Ok(nav) = s.engine.get_navigation_state(view_id) {
                    addr.set_text(&nav.url);
                }
            }
        });
    }

    // Forward button
    {
        let state = state.clone();
        let addr = tw.address_bar.clone();
        let status = status_label.clone();
        tw.forward_btn.connect_clicked(move |_| {
            let mut s = state.borrow_mut();
            if let Some(view_id) = s.tab_manager.active_tab_id() {
                if let Err(e) = s.engine.go_forward(view_id) {
                    status.set_text(&format!("{}", e));
                } else if let Ok(nav) = s.engine.get_navigation_state(view_id) {
                    addr.set_text(&nav.url);
                }
            }
        });
    }

    // Reload button
    {
        let state = state.clone();
        let status = status_label.clone();
        tw.reload_btn.connect_clicked(move |_| {
            let mut s = state.borrow_mut();
            if let Some(view_id) = s.tab_manager.active_tab_id() {
                status.set_text("Reloading...");
                match s.engine.reload(view_id) {
                    Ok(()) => status.set_text("Done"),
                    Err(e) => status.set_text(&format!("{}", e)),
                }
            }
        });
    }

    // Poll engine events (lightweight: just checks a Vec every 100ms)
    {
        let state = state.clone();
        let addr = tw.address_bar.clone();
        let status = status_label.clone();
        let content = content_label;
        let win = window.clone();
        glib::timeout_add_local(std::time::Duration::from_millis(100), move || {
            let mut s = state.borrow_mut();
            for event in s.engine.poll_events() {
                match event {
                    EngineEvent::UrlChanged(_, ref url) => {
                        addr.set_text(url);
                        content.set_text(url);
                    }
                    EngineEvent::LoadStarted(_) => {
                        status.set_text("Loading...");
                    }
                    EngineEvent::LoadProgress(_, p) => {
                        if p < 1.0 {
                            status.set_text(&format!("Loading {:.0}%", p * 100.0));
                        }
                    }
                    EngineEvent::LoadFinished(_) => {
                        status.set_text("Done");
                    }
                    EngineEvent::TitleChanged(_, ref title) => {
                        win.set_title(Some(&format!("{} - Asteroid Browser", title)));
                    }
                    _ => {}
                }
            }
            glib::ControlFlow::Continue
        });
    }

    // Set initial URL from active tab
    {
        let s = state.borrow();
        if let Some(view_id) = s.tab_manager.active_tab_id() {
            if let Ok(nav) = s.engine.get_navigation_state(view_id) {
                tw.address_bar.set_text(&nav.url);
            }
        }
    }

    window
}

/// Build the navigation toolbar, returning widget handles.
fn build_toolbar() -> ToolbarWidgets {
    let toolbar = GtkBox::new(Orientation::Horizontal, 4);
    toolbar.set_margin_start(4);
    toolbar.set_margin_end(4);
    toolbar.set_margin_top(4);
    toolbar.set_margin_bottom(4);
    toolbar.add_css_class("toolbar");

    let back_btn = Button::with_label("\u{2190}"); // <-
    back_btn.set_tooltip_text(Some("Back (Alt+Left)"));
    back_btn.add_css_class("nav-button");
    toolbar.append(&back_btn);

    let forward_btn = Button::with_label("\u{2192}"); // ->
    forward_btn.set_tooltip_text(Some("Forward (Alt+Right)"));
    forward_btn.add_css_class("nav-button");
    toolbar.append(&forward_btn);

    let reload_btn = Button::with_label("\u{27F3}"); // reload symbol
    reload_btn.set_tooltip_text(Some("Reload (F5)"));
    reload_btn.add_css_class("nav-button");
    toolbar.append(&reload_btn);

    let address_bar = Entry::new();
    address_bar.set_placeholder_text(Some("Enter URL or search..."));
    address_bar.set_hexpand(true);
    address_bar.add_css_class("address-bar");
    toolbar.append(&address_bar);

    let menu_btn = Button::with_label("\u{2630}"); // hamburger menu
    menu_btn.set_tooltip_text(Some("Menu"));
    menu_btn.add_css_class("menu-button");
    toolbar.append(&menu_btn);

    ToolbarWidgets {
        container: toolbar,
        back_btn,
        forward_btn,
        reload_btn,
        address_bar,
    }
}

/// Build the vertical tab sidebar.
fn build_tab_sidebar() -> GtkBox {
    let sidebar = GtkBox::new(Orientation::Vertical, 2);
    sidebar.set_width_request(200);
    sidebar.add_css_class("tab-sidebar");

    let header = Label::new(Some("Tabs"));
    header.add_css_class("sidebar-header");
    sidebar.append(&header);

    let separator = Separator::new(Orientation::Horizontal);
    sidebar.append(&separator);

    let scrolled = ScrolledWindow::new();
    scrolled.set_vexpand(true);

    let tab_list = GtkBox::new(Orientation::Vertical, 1);
    tab_list.add_css_class("tab-list");

    let new_tab_btn = Button::with_label("+ New Tab");
    new_tab_btn.set_tooltip_text(Some("New Tab (Ctrl+T)"));
    new_tab_btn.add_css_class("new-tab-button");

    scrolled.set_child(Some(&tab_list));
    sidebar.append(&scrolled);
    sidebar.append(&new_tab_btn);

    sidebar
}

/// Apply CSS styles to the application.
pub fn load_css() {
    let provider = gtk4::CssProvider::new();
    provider.load_from_data(CSS_STYLES);

    gtk4::style_context_add_provider_for_display(
        &gtk4::gdk::Display::default().expect("Could not get default display"),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

/// CSS styles for the browser UI.
const CSS_STYLES: &str = r#"
/* Asteroid Browser Styles - Minimal Chrome */

window {
    background-color: #1a1a2e;
    color: #e0e0e0;
}

.toolbar {
    background-color: #16213e;
    border-bottom: 1px solid #0f3460;
    padding: 4px;
    border-radius: 0;
}

.nav-button {
    min-width: 36px;
    min-height: 36px;
    padding: 4px 8px;
    background-color: transparent;
    color: #e0e0e0;
    border: none;
    border-radius: 4px;
    font-size: 16px;
}

.nav-button:hover {
    background-color: #0f3460;
}

.address-bar {
    background-color: #0a0e1a;
    color: #e0e0e0;
    border: 1px solid #0f3460;
    border-radius: 20px;
    padding: 6px 16px;
    margin: 0 8px;
    font-size: 14px;
}

.address-bar:focus {
    border-color: #7DC6DA;
    outline: none;
}

.menu-button {
    min-width: 36px;
    min-height: 36px;
    background-color: transparent;
    color: #e0e0e0;
    border: none;
    border-radius: 4px;
    font-size: 18px;
}

.menu-button:hover {
    background-color: #0f3460;
}

.tab-sidebar {
    background-color: #16213e;
    border-right: 1px solid #0f3460;
    padding: 4px;
}

.sidebar-header {
    font-weight: bold;
    padding: 8px;
    color: #7DC6DA;
}

.tab-list {
    padding: 4px;
}

.new-tab-button {
    margin: 4px;
    padding: 8px;
    background-color: transparent;
    color: #7DC6DA;
    border: 1px dashed #0f3460;
    border-radius: 4px;
}

.new-tab-button:hover {
    background-color: #0f3460;
}

.content-area {
    background-color: #ffffff;
}

.welcome-text {
    font-size: 24px;
    color: #666666;
}

.status-overlay {
    background-color: rgba(22, 33, 62, 0.9);
    color: #e0e0e0;
    padding: 4px 12px;
    font-size: 12px;
    border-top: 1px solid #0f3460;
}

/* Find bar */
.find-bar {
    background-color: #16213e;
    border-top: 1px solid #0f3460;
    padding: 4px 8px;
}

/* Tab entry in sidebar */
.tab-entry {
    padding: 8px;
    border-radius: 4px;
    margin: 2px 0;
}

.tab-entry:hover {
    background-color: #0f3460;
}

.tab-entry.active {
    background-color: #0f3460;
    border-left: 3px solid #7DC6DA;
}

.tab-entry.suspended {
    opacity: 0.6;
}
"#;
