use anyhow::{Context, Result};
use serde::Deserialize;

use crate::config::Config;

const NIXPKGS_API: &str = "https://api.github.com/repos/NixOS/nixpkgs/pulls";

#[derive(Deserialize)]
pub struct PullRequest {
    pub number: u64,
    pub title: String,
    pub user: User,
}

#[derive(Deserialize)]
pub struct User {
    pub login: String,
}

/// Patterns that indicate a testable package PR:
///   "package: init at X.Y.Z"
///   "package: X.Y.Z -> A.B.C"
pub fn is_package_pr(title: &str) -> bool {
    let Some((_name, rest)) = title.split_once(':') else {
        return false;
    };
    let rest = rest.trim().to_lowercase();
    rest.contains("init at") || rest.contains("->")
}

/// Extract the package name from a PR title (part before the colon).
pub fn extract_package_name(title: &str) -> Option<&str> {
    let (name, _) = title.split_once(':')?;
    let name = name.trim();
    if name.is_empty() { None } else { Some(name) }
}

pub fn fetch_open_prs(config: &Config, per_page: u8, page: u32) -> Result<Vec<PullRequest>> {
    let client = reqwest::blocking::Client::new();
    let mut request = client
        .get(NIXPKGS_API)
        .query(&[
            ("state", "open"),
            ("sort", "created"),
            ("direction", "desc"),
            ("per_page", &per_page.to_string()),
            ("page", &page.to_string()),
        ])
        .header("User-Agent", "nixpr")
        .header("Accept", "application/vnd.github.v3+json");

    if let Some(token) = config.github_token() {
        request = request.header("Authorization", format!("Bearer {token}"));
    }

    let response = request.send().context("Failed to fetch PRs from GitHub")?;

    if !response.status().is_success() {
        anyhow::bail!(
            "GitHub API returned {}: {}",
            response.status(),
            response.text().unwrap_or_default()
        );
    }

    response.json().context("Failed to parse GitHub response")
}

pub fn filter_prs(config: &Config, prs: Vec<PullRequest>) -> Vec<PullRequest> {
    prs.into_iter()
        .filter(|pr| {
            if config.is_user_excluded(&pr.user.login) {
                return false;
            }
            if config.is_title_excluded(&pr.title) {
                return false;
            }
            is_package_pr(&pr.title)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_package_pr() {
        assert!(is_package_pr("fastfetch-rs: init at 0.1.6"));
        assert!(is_package_pr("rundeck: 5.18.0 -> 5.19.0"));
        assert!(is_package_pr("python312Packages.foo: 1.0 -> 2.0"));
        assert!(!is_package_pr("nixos/nginx: add option"));
        assert!(!is_package_pr("treewide: update something"));
        assert!(!is_package_pr("Fix typo in readme"));
    }

    #[test]
    fn test_extract_package_name() {
        assert_eq!(
            extract_package_name("rundeck: 5.18.0 -> 5.19.0"),
            Some("rundeck")
        );
        assert_eq!(
            extract_package_name("fastfetch-rs: init at 0.1.6"),
            Some("fastfetch-rs")
        );
        assert_eq!(extract_package_name("No colon here"), None);
    }
}
