---
name: engram-subagent-driven-development
description: "Engram-integrated version. Use when executing implementation plans - dispatches one subagent per task, stores reviews as engram context, tracks progress via engram task UUIDs."
---

# Subagent-Driven Development

## Overview

Execute an implementation plan by dispatching one fresh subagent per subtask. Review between each task. Track all progress, reviews, and outcomes in engram — not in conversation.

**Core principle:** The orchestrator dispatches by UUID. Subagents retrieve context from engram and report back to engram.

## When to Use

Use after `engram-writing-plans` has created a task hierarchy in engram:
- You have a `PARENT_UUID` with subtasks ready to execute
- Each subtask needs implementation + review before the next begins
- You want a full audit trail of every agent action

## The Pattern

### 0. Search and Load the Plan

```bash
engram ask query "<feature name> implementation plan"
engram task show <PARENT_UUID>
engram relationship connected --entity-id <PARENT_UUID> --max-depth 2
```

This returns all subtask UUIDs. Work through them in order.

### 1. Dispatch Implementer Subagent

For the current subtask:

```bash
# Store context about the dispatch
engram context create \
  --title "Dispatch: <subtask title>" \
  --content "Dispatching implementer for: <subtask title>\nSubtask UUID: <SUBTASK_UUID>\nAgent: implementer-subagent\nContext available via: engram task show <SUBTASK_UUID>" \
  --source "dispatch-log"
# DISPATCH_CTX_UUID = ...

engram relationship create \
  --source-id <SUBTASK_UUID> --source-type task \
  --target-id <DISPATCH_CTX_UUID> --target-type context \
  --relationship-type relates_to --agent "<name>"
```

Tell the subagent:
```
Your task UUID is <SUBTASK_UUID>.
Run: engram task show <SUBTASK_UUID>
Use the engram-subagent-register skill.
```

### 2. Collect Implementer Results

When the subagent marks the task done:

```bash
engram relationship connected --entity-id <SUBTASK_UUID> --max-depth 2
engram ask query "<subtask title> implementation results"
```

### 3. Dispatch Spec Compliance Reviewer

Collect the diff and store it in engram first:

```bash
# Run git diff directly in your shell
git diff HEAD~1..HEAD --stat
git diff HEAD~1..HEAD

engram context create \
  --title "Spec review request: <subtask title>" \
  --content "Spec compliance review request for: <subtask title>\nSubtask UUID: <SUBTASK_UUID>\nDiff summary: <git diff --stat output>\nSpec requirements:\n- <requirement 1 from task description>\n- <requirement 2>" \
  --source "spec-review-request"
# SPEC_REQ_UUID = ...

engram relationship create \
  --source-id <SUBTASK_UUID> --source-type task \
  --target-id <SPEC_REQ_UUID> --target-type context \
  --relationship-type relates_to --agent "<name>"
```

Create review task and dispatch:

```bash
engram task create --title "Spec Review: <subtask title>"
# SPEC_REVIEW_UUID = ...
engram task update <SPEC_REVIEW_UUID> --status in_progress

engram relationship create \
  --source-id <SUBTASK_UUID> --source-type task \
  --target-id <SPEC_REVIEW_UUID> --target-type task \
  --relationship-type depends_on --agent "<name>"

engram relationship create \
  --source-id <SPEC_REVIEW_UUID> --source-type task \
  --target-id <SPEC_REQ_UUID> --target-type context \
  --relationship-type relates_to --agent "<name>"

# Tell reviewer: "Your task UUID is <SPEC_REVIEW_UUID>. Use engram-subagent-register."
```

### 4. Collect Spec Review Results

```bash
engram relationship connected --entity-id <SPEC_REVIEW_UUID> --max-depth 1
```

**If spec review fails:** retrieve the issues, dispatch implementer again with the same `SUBTASK_UUID`. The implementer will see the review task linked to their task.

**If spec review passes:** proceed to code quality review.

### 5. Dispatch Code Quality Reviewer

```bash
engram task create --title "Quality Review: <subtask title>"
# QUALITY_REVIEW_UUID = ...
engram task update <QUALITY_REVIEW_UUID> --status in_progress

engram relationship create \
  --source-id <SUBTASK_UUID> --source-type task \
  --target-id <QUALITY_REVIEW_UUID> --target-type task \
  --relationship-type depends_on --agent "<name>"

# Tell reviewer: "Your task UUID is <QUALITY_REVIEW_UUID>. Use engram-subagent-register."
```

### 6. Collect Quality Review Results

```bash
engram relationship connected --entity-id <QUALITY_REVIEW_UUID> --max-depth 1
```

**If quality review fails:** record issues, dispatch implementer again.

**If quality review passes:** record approval and move to next subtask.

### 7. Record Task Completion

```bash
engram reasoning create \
  --title "Subtask complete: <title>" \
  --task-id <SUBTASK_UUID> \
  --content "Subtask complete: <title>\nImplementer: done\nSpec review: PASSED\nQuality review: PASSED\nTests: <N>/<N> pass\nCommits: <list or count>"
# COMPLETE_UUID = ...

engram relationship create \
  --source-id <SUBTASK_UUID> --source-type task \
  --target-id <COMPLETE_UUID> --target-type reasoning \
  --relationship-type explains --agent "<name>"

engram task update <SUBTASK_UUID> --status done
```

### 8. Get Next Subtask

```bash
engram next
```

This returns the next pending subtask. Repeat from step 1.

### 9. Final Review

After all subtasks complete:

```bash
engram ask query "<feature name> all subtasks complete"
engram relationship connected --entity-id <PARENT_UUID> --max-depth 3
```

Dispatch a final integration reviewer:

```bash
engram task create --title "Final Review: <feature name>"
# FINAL_REVIEW_UUID = ...
engram task update <FINAL_REVIEW_UUID> --status in_progress

engram relationship create \
  --source-id <PARENT_UUID> --source-type task \
  --target-id <FINAL_REVIEW_UUID> --target-type task \
  --relationship-type depends_on --agent "<name>"

# Tell reviewer: "Your task UUID is <FINAL_REVIEW_UUID>. Use engram-subagent-register."
```

### 10. Validate and Close

```bash
engram validate check
engram task update <PARENT_UUID> --status done
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
Plan: Implement User Auth API — PARENT_UUID = auth-parent

[Load plan]
engram relationship connected --entity-id auth-parent --max-depth 2
# Returns: login-task, register-task, refresh-task, logout-task, tests-task

[Task 1: Login endpoint]

Dispatch implementer:
engram context create \
  --title "Dispatch: login endpoint" \
  --content "Dispatching implementer for login endpoint. UUID: login-task." \
  --source "dispatch-log"
# CTX = ctx-001

engram relationship create \
  --source-id login-task --source-type task \
  --target-id ctx-001 --target-type context \
  --relationship-type relates_to --agent "orchestrator"
# "Your task UUID is login-task. Use engram-subagent-register."

Collect:
engram relationship connected --entity-id login-task --max-depth 2

Collect diff (run directly in shell):
git diff HEAD~1..HEAD --stat

Spec review:
engram context create \
  --title "Spec review: login endpoint" \
  --content "Spec review: login-task. Requirements: POST /auth/login returns 200 + JWT on valid creds, 401 on invalid." \
  --source "spec-review-request"
# SPEC_CTX = ctx-002

engram task create --title "Spec Review: Login endpoint"
# SPEC_UUID = spec-001
engram task update spec-001 --status in_progress

engram relationship create \
  --source-id login-task --source-type task \
  --target-id spec-001 --target-type task \
  --relationship-type depends_on --agent "orchestrator"

engram relationship create \
  --source-id spec-001 --source-type task \
  --target-id ctx-002 --target-type context \
  --relationship-type relates_to --agent "orchestrator"
# Dispatch reviewer to spec-001

[Spec passes]

Quality review:
engram task create --title "Quality Review: Login endpoint"
# QUAL_UUID = qual-001
engram task update qual-001 --status in_progress

engram relationship create \
  --source-id login-task --source-type task \
  --target-id qual-001 --target-type task \
  --relationship-type depends_on --agent "orchestrator"
# Dispatch reviewer to qual-001

[Quality passes]

Record completion:
engram reasoning create \
  --title "Login task complete" \
  --task-id login-task \
  --content "Login task complete. Spec: PASSED. Quality: PASSED. Tests: 5/5."

engram task update login-task --status done

Get next:
engram next
# Returns: register-task

# ... repeat for each task ...

[All tasks done: Final review]
engram task create --title "Final Review: User Auth API"
# FINAL_UUID = final-001
engram task update final-001 --status in_progress

engram relationship create \
  --source-id auth-parent --source-type task \
  --target-id final-001 --target-type task \
  --relationship-type depends_on --agent "orchestrator"
# Dispatch reviewer to final-001

[Validate and close]
engram validate check
engram task update auth-parent --status done
```

## Review Status Reference

| Status | Meaning | Action |
|--------|---------|--------|
| Spec PASSED | All requirements met | Proceed to quality review |
| Spec FAILED | Requirements not met | Re-dispatch implementer |
| Quality APPROVED | Code is clean | Mark task complete, next |
| Quality CHANGES REQUESTED | Issues found | Re-dispatch implementer |

## Related Skills

- `engram-writing-plans` — creates the task hierarchy this skill executes
- `engram-subagent-register` — what each dispatched subagent uses
- `engram-requesting-code-review` — detailed review dispatch pattern
- `engram-dispatching-parallel-agents` — for independent tasks that can run simultaneously
- `engram-orchestrator` — the full orchestration loop
