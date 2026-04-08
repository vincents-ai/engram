---
name: engram-commit-convention
description: "Engram commit convention: every commit must reference a valid engram task UUID. Covers format, hook setup, rejection handling, and why --no-verify is prohibited."
---

# Engram Commit Convention

## Overview

Every git commit in an engram-managed project must reference a valid engram task UUID. This is the structural enforcement of the north star: every code change is traceable to a decision, intent, and context stored in engram.

The pre-commit hook enforces this automatically once installed. It is not a convention — it is a hard gate.

## Hook Setup

Install the pre-commit hook once per workspace:

```bash
engram validate hook install

# Verify it is active
engram validate hook status

# Check overall validation setup
engram validate check
```

**Rule:** Never uninstall the hook or use `--no-verify`. A commit that bypasses the hook is untraceable — it violates the north star (ADR-018).

## Commit Format

```
<type>: <title> [<ENGRAM_TASK_UUID>]
```

The UUID must be the full UUID of an existing engram task. Short UUIDs are not accepted.

**Valid examples:**

```
feat: add rate limiting middleware [3cc69e16-84da-47d2-8b3e-f625572d6cb4]
fix: normalise UTC timestamps in token validation [abc12345-0000-0000-0000-000000000001]
chore: update dependencies [abc12345-0000-0000-0000-000000000002]
docs: update AGENTS.md with north star [abc12345-0000-0000-0000-000000000003]
```

**Type prefixes follow conventional commits:**

| Type | When to use |
|---|---|
| `feat` | New feature or capability |
| `fix` | Bug fix |
| `chore` | Maintenance, deps, tooling |
| `docs` | Documentation only |
| `test` | Tests only |
| `refactor` | Code change with no behaviour change |
| `perf` | Performance improvement |

## Before You Commit

The hook validates that:
1. The commit message contains a UUID in `[UUID]` format
2. The UUID references a real, existing engram task
3. The task has at least one context relationship
4. The task has at least one reasoning relationship

This means you must have the task, its context, and its reasoning stored in engram **before** committing. Do this as you work — not as a last step before commit.

```bash
# Dry-run validation before committing
engram validate commit --message "feat: my change [<UUID>]" --dry-run
```

## When the Hook Rejects a Commit

If the hook rejects your commit, it will tell you exactly what is missing:

```
❌ Validation failed
  • Task must have a reasoning relationship
    💡 Create a reasoning entity linked to this task
  • Task must have a context relationship
    💡 Create a context entity linked to this task
```

Fix the missing relationships, then retry the commit:

```bash
# Missing context — create one and link it
engram context create \
  --title "<short title for what this commit does>" \
  --content "<what changed and why>" \
  --source "<file path or component>"
# CTX_UUID = ...

engram relationship create \
  --source-id <TASK_UUID> --source-type task \
  --target-id <CTX_UUID> --target-type context \
  --relationship-type relates_to --agent "<your-name>"

# Missing reasoning — create one and link it
engram reasoning create \
  --title "<why this change was made>" \
  --task-id <TASK_UUID> \
  --content "<the rationale — what problem this solves and why this approach>"
# RSN_UUID = ...

engram relationship create \
  --source-id <TASK_UUID> --source-type task \
  --target-id <RSN_UUID> --target-type reasoning \
  --relationship-type explains --agent "<your-name>"

# Retry commit
git commit -m "feat: my change [<TASK_UUID>]"
```

## Why --no-verify Is Prohibited

`git commit --no-verify` skips the pre-commit hook entirely. A commit made this way:

- Has no traceable task in engram
- Has no context or reasoning linked to it
- Cannot be picked up by the next agent with full fidelity
- Directly violates ADR-018 (north star: full-fidelity handoff)

If you are blocked from committing for a legitimate reason (e.g. no network access to validate, emergency hotfix), create a task and minimal reasoning record first — even a stub is better than nothing. Then commit normally through the hook.

## Full Example

```
Goal: fix a bug where token validation fails for UTC+0 expiry times

[Step 1: Create or find the task]
engram task create --title "Fix: token validation UTC+0 expiry" --priority high
# TASK_UUID = abc-001
engram task update abc-001 --status in_progress

[Step 2: Do the work — fix the bug in src/auth/validator.rs]

[Step 3: Store context immediately as you work]
engram context create \
  --title "Bug: UTC+0 expiry fails token validation" \
  --content "Token validation compares naive datetime against UTC-aware expiry. Fails when timezone offset is zero — treated as missing timezone rather than UTC." \
  --source "src/auth/validator.rs"
# CTX_UUID = ctx-002

engram relationship create \
  --source-id abc-001 --source-type task \
  --target-id ctx-002 --target-type context \
  --relationship-type relates_to --agent "implementer"

[Step 4: Store reasoning]
engram reasoning create \
  --title "Fix: normalize to UTC before comparison" \
  --task-id abc-001 \
  --content "Root cause: naive datetime compared against UTC-aware expiry. Fix: call .to_utc() on both sides before comparison. All expiry times should be stored and compared as UTC."
# RSN_UUID = rsn-003

engram relationship create \
  --source-id abc-001 --source-type task \
  --target-id rsn-003 --target-type reasoning \
  --relationship-type explains --agent "implementer"

[Step 5: Dry-run validate before committing]
engram validate commit \
  --message "fix: normalise UTC timestamps in token validation [abc-001]" \
  --dry-run
# ✅ Validation passed

[Step 6: Commit]
git add src/auth/validator.rs
git commit -m "fix: normalise UTC timestamps in token validation [abc-001]"
# ✅ Pre-commit validation passed
# ✅ Commit validation passed

[Step 7: Close the task]
engram task update abc-001 --status done --outcome "UTC+0 token validation fixed"
```

## Related Skills

- `engram-use-engram-memory` — creating context, reasoning, and task entities before committing
- `engram-session-end` — closing tasks and validating state at end of session
- `engram-subagent-register` — subagent pattern that keeps context and reasoning in sync as work progresses
