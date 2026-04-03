---
name: engram-workflow-guide
description: "Guide to creating and using engram state machine workflows: definitions with states/transitions/guards/actions, instances, CLI commands, and stage-based commit validation."
---

# Engram Workflow Guide

## Overview

An engram workflow is a **state machine definition** that governs how work progresses through defined phases. Workflows have states, transitions between states, guards that control entry/exit conditions, and actions that fire on state changes. When you start a workflow on a task, an **instance** is created that tracks the current state and records an audit trail of every transition.

This is not a conceptual "chain of skills." It is a persisted, queryable state machine with commit-time enforcement.

## Core Concepts

### Workflow Definition

A `Workflow` entity is the template. It defines:

| Component | What it does |
|-----------|-------------|
| **States** | Named phases in the lifecycle (e.g., `requirements`, `planning`, `development`, `review`, `done`) |
| **Transitions** | Allowed moves between states (e.g., `planning -> development`, `development -> review`) |
| **Guards** | Conditions that must be met to enter/leave a state (e.g., "must have reasoning entity") |
| **Actions** | Things that happen automatically on state entry (e.g., notifications, validation) |
| **Prompts** | System/user prompt templates that set agent behavior for each state |
| **Entity Types** | What entity types the workflow applies to (e.g., `"task"`) |
| **Permission Schemes** | Who can execute which transitions |

### Workflow Instance

A `WorkflowInstance` is a single execution run. It tracks:

- The workflow definition it belongs to
- The current state
- Context variables (bound entity, executing agent, permissions)
- Full execution history (audit trail of every state change)

### Tasks and Workflows

Tasks can be bound to workflows via `workflow_id` and `workflow_state` fields. The `WorkflowValidator` reads the task's current workflow state and enforces stage-based commit policies (e.g., "no code commits allowed in the requirements stage").

## Creating a Workflow Definition

### Step 1: Create the workflow

```bash
engram workflow create \
  --title "SDLC" \
  --description "Standard software development lifecycle" \
  --entity-types "task" \
  --agent "claude"
```

Returns the workflow UUID. The workflow starts in `Draft` status.

### Step 2: Add states

Each state represents a phase. State types: `start`, `in_progress`, `review`, `done`, `blocked`.

```bash
WORKFLOW_ID="<workflow-uuid>"

engram workflow add-state "$WORKFLOW_ID" \
  --name "requirements" \
  --state-type start \
  --description "Gather and document requirements" \
  --is-final

engram workflow add-state "$WORKFLOW_ID" \
  --name "planning" \
  --state-type in_progress \
  --description "Design solution and create implementation plan"

engram workflow add-state "$WORKFLOW_ID" \
  --name "development" \
  --state-type in_progress \
  --description "Write code with TDD"

engram workflow add-state "$WORKFLOW_ID" \
  --name "review" \
  --state-type review \
  --description "Code review and compliance check"

engram workflow add-state "$WORKFLOW_ID" \
  --name "done" \
  --state-type done \
  --description "Complete" \
  --is-final
```

### Step 3: Add transitions

Transitions define allowed state changes. Transition types: `automatic`, `manual`, `conditional`, `scheduled`.

```bash
engram workflow add-transition "$WORKFLOW_ID" \
  --name "begin_planning" \
  --from-state "requirements" \
  --to-state "planning" \
  --transition-type manual \
  --description "Requirements approved, begin planning"

engram workflow add-transition "$WORKFLOW_ID" \
  --name "begin_development" \
  --from-state "planning" \
  --to-state "development" \
  --transition-type manual \
  --description "Plan approved, begin development"

engram workflow add-transition "$WORKFLOW_ID" \
  --name "submit_for_review" \
  --from-state "development" \
  --to-state "review" \
  --transition-type manual \
  --description "Development complete, submit for review"

engram workflow add-transition "$WORKFLOW_ID" \
  --name "approve" \
  --from-state "review" \
  --to-state "done" \
  --transition-type manual \
  --description "Review passed"

engram workflow add-transition "$WORKFLOW_ID" \
  --name "request_changes" \
  --from-state "review" \
  --to-state "development" \
  --transition-type manual \
  --description "Review feedback, return to development"
```

### Step 4: Activate

```bash
engram workflow activate "$WORKFLOW_ID"
```

### Step 5: Verify

```bash
engram workflow get "$WORKFLOW_ID"
engram workflow query-actions "$WORKFLOW_ID"
```

## Using a Workflow

### Start an instance on a task

```bash
engram workflow start "$WORKFLOW_ID" \
  --agent "claude" \
  --entity-id "$TASK_ID" \
  --entity-type "task"
```

Returns the instance UUID and initial state.

### Transition through states

```bash
INSTANCE_ID="<instance-uuid>"

engram workflow transition "$INSTANCE_ID" \
  --transition "begin_planning" \
  --agent "claude"
```

### Check status and history

```bash
engram workflow status "$INSTANCE_ID"
```

### List instances

```bash
engram workflow instances --workflow-id "$WORKFLOW_ID" --running-only
engram workflow instances --agent "claude"
```

### Cancel an instance

```bash
engram workflow cancel "$INSTANCE_ID" --agent "claude" --reason "no longer needed"
```

## Stage-Based Commit Validation

When a task is bound to a workflow, the `WorkflowValidator` enforces commit policies based on the current workflow state. These are the built-in stage policies:

| Stage | Code Commits | Engram Only | Tests Pass | Build Pass | Quality Gates |
|-------|-------------|-------------|------------|------------|---------------|
| `requirements` | No | Yes | No | No | requirements_validation, must_reference_context |
| `planning` | No | Yes | No | No | planning_validation, must_have_reasoning |
| `bdd_red` | Yes | No | No | No | tests_only, tests_must_fail |
| `bdd_green` | Yes | No | Yes | Yes | minimal_implementation |
| `bdd_refactor` | Yes | No | Yes | Yes | no_new_tests, maintain_coverage |
| `development` | Yes | No | Yes | Yes | code_quality, test_coverage |
| `integration` | Yes | No | Yes | Yes | full_test_suite, integration_tests |
| `default` | Yes | No | No | No | (none) |

This means: if your task is in the `requirements` workflow state, commits containing code changes will be rejected. Only engram entity changes are allowed. This enforces process discipline.

## Full Example: SDLC Workflow

```bash
# 1. Create the task
engram task create --title "Add user authentication" --priority high --output json
# TASK_ID = task-001

# 2. Create the workflow definition
engram workflow create \
  --title "SDLC" \
  --description "Standard software development lifecycle" \
  --entity-types "task" \
  --agent "claude"
# WORKFLOW_ID = wf-002

# 3. Add states
engram workflow add-state wf-002 --name requirements --state-type start --description "Gather requirements" --is-final
engram workflow add-state wf-002 --name planning --state-type in_progress --description "Design and plan"
engram workflow add-state wf-002 --name development --state-type in_progress --description "TDD implementation"
engram workflow add-state wf-002 --name review --state-type review --description "Code review"
engram workflow add-state wf-002 --name done --state-type done --description "Complete" --is-final

# 4. Add transitions
engram workflow add-transition wf-002 --name begin_planning --from-state requirements --to-state planning --transition-type manual --description "Start planning"
engram workflow add-transition wf-002 --name begin_development --from-state planning --to-state development --transition-type manual --description "Start development"
engram workflow add-transition wf-002 --name submit_review --from-state development --to-state review --transition-type manual --description "Submit for review"
engram workflow add-transition wf-002 --name approve --from-state review --to-state done --transition-type manual --description "Approved"
engram workflow add-transition wf-002 --name reject --from-state review --to-state development --transition-type manual --description "Needs changes"

# 5. Activate
engram workflow activate wf-002

# 6. Start an instance on the task
engram workflow start wf-002 --agent "claude" --entity-id task-001 --entity-type "task"
# INSTANCE_ID = inst-003

# 7. Work through the states
# Requirements phase — only engram entity commits allowed
engram context create --title "Auth requirements" --content "JWT tokens, refresh rotation, password hashing with bcrypt" --source "requirements"
engram workflow transition inst-003 --transition begin_planning --agent "claude"

# Planning phase — only engram entity commits allowed
engram reasoning create --title "Auth design decisions" --task-id task-001 --content "JWT chosen over sessions: stateless, scales horizontally"
engram workflow transition inst-003 --transition begin_development --agent "claude"

# Development phase — code commits allowed, tests and build must pass
# ... write code ...
engram workflow transition inst-003 --transition submit_review --agent "claude"

# Review phase
# ... review passes ...
engram workflow transition inst-003 --transition approve --agent "claude"

# 8. Check final status
engram workflow status inst-003
```

## Inspecting Workflows

```bash
engram workflow list --status active
engram workflow list --search "SDLC"
engram workflow get "$WORKFLOW_ID"
engram workflow query-actions "$WORKFLOW_ID" --state-id "review"
engram workflow instances --running-only
```

## Transition Types

| Type | When to Use |
|------|------------|
| `manual` | Agent or human must explicitly trigger the transition |
| `automatic` | Fires as soon as the state is entered (no approval needed) |
| `conditional` | Fires when a condition evaluates to true (checked automatically) |
| `scheduled` | Fires on a schedule (e.g., timeout reminders) |

## State Types

| Type | Purpose |
|------|---------|
| `start` | Initial state of the workflow |
| `in_progress` | Active work phase |
| `review` | Awaiting approval or review |
| `done` | Terminal state |
| `blocked` | Waiting on external dependency |

## Workflow Actions

Transitions can trigger actions:

```bash
engram workflow execute-action \
  --action-type external_command \
  --command "cargo test" \
  --working-directory "/path/to/project"

engram workflow execute-action \
  --action-type notification \
  --message "Code review requested for task TASK_ID"

engram workflow execute-action \
  --action-type update_entity \
  --entity-id "$TASK_ID" \
  --entity-type "task"
```

## Related Skills

- `engram-orchestrator` -- coordination loop that can drive workflow transitions
- `engram-use-engram-memory` -- command reference for storing and retrieving entities
- `engram-audit-trail` -- traceability patterns for workflow auditing
- `engram-test-driven-development` -- TDD workflow that integrates with stage-based validation
- `engram-brainstorming` -- ideation before workflow creation
