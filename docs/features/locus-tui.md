# Locus TUI

Locus is Engram's terminal UI for browsing and navigating project memory without leaving the terminal.

## What It Shows

Locus provides views over common engram entities, including:

- Tasks
- Context
- Reasoning
- ADRs
- Knowledge
- Relationships
- Sessions
- Theories
- Sync status

## Usage

```bash
# Start the TUI
locus

# Or, when installed as an engram binary target, run the locus executable from the build output
cargo run --bin locus
```

## Navigation

Use the keyboard-driven interface to switch views, inspect details, and refresh current state. Locus reads the same git-backed engram storage as the CLI, so CLI changes and TUI views stay aligned.
