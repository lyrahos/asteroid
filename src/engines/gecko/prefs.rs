//! Gecko preference management for Asteroid Browser.
//!
//! Defines optimized preference values for memory efficiency,
//! performance, and privacy.

use std::collections::HashMap;

/// Get memory optimization preferences.
pub fn get_optimization_prefs() -> HashMap<String, String> {
    let mut prefs = HashMap::new();

    // Memory optimizations
    prefs.insert(
        "browser.sessionhistory.max_total_viewers".into(),
        "0".into(),
    );
    prefs.insert(
        "browser.sessionstore.interval".into(),
        "60000".into(),
    );
    prefs.insert(
        "browser.cache.memory.capacity".into(),
        "51200".into(), // 50MB max
    );
    prefs.insert(
        "media.memory_cache_max_size".into(),
        "32768".into(), // 32MB
    );
    prefs.insert(
        "browser.tabs.unloadOnLowMemory".into(),
        "true".into(),
    );

    // Aggressive tab discarding
    prefs.insert(
        "browser.tabs.min_inactive_duration_before_unload".into(),
        "300000".into(), // 5 minutes
    );

    // Disable animations/transitions
    prefs.insert("browser.tabs.animate".into(), "false".into());
    prefs.insert("browser.fullscreen.animate".into(), "false".into());

    // Video optimizations
    prefs.insert(
        "media.hardware-video-decoding.enabled".into(),
        "true".into(),
    );
    prefs.insert("media.ffmpeg.vaapi.enabled".into(), "true".into());
    prefs.insert(
        "layers.acceleration.force-enabled".into(),
        "true".into(),
    );
    prefs.insert("gfx.webrender.all".into(), "true".into());

    // Media autoplay control
    prefs.insert("media.autoplay.default".into(), "5".into()); // Block autoplay
    prefs.insert("media.autoplay.block-webaudio".into(), "true".into());

    // Disable unwanted features
    prefs.insert("browser.pocket.enabled".into(), "false".into());
    prefs.insert("browser.newtabpage.activity-stream.feeds.telemetry".into(), "false".into());
    prefs.insert("browser.newtabpage.activity-stream.telemetry".into(), "false".into());
    prefs.insert("toolkit.telemetry.enabled".into(), "false".into());
    prefs.insert("toolkit.telemetry.unified".into(), "false".into());
    prefs.insert("datareporting.healthreport.uploadEnabled".into(), "false".into());
    prefs.insert("datareporting.policy.dataSubmissionEnabled".into(), "false".into());
    prefs.insert("browser.safebrowsing.downloads.enabled".into(), "false".into());
    prefs.insert("browser.safebrowsing.malware.enabled".into(), "false".into());
    prefs.insert("browser.safebrowsing.phishing.enabled".into(), "false".into());

    // Privacy preferences
    prefs.insert("privacy.trackingprotection.enabled".into(), "true".into());
    prefs.insert("privacy.donottrackheader.enabled".into(), "false".into());
    prefs.insert("network.cookie.lifetimePolicy".into(), "0".into());

    // Network optimizations
    prefs.insert("network.http.pipelining".into(), "true".into());
    prefs.insert("network.http.max-persistent-connections-per-server".into(), "8".into());
    prefs.insert("network.dns.disablePrefetch".into(), "false".into());

    prefs
}

/// Get privacy-focused preferences.
pub fn get_privacy_prefs(send_dnt: bool) -> HashMap<String, String> {
    let mut prefs = HashMap::new();

    prefs.insert(
        "privacy.donottrackheader.enabled".into(),
        send_dnt.to_string(),
    );
    prefs.insert("privacy.trackingprotection.enabled".into(), "true".into());
    prefs.insert(
        "privacy.trackingprotection.socialtracking.enabled".into(),
        "true".into(),
    );
    prefs.insert(
        "privacy.trackingprotection.cryptomining.enabled".into(),
        "true".into(),
    );
    prefs.insert(
        "privacy.trackingprotection.fingerprinting.enabled".into(),
        "true".into(),
    );

    // Disable telemetry
    prefs.insert("toolkit.telemetry.enabled".into(), "false".into());
    prefs.insert("toolkit.telemetry.unified".into(), "false".into());
    prefs.insert("toolkit.telemetry.archive.enabled".into(), "false".into());
    prefs.insert(
        "datareporting.healthreport.uploadEnabled".into(),
        "false".into(),
    );

    prefs
}

/// Generate a prefs.js file content from a map of preferences.
pub fn generate_prefs_js(prefs: &HashMap<String, String>) -> String {
    let mut output = String::new();
    output.push_str("// Asteroid Browser - Auto-generated preferences\n");
    output.push_str("// Do not edit manually\n\n");

    let mut sorted_keys: Vec<&String> = prefs.keys().collect();
    sorted_keys.sort();

    for key in sorted_keys {
        let value = &prefs[key];
        // Determine if value should be quoted (string) or not (bool/int)
        if value == "true" || value == "false" || value.parse::<i64>().is_ok() {
            output.push_str(&format!("user_pref(\"{}\", {});\n", key, value));
        } else {
            output.push_str(&format!("user_pref(\"{}\", \"{}\");\n", key, value));
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optimization_prefs_not_empty() {
        let prefs = get_optimization_prefs();
        assert!(!prefs.is_empty());
        assert!(prefs.contains_key("browser.cache.memory.capacity"));
    }

    #[test]
    fn test_privacy_prefs() {
        let prefs = get_privacy_prefs(true);
        assert_eq!(
            prefs.get("privacy.donottrackheader.enabled"),
            Some(&"true".to_string())
        );

        let prefs = get_privacy_prefs(false);
        assert_eq!(
            prefs.get("privacy.donottrackheader.enabled"),
            Some(&"false".to_string())
        );
    }

    #[test]
    fn test_prefs_js_generation() {
        let mut prefs = HashMap::new();
        prefs.insert("test.bool".to_string(), "true".to_string());
        prefs.insert("test.int".to_string(), "42".to_string());
        prefs.insert("test.string".to_string(), "hello".to_string());

        let js = generate_prefs_js(&prefs);
        assert!(js.contains("user_pref(\"test.bool\", true);"));
        assert!(js.contains("user_pref(\"test.int\", 42);"));
        assert!(js.contains("user_pref(\"test.string\", \"hello\");"));
    }
}
