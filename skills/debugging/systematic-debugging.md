---
name: engram-systematic-debugging
description: "Engram-integrated version. Use when encountering bugs - stores investigation in engram reasoning chains for audit trail and pattern analysis."
---

# Systematic Debugging

## Overview

Random fixes waste time and create new bugs. Every debugging session must be stored in engram as a reasoning chain — queryable, reviewable, and useful to future agents facing similar issues.

## The Iron Law

```
NO FIXES WITHOUT ROOT CAUSE INVESTIGATION FIRST
```

If you have not completed Phase 1, you cannot propose a fix.

## The Four Phases

### Phase 0: Search First

Before investigating, check if this bug has been seen before:

```bash
engram ask query "<error message or bug description>"
engram ask query "<affected file or component> bug"
engram task show <UUID>
```

If a prior investigation exists, read it. Don't duplicate work.

Also check dependencies before touching anything:

```bash
engram relationship connected --entity-id <AFFECTED_ENTITY_UUID> --max-depth 2
```

### Phase 1: Root Cause Investigation

Anchor the task:

```bash
engram task create --title "Debug: <issue description>"
# TASK_UUID = ...
engram task update <TASK_UUID> --status in_progress
```

**Step 1: Read the error carefully**

Run the command that reproduces the error directly in your shell:

```bash
<command that reproduces the error>
```

Store the raw output immediately:

```bash
engram context create \
  --title "Error: <brief description>" \
  --content "Error type: <type>\nMessage: <full message>\nStack trace:\n<full stack trace>\nReproducible: YES/NO\nFrequency: <always/sometimes/rarely>" \
  --source "<file:line or command>"
# ERROR_CTX_UUID = ...

engram relationship create \
  --source-id <TASK_UUID> --source-type task \
  --target-id <ERROR_CTX_UUID> --target-type context \
  --relationship-type relates_to --agent "<name>"
```

**Step 2: Check recent changes**

Run git commands directly in your shell:

```bash
git log --oneline -10
git diff HEAD~3..HEAD -- <affected-file>
```

```bash
engram context create \
  --title "Recent changes: <affected area>" \
  --content "Recent commits:\n- <sha>: <message> — <relevance>\n- <sha>: <message> — <relevance>\n\nFiles changed in last 3 commits that touch affected area:\n- <file>: <what changed>" \
  --source "git-log"
# CHANGES_CTX_UUID = ...

engram relationship create \
  --source-id <TASK_UUID> --source-type task \
  --target-id <CHANGES_CTX_UUID> --target-type context \
  --relationship-type relates_to --agent "<name>"
```

**Step 3: Examine the code**

Read the affected file directly using your file-reading tools or shell.

```bash
engram context create \
  --title "Code observations: <affected file>" \
  --content "Code examined:\n- <file:line>: <what it does and what looks wrong>\n- <file:line>: <observation>\n\nInitial observations:\n- <observation 1>\n- <observation 2>" \
  --source "<file-path>"
# CODE_CTX_UUID = ...

engram relationship create \
  --source-id <TASK_UUID> --source-type task \
  --target-id <CODE_CTX_UUID> --target-type context \
  --relationship-type relates_to --agent "<name>"
```

### Phase 2: Pattern Analysis

Check for similar bugs before forming a hypothesis:

```bash
engram ask query "<error type> pattern similar bugs"
```

Store the analysis:

```bash
engram context create \
  --title "Pattern analysis: <error type>" \
  --content "Similar prior issues found:\n- <UUID>: <similarity> — root cause was <what>\n\nRecurring pattern: <yes/no — describe if yes>\nSystemic or isolated: <assessment and reason>" \
  --source "pattern-analysis"
# PATTERN_CTX_UUID = ...

engram relationship create \
  --source-id <TASK_UUID> --source-type task \
  --target-id <PATTERN_CTX_UUID> --target-type context \
  --relationship-type relates_to --agent "<name>"
```

### Phase 3: Hypothesis Testing

Form one hypothesis at a time. State it, test it, record the result.

```bash
engram reasoning create \
  --title "Hypothesis: <root cause statement>" \
  --task-id <TASK_UUID> \
  --content "Hypothesis: <root cause statement>\n\nSupporting evidence:\n- <evidence 1> → <UUID>\n- <evidence 2> → <UUID>\n\nTest: run <exact command>\nExpected if correct: <result>\nExpected if wrong: <result>"
# HYPO_UUID = ...

engram relationship create \
  --source-id <TASK_UUID> --source-type task \
  --target-id <HYPO_UUID> --target-type reasoning \
  --relationship-type explains --agent "<name>"
```

Run the test directly in your shell.

Store the result:

```bash
engram reasoning create \
  --title "Hypothesis result: <statement>" \
  --task-id <TASK_UUID> \
  --content "Hypothesis tested: <statement>\nOutput:\n<exact output>\nConclusion: VERIFIED / REFUTED\nNext: <if verified: proceed to fix / if refuted: next hypothesis>"
# RESULT_UUID = ...

engram relationship create \
  --source-id <TASK_UUID> --source-type task \
  --target-id <RESULT_UUID> --target-type reasoning \
  --relationship-type explains --agent "<name>"
```

If refuted, form the next hypothesis and repeat. If still stuck:

```bash
engram next
```

### Phase 4: Implementation

Only begin once root cause is confirmed.

Apply the fix and run tests directly in your shell.

Store the fix:

```bash
engram reasoning create \
  --title "Fix: <root cause summary>" \
  --task-id <TASK_UUID> \
  --content "Root cause confirmed: <final statement>\n\nFix applied:\n<description of change>\n\nFiles modified:\n- <file>: <what changed>\n\nRegression test added:\n<test name and what it covers>"
# FIX_UUID = ...

engram relationship create \
  --source-id <TASK_UUID> --source-type task \
  --target-id <FIX_UUID> --target-type reasoning \
  --relationship-type explains --agent "<name>"
```

Record test verification:

```bash
# Run full test suite directly in shell
cargo test   # or npm test, pytest, etc.

engram context create \
  --title "Verification: <fix description>" \
  --content "Verification:\nTests: <N>/<N> passed\nFailed: <none / list>\nRegression test: PASS\nNo new issues introduced: YES" \
  --source "test-runner"
# VERIFY_UUID = ...

engram relationship create \
  --source-id <TASK_UUID> --source-type task \
  --target-id <VERIFY_UUID> --target-type context \
  --relationship-type relates_to --agent "<name>"
```

### Phase 5: Validate and Close

```bash
engram validate check
engram task update <TASK_UUID> --status done
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
Issue: API returns 500 on user creation

[Search first]
engram ask query "API 500 user creation"
# Returns: prior NullPointerException pattern — read with engram task show

[Anchor]
engram task create --title "Debug: API 500 on user creation"
# TASK_UUID = task-001
engram task update task-001 --status in_progress

[Reproduce — run directly in shell]
curl -X POST /api/users -d '{...}'

[Store error]
engram context create \
  --title "Error: 500 on user creation" \
  --content "Error: 500 Internal Server Error\nType: NullPointerException\nAt: UserService.java:142\nMessage: user.getProfile() on null" \
  --source "api-logs"
# ERROR_CTX_UUID = ctx-002

engram relationship create \
  --source-id task-001 --source-type task \
  --target-id ctx-002 --target-type context \
  --relationship-type relates_to --agent "debugger"

[Pattern check]
engram ask query "NullPointerException profile null"
# Returns: ctx-089 (prior issue, same pattern)

engram context create \
  --title "Pattern: NullPointerException on profile" \
  --content "Prior NullPointerException issues: 2 found. Pattern: missing null check before property access on optional objects." \
  --source "pattern-analysis"
# PATTERN_UUID = ctx-003

engram relationship create \
  --source-id task-001 --source-type task \
  --target-id ctx-003 --target-type context \
  --relationship-type relates_to --agent "debugger"

[Hypothesis]
engram reasoning create \
  --title "Hypothesis: profile not loaded in create flow" \
  --task-id task-001 \
  --content "Hypothesis: profile not loaded in create flow before access.\nTest: grep for loadProfile in UserService.java\nExpected if correct: loadProfile not called in create path"
# HYPO_UUID = rsn-004

engram relationship create \
  --source-id task-001 --source-type task \
  --target-id rsn-004 --target-type reasoning \
  --relationship-type explains --agent "debugger"

[Run test — directly in shell]
grep -n 'getProfile\|loadProfile' src/UserService.java

engram reasoning create \
  --title "Hypothesis result: profile not loaded" \
  --task-id task-001 \
  --content "Hypothesis tested. loadProfile() not called in create flow, only in update flow. VERIFIED."
# RESULT_UUID = rsn-005

engram relationship create \
  --source-id task-001 --source-type task \
  --target-id rsn-005 --target-type reasoning \
  --relationship-type explains --agent "debugger"

[Fix and run tests — directly in shell]
cargo test user_creation

engram reasoning create \
  --title "Fix: add loadProfile() in create flow" \
  --task-id task-001 \
  --content "Fix: added loadProfile() call in UserService.create() before getProfile() access.\nFiles: src/UserService.java:138\nTest added: test_user_creation_with_null_profile_is_safe"
# FIX_UUID = rsn-006

engram relationship create \
  --source-id task-001 --source-type task \
  --target-id rsn-006 --target-type reasoning \
  --relationship-type explains --agent "debugger"

[Validate]
engram validate check
engram task update task-001 --status done
```

## Related Skills

- `engram-audit-trail` — traceability patterns
- `engram-test-driven-development` — write tests to reproduce bugs before fixing
- `engram-use-engram-memory` — memory patterns reference
