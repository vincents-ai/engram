---
name: engram-requesting-code-review
description: "Engram-integrated version. Use when completing tasks or before merging - stores review context in engram and dispatches reviewer subagent via task UUID."
---

# Requesting Code Review

## Overview

Dispatch a code-reviewer subagent to catch issues before they cascade. Store all review context in engram. The reviewer retrieves everything it needs from engram — you do not paste diffs inline.

**Core principle:** Review early, review often. All artifacts documented in engram.

## When to Request Review

**Mandatory:**
- After each task in subagent-driven development
- After completing a major feature
- Before merge to main
- After `engram validate check` passes

**Optional:**
- When stuck (fresh perspective)
- Before refactoring (baseline check)
- After fixing a complex bug

## The Pattern

### 0. Search First

```bash
engram ask query "<task or feature name> review"
engram task show <UUID>
```

### 1. Store Review Context in Engram

Collect the diff by running git directly, then store it in engram — the reviewer will retrieve this, not receive it inline:

```bash
# Run these directly in your shell:
git diff HEAD~1..HEAD --stat
git diff HEAD~1..HEAD

engram context create \
  --title "Code review request: <task title>" \
  --content "Task: <task title>\nBase: <base SHA>\nHead: <head SHA>\n\nFiles changed:\n<git diff --stat output>\n\nDiff:\n<git diff output>\n\nRequirements from plan:\n<what this task was supposed to do>" \
  --source "code-review-request"
# REVIEW_CTX_UUID = ...

engram relationship create \
  --source-id <TASK_UUID> --source-type task \
  --target-id <REVIEW_CTX_UUID> --target-type context \
  --relationship-type relates_to --agent "<name>"
```

### 2. Create a Review Subtask

```bash
engram task create --title "Code Review: <task title>"
# REVIEW_TASK_UUID = ...
engram task update <REVIEW_TASK_UUID> --status in_progress

engram relationship create \
  --source-id <TASK_UUID> --source-type task \
  --target-id <REVIEW_TASK_UUID> --target-type task \
  --relationship-type depends_on --agent "<name>"

# Link the context to the review task so the reviewer finds it
engram relationship create \
  --source-id <REVIEW_TASK_UUID> --source-type task \
  --target-id <REVIEW_CTX_UUID> --target-type context \
  --relationship-type relates_to --agent "<name>"
```

### 3. Dispatch Reviewer — UUID Only

```bash
# Tell the reviewer subagent:
# "Your task UUID is <REVIEW_TASK_UUID>.
# Run: engram task show <REVIEW_TASK_UUID>
# Use engram-subagent-register to store your findings and report back."
```

Store a record that review was dispatched:

```bash
engram reasoning create \
  --title "Review dispatched: <task title>" \
  --task-id <TASK_UUID> \
  --content "Code review dispatched for: <task title>. Reviewer task: <REVIEW_TASK_UUID>. Context stored: <REVIEW_CTX_UUID>. Review focus: spec compliance, code quality, test coverage, security."
# DISPATCH_UUID = ...

engram relationship create \
  --source-id <TASK_UUID> --source-type task \
  --target-id <DISPATCH_UUID> --target-type reasoning \
  --relationship-type explains --agent "<name>"
```

### 4. Collect Review Results

When the reviewer marks their task done, retrieve what they stored:

```bash
engram relationship connected --entity-id <REVIEW_TASK_UUID> --max-depth 2
engram ask query "<task name> review results"
```

### 5. Act on Feedback

If issues found, record the remediation plan:

```bash
engram reasoning create \
  --title "Review feedback: <task title>" \
  --task-id <TASK_UUID> \
  --content "Review issues found:\n- Critical: <issue> → fix: <plan>\n- Important: <issue> → fix: <plan>\n- Minor: <issue> → noting for later\n\nDecision: <fix critical → re-review / fix important → continue / note minor → proceed>"
# FEEDBACK_UUID = ...

engram relationship create \
  --source-id <TASK_UUID> --source-type task \
  --target-id <FEEDBACK_UUID> --target-type reasoning \
  --relationship-type explains --agent "<name>"
```

If issues require fixes, apply them in your shell and request re-review.

### 6. Record Approval and Validate

When the review passes:

```bash
engram reasoning create \
  --title "Review approved: <task title>" \
  --task-id <TASK_UUID> \
  --content "Review APPROVED for: <task title>. Critical: 0. Important: 0. Minor: <N>. Reviewer: <subagent name>. Ready to proceed."
# APPROVAL_UUID = ...

engram relationship create \
  --source-id <TASK_UUID> --source-type task \
  --target-id <APPROVAL_UUID> --target-type reasoning \
  --relationship-type explains --agent "<name>"

# Validate system state before closing
engram validate check
```

### 7. Get Next Action

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
Task: "Implement login endpoint" (TASK_UUID = abc-001)

[Search first]
engram ask query "login endpoint code review"

[Collect diff — run directly in shell]
git diff HEAD~1..HEAD --stat
git diff HEAD~1..HEAD

[Store review context]
engram context create \
  --title "Code review request: login endpoint" \
  --content "Task: Implement login endpoint\nBase: a1b2c3d\nHead: e4f5g6h\nFiles: src/api/auth/login.rs, tests/api/auth/login_test.rs\nRequirement: POST /auth/login returns JWT on valid credentials, 401 on invalid" \
  --source "code-review-request"
# REVIEW_CTX_UUID = ctx-002

engram relationship create \
  --source-id abc-001 --source-type task \
  --target-id ctx-002 --target-type context \
  --relationship-type relates_to --agent "orchestrator"

[Create review task]
engram task create --title "Code Review: Login endpoint"
# REVIEW_TASK_UUID = abc-003
engram task update abc-003 --status in_progress

engram relationship create \
  --source-id abc-001 --source-type task \
  --target-id abc-003 --target-type task \
  --relationship-type depends_on --agent "orchestrator"

engram relationship create \
  --source-id abc-003 --source-type task \
  --target-id ctx-002 --target-type context \
  --relationship-type relates_to --agent "orchestrator"

[Record dispatch]
engram reasoning create \
  --title "Review dispatched: login endpoint" \
  --task-id abc-001 \
  --content "Review dispatched for login endpoint. Reviewer task: abc-003. Context: ctx-002."
# DISPATCH_UUID = rsn-004

engram relationship create \
  --source-id abc-001 --source-type task \
  --target-id rsn-004 --target-type reasoning \
  --relationship-type explains --agent "orchestrator"

[Dispatch reviewer — UUID only]
# "Your task UUID is abc-003. Use engram-subagent-register."

[Collect results]
engram relationship connected --entity-id abc-003 --max-depth 2

[Reviewer found 1 important issue: missing rate limiting]
engram reasoning create \
  --title "Review feedback: login endpoint" \
  --task-id abc-001 \
  --content "Review issues: Important: missing rate limiting on login endpoint → add Redis-based rate limiter before re-review. Minor: 2 style nits → will fix inline."
# FEEDBACK_UUID = rsn-005

engram relationship create \
  --source-id abc-001 --source-type task \
  --target-id rsn-005 --target-type reasoning \
  --relationship-type explains --agent "orchestrator"

[Fix and re-review — run tests directly in shell]
cargo test

[Re-review passes]
engram reasoning create \
  --title "Review approved: login endpoint" \
  --task-id abc-001 \
  --content "Review APPROVED. Critical: 0, Important: 0 (rate limiting added), Minor: 0. Ready to proceed."
# APPROVAL_UUID = rsn-006

engram relationship create \
  --source-id abc-001 --source-type task \
  --target-id rsn-006 --target-type reasoning \
  --relationship-type explains --agent "orchestrator"

[Validate]
engram validate check

engram next
```

## Review Checklist (Store in Context for Reviewer)

```
- Tests pass
- Code follows project patterns
- No hardcoded secrets
- Error handling complete
- Documentation updated
- No debug code left
- Edge cases handled
- Performance acceptable
- Security considerations met
```

## Related Skills

- `engram-subagent-register` — reviewer uses this to claim task and report findings
- `engram-audit-trail` — full traceability of review process
- `engram-test-driven-development` — review after tests pass
- `engram-subagent-driven-development` — reviews between agent tasks
