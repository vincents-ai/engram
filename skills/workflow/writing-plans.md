---
name: engram-writing-plans
description: "Engram-integrated version. Use when you have a spec or requirements for a multi-step task, before touching code - creates engram task hierarchy instead of markdown files."
---

# Writing Plans (Engram-Integrated)

## Overview

Write comprehensive implementation plans assuming the engineer has zero context, storing the plan as engram task hierarchy with subtasks instead of markdown files.

## Key Changes from Original

**Original:** Saves plan to `docs/plans/YYYY-MM-DD-<feature-name>.md`
**Engram-integrated:** Creates parent engram task with bite-sized subtasks linked via `contains` relationships.

## Plan Document Header (Engram Task)

**Create parent task with header content as description:**

```bash
engram task create \
  --title "[Feature Name] Implementation Plan" \
  --description "**Goal:** [One sentence describing what this builds]\n\n**Architecture:** [2-3 sentences about approach]\n\n**Tech Stack:** [Key technologies/libraries]\n\n**For:** Engram task-based plan - see subtasks for implementation steps" \
  --priority [high/medium/low] \
  --agent [AGENT]
```

## Bite-Sized Task Granularity

**Each step is one action (2-5 minutes):**
- "Write the failing test" - step (subtask)
- "Run it to make sure it fails" - step (subtask)
- "Implement the minimal code to make the test pass" - step (subtask)
- "Run the tests and make sure they pass" - step (subtask)
- "Commit" - step (subtask)

## Task Structure (Engram Subtasks)

**Create subtask for each step:**

```bash
engram task create \
  --title "[Feature] Task [N]: [Step Description]" \
  --description "**Files:**\n- Create: \`exact/path/to/file.py\`\n- Modify: \`exact/path/to/existing.py:123-145\`\n- Test: \`tests/exact/path/to/test.py\`\n\n**Step 1: Write the failing test**\n\n\`\`\`python\ndef test_specific_behavior():\n    result = function(input)\n    assert result == expected\n\`\`\`\n\n**Step 2: Run test to verify it fails**\n\nRun: \`pytest tests/path/test.py::test_name -v\`\nExpected: FAIL with \"function not defined\"\n\n**Step 3: Write minimal implementation**\n\n\`\`\`python\ndef function(input):\n    return expected\n\`\`\`\n\n**Step 4: Run test to verify it passes**\n\nRun: \`pytest tests/path/test.py::test_name -v\`\nExpected: PASS\n\n**Step 5: Commit**\n\n\`\`\`bash\ngit add tests/path/test.py src/path/file.py\ngit commit -m \"feat: add specific feature\"\n\`\`\`" \
  --parent [PARENT_TASK_ID] \
  --priority [high/medium/low] \
  --agent [AGENT]
```

## Creating Task Hierarchy

### Step 1: Create Parent Task

```bash
# Create parent task
PARENT_TASK=$(engram task create \
  --title "Implement [Feature Name]" \
  --description "**Goal:** [One sentence]\n\n**Architecture:** [2-3 sentences]\n\n**Tech Stack:** [Key technologies]" \
  --priority high \
  --agent default \
  --json | jq -r '.id')
```

### Step 2: Create Subtasks

```bash
# Create subtask 1
SUBTASK1=$(engram task create \
  --title "Task 1: Write failing test" \
  --description "[Test code and verification steps]" \
  --parent $PARENT_TASK \
  --priority high \
  --agent default \
  --json | jq -r '.id')

# Create subtask 2
SUBTASK2=$(engram task create \
  --title "Task 2: Run test to verify failure" \
  --description "Run: \`pytest tests/... -v\`\nExpected: FAIL" \
  --parent $PARENT_TASK \
  --priority high \
  --agent default \
  --json | jq -r '.id')

# ... continue for all steps
```

### Step 3: Create Step Details as Reasoning Entities

```bash
# Store detailed step instructions as reasoning
engram reasoning create \
  --title "[Feature] Task 1: Detailed Instructions" \
  --task-id $SUBTASK1 \
  --content "**Test to write:**\n\`\`\`python\n[test code]\n\`\`\`\n\n**Expected failure:**\n\`FunctionNotFoundError: function not defined\`\n\n**Files to modify:**\n- \`src/module.py\` (add function stub)" \
  --confidence 1.0 \
  --tags "implementation-plan,[feature-name]"
```

### Step 4: Verify Hierarchy

```bash
# Check parent task has subtasks
engram relationship connected --entity-id $PARENT_TASK --relationship-type contains
```

## Remember

- Exact file paths always
- Complete code in plan (not "add validation")
- Exact commands with expected output
- Reference relevant skills with @ syntax
- DRY, YAGNI, TDD, frequent commits
- All in engram, nothing in markdown

## Execution Handoff

After creating the engram task hierarchy, offer execution choice:

**"Plan complete and stored in Engram. Two execution options:**

**1. Subagent-Driven (this session)** - I dispatch fresh subagent per task, review between tasks, fast iteration

**2. Parallel Session (separate)** - Open new session with executing-plans, batch execution with checkpoints

**Which approach?"**

**If Subagent-Driven chosen:**
- **REQUIRED SUB-SKILL:** Use superpowers:subagent-driven-development
- Stay in this session
- Fresh subagent per task + code review
- Subagents query engram for task details: `engram task show [SUBTASK_ID]`

**If Parallel Session chosen:**
- Guide them to open new session in worktree
- **REQUIRED SUB-SKILL:** New session uses superpowers:executing-plans
- Subagents query engram: `engram task list --parent [PARENT_TASK_ID]`

## Example Workflow

```bash
# Create parent task
PARENT=$(engram task create \
  --title "Implement User Authentication API" \
  --description "**Goal:** Add JWT-based authentication endpoints\n\n**Architecture:** Stateless JWT with refresh tokens\n\n**Tech Stack:** Rust, actix-web, jsonwebtoken" \
  --priority high \
  --agent default \
  --json | jq -r '.id')

# Create subtasks
TASK1=$(engram task create \
  --title "Auth API Task 1: Write failing test for login endpoint" \
  --description "**Files:**\n- Create: src/api/auth/login.rs\n- Test: tests/api/auth/login_test.rs\n\n..." \
  --parent $PARENT \
  --priority high \
  --agent default \
  --json | jq -r '.id')

# Create reasoning with step details
engram reasoning create \
  --title "Auth API Task 1: Test Details" \
  --task-id $TASK1 \
  --content "**Test code:**\n\`\`\`rust\n#[test]\nasync fn login_rejects_invalid_credentials() {\n    let response = make_request(\n        Method::POST,\n        \"/api/auth/login\",\n        json!({ \"email\": \"invalid\", \"password\": \"test\" })\n    ).await;\n    \n    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);\n}\n\`\`\`\n\n**Expected failure:**\n\`Compile error: make_request not found\`" \
  --confidence 1.0

# Verify hierarchy
engram relationship connected --entity-id $PARENT --relationship-type contains
# Output: TASK1, TASK2, TASK3, ...

Agent: "Plan complete. 5 subtasks created in Engram. Ready to execute?"
```

## Querying the Plan

After creating the plan, agents can retrieve the full implementation:

```bash
# Get all subtasks
engram task list --parent [PARENT_TASK_ID]

# Get task details
engram task show [SUBTASK_ID]

# Get step details (reasoning)
engram reasoning get --task-id [SUBTASK_ID]

# Get all plan-related entities
engram relationship connected --entity-id [PARENT_TASK_ID]
```

## Related Skills

This skill integrates with:
- `engram-plan-feature` - More structured planning using pipeline templates
- `engram-brainstorming` - Design features before planning implementation
- `engram-use-memory` - Store plans for future reference
- `engram-delegate-to-agents` - Delegate plan execution to agents
- `engram-audit-trail` - Track plan execution progress
