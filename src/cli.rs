use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "nixpr",
    about = "Automatically review NixOS/nixpkgs pull requests"
)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Fetch open PRs, filter them, and trigger reviews
    Run {
        /// Total number of PRs to fetch (pagination is automatic)
        #[arg(short, long, default_value_t = 100)]
        limit: u32,

        /// Dry run: show matching PRs without triggering reviews
        #[arg(short, long)]
        dry_run: bool,

        /// Force re-review of already processed PRs
        #[arg(short, long)]
        force: bool,
    },
    /// Show recent workflow runs
    Status {
        /// Number of runs to display
        #[arg(short, long, default_value_t = 10)]
        limit: u8,
    },
    /// Show logs of the latest workflow run
    Logs,
    /// Open the GitHub Actions page in a browser
    Web,
    /// Check runs for a specific PR
    Check {
        /// PR number to check
        pr_number: u64,
    },
    /// Clear the list of already processed PRs
    Reset,
}

pub fn parse() -> Args {
    Args::parse()
}
