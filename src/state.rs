use anyhow::{Context, Result};
use std::collections::HashSet;
use std::path::PathBuf;

fn state_path() -> PathBuf {
    dirs::data_dir()
        .unwrap_or_else(|| PathBuf::from("~/.local/share"))
        .join("nixpr")
        .join("processed.json")
}

pub fn load_processed() -> Result<HashSet<u64>> {
    let path = state_path();
    if !path.exists() {
        return Ok(HashSet::new());
    }

    let content = std::fs::read_to_string(&path)
        .with_context(|| format!("Failed to read state: {}", path.display()))?;

    serde_json::from_str(&content)
        .with_context(|| format!("Failed to parse state: {}", path.display()))
}

pub fn save_processed(processed: &HashSet<u64>) -> Result<()> {
    let path = state_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create state directory: {}", parent.display()))?;
    }

    let content = serde_json::to_string_pretty(processed).context("Failed to serialize state")?;
    std::fs::write(&path, content)
        .with_context(|| format!("Failed to write state: {}", path.display()))
}

pub fn mark_processed(processed: &mut HashSet<u64>, pr_number: u64) {
    processed.insert(pr_number);
}
