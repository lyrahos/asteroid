//! Toolbar component for Asteroid Browser.
//!
//! Minimal navigation toolbar with:
//! - Back/Forward/Reload buttons
//! - Combined address/search bar (omnibox)
//! - Menu button

/// Toolbar action events.
#[derive(Debug, Clone)]
pub enum ToolbarAction {
    /// Navigate back
    GoBack,
    /// Navigate forward
    GoForward,
    /// Reload current page
    Reload,
    /// Navigate to URL or perform search
    Navigate(String),
    /// Open menu
    OpenMenu,
    /// Stop loading
    Stop,
}

/// Determine if input is a URL or search query.
pub fn parse_address_input(input: &str) -> String {
    let trimmed = input.trim();

    // Check if it looks like a URL
    if trimmed.contains("://") {
        return trimmed.to_string();
    }

    // Check for common URL patterns
    if trimmed.contains('.') && !trimmed.contains(' ') {
        // Looks like a domain name
        return format!("https://{}", trimmed);
    }

    // Treat as a search query
    format!(
        "https://duckduckgo.com/?q={}",
        urlencoding_encode(trimmed)
    )
}

/// Simple URL encoding for search queries.
fn urlencoding_encode(input: &str) -> String {
    input
        .chars()
        .map(|c| match c {
            'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '~' => {
                c.to_string()
            }
            ' ' => "+".to_string(),
            _ => format!("%{:02X}", c as u8),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_url() {
        assert_eq!(
            parse_address_input("https://example.com"),
            "https://example.com"
        );
        assert_eq!(
            parse_address_input("http://localhost:8080"),
            "http://localhost:8080"
        );
    }

    #[test]
    fn test_parse_domain() {
        assert_eq!(
            parse_address_input("example.com"),
            "https://example.com"
        );
        assert_eq!(
            parse_address_input("google.com"),
            "https://google.com"
        );
    }

    #[test]
    fn test_parse_search() {
        let result = parse_address_input("rust programming");
        assert!(result.starts_with("https://duckduckgo.com/?q="));
        assert!(result.contains("rust"));
    }
}
