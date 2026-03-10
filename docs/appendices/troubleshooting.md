# Troubleshooting

## Hook Installation Fails

```bash
# Check if already installed
engram validate hook status

# Try manual install
cp .git/hooks/pre-commit.sample .git/hooks/pre-commit
```

## Data Not Persisting

Ensure you're in a Git repository. Engram stores data in `.git/refs/engram/`.

## Entity Not Found

Use `--json` flag with `jq` to get IDs:

```bash
engram task list --json | jq -r '.[0].id'
```

## Build Errors

```bash
# Clean and rebuild
cargo clean
cargo build --release
```
