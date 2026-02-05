//! Settings page for Asteroid Browser.
//!
//! Generates an HTML-based settings UI that is displayed
//! within the browser itself (at asteroid://settings).

use crate::core::config::Config;

/// Generate the settings HTML page.
pub fn generate_settings_html(config: &Config) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Asteroid Browser Settings</title>
    <style>
        * {{ margin: 0; padding: 0; box-sizing: border-box; }}
        body {{
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
            background-color: #0a0e1a;
            color: #e0e0e0;
            max-width: 800px;
            margin: 0 auto;
            padding: 40px 20px;
        }}
        h1 {{
            color: #7DC6DA;
            margin-bottom: 30px;
            font-size: 28px;
        }}
        h2 {{
            color: #AAC9DC;
            margin: 30px 0 15px;
            font-size: 20px;
            border-bottom: 1px solid #16213e;
            padding-bottom: 8px;
        }}
        .setting {{
            display: flex;
            align-items: center;
            justify-content: space-between;
            padding: 12px 0;
            border-bottom: 1px solid #16213e;
        }}
        .setting label {{
            font-size: 14px;
        }}
        .setting .description {{
            font-size: 12px;
            color: #888;
            margin-top: 4px;
        }}
        input[type="checkbox"] {{
            width: 20px;
            height: 20px;
            accent-color: #7DC6DA;
        }}
        input[type="number"], select {{
            background-color: #16213e;
            color: #e0e0e0;
            border: 1px solid #0f3460;
            border-radius: 4px;
            padding: 6px 12px;
            font-size: 14px;
        }}
        .version-info {{
            margin-top: 40px;
            padding: 20px;
            background-color: #16213e;
            border-radius: 8px;
            text-align: center;
            color: #888;
        }}
        .version-info .name {{
            font-size: 18px;
            color: #7DC6DA;
        }}
    </style>
</head>
<body>
    <h1>Settings</h1>

    <h2>General</h2>
    <div class="setting">
        <div>
            <label>Enable tab suspension</label>
            <div class="description">Suspend inactive tabs after {} seconds</div>
        </div>
        <input type="checkbox" id="tab-suspension" {}>
    </div>
    <div class="setting">
        <div>
            <label>Show vertical tab sidebar</label>
            <div class="description">Display tabs in a sidebar instead of horizontal strip</div>
        </div>
        <input type="checkbox" id="vertical-tabs" {}>
    </div>
    <div class="setting">
        <div>
            <label>Vim-style link hints</label>
            <div class="description">Press 'f' to show numbered hints on clickable elements</div>
        </div>
        <input type="checkbox" id="vim-hints" {}>
    </div>
    <div class="setting">
        <div>
            <label>Check for updates automatically</label>
            <div class="description">Check GitHub releases for new versions</div>
        </div>
        <input type="checkbox" id="auto-update" {}>
    </div>

    <h2>Performance</h2>
    <div class="setting">
        <div>
            <label>Hardware video acceleration</label>
            <div class="description">Use VA-API for hardware video decoding</div>
        </div>
        <input type="checkbox" id="hw-accel" {}>
    </div>
    <div class="setting">
        <div>
            <label>Memory management</label>
            <div class="description">Aggressiveness of memory trimming</div>
        </div>
        <select id="memory-trim">
            <option value="off" {}>Off</option>
            <option value="moderate" {}>Moderate</option>
            <option value="aggressive" {}>Aggressive</option>
        </select>
    </div>
    <div class="setting">
        <div>
            <label>Cache size (MB)</label>
            <div class="description">Maximum disk cache size</div>
        </div>
        <input type="number" id="cache-size" value="{}" min="10" max="500">
    </div>

    <h2>Privacy</h2>
    <div class="setting">
        <div>
            <label>Block ads and trackers</label>
            <div class="description">Built-in content blocking using filter lists</div>
        </div>
        <input type="checkbox" id="block-ads" {}>
    </div>
    <div class="setting">
        <div>
            <label>Send Do Not Track header</label>
            <div class="description">Request websites not to track you</div>
        </div>
        <input type="checkbox" id="send-dnt" {}>
    </div>
    <div class="setting">
        <div>
            <label>Delete cookies on close</label>
            <div class="description">Clear all cookies when the browser is closed</div>
        </div>
        <input type="checkbox" id="clear-cookies" {}>
    </div>
    <div class="setting">
        <div>
            <label>HTTPS-only mode</label>
            <div class="description">Warn when navigating to non-HTTPS sites</div>
        </div>
        <input type="checkbox" id="https-only" {}>
    </div>

    <h2>Advanced</h2>
    <div class="setting">
        <div>
            <label>Rendering engine</label>
            <div class="description">Current rendering engine</div>
        </div>
        <select id="engine" disabled>
            <option value="gecko" selected>Gecko v124</option>
            <option value="servo">Servo (when available)</option>
        </select>
    </div>
    <div class="setting">
        <div>
            <label>Enable developer tools</label>
            <div class="description">Show web inspector and console</div>
        </div>
        <input type="checkbox" id="devtools" {}>
    </div>

    <div class="version-info">
        <div class="name">Asteroid Browser</div>
        <div>Version {}</div>
        <div>Engine: {} v{}</div>
    </div>
</body>
</html>"#,
        config.general.tab_suspension_delay,
        if config.general.tab_suspension_enabled { "checked" } else { "" },
        if config.general.vertical_tabs { "checked" } else { "" },
        if config.general.vim_hints { "checked" } else { "" },
        if config.general.auto_update_check { "checked" } else { "" },
        if config.performance.hardware_acceleration { "checked" } else { "" },
        if config.performance.memory_trim_level == "off" { "selected" } else { "" },
        if config.performance.memory_trim_level == "moderate" { "selected" } else { "" },
        if config.performance.memory_trim_level == "aggressive" { "selected" } else { "" },
        config.performance.cache_size_mb,
        if config.privacy.block_ads { "checked" } else { "" },
        if config.privacy.send_dnt { "checked" } else { "" },
        if config.privacy.clear_cookies_on_close { "checked" } else { "" },
        if config.privacy.https_only { "checked" } else { "" },
        if config.ui.developer_tools { "checked" } else { "" },
        env!("CARGO_PKG_VERSION"),
        config.engine.current,
        "124.0",
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_settings_html_generation() {
        let config = Config::default();
        let html = generate_settings_html(&config);
        assert!(html.contains("Asteroid Browser Settings"));
        assert!(html.contains("tab suspension"));
        assert!(html.contains("Hardware video acceleration"));
        assert!(html.contains("Block ads"));
    }
}
