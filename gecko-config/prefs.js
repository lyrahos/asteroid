// Asteroid Browser - Gecko Preferences
// Auto-generated optimized preferences for minimal memory usage

// Memory optimizations
user_pref("browser.sessionhistory.max_total_viewers", 0);
user_pref("browser.sessionstore.interval", 60000);
user_pref("browser.cache.memory.capacity", 51200); // 50MB max
user_pref("media.memory_cache_max_size", 32768); // 32MB
user_pref("browser.tabs.unloadOnLowMemory", true);

// Aggressive tab discarding
user_pref("browser.tabs.min_inactive_duration_before_unload", 300000); // 5min

// Disable animations/transitions
user_pref("browser.tabs.animate", false);
user_pref("browser.fullscreen.animate", false);

// Video optimizations
user_pref("media.hardware-video-decoding.enabled", true);
user_pref("media.ffmpeg.vaapi.enabled", true); // Linux hardware decode
user_pref("layers.acceleration.force-enabled", true);
user_pref("gfx.webrender.all", true); // GPU rendering

// Media autoplay control
user_pref("media.autoplay.default", 5); // Block autoplay
user_pref("media.autoplay.block-webaudio", true);

// Disable unwanted features
user_pref("browser.pocket.enabled", false);
user_pref("browser.newtabpage.activity-stream.feeds.telemetry", false);
user_pref("browser.newtabpage.activity-stream.telemetry", false);
user_pref("toolkit.telemetry.enabled", false);
user_pref("toolkit.telemetry.unified", false);
user_pref("toolkit.telemetry.archive.enabled", false);
user_pref("datareporting.healthreport.uploadEnabled", false);
user_pref("datareporting.policy.dataSubmissionEnabled", false);

// Disable Safe Browsing lookups (we use local content blocking)
user_pref("browser.safebrowsing.downloads.enabled", false);
user_pref("browser.safebrowsing.malware.enabled", false);
user_pref("browser.safebrowsing.phishing.enabled", false);

// Privacy
user_pref("privacy.trackingprotection.enabled", true);
user_pref("privacy.trackingprotection.socialtracking.enabled", true);
user_pref("privacy.trackingprotection.cryptomining.enabled", true);
user_pref("privacy.trackingprotection.fingerprinting.enabled", true);

// Network optimizations
user_pref("network.http.pipelining", true);
user_pref("network.http.max-persistent-connections-per-server", 8);
user_pref("network.dns.disablePrefetch", false);
