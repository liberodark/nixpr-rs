mod cli;
mod config;
mod github;
mod runner;
mod state;

use anyhow::Result;
use std::collections::HashSet;

fn main() -> Result<()> {
    let args = cli::parse();
    let config = config::load()?;

    match args.command {
        cli::Command::Run {
            limit,
            dry_run,
            force,
        } => cmd_run(&config, limit, dry_run, force),
        cli::Command::Status { limit } => runner::show_status(&config, limit),
        cli::Command::Logs => runner::show_logs(&config),
        cli::Command::Web => runner::open_web(&config),
        cli::Command::Check { pr_number } => runner::check_pr(&config, pr_number),
        cli::Command::Reset => cmd_reset(),
    }
}

fn cmd_run(config: &config::Config, total_limit: u32, dry_run: bool, force: bool) -> Result<()> {
    let mut processed = state::load_processed()?;
    let mut all_prs = Vec::new();

    let per_page: u8 = 100;
    let pages = total_limit.div_ceil(u32::from(per_page));

    for page in 1..=pages {
        println!("Fetching PRs (page {page}/{pages})...");
        let prs = github::fetch_open_prs(config, per_page, page)?;
        let exhausted = prs.len() < usize::from(per_page);
        all_prs.extend(prs);
        if exhausted || all_prs.len() >= total_limit as usize {
            break;
        }
    }

    all_prs.truncate(total_limit as usize);

    println!("Fetched {} open PRs total", all_prs.len());

    let matching = github::filter_prs(config, all_prs);
    println!("{} PRs match filters", matching.len());

    let new_prs: Vec<_> = if force {
        matching
    } else {
        matching
            .into_iter()
            .filter(|pr| !processed.contains(&pr.number))
            .collect()
    };

    println!("{} new PRs to review\n", new_prs.len());

    if new_prs.is_empty() {
        println!("Nothing new to review.");
        return Ok(());
    }

    for pr in &new_prs {
        let pkg = github::extract_package_name(&pr.title).unwrap_or("unknown");
        println!(
            "  #{} [{}] {} (by @{})",
            pr.number, pkg, pr.title, pr.user.login
        );
    }
    println!();

    if dry_run {
        println!("Dry run, no reviews triggered.");
        return Ok(());
    }

    let mut triggered = Vec::new();
    for pr in &new_prs {
        print!("Triggering review for PR #{}... ", pr.number);
        match runner::trigger_review(config, pr.number) {
            Ok(()) => {
                println!("done");
                state::mark_processed(&mut processed, pr.number);
                triggered.push(pr.number);
            }
            Err(e) => println!("FAILED: {e}"),
        }
    }

    if !triggered.is_empty() {
        state::save_processed(&processed)?;
        println!("\nMarked {} PRs as processed", triggered.len());
    }

    println!(
        "Follow progress: https://github.com/{}/actions",
        config.review.repo
    );
    Ok(())
}

fn cmd_reset() -> Result<()> {
    let processed = HashSet::new();
    state::save_processed(&processed)?;
    println!("Processed PR list cleared.");
    Ok(())
}
