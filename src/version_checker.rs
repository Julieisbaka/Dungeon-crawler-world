use reqwest::blocking::Client;
use semver::Version;
use serde::{Deserialize, Serialize};
use std::time::Duration;

const GITHUB_API_URL: &str = "https://api.github.com/repos/Julieisbaka/Dungeon-crawler-world/releases";
const REQUEST_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Debug, Deserialize, Serialize)]
pub struct GitHubRelease {
    pub tag_name: String,
    pub name: String,
    pub published_at: String,
    pub html_url: String,
    pub prerelease: bool,
    pub draft: bool,
    pub assets: Vec<GitHubAsset>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GitHubAsset {
    pub name: String,
    pub browser_download_url: String,
    pub size: u64,
}

pub struct VersionChecker {
    client: Client,
}

impl VersionChecker {
    pub fn new() -> Self {
        let client = Client::builder()
            .timeout(REQUEST_TIMEOUT)
            .user_agent("Dungeon-Crawler-World/0.0.7")
            .build()
            .unwrap_or_else(|_| Client::new());
        
        Self { client }
    }

    /// Check for the latest version on GitHub
    pub fn check_latest_version(&self) -> Result<Option<String>, String> {
        match self.fetch_releases() {
            Ok(releases) => {
                // Filter out prereleases and drafts, get the latest stable release
                let latest_release = releases
                    .into_iter()
                    .filter(|r| !r.prerelease && !r.draft)
                    .next();
                
                if let Some(release) = latest_release {
                    // Clean tag name (remove 'v' prefix if present)
                    let version = release.tag_name.trim_start_matches('v');
                    Ok(Some(version.to_string()))
                } else {
                    Ok(None)
                }
            }
            Err(e) => Err(e),
        }
    }

    /// Compare current version with latest version
    pub fn is_update_available(&self, current_version: &str, latest_version: &str) -> bool {
        match (Version::parse(current_version), Version::parse(latest_version)) {
            (Ok(current), Ok(latest)) => latest > current,
            _ => false, // If parsing fails, assume no update available
        }
    }

    /// Get download URL for the current platform
    pub fn get_download_url(&self, latest_version: &str) -> Result<Option<String>, String> {
        match self.fetch_releases() {
            Ok(releases) => {
                let target_release = releases
                    .into_iter()
                    .find(|r| r.tag_name.trim_start_matches('v') == latest_version);
                
                if let Some(release) = target_release {
                    // Determine the correct asset based on platform
                    let platform_suffix = if cfg!(target_os = "windows") {
                        if cfg!(feature = "dev-mode") {
                            "dev.exe"
                        } else {
                            "release.exe"
                        }
                    } else if cfg!(target_os = "macos") {
                        if cfg!(feature = "dev-mode") {
                            "dev-macos.dmg"
                        } else {
                            "release-macos.dmg"
                        }
                    } else {
                        // Linux - for now we don't have Linux releases, so return None
                        return Ok(None);
                    };

                    let asset = release.assets
                        .into_iter()
                        .find(|a| a.name.ends_with(platform_suffix));
                    
                    Ok(asset.map(|a| a.browser_download_url))
                } else {
                    Ok(None)
                }
            }
            Err(e) => Err(e),
        }
    }

    fn fetch_releases(&self) -> Result<Vec<GitHubRelease>, String> {
        let response = self
            .client
            .get(GITHUB_API_URL)
            .send()
            .map_err(|e| format!("Failed to fetch releases: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("GitHub API returned status: {}", response.status()));
        }

        let releases: Vec<GitHubRelease> = response
            .json()
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        Ok(releases)
    }
}

impl Default for VersionChecker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_comparison() {
        let checker = VersionChecker::new();
        
        // Test basic version comparison
        assert!(checker.is_update_available("0.0.7", "0.0.8"));
        assert!(checker.is_update_available("0.0.7", "0.1.0"));
        assert!(checker.is_update_available("0.0.7", "1.0.0"));
        
        // Test same version
        assert!(!checker.is_update_available("0.0.7", "0.0.7"));
        
        // Test older version
        assert!(!checker.is_update_available("0.0.8", "0.0.7"));
        
        // Test pre-release versions
        assert!(checker.is_update_available("0.0.7", "0.0.8-beta"));
        assert!(!checker.is_update_available("0.0.8-beta", "0.0.7"));
    }
    
    #[test]
    fn test_invalid_versions() {
        let checker = VersionChecker::new();
        
        // Invalid version strings should return false (no update)
        assert!(!checker.is_update_available("invalid", "0.0.8"));
        assert!(!checker.is_update_available("0.0.7", "invalid"));
        assert!(!checker.is_update_available("invalid", "invalid"));
    }
    
    #[test]
    fn test_github_api_call() {
        let checker = VersionChecker::new();
        
        // This is a real API call - may fail in CI without internet
        // But it's useful for manual testing
        match checker.check_latest_version() {
            Ok(Some(version)) => {
                println!("Latest version found: {}", version);
                assert!(!version.is_empty());
            }
            Ok(None) => {
                println!("No stable releases found");
            }
            Err(e) => {
                println!("API call failed (expected in CI): {}", e);
                // Don't fail the test if API is unavailable
            }
        }
    }
}