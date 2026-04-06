---
name: engram-workflow-execution
description: "Execute and manage engram state machine workflow instances: create, advance, monitor, and complete a workflow after it has been defined."
---

# Engram Workflow Execution

## Overview

This skill teaches agents how to **run** an engram state machine workflow — after the definition exists. It covers creating an instance from a definition, advancing it through states, checking current state and available transitions, firing guards and actions, linking instances to tasks, and enforcing stage-based commit policies. It is the operational companion to `engram-workflow-guide`, which covers *building* definitions.

**Key distinction:**
- `engram-workflow-guide` — define states, transitions, guards, and activate the definition
- `engram-workflow-execution` — start an instance, advance it, monitor it, complete it

## When to Use

- A workflow definition already exists and you need to run it on a task
- You need to advance a workflow instance to the next state
- You want to check what state a task is currently in and what transitions are available
- You are enforcing stage-based commit policies (e.g., no code commits in `requirements` state)
- You need to cancel or inspect a running workflow instance
- You want to fire actions (external commands, notifications, entity updates) from a workflow step

## The Pattern

### 0. Search First

Before starting a new workflow instance, check whether one is already running:

```bash
# Search for existing context
engram ask query "workflow instance <task-title>"

# List all running instances
engram workflow instances --running-only

# Filter by a specific workflow definition
engram workflow instances --workflow-id "<WORKFLOW_ID>" --running-only

# Filter by agent
engram workflow instances --agent "<agent-name>"
```

Never start a duplicate instance on the same task.

### 1. Find the Workflow Definition

```bash
# List all active workflow definitions
engram workflow list --status active

# Search by name
engram workflow list --search "SDLC"

# Inspect a specific definition (shows states and transitions)
engram workflow get "<WORKFLOW_ID>"

# Query available actions, guards, and checks
engram workflow query-actions "<WORKFLOW_ID>"

# Query actions for a specific state
engram workflow query-actions "<WORKFLOW_ID>" --state-id "<STATE_NAME>"
```

Note down the `WORKFLOW_ID` and the names of all transitions — you will need exact transition names when calling `workflow transition`.

### 2. Start a Workflow Instance

An **instance** is a single execution run of a workflow definition. Every instance tracks current state, executing agent, bound entity, and the full history of every state change.

```bash
# Start an instance bound to a task
engram workflow start "<WORKFLOW_ID>" \
  --agent "<agent-name>" \
  --entity-id "<TASK_ID>" \
  --entity-type "task"
# Returns: INSTANCE_ID and initial state
```

To pass initial context variables into the instance:

```bash
engram workflow start "<WORKFLOW_ID>" \
  --agent "<agent-name>" \
  --entity-id "<TASK_ID>" \
  --entity-type "task" \
  --variables "environment=staging,version=1.2.0"
```

To pass a JSON context file:

```bash
# context.json: {"environment": "staging", "version": "1.2.0"}
engram workflow start "<WORKFLOW_ID>" \
  --agent "<agent-name>" \
  --entity-id "<TASK_ID>" \
  --entity-type "task" \
  --context-file context.json
```

Save the returned `INSTANCE_ID` — it is required for all subsequent operations.

### 3. Check Current State

```bash
# Get full status: current state, history, bound entity
engram workflow status "<INSTANCE_ID>"
```

The output shows:
- Current state name and type
- The entity bound to the instance
- Execution history (every past transition with timestamp and agent)

### 4. List Available Transitions

The transitions available from the current state are determined by the workflow definition. To see all transitions defined on the workflow (including which states they connect):

```bash
engram workflow get "<WORKFLOW_ID>"
```

To see actions and guards configured per state:

```bash
engram workflow query-actions "<WORKFLOW_ID>" --state-id "<CURRENT_STATE_NAME>"
```

Transitions have types:

| Type | Behaviour |
|------|-----------|
| `manual` | You must call `workflow transition` explicitly |
| `automatic` | Fires immediately on entering the from-state |
| `conditional` | Fires when its guard condition evaluates to true |
| `scheduled` | Fires on a time schedule |

### 5. Advance the Instance (Transition)

Execute a named transition to move the instance to its next state:

```bash
engram workflow transition "<INSTANCE_ID>" \
  --transition "<transition-name>" \
  --agent "<agent-name>"
```

To pass additional context when transitioning:

```bash
engram workflow transition "<INSTANCE_ID>" \
  --transition "<transition-name>" \
  --agent "<agent-name>" \
  --context-file context.json
```

The transition name must exactly match a transition defined in the workflow definition (`--name` value from `workflow add-transition`). Call `engram workflow get` if you are unsure of the exact name.

Repeat `workflow transition` for each state change until the instance reaches a final state.

### 6. Guards and Actions

**Guards** are conditions that must pass for a transition to fire. They are evaluated by the workflow engine when `workflow transition` is called. If a guard fails, the transition is rejected. Guards are configured on the workflow definition — use `query-actions` to inspect them.

**Actions** fire on state entry or exit. The workflow engine triggers them automatically, but you can also fire them manually:

```bash
# Run an external command (e.g., a test suite)
engram workflow execute-action \
  --action-type external_command \
  --command "cargo test" \
  --args "--all-features" \
  --working-directory "/path/to/project" \
  --timeout-seconds 120

# Send a notification
engram workflow execute-action \
  --action-type notification \
  --message "Review requested for task <TASK_ID>"

# Update a linked entity
engram workflow execute-action \
  --action-type update_entity \
  --entity-id "<TASK_ID>" \
  --entity-type "task"
```

Action types: `external_command`, `notification`, `update_entity`.

### 7. Cancel an Instance

If the workflow must be abandoned:

```bash
engram workflow cancel "<INSTANCE_ID>" \
  --agent "<agent-name>" \
  --reason "Task scope changed — workflow no longer relevant"
```

### 8. Stage-Based Commit Validation

When a task is bound to a workflow instance, the `WorkflowValidator` reads the task's current workflow state and enforces commit policies. The built-in stage policies are:

| Workflow State | Code Commits | Engram Only | Tests Pass | Build Pass | Quality Gates |
|----------------|-------------|-------------|------------|------------|---------------|
| `requirements` | No | Yes | No | No | requirements_validation, must_reference_context |
| `planning` | No | Yes | No | No | planning_validation, must_have_reasoning |
| `bdd_red` | Yes | No | No | No | tests_only, tests_must_fail |
| `bdd_green` | Yes | No | Yes | Yes | minimal_implementation |
| `bdd_refactor` | Yes | No | Yes | Yes | no_new_tests, maintain_coverage |
| `development` | Yes | No | Yes | Yes | code_quality, test_coverage |
| `integration` | Yes | No | Yes | Yes | full_test_suite, integration_tests |
| `default` | Yes | No | No | No | (none) |

**Practical implication:** If your task's workflow instance is in the `requirements` state, a `git commit` containing code changes will be rejected at the pre-commit hook. Only engram entity changes (context, reasoning, ADRs) are allowed. Transition to `development` before writing code.

### 9. Inspect and Audit

```bash
# All running instances (across all workflows)
engram workflow instances --running-only

# All instances for a specific workflow
engram workflow instances --workflow-id "<WORKFLOW_ID>"

# All instances run by a specific agent
engram workflow instances --agent "<agent-name>"

# Full status of a specific instance (history + current state)
engram workflow status "<INSTANCE_ID>"
```

### 10. Store the Outcome in Engram

After the workflow instance completes, record it:

```bash
engram context create \
  --title "Workflow complete: <task-title>" \
  --content "Instance <INSTANCE_ID> completed. Workflow: <WORKFLOW_ID>. Final state: done. Transitions executed: begin_planning → begin_development → submit_review → approve." \
  --source "engram-workflow"
# OUTCOME_UUID = ...

engram relationship create \
  --source-id "<TASK_ID>" --source-type task \
  --target-id "<OUTCOME_UUID>" --target-type context \
  --relationship-type relates_to --agent "<agent-name>"
```

## Example: End-to-End SDLC Workflow

```bash
# --- Prerequisites: assume SDLC workflow definition already exists ---
WORKFLOW_ID="wf-sdlc-001"
TASK_ID="task-auth-002"

# 0. Search for existing instances
engram workflow instances --workflow-id "$WORKFLOW_ID" --running-only
# None found — safe to proceed

# 1. Inspect the definition
engram workflow get "$WORKFLOW_ID"
# Shows states: requirements, planning, development, review, done
# Shows transitions: begin_planning, begin_development, submit_review, approve, request_changes

# 2. Start an instance on the task
engram workflow start "$WORKFLOW_ID" \
  --agent "claude" \
  --entity-id "$TASK_ID" \
  --entity-type "task"
# INSTANCE_ID = inst-003

# 3. Confirm initial state
engram workflow status inst-003
# current_state: requirements

# --- REQUIREMENTS PHASE ---
# Stage policy: only engram entity commits allowed (no code)
engram context create \
  --title "Auth requirements" \
  --content "JWT tokens, refresh token rotation, bcrypt password hashing. Must support OAuth2." \
  --source "requirements-doc"
# CTX_UUID = ctx-010

engram relationship create \
  --source-id "$TASK_ID" --source-type task \
  --target-id ctx-010 --target-type context \
  --relationship-type relates_to --agent "claude"

# Advance to planning
engram workflow transition inst-003 \
  --transition begin_planning \
  --agent "claude"

# 4. Confirm transition
engram workflow status inst-003
# current_state: planning

# --- PLANNING PHASE ---
# Stage policy: only engram entity commits allowed (no code)
engram reasoning create \
  --title "Auth design decision: JWT over sessions" \
  --task-id "$TASK_ID" \
  --content "JWT chosen: stateless, scales horizontally. Refresh rotation mitigates token theft risk."
# RSN_UUID = rsn-011

engram relationship create \
  --source-id "$TASK_ID" --source-type task \
  --target-id rsn-011 --target-type reasoning \
  --relationship-type explains --agent "claude"

# Advance to development
engram workflow transition inst-003 \
  --transition begin_development \
  --agent "claude"

engram workflow status inst-003
# current_state: development

# --- DEVELOPMENT PHASE ---
# Stage policy: code commits allowed; tests and build must pass
# ... write code, run tests, commit ...

# Advance to review
engram workflow transition inst-003 \
  --transition submit_review \
  --agent "claude"

engram workflow status inst-003
# current_state: review

# --- REVIEW PHASE ---
# Run external command action to validate
engram workflow execute-action \
  --action-type external_command \
  --command "cargo test" \
  --working-directory "/home/user/project" \
  --timeout-seconds 120

# Notify reviewer
engram workflow execute-action \
  --action-type notification \
  --message "Code review requested for task auth-002"

# Review passes — approve
engram workflow transition inst-003 \
  --transition approve \
  --agent "claude"

engram workflow status inst-003
# current_state: done

# 5. Record outcome
engram context create \
  --title "Workflow complete: Add user authentication" \
  --content "Instance inst-003 completed. States traversed: requirements → planning → development → review → done." \
  --source "engram-workflow"
# OUTCOME_UUID = ctx-020

engram relationship create \
  --source-id "$TASK_ID" --source-type task \
  --target-id ctx-020 --target-type context \
  --relationship-type relates_to --agent "claude"
```

## Command Reference

| Goal | Command |
|------|---------|
| Find a workflow definition | `engram workflow list --status active` |
| Inspect a definition | `engram workflow get <WORKFLOW_ID>` |
| Query actions/guards for a state | `engram workflow query-actions <WORKFLOW_ID> --state-id <STATE>` |
| Start an instance | `engram workflow start <WORKFLOW_ID> --agent <NAME> --entity-id <ID> --entity-type task` |
| Check current state | `engram workflow status <INSTANCE_ID>` |
| Execute a transition | `engram workflow transition <INSTANCE_ID> --transition <NAME> --agent <NAME>` |
| List running instances | `engram workflow instances --running-only` |
| List instances for a workflow | `engram workflow instances --workflow-id <WORKFLOW_ID>` |
| Cancel an instance | `engram workflow cancel <INSTANCE_ID> --agent <NAME> --reason <REASON>` |
| Fire an action manually | `engram workflow execute-action --action-type <TYPE> ...` |

## Related Skills

- `engram-workflow-guide` — define workflow definitions with states, transitions, and activate them; use this before execution
- `engram-test-driven-development` — TDD workflow that integrates with stage-based commit validation
- `engram-orchestrator` — coordination loop that can drive workflow transitions across multiple tasks
- `engram-subagent-register` — subagent protocol; subagents advance workflow instances as they complete stages
- `engram-audit-trail` — traceability patterns; workflow execution history complements engram audit trails
- `engram-use-engram-memory` — entity creation reference for storing findings during workflow execution
