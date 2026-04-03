---
name: engram-writing-plans
description: "Engram-integrated version. Use when you have a spec or requirements for a multi-step task, before touching code - creates engram task hierarchy instead of markdown files."
---

# Writing Plans

## Overview

Write comprehensive implementation plans before touching code. Store the plan as an engram task hierarchy — not a markdown file — so any agent can retrieve it with `engram task show <UUID>`.

## When to Use

Use this skill when:
- You have a validated design or spec ready for implementation
- You need to break work into subtasks before delegating
- A task is too large to do in one step

## The Pattern

### 0. Search First

Before creating any plan, check for existing plans or prior context:

```bash
engram ask query "<feature name> implementation plan"
engram task show <UUID>
```

### 1. Create the Parent Task

```bash
engram task create --title "<Feature Name> Implementation Plan"
# PARENT_UUID = ...
engram task update <PARENT_UUID> --status in_progress

# Store the plan overview as context linked to the parent
engram context create \
  --title "Plan overview: <feature name>" \
  --content "Goal: <one sentence>\nArchitecture: <2-3 sentences>\nTech stack: <key technologies>\nSubtasks: <N> steps" \
  --source "implementation-plan"
# OVERVIEW_UUID = ...

engram relationship create \
  --source-id <PARENT_UUID> --source-type task \
  --target-id <OVERVIEW_UUID> --target-type context \
  --relationship-type relates_to --agent "<name>"
```

### 2. Create Bite-Sized Subtasks

Each subtask is one action — 2 to 5 minutes of work:

```bash
engram task create --title "<Feature> Task <N>: <Step description>"
# SUBTASK_UUID = ...

engram relationship create \
  --source-id <PARENT_UUID> --source-type task \
  --target-id <SUBTASK_UUID> --target-type task \
  --relationship-type depends_on --agent "<name>"
```

Good granularity examples:
- "Write the failing test"
- "Run it to make sure it fails"
- "Write minimal code to make the test pass"
- "Run the tests and verify they pass"
- "Commit"

### 3. Store Step Details

For each subtask, store complete instructions as context — exact file paths, full code, exact commands with expected output:

```bash
engram context create \
  --title "Step detail: <subtask title>" \
  --content "Files:\n- Create: exact/path/to/file.rs\n- Modify: exact/path/to/existing.rs:123-145\n\nStep 1: Write the failing test\n\`\`\`rust\n[full test code]\n\`\`\`\n\nStep 2: Run test, verify it fails\nRun: cargo test test_name -- --nocapture\nExpected: FAIL with 'function not found'\n\nStep 3: Write minimal implementation\n\`\`\`rust\n[minimal code]\n\`\`\`\n\nStep 4: Run test, verify it passes\nExpected: PASS\n\nStep 5: Commit\ngit add <files> && git commit -m 'feat: <description>'" \
  --source "step-detail"
# DETAIL_UUID = ...

engram relationship create \
  --source-id <SUBTASK_UUID> --source-type task \
  --target-id <DETAIL_UUID> --target-type context \
  --relationship-type relates_to --agent "<name>"
```

### 4. Verify the Hierarchy

```bash
engram relationship connected --entity-id <PARENT_UUID> --max-depth 2
```

### 5. Offer Execution Choice

After storing the plan, present two options:

**"Plan stored in engram. Two execution paths:**

**1. Subagent-driven (this session)** — I dispatch one fresh subagent per task with review between each. Use `engram-subagent-driven-development`.**

**2. Parallel sessions** — Open a new session per domain, each agent queries: `engram task show <SUBTASK_UUID>`. Use `engram-dispatching-parallel-agents`."**

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

## Rules

- Exact file paths — never relative or vague
- Full code in plan — not "add validation here"
- Exact commands with expected output
- DRY, YAGNI, TDD, frequent commits
- Everything in engram — nothing in markdown files

## Example

```
[Search first]
engram ask query "user authentication implementation"

[Parent task]
engram task create --title "Implement User Authentication API"
# PARENT_UUID = abc-001
engram task update abc-001 --status in_progress

[Overview]
engram context create \
  --title "Plan overview: User Auth API" \
  --content "Goal: Add JWT auth endpoints\nArchitecture: Stateless JWT + refresh rotation\nTech: Rust, axum, jsonwebtoken\nSubtasks: 5 steps" \
  --source "implementation-plan"
# OVERVIEW_UUID = ctx-002

engram relationship create \
  --source-id abc-001 --source-type task \
  --target-id ctx-002 --target-type context \
  --relationship-type relates_to --agent "planner"

[Subtask 1]
engram task create --title "Auth Task 1: Write failing test for login endpoint"
# TASK1_UUID = abc-003

engram relationship create \
  --source-id abc-001 --source-type task \
  --target-id abc-003 --target-type task \
  --relationship-type depends_on --agent "planner"

[Step details for subtask 1]
engram context create \
  --title "Step detail: Write failing login test" \
  --content "Files:\n- Create: tests/api/auth/login_test.rs\n\nStep 1: Write failing test\n\`\`\`rust\n#[tokio::test]\nasync fn login_rejects_invalid_credentials() {\n    let resp = post(\"/api/auth/login\", json!({\"email\":\"x\",\"password\":\"y\"})).await;\n    assert_eq!(resp.status(), 401);\n}\n\`\`\`\n\nStep 2: Run to verify failure\ncargo test login_rejects -- --nocapture\nExpected: FAIL 'cannot find function post'\n\nStep 3: Commit stub\ngit add tests/api/auth/login_test.rs && git commit -m 'test: add failing login test'" \
  --source "step-detail"
# DETAIL1_UUID = ctx-004

engram relationship create \
  --source-id abc-003 --source-type task \
  --target-id ctx-004 --target-type context \
  --relationship-type relates_to --agent "planner"

# ... repeat for subtasks 2-5 ...

[Verify hierarchy]
engram relationship connected --entity-id abc-001 --max-depth 2
# Shows: abc-001 → abc-003, abc-005, abc-007, abc-009, abc-011

[Validate]
engram validate check
engram next
```

## Subagents Retrieve Steps Via

```bash
engram task show <SUBTASK_UUID>
engram relationship connected --entity-id <SUBTASK_UUID> --max-depth 1
```

## Related Skills

- `engram-brainstorming` — design the feature before writing this plan
- `engram-subagent-driven-development` — execute this plan with agent-per-subtask
- `engram-dispatching-parallel-agents` — execute independent subtasks in parallel
- `engram-plan-feature` — pipeline-based planning for structured feature types
