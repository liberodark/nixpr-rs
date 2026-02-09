use anyhow::{Context, Result};
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize)]
pub struct Config {
    #[serde(default)]
    pub github: GithubConfig,
    #[serde(default)]
    pub filters: FilterConfig,
    #[serde(default)]
    pub review: ReviewConfig,
}

#[derive(Deserialize, Default)]
pub struct GithubConfig {
    /// Optional GitHub token for higher API rate limits
    pub token: Option<String>,
}

#[derive(Deserialize)]
pub struct FilterConfig {
    /// Users whose PRs should be skipped (e.g. ["r-ryantm"])
    #[serde(default)]
    pub excluded_users: Vec<String>,
    /// Title prefixes to skip (e.g. ["nixos/", "treewide", "lib."])
    #[serde(default)]
    pub excluded_prefixes: Vec<String>,
}

impl Default for FilterConfig {
    fn default() -> Self {
        Self {
            excluded_users: vec!["r-ryantm".to_string()],
            excluded_prefixes: vec!["nixos/".to_string(), "treewide".to_string()],
        }
    }
}

#[derive(Deserialize)]
pub struct ReviewConfig {
    /// GitHub repo that runs nixpkgs-review (owner/name)
    #[serde(default = "default_repo")]
    pub repo: String,
    /// Workflow file name to trigger
    #[serde(default = "default_workflow")]
    pub workflow: String,
}

impl Default for ReviewConfig {
    fn default() -> Self {
        Self {
            repo: default_repo(),
            workflow: default_workflow(),
        }
    }
}

fn default_repo() -> String {
    "liberodark/nixpkgs-review-gha".to_string()
}

fn default_workflow() -> String {
    "review.yml".to_string()
}

impl Config {
    pub fn is_user_excluded(&self, user: &str) -> bool {
        self.filters
            .excluded_users
            .iter()
            .any(|u| u.eq_ignore_ascii_case(user))
    }

    pub fn is_title_excluded(&self, title: &str) -> bool {
        let lower = title.to_lowercase();
        self.filters
            .excluded_prefixes
            .iter()
            .any(|prefix| lower.starts_with(&prefix.to_lowercase()))
    }
}

fn config_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("~/.config"))
        .join("nixpr")
        .join("config.toml")
}

pub fn load() -> Result<Config> {
    let path = config_path();
    if !path.exists() {
        eprintln!("No config at {}, using defaults", path.display());
        return Ok(Config {
            github: GithubConfig::default(),
            filters: FilterConfig::default(),
            review: ReviewConfig::default(),
        });
    }

    let content = std::fs::read_to_string(&path)
        .with_context(|| format!("Failed to read config: {}", path.display()))?;

    toml::from_str(&content).with_context(|| format!("Failed to parse config: {}", path.display()))
}
