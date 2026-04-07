---
name: engram-write-before-responding
description: "Before replying to the user, always write findings, decisions, and completed work to engram first so nothing is lost to context compaction or session end."
---

# Write Before Responding

## Overview

Agents lose work when they reply to the user before storing findings in engram. Context compaction and session end are silent — they do not warn you. If you reply first, then write, the write may never happen.

The rule is simple: **engram first, reply second. Always.**

## When to Use

This rule fires on every response where you have something new to record:

1. After any research or investigation — you found something
2. After any decision — you chose something
3. After completing a unit of work — a task or subtask is done

## The Rule

Before sending ANY response to the user:

1. Store new findings → `engram context create`
2. Store new decisions → `engram reasoning create` or `engram adr create`
3. Link new records → `engram relationship create`
4. Update task status if a unit of work is complete → `engram task update`

**Then** reply to the user.

## Pattern: Research Response

You investigated something. Before replying:

```bash
# Store the finding
engram context create \
  --title "<what you found — be specific>" \
  --content "<full finding with details>" \
  --source "<file, URL, or command that produced this>"
# Returns: CTX_UUID

# Link it to the current task
engram relationship create \
  --source-id <TASK_UUID> --source-type task \
  --target-id <CTX_UUID> --target-type context \
  --relationship-type relates_to --agent "<your-name>"
```

Now reply to the user.

## Pattern: Decision Response

You made a choice. Before replying:

```bash
# For architectural or lasting decisions
engram adr create \
  --title "<decision title>" \
  --number <N> \
  --context "<situation, options considered, and what was decided and why>" \
  --agent "<your-name>"
# Returns: ADR_UUID

# Or for reasoning chains and logic
engram reasoning create \
  --title "<reasoning title>" \
  --task-id <TASK_UUID> \
  --content "<your logic, evidence, and conclusion>"
# Returns: RSN_UUID

# Link to task
engram relationship create \
  --source-id <TASK_UUID> --source-type task \
  --target-id <ADR_UUID> --target-type adr \
  --relationship-type explains --agent "<your-name>"
```

Now reply to the user.

## Pattern: Work Completion Response

A unit of work is done. Before replying:

```bash
# Update the task
engram task update <TASK_UUID> --status done --outcome "<one-line summary of what was achieved>"

# If you produced an artifact (file, context, reasoning), link it
engram relationship create \
  --source-id <TASK_UUID> --source-type task \
  --target-id <ARTIFACT_UUID> --target-type context \
  --relationship-type relates_to --agent "<your-name>"
```

Now reply to the user.

## Anti-Patterns (what NOT to do)

**Reply first, store later** — "Here is what I found. [stores context]"
This is wrong. The store must precede the reply.

**Store without linking** — creating context or reasoning but omitting `engram relationship create`
Unlinked records cannot be found by graph traversal. They are effectively lost.

**Omit the store entirely** — summarising findings verbally without writing them to engram
This is the root failure this skill exists to prevent. Verbal summaries vanish on compaction.

**Use wrong command syntax:**
- `engram ask "<text>"` — wrong; correct is `engram ask query "<text>"`
- `engram task create --status in_progress` — wrong; use `engram task update` for status
- `engram adr create --decision "<text>"` — wrong; there is no `--decision` flag; put the decision in `--context`
- `engram reasoning create` without `--task-id` — wrong; `--task-id` is required

## Related Skills

- `engram-use-engram-memory` — full command reference for storing context, reasoning, and ADRs
- `engram-orchestrator` — orchestrator execution loop (embeds this rule as a top-level step)
- `engram-subagent-register` — subagent protocol (stores findings before reporting back)
- `engram-audit-trail` — full traceability patterns
