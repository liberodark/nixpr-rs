# nixpr

Automatically discover and review [NixOS/nixpkgs](https://github.com/NixOS/nixpkgs) pull requests via GitHub Actions.

Fetches open PRs, filters out bots and non-package changes, and triggers [nixpkgs-review](https://github.com/Mic92/nixpkgs-review) workflows for each matching PR.

## Requirements

- [gh](https://cli.github.com/) (authenticated)
- A [nixpkgs-review-gha](https://github.com/liberodark/nixpkgs-review-gha) fork with `review.yml`

## Install

```bash
cargo install --path .
```

## Configuration

```bash
mkdir -p ~/.config/nixpr
cp config.example.toml ~/.config/nixpr/config.toml
```

## Usage

```bash
nixpr run --dry-run          # preview matching PRs
nixpr run                    # trigger reviews
nixpr run --limit 1000       # fetch more PRs
nixpr run --force            # re-review already processed PRs
nixpr status                 # list recent workflow runs
nixpr check 488583           # check a specific PR
nixpr logs                   # latest run logs
nixpr web                    # open Actions in browser
nixpr reset                  # clear processed PR list
```

## Filtering

A PR is reviewed only if:

- Author is not in `excluded_users`
- Title does not start with any `excluded_prefixes`
- Title matches `package: init at X.Y.Z` or `package: X.Y.Z -> A.B.C`
