---
name: engram-dispatching-parallel-agents
description: "Engram-integrated version. Use when facing 2+ independent tasks - creates engram workflow with parallel states instead of manual task dispatch."
---

# Dispatching Parallel Agents

## Overview

When you have multiple independent tasks, dispatching agents sequentially wastes time. Dispatch one agent per independent domain concurrently, using engram workflows to coordinate and engram relationships to collect results.

**Core principle:** Dispatch by UUID. Agents pull their own context. Results are stored in engram, not returned in conversation.

## When to Use

Use when:
- 2+ independent failures or tasks exist simultaneously
- Each domain can be understood without context from the others
- No shared mutable state between investigations

Do not use when:
- Tasks are related (fixing one may fix others)
- Agents would write to the same files
- Quick one-off fix with no audit trail needed

## The Pattern

### 0. Search First

```bash
engram ask query "<domain or feature name>"
# Returns: summary + UUIDs

engram task show <UUID>
# Returns: full record for a specific task
```

### 1. Anchor Work and Analyse Domains

```bash
engram task create --title "<Goal: description>"
# PARENT_UUID = ...
engram task update <PARENT_UUID> --status in_progress

# Store the domain analysis
engram context create \
  --title "Domain analysis: <goal>" \
  --content "## Independent Domains\n\nDomain 1: <name>\n- Scope: <what files/components>\n- Hypothesis: <root cause guess>\n\nDomain 2: <name>\n- Scope: <what files/components>\n- Hypothesis: <root cause guess>\n\n## Independence Justification\n- <why domain 1 does not affect domain 2>\n\n## Why Parallel\n- Sequential: N x <time> = <total>\n- Parallel: <time>" \
  --source "domain-analysis"
# ANALYSIS_UUID = ...

engram relationship create \
  --source-id <PARENT_UUID> --source-type task \
  --target-id <ANALYSIS_UUID> --target-type context \
  --relationship-type relates_to --agent "<name>"
```

### 2. Create a Workflow

Workflows require a definition first, then an instance:

```bash
# Create the workflow definition
engram workflow create \
  --title "Parallel Investigation" \
  --description "Coordinate parallel agent domains" \
  --agent "<name>"
# WORKFLOW_DEF_UUID = ...

# Start an instance of the workflow
engram workflow start <WORKFLOW_DEF_UUID> --agent "<name>"
# WORKFLOW_INSTANCE_UUID = ...
```

### 3. Create Subtasks — One Per Domain

```bash
engram task create --title "<Domain 1>: <Problem description>"
# D1_UUID = ...
engram task update <D1_UUID> --status in_progress

engram relationship create \
  --source-id <PARENT_UUID> --source-type task \
  --target-id <D1_UUID> --target-type task \
  --relationship-type depends_on --agent "<name>"

engram task create --title "<Domain 2>: <Problem description>"
# D2_UUID = ...
engram task update <D2_UUID> --status in_progress

engram relationship create \
  --source-id <PARENT_UUID> --source-type task \
  --target-id <D2_UUID> --target-type task \
  --relationship-type depends_on --agent "<name>"
```

Store each domain's scope and constraints in engram so agents can retrieve them:

```bash
engram context create \
  --title "Domain 1 brief" \
  --content "Domain 1 scope: <files/components to investigate>\nConstraints: do NOT modify domain 2 files\nGoal: <specific outcome>" \
  --source "domain-1-brief"
# BRIEF1_UUID = ...

engram relationship create \
  --source-id <D1_UUID> --source-type task \
  --target-id <BRIEF1_UUID> --target-type context \
  --relationship-type relates_to --agent "<name>"
```

### 4. Dispatch — Pass UUID Only

```bash
# Tell each subagent only their task UUID:
# "Your task UUID is <D1_UUID>. Run: engram task show <D1_UUID>. Use engram-subagent-register."
```

Subagents retrieve their brief via:
```bash
engram relationship connected --entity-id <D1_UUID> --max-depth 2
```

### 5. Advance Workflow State

```bash
engram workflow transition <WORKFLOW_INSTANCE_UUID> \
  --transition "domains-dispatched" \
  --agent "<name>"
```

### 6. Collect Results

When subagents mark their tasks done:

```bash
engram relationship connected --entity-id <D1_UUID> --max-depth 2
engram relationship connected --entity-id <D2_UUID> --max-depth 2
engram ask query "<feature name> parallel results"
```

### 7. Integrate and Record

```bash
engram reasoning create \
  --title "Integration summary: <goal>" \
  --task-id <PARENT_UUID> \
  --content "## Integration Summary\n\nDomain 1 (<name>):\n- Root cause: <description>\n- Changes: <files modified>\n- Status: COMPLETE\n\nDomain 2 (<name>):\n- Root cause: <description>\n- Changes: <files modified>\n- Status: COMPLETE\n\n## Conflict Check\n- Domain 1 files: <list>\n- Domain 2 files: <list>\n- Conflicts: none / <describe any>"
# SUMMARY_UUID = ...

engram relationship create \
  --source-id <PARENT_UUID> --source-type task \
  --target-id <SUMMARY_UUID> --target-type reasoning \
  --relationship-type explains --agent "<name>"
```

### 8. Validate and Close

```bash
engram validate check

engram workflow transition <WORKFLOW_INSTANCE_UUID> \
  --transition "integration-complete" \
  --agent "<name>"

engram task update <PARENT_UUID> --status done
```

### 9. Get Next Action When Unsure

```bash
engram next
```

## Terminal Commands

Run terminal commands directly in your shell. Do not use `engram sandbox execute` — that command does not exist.

If you need elevated permissions or human approval:

```bash
engram escalation create \
  --agent "<name>" \
  --operation-type "<type>" \
  --operation "<what you need to do>" \
  --justification "<why this is needed>"
```

## Example

```
Scenario: 6 test failures across 3 independent files

[Search]
engram ask query "test failures parallel domains"

[Anchor]
engram task create --title "Fix: 6 test failures (parallel)"
# PARENT_UUID = abc-001
engram task update abc-001 --status in_progress

[Domain analysis]
engram context create \
  --title "Domain analysis: 6 test failures" \
  --content "Domain 1: agent-tool-abort.test.ts — timing issues (3 failures)\nDomain 2: batch-completion.test.ts — event structure (2 failures)\nDomain 3: tool-approval.test.ts — async wait (1 failure)\nIndependent: Yes, separate subsystems" \
  --source "domain-analysis"
# ANALYSIS_UUID = ctx-002

engram relationship create \
  --source-id abc-001 --source-type task \
  --target-id ctx-002 --target-type context \
  --relationship-type relates_to --agent "orchestrator"

[Create workflow]
engram workflow create \
  --title "Parallel Investigation" \
  --description "3-domain parallel fix" \
  --agent "orchestrator"
# WORKFLOW_DEF_UUID = wf-def-001

engram workflow start wf-def-001 --agent "orchestrator"
# WORKFLOW_INSTANCE_UUID = wf-003

[Create subtasks]
engram task create --title "Fix: agent-tool-abort.test.ts"
# D1_UUID = abc-004
engram task update abc-004 --status in_progress

engram relationship create \
  --source-id abc-001 --source-type task \
  --target-id abc-004 --target-type task \
  --relationship-type depends_on --agent "orchestrator"

engram context create \
  --title "Domain 1 brief" \
  --content "Scope: agent-tool-abort.test.ts only. Goal: fix 3 timing failures. Do not touch batch or approval files." \
  --source "domain-1-brief"
# BRIEF1_UUID = ctx-005

engram relationship create \
  --source-id abc-004 --source-type task \
  --target-id ctx-005 --target-type context \
  --relationship-type relates_to --agent "orchestrator"

# ... repeat for D2 and D3 ...

[Dispatch — UUID only to each agent]
# "Your task UUID is abc-004. Use engram-subagent-register."

[Advance workflow]
engram workflow transition wf-003 --transition "dispatched" --agent "orchestrator"

[Collect results when done]
engram relationship connected --entity-id abc-004 --max-depth 2
engram relationship connected --entity-id abc-005 --max-depth 2

[Integrate]
engram reasoning create \
  --title "Integration: all 3 domains resolved" \
  --task-id abc-001 \
  --content "All 3 domains resolved. No file conflicts. Full test suite: 47/47 pass."
# RSN_UUID = rsn-010

engram relationship create \
  --source-id abc-001 --source-type task \
  --target-id rsn-010 --target-type reasoning \
  --relationship-type explains --agent "orchestrator"

[Validate and close]
engram validate check

engram workflow transition wf-003 --transition "complete" --agent "orchestrator"
engram task update abc-001 --status done
```

## Related Skills

- `engram-delegate-to-agents` — single-agent delegation pattern
- `engram-subagent-register` — what subagents use to claim tasks and report back
- `engram-orchestrator` — full orchestration loop
- `engram-audit-trail` — traceability of parallel agent work
