//! Main browser window implementation.
//!
//! Creates the primary GTK4 application window with minimal chrome:
//! - Navigation toolbar (back, forward, reload, address bar, menu)
//! - Optional vertical tab sidebar
//! - WebKitGTK web content area
//! - Status overlay (bottom-left, appears on hover/activity)

#![allow(dead_code)]

use std::cell::RefCell;
use std::rc::Rc;

use gtk4::prelude::*;
use gtk4::{
    Application, ApplicationWindow, Box as GtkBox, Button, Entry,
    Label, Orientation, Paned, ScrolledWindow, Separator,
};

use webkit6::prelude::*;
use webkit6::WebView;

use crate::core::engine::BrowserEngine;
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

/// Build the main browser window with WebKitGTK rendering.
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

    // WebView content area (WebKitGTK renders actual web pages)
    let content_area = GtkBox::new(Orientation::Vertical, 0);
    content_area.set_hexpand(true);
    content_area.set_vexpand(true);
    content_area.add_css_class("content-area");

    let webview = WebView::new();
    webview.set_vexpand(true);
    webview.set_hexpand(true);

    // Configure WebView for speed + reasonable RAM
    let settings: webkit6::Settings = webkit6::prelude::WebViewExt::settings(&webview).unwrap();
    // Page cache: keeps rendered pages in memory for instant back/forward
    settings.set_enable_page_cache(true);
    // DNS prefetching: resolve DNS for links before user clicks them
    settings.set_enable_dns_prefetching(true);
    // Hardware acceleration defaults to OnDemand (WebKit picks fastest path)
    // Disable features we don't need (saves RAM without hurting speed)
    settings.set_enable_offline_web_application_cache(false);
    settings.set_enable_html5_database(false);
    settings.set_enable_smooth_scrolling(false);
    settings.set_enable_developer_extras(false);
    settings.set_enable_media_stream(false);
    settings.set_enable_webrtc(false);

    content_area.append(&webview);

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

    // Address bar: Enter key loads URL in WebView
    {
        let wv = webview.clone();
        tw.address_bar.connect_activate(move |entry| {
            let text = entry.text().to_string();
            if text.trim().is_empty() {
                return;
            }
            let url = parse_address_input(&text);
            entry.set_text(&url);
            wv.load_uri(&url);
        });
    }

    // Back button
    {
        let wv = webview.clone();
        tw.back_btn.connect_clicked(move |_| {
            wv.go_back();
        });
    }

    // Forward button
    {
        let wv = webview.clone();
        tw.forward_btn.connect_clicked(move |_| {
            wv.go_forward();
        });
    }

    // Reload button
    {
        let wv = webview.clone();
        tw.reload_btn.connect_clicked(move |_| {
            wv.reload();
        });
    }

    // WebView URI changed -> update address bar
    {
        let addr = tw.address_bar.clone();
        webview.connect_notify_local(Some("uri"), move |wv, _| {
            if let Some(uri) = wv.uri() {
                addr.set_text(&uri);
            }
        });
    }

    // WebView title changed -> update window title
    {
        let win = window.clone();
        webview.connect_notify_local(Some("title"), move |wv, _| {
            if let Some(title) = wv.title() {
                let t = title.to_string();
                if !t.is_empty() {
                    win.set_title(Some(&format!("{} - Asteroid Browser", t)));
                }
            }
        });
    }

    // WebView load progress -> update status bar
    {
        let status = status_label.clone();
        webview.connect_notify_local(Some("estimated-load-progress"), move |wv, _| {
            let progress = wv.estimated_load_progress();
            if progress < 1.0 {
                status.set_text(&format!("Loading {:.0}%", progress * 100.0));
            } else {
                status.set_text("Done");
            }
        });
    }

    // Load initial page from active tab state
    {
        let s = state.borrow();
        if let Some(view_id) = s.tab_manager.active_tab_id() {
            if let Ok(nav) = s.engine.get_navigation_state(view_id) {
                if !nav.url.is_empty() && nav.url != "about:blank" {
                    webview.load_uri(&nav.url);
                    tw.address_bar.set_text(&nav.url);
                }
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
