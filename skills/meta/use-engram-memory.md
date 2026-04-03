---
name: engram-use-engram-memory
description: "Use engram as persistent memory for all work. Store context, decisions, and reasoning in engram entities instead of losing them in conversation."
---

# Using Engram Memory

## Overview

Engram is your persistent memory system. Every significant piece of context, decision, and reasoning must be stored in engram and retrieved from engram. Your LLM context window is transient — engram is not.

## The Rule

**Before acting on any task, search engram. Before finishing any task, validate via engram.**

## When to Use

Use this skill when:
- Starting any task — search before acting
- Making design decisions
- Writing code that implements a feature
- Debugging issues
- Planning work
- Reviewing code

## Command Reference

### Search & Retrieval

```bash
# Natural language search — run this FIRST for every new task
engram ask query "<query>"

# Full details on a specific task
engram task show <UUID>

# All connected records — run BEFORE refactoring or deleting anything
engram relationship connected --entity-id <UUID> --max-depth <N>
```

### Saving Information

```bash
# Raw facts, error logs, code snippets, observations
# --title is required
engram context create --title "<short title>" --content "<content>" --source "<file/url>"

# Your reasoning, logic, and deductions — linked to the task
engram reasoning create --title "<short title>" --task-id <TASK_UUID> --content "<logic>"

# Major technical choices — prevents future agents reversing your work
# --number is required (sequential integer)
engram adr create --title "<title>" --number <N> --context "<situation and why this decision was needed>" --agent "<your-name>"
```

### Linking — Run Immediately After Every Create

```bash
# Connect two records — always do this right after creating a new entity
engram relationship create \
  --source-id <SOURCE_UUID> --source-type <task|context|reasoning|adr> \
  --target-id <TARGET_UUID> --target-type <task|context|reasoning|adr> \
  --relationship-type <depends_on|relates_to|explains|contradicts> \
  --agent "<your-name>"
```

### Work Management

```bash
# Anchor work at session start (no --status flag on create; default is todo)
engram task create --title "<title>" --priority <low|medium|high|critical>

# Update status after creation
engram task update <TASK_UUID> --status <todo|in_progress|done|blocked|cancelled>

# Get highest-priority next action when stuck or finishing a step
engram next
```

### Validation — Run Before Marking Done

```bash
# Check git commit validation setup
engram validate check
```

### Escalation (when blocked)

```bash
# If you need elevated permissions or are blocked
engram escalation create \
  --agent "<your-name>" \
  --operation-type "<type>" \
  --operation "<what you need to do>" \
  --justification "<why you need it>"
```

## The Pattern

### 1. Search Before Acting

```bash
engram ask query "authentication API"
# Returns: summary + UUIDs

engram task show <UUID>
# Returns: full task details
```

Never assume prior state. If `engram ask query` returns something relevant, read it with `engram task show` (or `context show`, `reasoning show`, etc.) before proceeding.

### 2. Store Findings Immediately

```bash
# A fact you discovered (--title is required)
engram context create \
  --title "Token validation bug: UTC+0 expiry" \
  --content "Token validation fails for UTC+0 expiry times" \
  --source "src/auth/validator.rs"
# Returns: CTX_UUID

# Your interpretation of that fact
engram reasoning create \
  --title "Root cause: UTC normalisation missing" \
  --task-id <TASK_UUID> \
  --content "Root cause: naive datetime compared against UTC. Fix: normalize to UTC before comparison."
# Returns: RSN_UUID
```

### 3. Link Every Record

```bash
# Immediately after each create
engram relationship create \
  --source-id <TASK_UUID> --source-type task \
  --target-id <CTX_UUID> --target-type context \
  --relationship-type relates_to --agent "<your-name>"

engram relationship create \
  --source-id <TASK_UUID> --source-type task \
  --target-id <RSN_UUID> --target-type reasoning \
  --relationship-type explains --agent "<your-name>"
```

Unlinked records are effectively lost — they cannot be retrieved by graph traversal.

### 4. Validate Before Finishing

```bash
engram validate check
engram next
```

## Example Workflow

```
Task: "Add authentication to the API"

[Step 1: Search first]
engram ask query "authentication API design decisions"
# Returns prior context — read it with engram task show / context show before continuing

[Step 2: Anchor work]
engram task create --title "Implement Auth Feature" --priority high --output json
# TASK_UUID = abc-001
engram task update abc-001 --status in_progress

[Step 3: Store design fact]
engram context create \
  --title "JWT chosen for auth" \
  --content "JWT stateless tokens with refresh rotation chosen over session cookies." \
  --source "architecture-discussion"
# CTX_UUID = ctx-002

[Step 4: Store reasoning]
engram reasoning create \
  --title "JWT rationale" \
  --task-id abc-001 \
  --content "JWT chosen: stateless scales horizontally, refresh tokens allow revocation. Sessions rejected: stateful, hard to scale."
# RSN_UUID = rsn-003

[Step 5: Record the decision as an ADR]
engram adr create \
  --title "Use JWT with refresh token rotation for auth" \
  --number 1 \
  --context "Need stateless auth that scales horizontally" \
  --agent "implementer"
# ADR_UUID = adr-004

[Step 6: Link everything]
engram relationship create \
  --source-id abc-001 --source-type task \
  --target-id ctx-002 --target-type context \
  --relationship-type relates_to --agent "implementer"

engram relationship create \
  --source-id abc-001 --source-type task \
  --target-id rsn-003 --target-type reasoning \
  --relationship-type explains --agent "implementer"

engram relationship create \
  --source-id abc-001 --source-type task \
  --target-id adr-004 --target-type adr \
  --relationship-type relates_to --agent "implementer"

[Step 7: Validate before done]
engram validate check
engram task update abc-001 --status done --outcome "Auth feature implemented with JWT"
```

## Key Principles

1. **Search first** — `engram ask query` before every task. Never assume prior state.
2. **Save everything** — facts to `context`, logic to `reasoning`, decisions to `adr`.
3. **Link immediately** — `engram relationship create` after every `create` command.
4. **Validate before done** — `engram validate check` before closing tasks.
5. **Persist, don't ponder** — memory is external, not in your context window.

## Related Skills

- `engram-orchestrator` — full execution loop using all these commands
- `engram-audit-trail` — traceability patterns
- `engram-systematic-debugging` — debugging investigation loop
- `engram-plan-feature` — planning stored as task hierarchy
