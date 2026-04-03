---
name: engram-orchestrator
description: "Engram Orchestrator agent loop. Governs coordination: search before acting, dispatch subagents with task UUIDs, validate system state before finishing. Never guess — use engram."
---

# Engram Orchestrator

## Overview

You are the orchestrating agent. Your job is to coordinate work across subagents using engram as the shared state. You do not implement — you plan, dispatch, track, and validate.

**Core principle:** Engram is the single source of truth. Search it before acting. Write to it after every decision.

## When to Use

Use this skill when:
- Starting a new session or receiving a new top-level task
- You need to know what has already been done before acting
- You are about to dispatch work to a subagent
- A subagent has reported back and you need to record the outcome
- You believe work is complete and need to verify

## The Execution Loop

```
New Task Received
    ↓
1. SEARCH — engram ask query / engram task show
    ↓
2. PLAN — engram task create (hierarchy with --parent)
    ↓
3. DISPATCH — engram task update --status in_progress
              (pass TASK_UUID to subagent, not instructions inline)
    ↓
4. WAIT — subagent runs engram-subagent-register skill
    ↓
5. COLLECT — engram relationship connected --entity-id <SUBTASK_UUID>
    ↓
6. DECIDE — record outcome as reasoning or adr
    ↓
Ready to finish? → NO → engram next → back to step 3
    ↓ YES
7. VALIDATE — engram validate check
    ↓
8. CLOSE — engram task update --status done
```

## The Pattern

### 1. Search Before Anything

Before touching any task, check what already exists:

```bash
# Natural language search — run this first on every new prompt
engram ask query "<what you need to know>"

# Get full details on a task
engram task show <UUID>

# Check all records connected to an entity
engram relationship connected --entity-id <UUID> --max-depth 3
```

**Rule:** Never assume prior state. If `engram ask query` returns a relevant record, read it before proceeding.

### 2. Build the Task Hierarchy

Create one parent task per goal, then subtasks per unit of delegable work:

```bash
# Parent task anchors the entire piece of work
# No --status flag on create; default is todo
engram task create \
  --title "<Goal: short description>" \
  --priority high \
  --output json
# Returns: PARENT_UUID
engram task update <PARENT_UUID> --status in_progress

# One subtask per agent assignment — use --parent to nest it
engram task create \
  --title "<Subtask: what this agent does>" \
  --parent <PARENT_UUID> \
  --priority medium \
  --output json
# Returns: SUBTASK_UUID
```

### 3. Dispatch to Subagents

Update the subtask to in_progress and pass only the UUID to the subagent:

```bash
engram task update <SUBTASK_UUID> --status in_progress
```

**What to tell the subagent:**
```
Your task UUID is <SUBTASK_UUID>.
Run `engram task show <SUBTASK_UUID>` to get your instructions.
Use the engram-subagent-register skill to record your findings and report back.
```

Do not paste instructions or context inline. The subagent must pull from engram.

### 4. Collect Subagent Results

When a subagent completes, retrieve what it stored:

```bash
# Get all records connected to the subtask (BFS traversal)
engram relationship connected --entity-id <SUBTASK_UUID> --max-depth 2
```

### 5. Record Decisions

When you synthesise subagent results into a decision:

```bash
# For technical choices with lasting impact
# --number is required (sequential integer); no --status or --decision flags
engram adr create \
  --title "<short decision title>" \
  --number <N> \
  --context "<situation and what was decided and why>" \
  --agent "<your-name>"

# For reasoning that links back to evidence
engram reasoning create \
  --title "<reasoning title>" \
  --task-id <PARENT_UUID> \
  --content "<your logic>"

# Link reasoning to the parent task
engram relationship create \
  --source-id <PARENT_UUID> --source-type task \
  --target-id <REASONING_UUID> --target-type reasoning \
  --relationship-type explains --agent "<your-name>"
```

### 6. Get Next Action When Unsure

```bash
engram next
```

Use this whenever you finish a step and are unsure what to do next.

### 7. Validate Before Closing

Before marking any task done:

```bash
# Check git commit validation setup
engram validate check
```

**Rule:** Do not mark done until validate check passes clean.

```bash
engram task update <PARENT_UUID> --status done --outcome "<summary>"
```

## Orchestrator Rules

1. **Search first** — `engram ask query` before every action.
2. **Dispatch by UUID** — subagents pull their own context; you don't push it.
3. **Record decisions, not just actions** — use `adr` for architectural choices, `reasoning` for logic chains.
4. **Link everything** — run `engram relationship create` after every `create` command (requires --source-type, --target-type, --agent).
5. **Validate before closing** — `engram validate check` must pass.
6. **Never hallucinate state** — if it isn't in engram, it is unknown. Query or store it.

## Example Session

```
Goal: "Add rate limiting to the API"

[Step 1: Search]
engram ask query "rate limiting API"
# Returns prior context nodes — read them before continuing

[Step 2: Create hierarchy]
engram task create --title "Goal: Add API rate limiting" --priority high --output json
# PARENT_UUID = abc-001
engram task update abc-001 --status in_progress

engram task create --title "Research: evaluate rate limit libraries" --parent abc-001 --priority medium --output json
# SUBTASK_UUID = abc-002

engram task create --title "Implement: add middleware and config" --parent abc-001 --priority medium --output json
# SUBTASK_UUID = abc-003

[Step 3: Dispatch research subagent]
engram task update abc-002 --status in_progress
# Tell subagent: "Your task is abc-002. Use engram-subagent-register."

[Step 4: Collect results]
engram relationship connected --entity-id abc-002 --max-depth 2
# Returns linked context and reasoning the subagent stored

[Step 5: Record architectural decision]
engram adr create \
  --title "Use tower-governor for API rate limiting" \
  --number 3 \
  --context "Need per-IP rate limiting on public API endpoints. tower-governor chosen: integrates with axum, supports Redis backend, actively maintained." \
  --agent "orchestrator"
# ADR_UUID = abc-004

engram relationship create \
  --source-id abc-001 --source-type task \
  --target-id abc-004 --target-type adr \
  --relationship-type explains --agent "orchestrator"

[Step 6: Dispatch implementation subagent]
engram task update abc-003 --status in_progress
# Tell subagent: "Your task is abc-003. Use engram-subagent-register."

[Step 7: Validate and close]
engram validate check
engram task update abc-001 --status done --outcome "Rate limiting implemented with tower-governor"
```

## Related Skills

- `engram-subagent-register` — what subagents use to claim tasks and report back
- `engram-delegate-to-agents` — agent catalog and delegation patterns
- `engram-dispatching-parallel-agents` — running multiple subagents concurrently
- `engram-writing-plans` — creating task hierarchies from specs
- `engram-audit-trail` — full traceability of orchestrated work
