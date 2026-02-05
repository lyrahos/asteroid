//! Auto-update mechanism for Asteroid Browser.
//!
//! Checks GitHub releases for new versions and notifies the user.
//! Does not auto-install; requires user confirmation.

use serde::Deserialize;

/// Represents a GitHub release.
#[derive(Debug, Clone, Deserialize)]
pub struct GitHubRelease {
    pub tag_name: String,
    pub name: Option<String>,
    pub body: Option<String>,
    pub html_url: String,
    pub assets: Vec<Asset>,
    pub prerelease: bool,
    pub draft: bool,
}

/// Represents a release asset (downloadable file).
#[derive(Debug, Clone, Deserialize)]
pub struct Asset {
    pub name: String,
    pub browser_download_url: String,
    pub size: u64,
    pub content_type: String,
}

/// Information about an available update.
#[derive(Debug, Clone)]
pub struct UpdateInfo {
    pub version: String,
    pub release_url: String,
    pub release_notes: Option<String>,
    pub download_url: Option<String>,
    pub package_size: Option<u64>,
}

/// Update checker that polls GitHub releases.
pub struct UpdateChecker {
    current_version: String,
    repo: String,
    check_prerelease: bool,
}

impl UpdateChecker {
    /// Create a new update checker.
    ///
    /// `current_version` should be the semantic version string (e.g., "1.0.0").
    /// `repo` should be in "owner/repo" format.
    pub fn new(current_version: &str, repo: &str) -> Self {
        Self {
            current_version: current_version.to_string(),
            repo: repo.to_string(),
            check_prerelease: false,
        }
    }

    /// Create an update checker with the default repository.
    pub fn with_defaults() -> Self {
        Self::new(
            env!("CARGO_PKG_VERSION"),
            "asteroid-browser/asteroid-browser",
        )
    }

    /// Enable or disable pre-release checking.
    pub fn set_check_prerelease(&mut self, check: bool) {
        self.check_prerelease = check;
    }

    /// Check for available updates by querying the GitHub releases API.
    pub async fn check_for_updates(&self) -> Result<Option<UpdateInfo>, Box<dyn std::error::Error + Send + Sync>> {
        let url = format!(
            "https://api.github.com/repos/{}/releases/latest",
            self.repo
        );

        let client = reqwest::Client::builder()
            .user_agent("asteroid-browser")
            .timeout(std::time::Duration::from_secs(15))
            .build()?;

        let response = client.get(&url).send().await?;

        if !response.status().is_success() {
            return Ok(None);
        }

        let release: GitHubRelease = response.json().await?;

        // Skip drafts and pre-releases (unless configured)
        if release.draft || (release.prerelease && !self.check_prerelease) {
            return Ok(None);
        }

        // Compare versions (strip leading 'v' if present)
        let remote_version = release.tag_name.trim_start_matches('v');

        if self.is_newer(remote_version) {
            let download_url = self.find_compatible_asset(&release.assets);
            let package_size = download_url
                .as_ref()
                .and_then(|url| {
                    release
                        .assets
                        .iter()
                        .find(|a| &a.browser_download_url == url)
                        .map(|a| a.size)
                });

            Ok(Some(UpdateInfo {
                version: remote_version.to_string(),
                release_url: release.html_url,
                release_notes: release.body,
                download_url,
                package_size,
            }))
        } else {
            Ok(None)
        }
    }

    /// Get the download URL for a specific version.
    pub async fn get_download_url(
        &self,
        version: &str,
    ) -> Result<Option<String>, Box<dyn std::error::Error + Send + Sync>> {
        let tag = if version.starts_with('v') {
            version.to_string()
        } else {
            format!("v{}", version)
        };

        let url = format!(
            "https://api.github.com/repos/{}/releases/tags/{}",
            self.repo, tag
        );

        let client = reqwest::Client::builder()
            .user_agent("asteroid-browser")
            .timeout(std::time::Duration::from_secs(15))
            .build()?;

        let response = client.get(&url).send().await?;

        if !response.status().is_success() {
            return Ok(None);
        }

        let release: GitHubRelease = response.json().await?;
        Ok(self.find_compatible_asset(&release.assets))
    }

    /// Compare a remote version against the current version.
    fn is_newer(&self, remote: &str) -> bool {
        use semver::Version;

        let current = Version::parse(&self.current_version).ok();
        let remote_ver = Version::parse(remote).ok();

        match (current, remote_ver) {
            (Some(c), Some(r)) => r > c,
            (None, Some(_)) => true,
            _ => false,
        }
    }

    /// Find a compatible package asset for this system.
    fn find_compatible_asset(&self, assets: &[Asset]) -> Option<String> {
        // Detect the package manager
        let preferred_extensions = if std::path::Path::new("/usr/bin/dpkg").exists() {
            vec![".deb"]
        } else if std::path::Path::new("/usr/bin/rpm").exists() {
            vec![".rpm"]
        } else {
            vec![".flatpak", ".AppImage", ".tar.gz"]
        };

        for ext in &preferred_extensions {
            if let Some(asset) = assets.iter().find(|a| a.name.ends_with(ext)) {
                return Some(asset.browser_download_url.clone());
            }
        }

        None
    }
}

/// Start the background update checker (runs every 24 hours).
pub fn start_update_checker(
    update_tx: tokio::sync::mpsc::Sender<UpdateInfo>,
) {
    tokio::spawn(async move {
        let checker = UpdateChecker::with_defaults();

        loop {
            match checker.check_for_updates().await {
                Ok(Some(info)) => {
                    log::info!("Update available: v{}", info.version);
                    if let Err(e) = update_tx.send(info).await {
                        log::error!("Failed to send update notification: {}", e);
                    }
                }
                Ok(None) => {
                    log::debug!("No updates available");
                }
                Err(e) => {
                    log::warn!("Update check failed: {}", e);
                }
            }

            // Check every 24 hours
            tokio::time::sleep(tokio::time::Duration::from_secs(86400)).await;
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_checker_creation() {
        let checker = UpdateChecker::new("1.0.0", "test/repo");
        assert_eq!(checker.current_version, "1.0.0");
        assert_eq!(checker.repo, "test/repo");
        assert!(!checker.check_prerelease);
    }

    #[test]
    fn test_version_comparison() {
        let checker = UpdateChecker::new("1.0.0", "test/repo");
        assert!(checker.is_newer("1.1.0"));
        assert!(checker.is_newer("2.0.0"));
        assert!(!checker.is_newer("1.0.0"));
        assert!(!checker.is_newer("0.9.0"));
    }

    #[test]
    fn test_find_compatible_asset() {
        let checker = UpdateChecker::new("1.0.0", "test/repo");
        let assets = vec![
            Asset {
                name: "asteroid-browser-1.1.0.deb".to_string(),
                browser_download_url: "https://example.com/test.deb".to_string(),
                size: 1024,
                content_type: "application/octet-stream".to_string(),
            },
            Asset {
                name: "asteroid-browser-1.1.0.rpm".to_string(),
                browser_download_url: "https://example.com/test.rpm".to_string(),
                size: 1024,
                content_type: "application/octet-stream".to_string(),
            },
        ];

        let result = checker.find_compatible_asset(&assets);
        assert!(result.is_some());
    }
}
