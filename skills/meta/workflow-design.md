---
name: engram-workflow-design
description: "Design and define engram state machine workflows: author workflow definitions with states, transitions, guards, and actions. Use before creating a new workflow type, when designing process automation, or when modelling multi-stage agent pipelines."
---

# Engram Workflow Design

## Overview

An engram workflow **definition** is a reusable state machine template. Before any instance can run, you must design and register the definition: decide what states exist, what transitions are allowed, what guards block a transition, and what actions fire when a state changes. This skill guides that authoring process step by step, using only commands verified against the live binary.

The output is an active workflow registered in engram that can be started on any task or entity via `engram workflow start`.

## When to Use

- You need a repeatable multi-stage process (SDLC, code review, incident response, etc.)
- You want commit-level stage enforcement (only certain activity allowed per stage)
- You are designing a pipeline where transitions need to be explicit and auditable
- You want to model branching flows (e.g., review → approved OR rejected → rework)
- You are orchestrating parallel agent workstreams that converge at a gate

## Core Concepts

### States

A state represents a named phase in the lifecycle. Every workflow has one `start` state and at least one final state.

| State Type | Purpose |
|------------|---------|
| `start` | The initial state when an instance begins |
| `in_progress` | Active work phase |
| `review` | Awaiting review or external input |
| `done` | Terminal success state |
| `blocked` | Waiting on an external dependency |

**Naming convention:** Use `snake_case` lowercase nouns or verb phrases that describe the phase, not the action. Examples: `requirements`, `planning`, `development`, `review`, `done`, `blocked`, `bdd_red`, `bdd_green`, `awaiting_approval`.

### Transitions

A transition defines an allowed move from one state to another. It has a name, a source state, a target state, and a type.

| Transition Type | When to Use |
|-----------------|------------|
| `manual` | An agent or human must explicitly trigger it |
| `automatic` | Fires as soon as the source state is entered (no approval needed) |
| `conditional` | Fires when a condition evaluates to true |
| `scheduled` | Time-based trigger |

**Naming convention:** Use `snake_case` verb phrases that describe the act of moving. Examples: `begin_planning`, `submit_for_review`, `approve`, `reject`, `request_changes`, `unblock`, `escalate`.

### Guards

Guards are **conditions that must be true for a transition to be allowed**. In engram, guards are expressed through workflow-level validation: before executing a transition, verify that the required engram entities exist. Common guard patterns:

- Require a reasoning entity linked to the task before leaving `planning`
- Require a context entity with test results before `submit_for_review`
- Require a passing build (verified via `execute-action`) before `approve`

Guards are enforced by the agent following the skill — they are checked before calling `engram workflow transition`. See the "Implementing Guards" section below.

### Actions

Actions are side effects that fire on a transition. Engram supports three action types via `engram workflow execute-action`:

| Action Type | What it Does |
|-------------|-------------|
| `external_command` | Runs a shell command (e.g., `cargo test`, `npm run build`) |
| `notification` | Emits a notification message |
| `update_entity` | Updates a linked entity's fields |

Actions are executed by the agent immediately before or after calling `engram workflow transition`, using `engram workflow execute-action`.

## The Pattern

### 0. Search First

Before creating a new workflow, check if one already exists that meets your needs:

```bash
engram workflow list --status active
engram workflow list --search "<keywords>"
engram ask query "workflow definition for <process name>"
```

If a workflow exists that is close but not quite right, consider updating it with `engram workflow update` rather than creating a new one.

### 1. Anchor in Engram

```bash
engram task create --title "Design workflow: <workflow name>"
# DESIGN_TASK_UUID = ...

engram task update <DESIGN_TASK_UUID> --status in_progress
```

### 2. Create the Workflow Definition

```bash
engram workflow create \
  --title "<human-readable workflow title>" \
  --description "<what this workflow governs and when to use it>" \
  --entity-types "task" \
  --agent "<your-agent-name>"
# Returns: WORKFLOW_ID
```

`--entity-types` accepts a comma-separated list. Common values: `task`, `context`, `reasoning`. Use `task` for most process workflows.

The workflow starts in `Draft` status. No instances can start until you activate it.

### 3. Add States

Add every phase as a state. Add the initial state first, then intermediate states, then final states.

```bash
WORKFLOW_ID="<workflow-uuid>"

# Initial state (start type)
engram workflow add-state "$WORKFLOW_ID" \
  --name "<initial_state_name>" \
  --state-type start \
  --description "<what happens in this phase>"

# Intermediate states (in_progress, review, or blocked)
engram workflow add-state "$WORKFLOW_ID" \
  --name "<state_name>" \
  --state-type in_progress \
  --description "<what work is done in this phase>"

engram workflow add-state "$WORKFLOW_ID" \
  --name "<review_state_name>" \
  --state-type review \
  --description "<what is reviewed and by whom>"

# Terminal state (is-final flag required for done states)
engram workflow add-state "$WORKFLOW_ID" \
  --name "done" \
  --state-type done \
  --description "<what completion means>" \
  --is-final
```

**`--is-final`**: Mark any state that represents a terminal condition (done, cancelled, rejected-final) with `--is-final`. This tells the instance runner that no further transitions are expected.

State types and their commit-time enforcement (when a task is bound to this workflow):

| State name | Code commits | Engram-only | Tests required | Build required |
|------------|-------------|-------------|----------------|----------------|
| `requirements` | No | Yes | No | No |
| `planning` | No | Yes | No | No |
| `bdd_red` | Yes | No | No | No |
| `bdd_green` | Yes | No | Yes | Yes |
| `bdd_refactor` | Yes | No | Yes | Yes |
| `development` | Yes | No | Yes | Yes |
| `integration` | Yes | No | Yes | Yes |
| `review` | Yes | No | No | No |
| `done` | Yes | No | No | No |

### 4. Add Transitions

Wire the states together. Every pair of states you want to allow movement between needs a transition.

```bash
# Forward transition (manual)
engram workflow add-transition "$WORKFLOW_ID" \
  --name "<transition_name>" \
  --from-state "<source_state_name>" \
  --to-state "<target_state_name>" \
  --transition-type manual \
  --description "<why this transition is allowed and what triggers it>"

# Backward transition (e.g., rework loop)
engram workflow add-transition "$WORKFLOW_ID" \
  --name "request_changes" \
  --from-state "review" \
  --to-state "development" \
  --transition-type manual \
  --description "Review found issues; return to development"

# Automatic transition
engram workflow add-transition "$WORKFLOW_ID" \
  --name "auto_advance" \
  --from-state "<state>" \
  --to-state "<next_state>" \
  --transition-type automatic \
  --description "Automatically advances when state is entered"

# Conditional transition
engram workflow add-transition "$WORKFLOW_ID" \
  --name "escalate_if_blocked" \
  --from-state "development" \
  --to-state "blocked" \
  --transition-type conditional \
  --description "Escalate to blocked when dependencies are missing"
```

`--from-state` and `--to-state` take the **name** of the state (the string passed to `--name` in `add-state`), not a UUID.

### 5. Implementing Guards

Guards are pre-transition checks the agent performs before calling `engram workflow transition`. Pattern:

```bash
# Guard: require a reasoning entity linked to the task before leaving 'planning'
LINKED=$(engram relationship connected --entity-id "$TASK_UUID" --max-depth 1)
# Check the output contains a 'reasoning' entity
# If not present, do NOT call transition — store a blocker instead:

engram context create \
  --title "Guard failed: missing reasoning before <transition_name>" \
  --content "Transition '<transition_name>' blocked. Required: reasoning entity linked to task $TASK_UUID. Found: none." \
  --source "workflow-guard"

# Only proceed when the guard passes:
engram workflow transition "$INSTANCE_ID" \
  --transition "<transition_name>" \
  --agent "<your-agent-name>"
```

Common guard patterns to implement before a transition:

| Transition | Guard to check |
|------------|---------------|
| `begin_development` | Reasoning entity exists (design decision recorded) |
| `submit_for_review` | Context entity with test results linked to task |
| `approve` | External command exits 0 (build/test pass) |
| `close` | Task status is `done`, no open sub-tasks |

### 6. Implementing Actions

Execute actions immediately before or after a transition using `engram workflow execute-action`:

```bash
# Run a shell command (e.g., tests before submit_for_review)
engram workflow execute-action \
  --action-type external_command \
  --command "cargo test" \
  --working-directory "/path/to/project"

# Send a notification after submit_for_review
engram workflow execute-action \
  --action-type notification \
  --message "Review requested: task $TASK_UUID has been submitted for code review"

# Update the linked entity after transitioning to done
engram workflow execute-action \
  --action-type update_entity \
  --entity-id "$TASK_UUID" \
  --entity-type "task"
```

Actions do not fire automatically — the agent orchestrating the workflow must call `execute-action` at the right point in the workflow loop.

### 7. Set the Initial State and Activate

After adding all states, set the initial state and activate:

```bash
# Set the initial state on the workflow definition
engram workflow update "$WORKFLOW_ID" \
  --initial-state "<initial_state_name>"

# Activate: moves status from Draft → Active
engram workflow activate "$WORKFLOW_ID"
```

The workflow is now available for `engram workflow start`.

### 8. Inspect and Verify

```bash
# View the full workflow definition
engram workflow get "$WORKFLOW_ID"

# Query all available transitions and actions
engram workflow query-actions "$WORKFLOW_ID"

# Query actions available from a specific state
engram workflow query-actions "$WORKFLOW_ID" --state-id "<state_name>"

# List all active workflows
engram workflow list --status active
```

### 9. Record the Design Decision

```bash
engram adr create \
  --title "Workflow definition: <workflow title>" \
  --number <N> \
  --context "Created workflow '<title>' to govern <process>. States: <list>. Key design decisions: <rationale for branching points or transition types>. Alternatives considered: <any rejected approaches>." \
  --agent "<your-agent-name>"
# ADR_UUID = ...

engram relationship create \
  --source-id <DESIGN_TASK_UUID> --source-type task \
  --target-id <ADR_UUID> --target-type adr \
  --relationship-type relates_to --agent "<your-agent-name>"
```

### 10. Close the Design Task

```bash
engram reasoning create \
  --title "Workflow design complete: <workflow title>" \
  --task-id <DESIGN_TASK_UUID> \
  --content "Workflow '$WORKFLOW_ID' designed and activated. States: <N>. Transitions: <N>. Guards defined: <list>. Actions defined: <list>. Ready for use with engram workflow start."
# REASONING_UUID = ...

engram relationship create \
  --source-id <DESIGN_TASK_UUID> --source-type task \
  --target-id <REASONING_UUID> --target-type reasoning \
  --relationship-type explains --agent "<your-agent-name>"

engram task update <DESIGN_TASK_UUID> --status done \
  --outcome "Workflow '<title>' designed and activated as $WORKFLOW_ID"
```

## Design Patterns

### Linear Pipeline

The simplest pattern: states in a straight line, no branching. Use for processes with a single success path.

```
requirements → planning → development → review → done
```

All transitions are `manual`. No backward transitions.

### Branching Flow

A state with multiple outgoing transitions. Use when a review can either approve or reject.

```
development → review
review → done        (transition: approve)
review → development (transition: request_changes)
```

The agent checks the review outcome and calls the appropriate transition name.

### Block/Unblock Pattern

A `blocked` state with transitions in and out. Use when an external dependency can halt progress.

```
development → blocked    (transition: block)
blocked → development    (transition: unblock)
```

Use `conditional` transition type for auto-blocking when a guard fails.

### Parallel Convergence

For parallel workstreams that must all complete before advancing, model with sub-workflows or by tracking sub-task completion as a guard:

```bash
# Guard: all sub-tasks done before transition
engram task list --parent "$TASK_UUID" --status active
# If count > 0, guard fails — do not advance
```

### BDD Stage Workflow

For test-driven development with stage-based commit enforcement:

```
requirements → planning → bdd_red → bdd_green → bdd_refactor → review → done
```

Each stage enforces different commit policies via the `WorkflowValidator`:
- `requirements`, `planning`: only engram entity commits allowed
- `bdd_red`: code commits allowed, tests must fail
- `bdd_green`: tests and build must pass
- `bdd_refactor`: tests must stay passing, no new tests

## Example: Feature Development Workflow

Complete end-to-end example defining a feature development workflow with branching review:

```bash
# 1. Anchor
engram task create --title "Design workflow: feature-development"
# DESIGN_TASK_UUID = task-aaa

engram task update task-aaa --status in_progress

# 2. Create definition
engram workflow create \
  --title "Feature Development" \
  --description "Full lifecycle for a single feature: requirements through done" \
  --entity-types "task" \
  --agent "claude"
# WORKFLOW_ID = wf-bbb

# 3. Add states
engram workflow add-state wf-bbb \
  --name "requirements" \
  --state-type start \
  --description "Gather requirements and acceptance criteria"

engram workflow add-state wf-bbb \
  --name "planning" \
  --state-type in_progress \
  --description "Design solution, record reasoning, create sub-tasks"

engram workflow add-state wf-bbb \
  --name "development" \
  --state-type in_progress \
  --description "TDD implementation — tests and build must pass"

engram workflow add-state wf-bbb \
  --name "review" \
  --state-type review \
  --description "Code review and compliance check"

engram workflow add-state wf-bbb \
  --name "done" \
  --state-type done \
  --description "Feature shipped and verified" \
  --is-final

# 4. Add transitions
engram workflow add-transition wf-bbb \
  --name "begin_planning" \
  --from-state "requirements" \
  --to-state "planning" \
  --transition-type manual \
  --description "Requirements accepted; move to planning"

engram workflow add-transition wf-bbb \
  --name "begin_development" \
  --from-state "planning" \
  --to-state "development" \
  --transition-type manual \
  --description "Plan approved with reasoning; begin TDD"

engram workflow add-transition wf-bbb \
  --name "submit_for_review" \
  --from-state "development" \
  --to-state "review" \
  --transition-type manual \
  --description "Tests pass, build passes; submit PR for review"

engram workflow add-transition wf-bbb \
  --name "approve" \
  --from-state "review" \
  --to-state "done" \
  --transition-type manual \
  --description "Review passed; feature complete"

engram workflow add-transition wf-bbb \
  --name "request_changes" \
  --from-state "review" \
  --to-state "development" \
  --transition-type manual \
  --description "Review found issues; return to development"

# 5. Set initial state and activate
engram workflow update wf-bbb --initial-state "requirements"
engram workflow activate wf-bbb

# 6. Verify
engram workflow get wf-bbb
engram workflow query-actions wf-bbb

# 7. Record design decision
engram adr create \
  --title "Workflow definition: Feature Development" \
  --number 12 \
  --context "Created feature-development workflow with 5 states and 5 transitions. Review loop allows returning to development. requirements and planning states block code commits, enforcing engram-first planning discipline. Chosen over a flat task-only approach because workflow provides commit-time enforcement and queryable instance history." \
  --agent "claude"
# ADR_UUID = adr-ccc

engram relationship create \
  --source-id task-aaa --source-type task \
  --target-id adr-ccc --target-type adr \
  --relationship-type relates_to --agent "claude"

# 8. Close design task
engram reasoning create \
  --title "Workflow design complete: Feature Development" \
  --task-id task-aaa \
  --content "Workflow wf-bbb activated. 5 states, 5 transitions. Guards: planning requires reasoning entity before begin_development; development requires passing tests before submit_for_review. Actions: notification on submit_for_review; external_command (cargo test) before approve."
# REASONING_UUID = rsn-ddd

engram relationship create \
  --source-id task-aaa --source-type task \
  --target-id rsn-ddd --target-type reasoning \
  --relationship-type explains --agent "claude"

engram task update task-aaa --status done \
  --outcome "Feature Development workflow (wf-bbb) designed and activated"

# --- Now use the workflow ---

# Start an instance on a task
engram workflow start wf-bbb \
  --agent "claude" \
  --entity-id "$FEATURE_TASK_ID" \
  --entity-type "task"
# INSTANCE_ID = inst-eee

# Requirements phase: gather context (code commits blocked by WorkflowValidator)
engram context create \
  --title "Requirements: <feature name>" \
  --content "Acceptance criteria: ..." \
  --source "requirements"

# Guard check: requirements captured before advancing
engram workflow transition inst-eee \
  --transition "begin_planning" \
  --agent "claude"

# Planning phase: record reasoning (code commits still blocked)
engram reasoning create \
  --title "Design decision: <feature name>" \
  --task-id "$FEATURE_TASK_ID" \
  --content "Approach: ..."

# Guard passes: reasoning exists — advance to development
engram workflow transition inst-eee \
  --transition "begin_development" \
  --agent "claude"

# Development phase: code commits allowed, tests must pass
# ... write code and tests ...

# Action: run tests before submit
engram workflow execute-action \
  --action-type external_command \
  --command "cargo test" \
  --working-directory "/path/to/project"

engram workflow transition inst-eee \
  --transition "submit_for_review" \
  --agent "claude"

# Action: notify reviewer
engram workflow execute-action \
  --action-type notification \
  --message "Feature ready for review: instance inst-eee"

# Review passes
engram workflow transition inst-eee \
  --transition "approve" \
  --agent "claude"

# Check final status
engram workflow status inst-eee
```

## Command Reference

```bash
# Create and configure
engram workflow create --title <T> --description <D> --entity-types <E> --agent <A>
engram workflow add-state <ID> --name <N> --state-type <T> --description <D> [--is-final]
engram workflow add-transition <ID> --name <N> --from-state <F> --to-state <T> --transition-type <T> --description <D>
engram workflow update <ID> --initial-state <S>
engram workflow activate <ID>

# Inspect
engram workflow list [--status <S>] [--search <Q>] [--limit <N>]
engram workflow get <ID>
engram workflow query-actions <ID> [--state-id <S>]

# Run instances
engram workflow start <WORKFLOW_ID> --agent <A> [--entity-id <E>] [--entity-type <T>]
engram workflow transition <INSTANCE_ID> --transition <T> --agent <A>
engram workflow status <INSTANCE_ID>
engram workflow instances [--workflow-id <W>] [--agent <A>] [--running-only]
engram workflow cancel <INSTANCE_ID> --agent <A> [--reason <R>]

# Actions
engram workflow execute-action --action-type external_command --command <C> [--working-directory <D>]
engram workflow execute-action --action-type notification --message <M>
engram workflow execute-action --action-type update_entity --entity-id <E> --entity-type <T>
```

## State Type Reference

| Type | `--state-type` value | Commit policy |
|------|----------------------|---------------|
| Initial phase | `start` | Depends on state name |
| Work phase | `in_progress` | Depends on state name |
| Awaiting review | `review` | Code commits allowed |
| Terminal success | `done` | Code commits allowed |
| Waiting on dep | `blocked` | Code commits allowed |

## Related Skills

- `engram-workflow-guide` — using and running workflow instances; stage-based commit validation reference
- `engram-orchestrator` — coordination loop that drives workflow transitions
- `engram-test-driven-development` — TDD workflow using BDD stage states
- `engram-use-engram-memory` — entity creation patterns for storing workflow artifacts
- `engram-audit-trail` — full traceability of workflow transitions
- `engram-author-skill` — pattern for authoring and validating new skills
