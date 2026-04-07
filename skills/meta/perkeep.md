---
name: engram-perkeep
description: "Configure and use Perkeep as a durable off-site backup for engram data. Use before major refactors, workspace wipes, or on a regular backup schedule to preserve the full engram knowledge graph."
---

# Perkeep Integration

## Overview

Engram normally stores all data in git refs within the local repository. This is fast and portable, but local-only. **Perkeep** (formerly Camlistore) is a content-addressed permanent storage system that provides durable, off-site backup for your engram data.

With Perkeep configured:
- Your tasks, contexts, reasoning, ADRs, and relationships are backed up to a remote server
- Data is content-addressed — every blob is identified by its hash, making it immutable and verifiable
- You can restore the full engram knowledge graph to a new workspace

## When to Use

Use this skill when:
- Setting up engram in a new project and you want off-site durability
- Before a major refactor that might wipe local git history
- Before deleting or archiving a workspace
- On a regular schedule (e.g., end of sprint, before deployments)
- Restoring engram data to a fresh workspace after a failure

## Command Reference

```bash
# Configure the Perkeep server connection
# --save persists the configuration
engram perkeep config \
  --server "<https://your-perkeep-server:3179>" \
  --auth-token "<your-auth-token>" \
  --save

# Check that the Perkeep server is reachable and healthy
engram perkeep health

# Back up engram data to Perkeep
# --entity-type: backup all entities (omit) or a specific type (task|context|reasoning|adr)
# --include-relationships: preserves the full graph, not just nodes
# --description: human-readable label for this backup snapshot
engram perkeep backup \
  --include-relationships \
  --description "<what this backup is and why>"

# Backup only a specific entity type
engram perkeep backup \
  --entity-type task \
  --include-relationships \
  --description "Task-only backup before sprint close"

# List available backups (shows blobrefs and descriptions)
engram perkeep list

# Restore from a backup
# --blobref: the content address returned by perkeep list
# --dry-run: simulate the restore without writing — always run this first
engram perkeep restore \
  --blobref "<sha224-...>" \
  --agent "<your-name>" \
  --dry-run

# Restore for real after verifying dry-run output
engram perkeep restore \
  --blobref "<sha224-...>" \
  --agent "<your-name>"
```

## What Perkeep Is

Perkeep is a content-addressed storage system. Every piece of data is identified by a **blobref** — a cryptographic hash of its content (e.g., `sha224-abc123...`). This means:

- Data cannot be silently corrupted — any change produces a different blobref
- The same content stored twice has the same blobref (deduplication)
- You can verify the integrity of any restore by checking the blobref

Engram uses Perkeep as an append-only archive. Backups do not overwrite previous backups — each backup creates new blobs. Your full history is preserved.

## Setup

### 1. Configure the Server

```bash
engram perkeep config \
  --server "https://perkeep.yourorg.com:3179" \
  --auth-token "your-secret-token-here" \
  --save
```

### 2. Verify Health

```bash
engram perkeep health
# Expected: server is reachable and responding
```

If health check fails:
- Confirm the server URL includes the correct port
- Verify the auth token matches the server configuration
- Check network connectivity to the Perkeep host

### 3. Run a Test Backup

```bash
engram perkeep backup \
  --include-relationships \
  --description "Initial backup — setup verification"
```

### 4. Verify the Backup Appears

```bash
engram perkeep list
# Should show your new backup with its blobref and description
```

## Backup Strategy

### Always Use `--include-relationships`

Engram's value is in the **graph** — the relationships between tasks, contexts, reasoning, and ADRs. Backing up nodes without edges loses the traceability network. Always pass `--include-relationships` unless you have a specific reason not to.

### Write Descriptive Backup Labels

The `--description` flag is your future self's navigation. Be specific:

```bash
# Good — identifies the point in time and the reason
--description "Before Q2 refactor: all tasks and ADRs as of sprint 12 close"

# Bad — gives no context when you need to restore 6 months later
--description "backup"
```

### When to Back Up

| Trigger | Reason |
|---|---|
| Before major refactor | Preserve the reasoning graph before git history changes |
| Before workspace wipe | Last chance to preserve local engram state |
| Sprint close / milestone | Snapshot of completed work |
| Before production deployment | Stable state of all decisions made |
| Weekly scheduled backup | Routine durability |

## Restore

### Always Dry-Run First

```bash
# Step 1: list available backups and note the blobref
engram perkeep list

# Step 2: dry-run to see what will be restored
engram perkeep restore \
  --blobref "sha224-abc123def456..." \
  --agent "restore-agent" \
  --dry-run

# Step 3: review the dry-run output — verify entity counts and types
# Step 4: restore for real
engram perkeep restore \
  --blobref "sha224-abc123def456..." \
  --agent "restore-agent"
```

The dry-run shows what entities and relationships would be written without actually writing them. Review it to confirm you are restoring the right snapshot before committing.

## The Full Pattern

```bash
# 1. Configure (once per project)
engram perkeep config --server "<url>" --auth-token "<token>" --save

# 2. Verify
engram perkeep health

# 3. Back up (with relationships and a clear description)
engram perkeep backup --include-relationships --description "before <event>"

# 4. Confirm backup is stored
engram perkeep list

# --- Later, on a new or wiped workspace ---

# 5. Configure again
engram perkeep config --server "<url>" --auth-token "<token>" --save

# 6. List to find the right snapshot
engram perkeep list

# 7. Dry-run the restore
engram perkeep restore --blobref "<sha224-...>" --agent "restore-agent" --dry-run

# 8. Restore for real
engram perkeep restore --blobref "<sha224-...>" --agent "restore-agent"
```

## Key Rules

1. **Always `--include-relationships`** when backing up — nodes without edges lose the graph structure
2. **Write meaningful `--description` values** — you will need them when choosing which backup to restore
3. **Always dry-run before restore** — review the entity count before writing to a live workspace
4. **Run `health` before `backup`** — a failed backup with no error is worse than a visible failure
5. **Perkeep is append-only** — backups accumulate; use `perkeep list` to find the right snapshot by description

## Related Skills

- `engram-use-engram-memory` — how engram stores data locally
- `engram-audit-trail` — full traceability before backing up
- `engram-orchestrator` — orchestration loop for scheduled backup tasks
- `engram-runbooks` — operational procedures including backup schedules
