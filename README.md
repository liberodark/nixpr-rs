# nixpr

Automatically discover and review [NixOS/nixpkgs](https://github.com/NixOS/nixpkgs) pull requests via GitHub Actions.

`nixpr` fetches open PRs from nixpkgs, filters out bots and non-package changes, and triggers [nixpkgs-review](https://github.com/Mic92/nixpkgs-review) workflows for each matching PR.

## Requirements

- [gh](https://cli.github.com/) (GitHub CLI, authenticated)
- A fork of [nixpkgs-review-gha](https://github.com/liberodark/nixpkgs-review-gha) with the `review.yml` workflow

## Installation

```bash
cargo install --path .
```

## Configuration

Copy the example config and edit it:

```bash
mkdir -p ~/.config/nixpr
cp config.example.toml ~/.config/nixpr/config.toml
```

### Key settings

| Section | Field | Description |
| --- | --- | --- |
| `github.token` | Optional GitHub PAT for higher rate limits |
| `filters.excluded_users` | Bot usernames to skip (e.g. `r-ryantm`) |
| `filters.excluded_prefixes` | PR title prefixes to skip (e.g. `nixos/`, `treewide`) |
| `review.repo` | Your nixpkgs-review-gha repo |
| `review.workflow` | Workflow file name |

## Usage

### Discover and trigger reviews

```bash
# Dry run — see what would be reviewed
nixpr run --dry-run

# Trigger reviews for all matching PRs (first 100)
nixpr run

# Fetch more PRs (3 pages × 100)
nixpr run --limit 100 --pages 3
```

### Monitor reviews

```bash
# List recent workflow runs
nixpr status

# Check a specific PR
nixpr check 488583

# View logs of the latest run
nixpr logs

# Open the Actions page in your browser
nixpr web
```

## How filtering works

A PR is reviewed only if **all** of the following are true:

1. The author is **not** in `excluded_users`
2. The title does **not** start with any `excluded_prefixes`
3. The title matches a **package PR pattern**:
   - `package-name: init at X.Y.Z` (new package)
   - `package-name: X.Y.Z -> A.B.C` (version update)

Everything else (NixOS module changes, treewide refactors, documentation, etc.) is skipped.

## License

MIT
