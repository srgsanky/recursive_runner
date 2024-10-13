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

## Development

Setup git pre-push hook to prevent push a change that does not build.

```bash
ln -s scripts/git-hooks/pre-push .git/hooks/pre-push
```
