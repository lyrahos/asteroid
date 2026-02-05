//! Main browser window implementation.
//!
//! Creates the primary GTK4 application window with minimal chrome:
//! - Navigation toolbar (back, forward, reload, address bar, menu)
//! - Optional vertical tab sidebar
//! - Web content area
//! - Status overlay (bottom-left, appears on hover/activity)

use gtk4::prelude::*;
use gtk4::{
    Application, ApplicationWindow, Box as GtkBox, Button, Entry,
    Label, Orientation, Paned, ScrolledWindow, Separator,
};

/// Build the main browser window.
pub fn build_window(app: &Application) -> ApplicationWindow {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Asteroid Browser")
        .default_width(1280)
        .default_height(800)
        .build();

    // Main vertical layout
    let main_box = GtkBox::new(Orientation::Vertical, 0);

    // Build toolbar
    let toolbar = build_toolbar();
    main_box.append(&toolbar);

    // Horizontal layout for sidebar + content
    let content_paned = Paned::new(Orientation::Horizontal);

    // Tab sidebar (hidden by default, toggleable with F1)
    let sidebar = build_tab_sidebar();
    sidebar.set_visible(false);
    content_paned.set_start_child(Some(&sidebar));
    content_paned.set_position(200);

    // Web content area placeholder
    let content_area = build_content_area();
    content_paned.set_end_child(Some(&content_area));

    main_box.append(&content_paned);

    // Status bar overlay
    let status_label = Label::new(Some("Ready"));
    status_label.set_halign(gtk4::Align::Start);
    status_label.set_margin_start(8);
    status_label.set_margin_bottom(4);
    status_label.add_css_class("status-overlay");
    main_box.append(&status_label);

    window.set_child(Some(&main_box));
    window
}

/// Build the navigation toolbar.
fn build_toolbar() -> GtkBox {
    let toolbar = GtkBox::new(Orientation::Horizontal, 4);
    toolbar.set_margin_start(4);
    toolbar.set_margin_end(4);
    toolbar.set_margin_top(4);
    toolbar.set_margin_bottom(4);
    toolbar.add_css_class("toolbar");

    // Back button
    let back_btn = Button::with_label("\u{2190}"); // ←
    back_btn.set_tooltip_text(Some("Back (Alt+Left)"));
    back_btn.add_css_class("nav-button");
    toolbar.append(&back_btn);

    // Forward button
    let forward_btn = Button::with_label("\u{2192}"); // →
    forward_btn.set_tooltip_text(Some("Forward (Alt+Right)"));
    forward_btn.add_css_class("nav-button");
    toolbar.append(&forward_btn);

    // Reload button
    let reload_btn = Button::with_label("\u{27F3}"); // ⟳
    reload_btn.set_tooltip_text(Some("Reload (F5)"));
    reload_btn.add_css_class("nav-button");
    toolbar.append(&reload_btn);

    // Address bar
    let address_bar = Entry::new();
    address_bar.set_placeholder_text(Some("Enter URL or search..."));
    address_bar.set_hexpand(true);
    address_bar.add_css_class("address-bar");
    toolbar.append(&address_bar);

    // Menu button
    let menu_btn = Button::with_label("\u{2630}"); // ☰
    menu_btn.set_tooltip_text(Some("Menu"));
    menu_btn.add_css_class("menu-button");
    toolbar.append(&menu_btn);

    toolbar
}

/// Build the vertical tab sidebar.
fn build_tab_sidebar() -> GtkBox {
    let sidebar = GtkBox::new(Orientation::Vertical, 2);
    sidebar.set_width_request(200);
    sidebar.add_css_class("tab-sidebar");

    // Sidebar header
    let header = Label::new(Some("Tabs"));
    header.add_css_class("sidebar-header");
    sidebar.append(&header);

    let separator = Separator::new(Orientation::Horizontal);
    sidebar.append(&separator);

    // Scrollable tab list
    let scrolled = ScrolledWindow::new();
    scrolled.set_vexpand(true);

    let tab_list = GtkBox::new(Orientation::Vertical, 1);
    tab_list.add_css_class("tab-list");

    // New Tab button at bottom
    let new_tab_btn = Button::with_label("+ New Tab");
    new_tab_btn.set_tooltip_text(Some("New Tab (Ctrl+T)"));
    new_tab_btn.add_css_class("new-tab-button");

    scrolled.set_child(Some(&tab_list));
    sidebar.append(&scrolled);
    sidebar.append(&new_tab_btn);

    sidebar
}

/// Build the main content area.
fn build_content_area() -> GtkBox {
    let content = GtkBox::new(Orientation::Vertical, 0);
    content.set_hexpand(true);
    content.set_vexpand(true);
    content.add_css_class("content-area");

    // Placeholder for web view
    let placeholder = Label::new(Some("Asteroid Browser\n\nLightweight. Fast. Independent."));
    placeholder.set_vexpand(true);
    placeholder.set_hexpand(true);
    placeholder.set_valign(gtk4::Align::Center);
    placeholder.set_halign(gtk4::Align::Center);
    placeholder.add_css_class("welcome-text");

    content.append(&placeholder);
    content
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
