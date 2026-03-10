# Setup & Configuration

## Building

```bash
# Clone and build
cargo build --release

# Run tests
cargo test

# Run with debug
cargo run -- --help
```

## Configuration

Engram stores data in `.git/refs/engram/`. No additional config required.

## Git Integration

Install the validation hook to enforce task linking:

```bash
engram validate hook install
```

This prevents commits without task IDs.
