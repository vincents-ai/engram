---
name: engram-sync
description: "Sync engram state between agents locally or with remote git repositories. Use for multi-agent coordination, branch-based isolation, and remote backup/sharing of engram knowledge."
---

# Engram Sync

## Overview

Engram sync moves engram state between agents and repositories. It supports two modes:

- **Local sync** — merge engram state between two agents on the same machine
- **Remote sync** — push/pull engram state to/from a git remote

Branches provide isolation: each agent works on its own branch and merges via sync when ready.

**Rule:** Always `--dry-run` before any real push or pull.

## When to Use

Use this skill when:
- Two agents on the same machine need to share engram state
- You want to back up engram state to a remote git repository
- You are coordinating a multi-agent pipeline with branch-per-agent isolation
- You need to pull engram state from a remote before starting work

## Command Reference

```bash
# --- Local sync ---
# Sync engram state between local agents; optional: --strategy, --dry-run
engram sync sync --agents <AGENTS>

# --- Remote management ---
# Add a remote; optional: --branch, --auth-type, --username, --password, --ssh-key
engram sync add-remote <NAME> <URL>

# List configured remotes
engram sync list-remotes

# Check sync status; optional: --remote
engram sync status

# --- Push / Pull ---
# Pull engram state from remote; required: --remote; optional: --branch, --agents, auth options, --dry-run
engram sync pull --remote <REMOTE>

# Push engram state to remote; required: --remote; optional: --branch, --agents, auth options, --dry-run
engram sync push --remote <REMOTE>

# --- Branch management ---
engram sync create-branch <NAME>
engram sync switch-branch <NAME>
engram sync list-branches
engram sync delete-branch <NAME>
```

## The Pattern

### Local Multi-Agent Sync

When two agents on the same machine need to share state:

```bash
# Dry run first — see what would be merged
engram sync sync --agents agent-a,agent-b --dry-run

# Execute the sync
engram sync sync --agents agent-a,agent-b
```

### Remote Sync

#### 1. Add a Remote

```bash
engram sync add-remote origin https://github.com/org/engram-store.git

# With SSH key auth
engram sync add-remote origin git@github.com:org/engram-store.git \
  --auth-type ssh \
  --ssh-key ~/.ssh/id_ed25519

# List remotes to confirm
engram sync list-remotes
```

#### 2. Check Status Before Acting

```bash
engram sync status
# or check against a specific remote
engram sync status --remote origin
```

#### 3. Dry Run, Then Push

```bash
# Always dry-run first
engram sync push --remote origin --dry-run

# Execute push when satisfied
engram sync push --remote origin
```

#### 4. Pull from Remote

```bash
# Dry run first
engram sync pull --remote origin --dry-run

# Execute pull
engram sync pull --remote origin
```

### Branch-Based Agent Isolation

Each agent works on its own branch to avoid state conflicts, then merges via sync.

```bash
# Agent A: create and switch to dedicated branch
engram sync create-branch agent-a-feature-xyz
engram sync switch-branch agent-a-feature-xyz

# Agent A: do all work on this branch, store context, tasks, reasoning...

# Agent A: push branch to remote when done
engram sync push --remote origin --branch agent-a-feature-xyz --dry-run
engram sync push --remote origin --branch agent-a-feature-xyz

# Agent B: list branches, pull agent A's branch
engram sync list-branches
engram sync pull --remote origin --branch agent-a-feature-xyz --dry-run
engram sync pull --remote origin --branch agent-a-feature-xyz

# Cleanup: delete branch when merged
engram sync delete-branch agent-a-feature-xyz
```

## Session Boundary Pattern

The most important use of sync is at session boundaries. Every agent session should begin with a pull and end with a push.

### Session Start (pull)

```bash
# 1. Check remotes
engram sync list-remotes

# 2. If remotes configured: pull before doing anything
engram sync pull --remote origin --dry-run
engram sync pull --remote origin

# 3. Then open your session
engram session start --name "<agent>-<goal>"
```

### Session End (push)

```bash
# 1. Generate session summary first
engram session end --id <SESSION_ID> --generate-summary

# 2. Then push — summary is now included in the push
engram sync push --remote origin --dry-run
engram sync push --remote origin
```

**Why this order matters:** generating the summary before pushing ensures the next agent who pulls gets the full handoff context, not just the raw entity data.

See `engram-session-start` and `engram-session-end` for the full protocol.

## Example: Multi-Agent Pipeline with Remote Backup

```
[Setup]
engram sync add-remote origin git@github.com:org/project-engram.git
engram sync list-branches
# main branch is default

[Agent: researcher]
engram sync create-branch researcher-findings
engram sync switch-branch researcher-findings

# ... do research, store context and reasoning in engram ...

engram sync push --remote origin --branch researcher-findings --dry-run
engram sync push --remote origin --branch researcher-findings
# Notify orchestrator: "research complete on branch researcher-findings"

[Agent: implementer]
engram sync pull --remote origin --branch researcher-findings --dry-run
engram sync pull --remote origin --branch researcher-findings
# Now has researcher's context, can query it via engram ask query

engram sync create-branch implementer-work
engram sync switch-branch implementer-work

# ... implement based on research findings ...

engram sync push --remote origin --branch implementer-work --dry-run
engram sync push --remote origin --branch implementer-work
```

## Key Principles

1. **Dry run first** — always `--dry-run` before any real push or pull.
2. **Branch per agent** — use `create-branch` to isolate agent state.
3. **Check status before acting** — `engram sync status` reveals drift between local and remote.
4. **List before creating** — `engram sync list-branches` prevents duplicate branch names.
5. **Push after storing findings** — remote sync is the backup and handoff mechanism.

## Related Skills

- `engram-sessions` — sessions group work; sync moves it between agents
- `engram-orchestrator` — orchestration loop that uses sync for multi-agent coordination
- `engram-dispatching-parallel-agents` — branch-per-agent pattern for parallel work
- `engram-use-engram-memory` — what to store before syncing
