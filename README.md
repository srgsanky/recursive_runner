# Recursive runner

Run a command recursively in sub-directories.


## Installation

```bash
cargo install --path .
```

## Usage

```bash
rr 'git status'

# Ignore errors
rr -i 'git status'

# Ignore errors and be quiet for directories that didn't produce any output
rr -i -q 'git status'
```

## Common uses

List all changes in the subdirectories

```bash
rr -q 'git -c color.status=always status -s'
```

Show diff of unpushed changes in the subdirectories

```bash
rr -q 'git log --color -p origin/mainline..HEAD'
# @{u} uses the current branch name
rr -q 'git log --color -p @{u}..HEAD'
```

Show one line log messages of unpushed changes

```bash
rr -q 'git log --oneline origin/mainline..HEAD'
# @{u} uses the current branch name
rr -q 'git log --oneline @{u}..HEAD'
```

Run Cargo commands on rust projects

```bash
# Check for existence of Cargo.toml before running the commands
rr -q '[ -f Cargo.toml ] && cargo fmt --all'
rr -q '[ -f Cargo.toml ] && cargo clippy --workspace --release --all-targets --all-features --fix --allow-dirty --allow-staged'
```

## Development

Setup git pre-push hook to prevent push a change that does not build.

```bash
ln -sf $(pwd)/scripts/git-hooks/pre-push .git/hooks/pre-push
```
