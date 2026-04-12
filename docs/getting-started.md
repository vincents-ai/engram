# Getting Started with Engram

**Stop re-explaining your codebase to every new AI session.**

Engram gives your AI agents persistent memory — a knowledge graph that lives in your git repo. Plans, decisions, and context survive across sessions. A new agent queries engram and picks up where the last one left off.

Git tracks *what* changed. Engram tracks *why*.

## Install

```bash
# Linux / macOS (one line)
curl -fsSL https://github.com/vincents-ai/engram/releases/latest/download/install.sh | bash

# Or: cargo install engram
# Or: nix run github:vincents-ai/engram -- --help

engram --version   # verify
```

See the [README](../README.md) for all platforms including Windows and manual installs.

## Set Up (2 minutes)

```bash
cd your-project

# 1. Initialize engram in your repo
engram setup workspace

# 2. Create your profile
engram setup agent --name "Your Name" --agent-type operator

# 3. Install the commit hook (requires task UUID in every commit)
engram validate hook install
```

Done. Now use it.

## The Workflow: Plan → Execute → Remember

### Plan

```bash
engram task create --title "Add user authentication" --priority high
```

### Execute & Document

```bash
# Save reference material
engram context create --title "OAuth2 Spec" --source "https://oauth.net/2/"

# Link it to the task
engram relationship create \
  --source-id <TASK_ID> --source-type task \
  --target-id <CONTEXT_ID> --target-type context \
  --relationship-type references

# Record a decision
engram reasoning create \
  --title "Chose JWT over sessions" \
  --task-id <TASK_ID> \
  --content "Stateless, scales horizontally, no server-side session store."
```

### Remember

```bash
# Ask a question across everything you've stored
engram ask query "why did we choose JWT for auth?"

# What should I work on next?
engram next
```

## For AI Agents

If you use Claude Code, OpenCode, Goose, or similar tools:

```bash
# Install core skills (14 — teaches your agent the engram loop)
engram skills setup

# Install all skills (44 — planning, architecture, review, debugging, TDD, compliance)
engram setup skills
```

Your agent now knows to search engram before acting, store decisions after making them, and link everything. When a new session starts, it runs:

```
engram ask query "full-fidelity handoff"
```

…and gets the full project state.

## Next Steps

- [User Guide](user-guide.md) — Complete walkthrough for human operators
- [Using Engram (for Agents)](skills/using-engram.md) — Agent integration details
- [CLI Reference](reference/cli.md) — All commands and flags
- [Theory Building](features/theory-building.md) — Capture mental models per Naur (1985)
