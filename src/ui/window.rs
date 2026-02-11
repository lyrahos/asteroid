//! Main browser window implementation.
//!
//! Creates the primary GTK4 application window with:
//! - Navigation toolbar (back, forward, reload, address bar, new tab, menu)
//! - Tabbed browsing via GTK4 Notebook (each tab is a WebKitGTK WebView)
//! - Status overlay (bottom-left, appears on hover/activity)

#![allow(dead_code)]

use std::cell::RefCell;
use std::rc::Rc;

use gtk4::prelude::*;
use gtk4::{
    Application, ApplicationWindow, Box as GtkBox, Button, Entry,
    Label, Notebook, Orientation, Separator,
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
    new_tab_btn: Button,
}

/// Build the main browser window with tabbed WebKitGTK rendering.
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

    // Notebook for tabbed browsing
    let notebook = Notebook::new();
    notebook.set_scrollable(true);
    notebook.set_show_border(false);
    notebook.set_vexpand(true);
    notebook.set_hexpand(true);
    notebook.add_css_class("browser-notebook");
    main_box.append(&notebook);

    // Status bar
    let status_label = Label::new(Some("Ready"));
    status_label.set_halign(gtk4::Align::Start);
    status_label.set_margin_start(8);
    status_label.set_margin_bottom(4);
    status_label.add_css_class("status-overlay");
    main_box.append(&status_label);

    window.set_child(Some(&main_box));

    // --- Create the first tab ---
    let initial_url = {
        let s = state.borrow();
        s.tab_manager.active_tab_id()
            .and_then(|vid| s.engine.get_navigation_state(vid).ok())
            .and_then(|nav| {
                if !nav.url.is_empty() && nav.url != "about:blank" {
                    Some(nav.url.clone())
                } else {
                    None
                }
            })
    };
    create_tab(&notebook, &tw.address_bar, &window, &status_label, initial_url.as_deref());

    // --- Wire toolbar signals ---

    // New tab button
    {
        let nb = notebook.clone();
        let addr = tw.address_bar.clone();
        let win = window.clone();
        let status = status_label.clone();
        tw.new_tab_btn.connect_clicked(move |_| {
            create_tab(&nb, &addr, &win, &status, None);
            addr.set_text("");
            addr.grab_focus();
        });
    }

    // Address bar: Enter key loads URL in active tab
    {
        let nb = notebook.clone();
        tw.address_bar.connect_activate(move |entry| {
            let text = entry.text().to_string();
            if text.trim().is_empty() {
                return;
            }
            let url = parse_address_input(&text);
            entry.set_text(&url);
            if let Some(wv) = active_webview(&nb) {
                wv.load_uri(&url);
            }
        });
    }

    // Back button
    {
        let nb = notebook.clone();
        tw.back_btn.connect_clicked(move |_| {
            if let Some(wv) = active_webview(&nb) {
                wv.go_back();
            }
        });
    }

    // Forward button
    {
        let nb = notebook.clone();
        tw.forward_btn.connect_clicked(move |_| {
            if let Some(wv) = active_webview(&nb) {
                wv.go_forward();
            }
        });
    }

    // Reload button
    {
        let nb = notebook.clone();
        tw.reload_btn.connect_clicked(move |_| {
            if let Some(wv) = active_webview(&nb) {
                wv.reload();
            }
        });
    }

    // Tab switch: update address bar and window title from newly active tab
    {
        let addr = tw.address_bar.clone();
        let win = window.clone();
        let status = status_label.clone();
        notebook.connect_switch_page(move |_nb, page, _page_num| {
            if let Some(wv) = page.downcast_ref::<WebView>() {
                // Update address bar
                match wv.uri() {
                    Some(uri) if uri != "about:blank" => addr.set_text(&uri),
                    _ => addr.set_text(""),
                }
                // Update window title
                match wv.title() {
                    Some(title) if !title.is_empty() => {
                        win.set_title(Some(&format!("{} - Asteroid Browser", title)));
                    }
                    _ => win.set_title(Some("Asteroid Browser")),
                }
                // Update status bar
                let progress = wv.estimated_load_progress();
                if progress < 1.0 {
                    status.set_text(&format!("Loading {:.0}%", progress * 100.0));
                } else {
                    status.set_text("Ready");
                }
            }
        });
    }

    window
}

/// Get the WebView from the currently active notebook tab.
fn active_webview(notebook: &Notebook) -> Option<WebView> {
    let page_num = notebook.current_page()?;
    let page = notebook.nth_page(Some(page_num))?;
    page.downcast::<WebView>().ok()
}

/// Create a new browser tab with its own WebView, add it to the notebook,
/// and wire all signals (URI, title, progress, crash recovery, close button).
fn create_tab(
    notebook: &Notebook,
    address_bar: &Entry,
    window: &ApplicationWindow,
    status_label: &Label,
    url: Option<&str>,
) {
    let webview = WebView::new();
    webview.set_vexpand(true);
    webview.set_hexpand(true);

    // Configure WebView settings
    configure_webview_settings(&webview);
    setup_content_filter(&webview);

    // Build tab label: [title] [x]
    let tab_box = GtkBox::new(Orientation::Horizontal, 4);
    let title_label = Label::new(Some("New Tab"));
    title_label.set_ellipsize(gtk4::pango::EllipsizeMode::End);
    title_label.set_max_width_chars(20);
    title_label.set_width_chars(8);

    let close_btn = Button::with_label("\u{00D7}"); // x
    close_btn.set_has_frame(false);
    close_btn.add_css_class("tab-close-button");

    tab_box.append(&title_label);
    tab_box.append(&close_btn);

    // Add to notebook and switch to the new tab
    notebook.append_page(&webview, Some(&tab_box));
    notebook.set_tab_reorderable(&webview, true);
    if let Some(page_num) = notebook.page_num(&webview) {
        notebook.set_current_page(Some(page_num));
    }

    // --- Wire WebView signals ---

    // Update tab label when page title changes
    {
        let lbl = title_label.clone();
        webview.connect_notify_local(Some("title"), move |wv, _| {
            if let Some(title) = wv.title() {
                let t = title.to_string();
                if !t.is_empty() {
                    lbl.set_text(&t);
                    lbl.set_tooltip_text(Some(&t));
                }
            }
        });
    }

    // URI changed -> update address bar (only if this is the active tab)
    {
        let nb = notebook.clone();
        let addr = address_bar.clone();
        webview.connect_notify_local(Some("uri"), move |wv, _| {
            if nb.page_num(wv) == nb.current_page() {
                if let Some(uri) = wv.uri() {
                    addr.set_text(&uri);
                }
            }
        });
    }

    // Title changed -> update window title (only if this is the active tab)
    {
        let nb = notebook.clone();
        let win = window.clone();
        webview.connect_notify_local(Some("title"), move |wv, _| {
            if nb.page_num(wv) == nb.current_page() {
                if let Some(title) = wv.title() {
                    let t = title.to_string();
                    if !t.is_empty() {
                        win.set_title(Some(&format!("{} - Asteroid Browser", t)));
                    }
                }
            }
        });
    }

    // Load progress -> update status bar (only if this is the active tab)
    {
        let nb = notebook.clone();
        let status = status_label.clone();
        webview.connect_notify_local(Some("estimated-load-progress"), move |wv, _| {
            if nb.page_num(wv) == nb.current_page() {
                let progress = wv.estimated_load_progress();
                if progress < 1.0 {
                    status.set_text(&format!("Loading {:.0}%", progress * 100.0));
                } else {
                    status.set_text("Done");
                }
            }
        });
    }

    // Crash recovery: auto-reload instead of leaving a frozen tab
    {
        let wv = webview.clone();
        let status = status_label.clone();
        webview.connect_web_process_terminated(move |_wv, reason| {
            log::error!("WebProcess terminated: {:?}", reason);
            status.set_text("Page crashed \u{2014} reloading...");
            let wv_reload = wv.clone();
            gtk4::glib::timeout_add_local_once(
                std::time::Duration::from_millis(500),
                move || {
                    wv_reload.reload();
                },
            );
        });
    }

    // Close button: remove this tab (but keep at least one tab open)
    {
        let nb = notebook.clone();
        let wv = webview.clone();
        close_btn.connect_clicked(move |_| {
            if nb.n_pages() > 1 {
                if let Some(page_num) = nb.page_num(&wv) {
                    nb.remove_page(Some(page_num));
                }
            }
        });
    }

    // Load URL if provided
    if let Some(url) = url {
        webview.load_uri(url);
    }
}

/// Configure WebView settings for speed + reasonable RAM usage.
fn configure_webview_settings(webview: &WebView) {
    let settings: webkit6::Settings = webkit6::prelude::WebViewExt::settings(webview).unwrap();
    settings.set_enable_page_cache(true);
    settings.set_enable_dns_prefetching(true);
    // Hardware acceleration defaults to OnDemand (WebKit picks fastest path)
    settings.set_enable_offline_web_application_cache(false);
    settings.set_enable_html5_database(false);
    settings.set_enable_smooth_scrolling(false);
    settings.set_enable_developer_extras(false);
    // media_stream + webrtc needed for YouTube, Twitch, and other video sites
    settings.set_enable_media_stream(true);
    settings.set_enable_webrtc(true);
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

    let new_tab_btn = Button::with_label("+");
    new_tab_btn.set_tooltip_text(Some("New Tab (Ctrl+T)"));
    new_tab_btn.add_css_class("nav-button");
    toolbar.append(&new_tab_btn);

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
        new_tab_btn,
    }
}

/// Build the vertical tab sidebar (hidden by default, for future sidebar mode).
fn build_tab_sidebar() -> GtkBox {
    let sidebar = GtkBox::new(Orientation::Vertical, 2);
    sidebar.set_width_request(200);
    sidebar.add_css_class("tab-sidebar");

    let header = Label::new(Some("Tabs"));
    header.add_css_class("sidebar-header");
    sidebar.append(&header);

    let separator = Separator::new(Orientation::Horizontal);
    sidebar.append(&separator);

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

/* Notebook tab strip */
.browser-notebook header {
    background-color: #1a1a2e;
    border-bottom: 1px solid #0f3460;
}

.browser-notebook tab {
    background-color: #16213e;
    color: #e0e0e0;
    border: none;
    padding: 4px 8px;
    border-radius: 4px 4px 0 0;
    margin: 0 1px;
}

.browser-notebook tab:checked {
    background-color: #0f3460;
    border-bottom: 2px solid #7DC6DA;
}

.browser-notebook tab:hover {
    background-color: #0f3460;
}

.tab-close-button {
    min-width: 16px;
    min-height: 16px;
    padding: 0 2px;
    margin: 0 0 0 4px;
    color: #888;
    font-size: 14px;
}

.tab-close-button:hover {
    color: #ff4444;
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

/* Tab sidebar (hidden by default) */
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
"#;

/// Set up WebKitGTK content filter to block ads/trackers at the network level.
/// Loads comprehensive rules from resources/filters/adblock.json (updatable
/// without recompile). Falls back to a minimal embedded list if file is missing.
fn setup_content_filter(webview: &WebView) {
    let filter_dir = dirs::cache_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("/tmp"))
        .join("asteroid-browser")
        .join("content-filters");
    let _ = std::fs::create_dir_all(&filter_dir);

    let store = webkit6::UserContentFilterStore::new(
        &filter_dir.to_string_lossy(),
    );

    // Try loading comprehensive filter list from file (user-updatable)
    let rules_json = load_filter_rules();
    let rules_bytes = gtk4::glib::Bytes::from_owned(rules_json.into_bytes());

    let ucm = webview.user_content_manager().unwrap();

    store.save(
        "asteroid-adblock",
        &rules_bytes,
        None::<&gtk4::gio::Cancellable>,
        move |result| {
            if let Ok(filter) = result {
                ucm.add_filter(&filter);
            }
        },
    );
}

/// Load filter rules from file, checking multiple paths.
/// Returns the JSON string, falling back to a minimal embedded list.
fn load_filter_rules() -> String {
    // Paths to check, in order of priority:
    // 1. User config override
    // 2. Installed system path (RPM/DEB)
    // 3. Source tree (development)
    let paths = [
        dirs::config_dir()
            .map(|d| d.join("asteroid-browser").join("filters").join("adblock.json")),
        Some(std::path::PathBuf::from("/usr/share/asteroid-browser/filters/adblock.json")),
        Some(std::path::PathBuf::from("resources/filters/adblock.json")),
    ];

    for path in paths.iter().flatten() {
        if let Ok(content) = std::fs::read_to_string(path) {
            if !content.trim().is_empty() {
                return content;
            }
        }
    }

    // Minimal fallback if no file found
    FALLBACK_FILTER_JSON.to_string()
}

/// Minimal fallback filter rules if the full adblock.json is not found.
const FALLBACK_FILTER_JSON: &str = r#"[
{"trigger":{"url-filter":"google-analytics\\.com"},"action":{"type":"block"}},
{"trigger":{"url-filter":"googletagmanager\\.com"},"action":{"type":"block"}},
{"trigger":{"url-filter":"doubleclick\\.net"},"action":{"type":"block"}},
{"trigger":{"url-filter":"googlesyndication\\.com"},"action":{"type":"block"}},
{"trigger":{"url-filter":"connect\\.facebook\\.net"},"action":{"type":"block"}},
{"trigger":{"url-filter":"adnxs\\.com"},"action":{"type":"block"}},
{"trigger":{"url-filter":"criteo\\.com"},"action":{"type":"block"}},
{"trigger":{"url-filter":"outbrain\\.com"},"action":{"type":"block"}},
{"trigger":{"url-filter":"taboola\\.com"},"action":{"type":"block"}}
]"#;
