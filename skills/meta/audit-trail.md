---
name: engram-audit-trail
description: "Create complete audit trail of all work. Store decisions, actions, and outcomes in engram for traceability."
---

# Creating Audit Trails

## Overview

Every significant action must leave a trace in engram. This enables traceability, knowledge transfer, and compliance. Store as you go — not at the end.

## When to Use

Use this skill when:
- Completing tasks
- Making decisions
- Deploying changes
- Responding to incidents
- Any work that needs to be remembered

## The Pattern

### 0. Search First

Before recording anything, check what already exists:

```bash
engram ask query "<topic or task name>"
engram task show <UUID>
```

### 1. Anchor the Work

```bash
# --status is not available on create; default is todo
engram task create --title "<Work description>" --priority medium --output json
# TASK_UUID = ...
engram task update <TASK_UUID> --status in_progress
```

### 2. Record Actions As They Happen

```bash
# Work started — --title is required on context create
engram context create \
  --title "Started: <work description>" \
  --content "Started: <timestamp>. Initial state: <what was found>" \
  --source "<file or system>"
# CTX_UUID = ...

engram relationship create \
  --source-id <TASK_UUID> --source-type task \
  --target-id <CTX_UUID> --target-type context \
  --relationship-type relates_to --agent "<your-name>"

# Progress checkpoint
engram reasoning create \
  --title "Progress: <step name>" \
  --task-id <TASK_UUID> \
  --content "Step: <what was done>. Result: <what happened>. Next: <what comes next>"
# RSN_UUID = ...

engram relationship create \
  --source-id <TASK_UUID> --source-type task \
  --target-id <RSN_UUID> --target-type reasoning \
  --relationship-type explains --agent "<your-name>"
```

### 3. Record Artifacts

When you produce a file, test result, or commit:

```bash
engram context create \
  --title "Artifact: <file name>" \
  --content "<file path and summary of what it contains>" \
  --source "<file path>"
# ARTIFACT_UUID = ...

engram relationship create \
  --source-id <TASK_UUID> --source-type task \
  --target-id <ARTIFACT_UUID> --target-type context \
  --relationship-type relates_to --agent "<your-name>"

# Test results
engram context create \
  --title "Test results: <suite name>" \
  --content "Tests: <N>/<N> passed. Failed: <list if any>." \
  --source "test-runner"
# TEST_UUID = ...

engram relationship create \
  --source-id <TASK_UUID> --source-type task \
  --target-id <TEST_UUID> --target-type context \
  --relationship-type relates_to --agent "<your-name>"
```

### 4. Record Major Decisions as ADRs

When making an architectural or significant technical choice:

```bash
# --number is required (sequential integer); status defaults to Proposed
engram adr create \
  --title "<short decision title>" \
  --number <N> \
  --context "<what situation led to this and what was decided>" \
  --agent "<your-name>"
# ADR_UUID = ...

engram relationship create \
  --source-id <TASK_UUID> --source-type task \
  --target-id <ADR_UUID> --target-type adr \
  --relationship-type relates_to --agent "<your-name>"
```

### 5. Create Summary on Completion

```bash
engram reasoning create \
  --title "Summary: <work description>" \
  --task-id <TASK_UUID> \
  --content "## Summary\nWhat: <work description>\nWhy: <purpose>\nHow: <approach>\n\n## Outcomes\n- <outcome 1>\n- <outcome 2>\n\n## Learnings\n- <what was learned>"
# SUMMARY_UUID = ...

engram relationship create \
  --source-id <TASK_UUID> --source-type task \
  --target-id <SUMMARY_UUID> --target-type reasoning \
  --relationship-type explains --agent "<your-name>"
```

### 6. Validate and Close

```bash
engram validate check
engram task update <TASK_UUID> --status done --outcome "<one-line summary>"
```

### 7. Get Next Action

```bash
engram next
```

## Example

```
Task: "Fix authentication bug"

[Search first]
engram ask query "authentication bug token"

[Anchor]
engram task create --title "Fix: Auth token bug" --priority high --output json
# TASK_UUID = task-001
engram task update task-001 --status in_progress

[Record investigation]
engram context create \
  --title "Error: token validation fails for UTC+0" \
  --content "Error: token validation fails for UTC+0 timestamps. Stack trace: UserService.java:142" \
  --source "logs/app.log"
# CTX_UUID = ctx-002

engram relationship create \
  --source-id task-001 --source-type task \
  --target-id ctx-002 --target-type context \
  --relationship-type relates_to --agent "debugger"

[Record root cause reasoning]
engram reasoning create \
  --title "Root cause: UTC normalisation missing" \
  --task-id task-001 \
  --content "Root cause: naive datetime compared against UTC. Token issued in UTC+0 compared without timezone normalisation."
# RSN_UUID = rsn-003

engram relationship create \
  --source-id task-001 --source-type task \
  --target-id rsn-003 --target-type reasoning \
  --relationship-type explains --agent "debugger"

[Record the fix decision as ADR]
engram adr create \
  --title "Normalize all auth timestamps to UTC before comparison" \
  --number 5 \
  --context "Token validation failed for UTC+0 tokens. Always call .with_timezone(&Utc) before datetime comparison in auth paths." \
  --agent "debugger"
# ADR_UUID = adr-004

engram relationship create \
  --source-id task-001 --source-type task \
  --target-id adr-004 --target-type adr \
  --relationship-type relates_to --agent "debugger"

[Record test results]
engram context create \
  --title "Test results: auth suite" \
  --content "Tests: 47/47 passed. Regression: none." \
  --source "test-runner"
# TEST_UUID = ctx-005

engram relationship create \
  --source-id task-001 --source-type task \
  --target-id ctx-005 --target-type context \
  --relationship-type relates_to --agent "debugger"

[Validate and close]
engram validate check
engram task update task-001 --status done --outcome "Fixed UTC token validation bug"
```

## Key Principles

1. **Record as you go** — don't wait until the end
2. **Link every record** — `engram relationship create` after every create (requires --source-type and --target-type)
3. **ADRs for decisions** — use `engram adr create` not `engram reasoning create` for architectural choices
4. **Validate before done** — `engram validate check` before closing

## Related Skills

- `engram-use-engram-memory` — core memory patterns
- `engram-orchestrator` — full execution loop
- `engram-systematic-debugging` — debugging investigation trail
- `engram-check-compliance` — store compliance evidence
