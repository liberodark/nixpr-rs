use anyhow::{Context, Result};
use serde::Deserialize;
use std::process::Command;

use crate::config::Config;

#[derive(Deserialize)]
struct WorkflowRun {
    #[serde(rename = "displayTitle")]
    display_title: String,
    status: String,
    conclusion: Option<String>,
    #[serde(rename = "createdAt")]
    created_at: String,
}

pub fn trigger_review(config: &Config, pr_number: u64) -> Result<()> {
    let status = Command::new("gh")
        .args([
            "workflow",
            "run",
            &config.review.workflow,
            "--repo",
            &config.review.repo,
            "--field",
            &format!("pr={pr_number}"),
        ])
        .status()
        .context("Failed to run `gh` CLI. Is it installed?")?;

    if !status.success() {
        anyhow::bail!("gh workflow run failed for PR #{pr_number}");
    }
    Ok(())
}

pub fn show_status(config: &Config, limit: u8) -> Result<()> {
    let status = Command::new("gh")
        .args([
            "run",
            "list",
            "--repo",
            &config.review.repo,
            "--limit",
            &limit.to_string(),
        ])
        .status()
        .context("Failed to run `gh run list`")?;

    if !status.success() {
        anyhow::bail!("gh run list failed");
    }
    Ok(())
}

pub fn show_logs(config: &Config) -> Result<()> {
    let output = Command::new("gh")
        .args([
            "run",
            "list",
            "--repo",
            &config.review.repo,
            "--limit",
            "1",
            "--json",
            "databaseId",
            "--jq",
            ".[0].databaseId",
        ])
        .output()
        .context("Failed to get latest run ID")?;

    let run_id = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if run_id.is_empty() {
        anyhow::bail!("No workflow runs found");
    }

    println!("Logs for run #{run_id}:");
    let status = Command::new("gh")
        .args([
            "run",
            "view",
            &run_id,
            "--repo",
            &config.review.repo,
            "--log",
        ])
        .status()
        .context("Failed to view run logs")?;

    if !status.success() {
        anyhow::bail!("gh run view failed");
    }
    Ok(())
}

pub fn open_web(config: &Config) -> Result<()> {
    let url = format!("https://github.com/{}/actions", config.review.repo);
    let opener = if cfg!(target_os = "macos") {
        "open"
    } else {
        "xdg-open"
    };

    Command::new(opener)
        .arg(&url)
        .status()
        .with_context(|| format!("Failed to open {url}"))?;
    Ok(())
}

pub fn check_pr(config: &Config, pr_number: u64) -> Result<()> {
    println!("Searching runs for PR #{pr_number}...");

    let output = Command::new("gh")
        .args([
            "run",
            "list",
            "--repo",
            &config.review.repo,
            "--json",
            "displayTitle,status,conclusion,createdAt",
        ])
        .output()
        .context("Failed to check PR runs")?;

    if !output.status.success() {
        anyhow::bail!("gh run list failed for PR #{pr_number}");
    }

    let runs: Vec<WorkflowRun> =
        serde_json::from_slice(&output.stdout).context("Failed to parse workflow runs")?;

    let needle = format!("#{pr_number}");
    let mut found = false;
    for run in &runs {
        if run.display_title.contains(&needle) {
            found = true;
            let conclusion = run.conclusion.as_deref().unwrap_or("running");
            println!("{} - {} - {}", run.created_at, run.status, conclusion);
        }
    }

    if !found {
        println!("No runs found for PR #{pr_number}");
    }

    Ok(())
}
