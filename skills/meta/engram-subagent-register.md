---
name: engram-subagent-register
description: "Subagent registration protocol. Use when assigned a task UUID by an orchestrator — claim the task, pull context from engram, store all findings, and report results back via engram relationships."
---

# Subagent Registration

## Overview

You are a subagent. You have been given a task UUID by an orchestrator. Your job is to pull your instructions from engram, do the work, store every finding back into engram, and signal completion — all via engram. Do not communicate results back through conversation; write them to engram so the orchestrator can retrieve them.

**Core principle:** Engram is the handshake protocol between you and the orchestrator. Your input comes from it. Your output goes to it.

## When to Use

Use this skill when:
- You receive a task UUID from an orchestrator
- You are starting work on an assigned task
- You find something during your work (fact, error, decision)
- You complete your work and need to report back
- You are blocked and cannot continue

## The Pattern

### 0. Engram-First Research Protocol

**Before doing any research or writing any code**, internalize this rule:

> **Every finding is written to engram the moment it is discovered — not at the end.**

This prevents data loss if you hit context limits, token exhaustion, or tool errors mid-task. The pattern is strictly sequential per finding:

```
for each finding:
  1. engram context create / engram reasoning create   ← write immediately
  2. engram relationship create                        ← link immediately
  3. only then: continue to next finding
```

**Never batch findings.** Never accumulate context and write at the end. If you have five findings, you make five separate `engram context create` calls as you go, each followed immediately by `engram relationship create`.

### 1. Claim the Task

The first thing you do is mark the task as claimed so other agents don't duplicate work.

**If the orchestrator has not already registered you, self-register with your role type.** This makes you queryable by type in engram. See `engram-agent-types` for the full list of role types.

```bash
# Self-register with your role type if not already registered
engram setup agent \
  --name "<your-agent-name>" \
  --agent-type "<your-role: researcher|coder|reviewer|tester|...>" \
  --specialization "<your area of focus>"

# Then claim the task
engram task show <TASK_UUID>

# Claim it — mark as in_progress
# Note: --status is only on task update, not task create
engram task update <TASK_UUID> --status in_progress
```

### 2. Pull All Context

Before writing a single line of code or making any change, retrieve everything the orchestrator has stored for this task:

```bash
# Get all records connected to your task
engram relationship connected --entity-id <TASK_UUID> --max-depth 2

# Search for broader relevant context
engram ask query "<keywords from your task title>"
```

**Rule:** Never start work based only on the task title. Always check for linked context first.

### 3. Store Every Finding Immediately — No Batching

**CRITICAL:** Write each finding to engram the moment you discover it. Do NOT accumulate findings and batch them at the end. Context limits and tool errors are real — if you batch and fail, everything is lost.

Required sequence per finding:
1. `engram context create` (or `engram reasoning create`) — write the finding
2. `engram relationship create` — link it to your task UUID immediately
3. Continue to the next finding only after both commands above succeed

Every time you discover something — a fact, an error, a working result — store it before moving to the next step.

**Raw facts, observations, error output:**

```bash
# --title is required on context create
engram context create \
  --title "<short title for this finding>" \
  --content "<what you found — be specific, include full error text or relevant output>" \
  --source "<file path, URL, or command that produced this>"
# Returns: CONTEXT_UUID

# Link it to your task immediately
engram relationship create \
  --source-id <TASK_UUID> --source-type task \
  --target-id <CONTEXT_UUID> --target-type context \
  --relationship-type relates_to --agent "<your-name>"
```

**Your reasoning about what a finding means:**

```bash
# --task-id links the reasoning to the task; use --title for identification
engram reasoning create \
  --title "<short reasoning title>" \
  --task-id <TASK_UUID> \
  --content "<your interpretation, why this matters, what it implies>"
# Returns: REASONING_UUID

engram relationship create \
  --source-id <TASK_UUID> --source-type task \
  --target-id <REASONING_UUID> --target-type reasoning \
  --relationship-type explains --agent "<your-name>"
```

**A decision you made while working:**

```bash
# --number is required (use next sequential integer); no --status or --decision flags
engram adr create \
  --title "<short decision title>" \
  --number <N> \
  --context "<what situation led to this choice and what was decided>" \
  --agent "<your-name>"
# Returns: ADR_UUID

engram relationship create \
  --source-id <TASK_UUID> --source-type task \
  --target-id <ADR_UUID> --target-type adr \
  --relationship-type relates_to --agent "<your-name>"
```

### 4. Store Artifacts

When you produce a file, commit, or test result, record it:

```bash
engram context create \
  --title "Artifact: <file name>" \
  --content "<file path and summary of what it contains>" \
  --source "<file path>"
# Returns: ARTIFACT_UUID

engram relationship create \
  --source-id <TASK_UUID> --source-type task \
  --target-id <ARTIFACT_UUID> --target-type context \
  --relationship-type relates_to --agent "<your-name>"

# Record test results
engram context create \
  --title "Test results: <suite name>" \
  --content "Tests: <N>/<N> passed. Failed: <list if any>. Coverage: <N>%" \
  --source "test-results"
# Returns: TEST_UUID

engram relationship create \
  --source-id <TASK_UUID> --source-type task \
  --target-id <TEST_UUID> --target-type context \
  --relationship-type relates_to --agent "<your-name>"
```

### 5. Write Your Completion Report

When your work is done, create a single reasoning record that summarises everything the orchestrator needs to know:

```bash
engram reasoning create \
  --title "Completion report: <task title>" \
  --task-id <TASK_UUID> \
  --content "## Result\n<one-sentence outcome>\n\n## What I did\n- <action 1>\n- <action 2>\n\n## Findings\n- <key finding 1> → <CONTEXT_UUID>\n- <key finding 2> → <CONTEXT_UUID>\n\n## Artifacts\n- <file or output> → <ARTIFACT_UUID>\n\n## Decisions made\n- <decision> → <ADR_UUID>\n\n## Status\n<COMPLETED | BLOCKED: reason | PARTIAL: what remains>"
# Returns: REPORT_UUID

engram relationship create \
  --source-id <TASK_UUID> --source-type task \
  --target-id <REPORT_UUID> --target-type reasoning \
  --relationship-type explains --agent "<your-name>"
```

### 6. Mark the Task Done

```bash
engram task update <TASK_UUID> \
  --status done \
  --outcome "<one-line summary of what was achieved>"
```

The orchestrator will now see `status: done` and retrieve your report via `engram relationship connected`.

## Handling Blockers

If you cannot complete the task:

```bash
# Store what you found before being blocked
engram context create \
  --title "Blocker: <short description>" \
  --content "<what you tried, what the blocker is, any partial findings>" \
  --source "<relevant file or command>"
# CONTEXT_UUID = ...

# Record why you are blocked
engram reasoning create \
  --title "Blocked: <reason>" \
  --task-id <TASK_UUID> \
  --content "## Blocked\n**Reason:** <why you cannot continue>\n**Attempted:** <what you tried>\n**Needs:** <what the orchestrator must provide or decide>\n**Partial work:** <what was completed before the block>"
# REASONING_UUID = ...

engram relationship create \
  --source-id <TASK_UUID> --source-type task \
  --target-id <REASONING_UUID> --target-type reasoning \
  --relationship-type explains --agent "<your-name>"

# Request escalation if you need elevated permissions
engram escalation create \
  --agent "<your-name>" \
  --operation-type "<type>" \
  --operation "<what you need to do>" \
  --justification "<why you need it>"

# Update task status
engram task update <TASK_UUID> --status blocked --reason "<short reason>"
```

## Subagent Rules

1. **Claim first** — `engram task update --status in_progress` before any work.
2. **Pull context before acting** — `engram relationship connected` on your task UUID.
3. **Engram-first, per finding** — write each finding to engram the moment you discover it; do NOT batch at the end. Pattern: `create → relationship create → next finding`.
4. **Link every record** — `engram relationship create` after every `context create`, `reasoning create`, or `adr create` (requires --source-type, --target-type, --agent).
5. **Escalate blockers** — `engram escalation create` then set task to `blocked`. Never work around restrictions.
6. **Report via engram** — your completion report is a reasoning record linked to your task UUID, not a conversation message.

## Example: Research Subagent

```
Received: TASK_UUID = abc-002
Task title: "Research: evaluate rate limit libraries for axum"

[Step 0: Review engram-first protocol — write each finding immediately, no batching]

[Step 1: Claim]
engram task show abc-002
engram task update abc-002 --status in_progress

[Step 2: Pull context]
engram relationship connected --entity-id abc-002 --max-depth 2
# Returns linked context — review before continuing

[Step 3: Finding 1 — write immediately, then link, then move on]
engram context create \
  --title "tower-governor: production-ready rate limiting" \
  --content "tower-governor v0.3: integrates with tower middleware, supports Redis, 1200 downloads/week, last commit 3 weeks ago" \
  --source "https://crates.io/crates/tower-governor"
# CONTEXT_tg = cxt-010

engram relationship create \
  --source-id abc-002 --source-type task \
  --target-id cxt-010 --target-type context \
  --relationship-type relates_to --agent "researcher"
# ← relationship linked. Now safe to move to next finding.

[Step 3: Finding 2 — write immediately, then link, then move on]
engram context create \
  --title "axum-ratelimit: alpha, not suitable" \
  --content "axum-ratelimit v0.1: alpha quality, no Redis support, 40 downloads/week, last commit 8 months ago" \
  --source "https://crates.io/crates/axum-ratelimit"
# CONTEXT_ar = cxt-011

engram relationship create \
  --source-id abc-002 --source-type task \
  --target-id cxt-011 --target-type context \
  --relationship-type relates_to --agent "researcher"
# ← relationship linked. Now safe to move to reasoning.

[Step 4: Record reasoning]
engram reasoning create \
  --title "Recommendation: tower-governor" \
  --task-id abc-002 \
  --content "tower-governor is the clear choice: actively maintained, Redis backend for distributed rate limiting, integrates cleanly with axum via tower middleware layer."
# REASONING_UUID = rsn-005

engram relationship create \
  --source-id abc-002 --source-type task \
  --target-id rsn-005 --target-type reasoning \
  --relationship-type explains --agent "researcher"

[Step 5: Completion report]
engram reasoning create \
  --title "Completion report: rate limit library research" \
  --task-id abc-002 \
  --content "## Result\nRecommend tower-governor for rate limiting.\n\n## Findings\n- tower-governor: production-ready, Redis-backed → cxt-010\n- axum-ratelimit: alpha, not suitable → cxt-011\n\n## Recommendation\ntower-governor → rsn-005\n\n## Status\nCOMPLETED"
# REPORT_UUID = rsn-006

engram relationship create \
  --source-id abc-002 --source-type task \
  --target-id rsn-006 --target-type reasoning \
  --relationship-type explains --agent "researcher"

[Step 6: Done]
engram task update abc-002 --status done --outcome "Recommended tower-governor"
```

## Related Skills

- `engram-orchestrator` — the coordinating agent that dispatches you and reads your results
- `engram-use-engram-memory` — entity types and memory patterns reference
- `engram-audit-trail` — detailed traceability patterns
- `engram-systematic-debugging` — investigation loop for debugging tasks
