//! Content blocking engine for Asteroid Browser.
//!
//! Provides built-in ad and tracker blocking using filter lists
//! (EasyList, EasyPrivacy format). Blocks requests before they reach
//! the network, saving bandwidth, RAM, and CPU.

use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Resource types that can be blocked.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ResourceType {
    Script,
    Image,
    Stylesheet,
    Font,
    Media,
    XmlHttpRequest,
    SubDocument,
    WebSocket,
    Other,
}

impl ResourceType {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "script" => Self::Script,
            "image" | "img" => Self::Image,
            "stylesheet" | "css" => Self::Stylesheet,
            "font" => Self::Font,
            "media" | "video" | "audio" => Self::Media,
            "xmlhttprequest" | "xhr" | "fetch" => Self::XmlHttpRequest,
            "subdocument" | "iframe" => Self::SubDocument,
            "websocket" | "ws" => Self::WebSocket,
            _ => Self::Other,
        }
    }
}

/// A single filter rule.
#[derive(Debug, Clone)]
pub struct FilterRule {
    /// The pattern to match against URLs
    pub pattern: String,
    /// Whether this is a block rule (true) or allow rule (false)
    pub is_block: bool,
    /// Resource types this rule applies to (empty = all)
    pub resource_types: HashSet<ResourceType>,
    /// Domain restrictions (empty = all domains)
    pub domains: HashSet<String>,
    /// Whether this is a third-party only rule
    pub third_party_only: bool,
}

/// Result of checking a URL against the filter engine.
#[derive(Debug, Clone)]
pub struct BlockResult {
    /// Whether the request should be blocked
    pub matched: bool,
    /// The rule that matched (if any)
    pub matching_rule: Option<String>,
    /// Whether this was an exception (allow) rule
    pub is_exception: bool,
}

/// Statistics about content blocking.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BlockerStats {
    /// Total requests checked
    pub total_checked: u64,
    /// Total requests blocked
    pub total_blocked: u64,
    /// Bytes saved (estimated)
    pub bytes_saved: u64,
    /// Number of loaded filter rules
    pub filter_count: usize,
}

impl BlockerStats {
    /// Get the block rate as a percentage.
    pub fn block_rate(&self) -> f64 {
        if self.total_checked == 0 {
            return 0.0;
        }
        (self.total_blocked as f64 / self.total_checked as f64) * 100.0
    }

    /// Get estimated bytes saved in megabytes.
    pub fn bytes_saved_mb(&self) -> f64 {
        self.bytes_saved as f64 / (1024.0 * 1024.0)
    }
}

/// Content blocking engine.
///
/// Uses filter lists in EasyList/AdBlock Plus format to block
/// ads, trackers, and other unwanted content.
pub struct ContentBlocker {
    /// Block rules
    block_rules: Vec<FilterRule>,
    /// Exception (allow) rules
    exception_rules: Vec<FilterRule>,
    /// Known ad/tracker domains for fast lookup
    domain_blocklist: HashSet<String>,
    /// Blocking statistics
    stats: BlockerStats,
    /// Whether blocking is enabled
    enabled: bool,
}

impl ContentBlocker {
    /// Create a new content blocker with default filter lists.
    pub fn new() -> Self {
        let mut blocker = Self {
            block_rules: Vec::new(),
            exception_rules: Vec::new(),
            domain_blocklist: HashSet::new(),
            stats: BlockerStats::default(),
            enabled: true,
        };

        // Load built-in domain blocklist (common ad/tracker domains)
        blocker.load_builtin_domains();

        blocker
    }

    /// Load built-in known ad/tracker domains.
    fn load_builtin_domains(&mut self) {
        let domains = [
            "doubleclick.net",
            "googlesyndication.com",
            "googleadservices.com",
            "google-analytics.com",
            "googletagmanager.com",
            "facebook.net",
            "facebook.com/tr",
            "analytics.google.com",
            "adservice.google.com",
            "pagead2.googlesyndication.com",
            "amazon-adsystem.com",
            "ads.yahoo.com",
            "ad.doubleclick.net",
            "stats.wp.com",
            "pixel.wp.com",
            "scorecardresearch.com",
            "quantserve.com",
            "outbrain.com",
            "taboola.com",
            "criteo.com",
            "rubiconproject.com",
            "pubmatic.com",
            "openx.net",
            "casalemedia.com",
            "adnxs.com",
            "moatads.com",
            "serving-sys.com",
            "sentry-cdn.com",
            "hotjar.com",
            "mouseflow.com",
            "fullstory.com",
            "crazyegg.com",
            "mixpanel.com",
            "amplitude.com",
            "segment.io",
            "segment.com",
            "optimizely.com",
            "newrelic.com",
            "nr-data.net",
        ];

        for domain in &domains {
            self.domain_blocklist.insert(domain.to_string());
        }
    }

    /// Add a filter list in EasyList/AdBlock Plus format.
    pub fn add_filter_list(&mut self, content: &str) {
        for line in content.lines() {
            let line = line.trim();

            // Skip comments and metadata
            if line.is_empty()
                || line.starts_with('!')
                || line.starts_with('[')
            {
                continue;
            }

            // Parse exception rules (@@)
            if line.starts_with("@@") {
                if let Some(rule) = self.parse_rule(&line[2..], false) {
                    self.exception_rules.push(rule);
                }
                continue;
            }

            // Parse block rules
            if let Some(rule) = self.parse_rule(line, true) {
                self.block_rules.push(rule);
            }
        }

        self.stats.filter_count = self.block_rules.len() + self.exception_rules.len();
    }

    /// Parse a single filter rule.
    fn parse_rule(&self, pattern: &str, is_block: bool) -> Option<FilterRule> {
        // Skip element hiding rules (##)
        if pattern.contains("##") || pattern.contains("#@#") {
            return None;
        }

        // Extract options after $
        let (pattern, options) = if let Some(idx) = pattern.rfind('$') {
            (&pattern[..idx], Some(&pattern[idx + 1..]))
        } else {
            (pattern, None)
        };

        let mut rule = FilterRule {
            pattern: pattern.to_string(),
            is_block,
            resource_types: HashSet::new(),
            domains: HashSet::new(),
            third_party_only: false,
        };

        // Parse options
        if let Some(opts) = options {
            for opt in opts.split(',') {
                let opt = opt.trim();
                match opt {
                    "third-party" => rule.third_party_only = true,
                    "script" => { rule.resource_types.insert(ResourceType::Script); }
                    "image" => { rule.resource_types.insert(ResourceType::Image); }
                    "stylesheet" => { rule.resource_types.insert(ResourceType::Stylesheet); }
                    "font" => { rule.resource_types.insert(ResourceType::Font); }
                    "media" => { rule.resource_types.insert(ResourceType::Media); }
                    "xmlhttprequest" => { rule.resource_types.insert(ResourceType::XmlHttpRequest); }
                    "subdocument" => { rule.resource_types.insert(ResourceType::SubDocument); }
                    "websocket" => { rule.resource_types.insert(ResourceType::WebSocket); }
                    _ if opt.starts_with("domain=") => {
                        let domains = &opt[7..];
                        for domain in domains.split('|') {
                            let domain = domain.trim_start_matches('~');
                            rule.domains.insert(domain.to_string());
                        }
                    }
                    _ => {}
                }
            }
        }

        Some(rule)
    }

    /// Check if a URL should be blocked.
    pub fn should_block(
        &mut self,
        url: &str,
        source_url: &str,
        resource_type: &str,
    ) -> BlockResult {
        self.stats.total_checked += 1;

        if !self.enabled {
            return BlockResult {
                matched: false,
                matching_rule: None,
                is_exception: false,
            };
        }

        // Fast path: check domain blocklist
        if let Some(domain) = extract_domain(url) {
            if self.domain_blocklist.contains(domain) {
                self.stats.total_blocked += 1;
                self.stats.bytes_saved += estimate_resource_size(resource_type);
                return BlockResult {
                    matched: true,
                    matching_rule: Some(format!("domain:{}", domain)),
                    is_exception: false,
                };
            }
        }

        let res_type = ResourceType::from_str(resource_type);

        // Check exception rules first
        for rule in &self.exception_rules {
            if self.rule_matches(rule, url, source_url, &res_type) {
                return BlockResult {
                    matched: false,
                    matching_rule: Some(rule.pattern.clone()),
                    is_exception: true,
                };
            }
        }

        // Check block rules
        for rule in &self.block_rules {
            if self.rule_matches(rule, url, source_url, &res_type) {
                self.stats.total_blocked += 1;
                self.stats.bytes_saved += estimate_resource_size(resource_type);
                return BlockResult {
                    matched: true,
                    matching_rule: Some(rule.pattern.clone()),
                    is_exception: false,
                };
            }
        }

        BlockResult {
            matched: false,
            matching_rule: None,
            is_exception: false,
        }
    }

    /// Check if a rule matches a request.
    fn rule_matches(
        &self,
        rule: &FilterRule,
        url: &str,
        _source_url: &str,
        resource_type: &ResourceType,
    ) -> bool {
        // Check resource type filter
        if !rule.resource_types.is_empty() && !rule.resource_types.contains(resource_type) {
            return false;
        }

        // Simple pattern matching
        let pattern = &rule.pattern;

        if pattern.starts_with("||") {
            // Domain anchor
            let domain_pattern = &pattern[2..];
            if let Some(domain) = extract_domain(url) {
                return domain.contains(domain_pattern)
                    || url.contains(domain_pattern);
            }
        } else if pattern.starts_with('|') && pattern.ends_with('|') {
            // Exact match
            let exact = &pattern[1..pattern.len() - 1];
            return url == exact;
        } else if pattern.contains('*') {
            // Wildcard matching
            return wildcard_match(pattern, url);
        } else {
            // Substring match
            return url.contains(pattern.as_str());
        }

        false
    }

    /// Enable or disable content blocking.
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Check if blocking is enabled.
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Get blocking statistics.
    pub fn stats(&self) -> &BlockerStats {
        &self.stats
    }

    /// Reset statistics.
    pub fn reset_stats(&mut self) {
        self.stats.total_checked = 0;
        self.stats.total_blocked = 0;
        self.stats.bytes_saved = 0;
    }
}

impl Default for ContentBlocker {
    fn default() -> Self {
        Self::new()
    }
}

/// Extract the domain from a URL.
fn extract_domain(url: &str) -> Option<&str> {
    let url = url.trim_start_matches("https://")
        .trim_start_matches("http://");
    url.split('/').next()
        .map(|d| d.split(':').next().unwrap_or(d))
}

/// Simple wildcard pattern matching.
fn wildcard_match(pattern: &str, text: &str) -> bool {
    let parts: Vec<&str> = pattern.split('*').collect();

    if parts.is_empty() {
        return true;
    }

    let mut pos = 0;
    for (i, part) in parts.iter().enumerate() {
        if part.is_empty() {
            continue;
        }
        if let Some(found) = text[pos..].find(part) {
            if i == 0 && found != 0 && !pattern.starts_with('*') {
                return false;
            }
            pos += found + part.len();
        } else {
            return false;
        }
    }

    true
}

/// Estimate the size of a resource based on its type.
fn estimate_resource_size(resource_type: &str) -> u64 {
    match resource_type.to_lowercase().as_str() {
        "script" => 50_000,       // ~50KB average
        "image" | "img" => 30_000, // ~30KB average
        "stylesheet" | "css" => 20_000,
        "font" => 40_000,
        "media" | "video" => 500_000,
        "xmlhttprequest" | "xhr" => 10_000,
        _ => 15_000,
    }
}

/// Default EasyList filter list content (abbreviated - in production,
/// this would be loaded from a file or fetched from the network).
pub const DEFAULT_FILTERS: &str = r#"[Adblock Plus 2.0]
! Title: Asteroid Browser Default Filters
! Description: Built-in ad and tracker blocking filters
! Last modified: 2026-02-01

! --- Ad Networks ---
||doubleclick.net^
||googlesyndication.com^
||googleadservices.com^
||google-analytics.com^
||googletagmanager.com^
||amazon-adsystem.com^
||ads.yahoo.com^
||adnxs.com^
||rubiconproject.com^
||pubmatic.com^
||openx.net^
||casalemedia.com^
||criteo.com^
||outbrain.com^
||taboola.com^
||moatads.com^
||serving-sys.com^

! --- Trackers ---
||scorecardresearch.com^
||quantserve.com^
||hotjar.com^
||mouseflow.com^
||fullstory.com^
||crazyegg.com^
||mixpanel.com^
||amplitude.com^
||segment.io^
||segment.com^
||optimizely.com^
||newrelic.com^
||nr-data.net^

! --- Social Trackers ---
||facebook.net/tr^
||connect.facebook.net/en_US/fbevents.js
||platform.twitter.com/widgets.js$third-party
||platform.linkedin.com/badges/$third-party

! --- Common Ad Paths ---
*/ads/*$image,script,subdocument
*/adserver/*$script
*/advert/*$image,script
*/banner/*$image
*/popup/*$subdocument
*/tracking/*$script,xmlhttprequest
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_content_blocker_creation() {
        let blocker = ContentBlocker::new();
        assert!(blocker.is_enabled());
        assert!(!blocker.domain_blocklist.is_empty());
    }

    #[test]
    fn test_domain_extraction() {
        assert_eq!(
            extract_domain("https://example.com/path"),
            Some("example.com")
        );
        assert_eq!(
            extract_domain("http://sub.example.com:8080/path"),
            Some("sub.example.com")
        );
    }

    #[test]
    fn test_block_known_domain() {
        let mut blocker = ContentBlocker::new();
        let result = blocker.should_block(
            "https://doubleclick.net/ads/something",
            "https://example.com",
            "script",
        );
        assert!(result.matched);
    }

    #[test]
    fn test_allow_normal_domain() {
        let mut blocker = ContentBlocker::new();
        let result = blocker.should_block(
            "https://example.com/page.html",
            "https://example.com",
            "document",
        );
        assert!(!result.matched);
    }

    #[test]
    fn test_wildcard_matching() {
        assert!(wildcard_match("*.js", "script.js"));
        assert!(wildcard_match("ads*banner", "ads_big_banner"));
        assert!(!wildcard_match("exact", "not_exact"));
    }

    #[test]
    fn test_filter_list_loading() {
        let mut blocker = ContentBlocker::new();
        blocker.add_filter_list(DEFAULT_FILTERS);
        assert!(blocker.stats.filter_count > 0);
    }

    #[test]
    fn test_blocker_stats() {
        let mut blocker = ContentBlocker::new();
        blocker.should_block("https://doubleclick.net/ad", "https://example.com", "script");
        blocker.should_block("https://example.com/page", "https://example.com", "document");

        assert_eq!(blocker.stats().total_checked, 2);
        assert_eq!(blocker.stats().total_blocked, 1);
        assert!(blocker.stats().block_rate() > 0.0);
    }

    #[test]
    fn test_resource_type_parsing() {
        assert_eq!(ResourceType::from_str("script"), ResourceType::Script);
        assert_eq!(ResourceType::from_str("image"), ResourceType::Image);
        assert_eq!(ResourceType::from_str("xhr"), ResourceType::XmlHttpRequest);
        assert_eq!(ResourceType::from_str("unknown"), ResourceType::Other);
    }
}
