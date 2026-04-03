---
name: engram-workflow-guide
description: "Guide to engram workflows: what they are, which existing skill chain to use for a given goal, and how to design custom workflows by chaining skills together."
---

# Engram Workflow Guide

## Overview

A workflow is a sequence of skills chained together to accomplish a goal. Each step in a workflow creates engram entities (tasks, context, reasoning, ADRs, relationships). The graph structure means work is auditable, resumable, and queryable.

Workflows differ from ad-hoc work in three ways:

1. **Process enforced** -- a workflow prescribes the order of operations. You do not skip steps.
2. **Reasoning captured** -- every decision and finding is stored as a reasoning or context entity, not lost in conversation.
3. **Outcomes linked** -- each phase's output is connected to the next phase's input via relationships, so the full chain of cause and effect is traversable.

## The Workflow Lifecycle

Every workflow, whether built-in or custom, follows the same six phases:

```
1. SEARCH   — engram ask query / engram task show
2. PLAN     — engram task create (hierarchy with --parent)
3. EXECUTE  — dispatch to skills or subagents
4. COLLECT  — engram relationship connected to gather results
5. VALIDATE — engram validate check
6. CLOSE    — engram task update --status done
```

These phases are not optional. Skipping search means duplicating work. Skipping validation means shipping unverified changes.

```bash
engram ask query "<goal or topic>"
engram task create --title "<Goal: description>" --priority high --output json
engram task update <PARENT_UUID> --status in_progress
engram task create --title "<Phase: description>" --parent <PARENT_UUID> --priority medium --output json
engram task update <SUBTASK_UUID> --status in_progress
engram relationship connected --entity-id <SUBTASK_UUID> --max-depth 2
engram validate check
engram task update <PARENT_UUID> --status done --outcome "<summary>"
```

## Workflow Selection Guide

Use this table to pick the right workflow for your goal. The entry-point skill is where you start.

| Goal | Workflow | Skills (in order) | Entry Point |
|------|----------|-------------------|-------------|
| Build a new feature | Feature Development | brainstorming -> writing-plans -> test-driven-development -> requesting-code-review | `engram-brainstorming` |
| Debug a problem | Debugging | systematic-debugging -> test-driven-development -> audit-trail | `engram-systematic-debugging` |
| Design a system | Architecture | system-design -> risk-assessment -> writing-plans | `system-design` |
| Review code | Review | requesting-code-review | `engram-requesting-code-review` |
| Plan a release | Release | release-planning -> changelog -> testing | `release-planning` |
| Check compliance | Compliance | check-compliance -> testing -> audit-trail | `engram-check-compliance` |
| Multiple independent tasks | Parallel | plan-feature -> delegate-to-agents -> dispatching-parallel-agents -> subagent-driven-development | `engram-plan-feature` |
| Launch a product | GTM Pipeline | market-validation -> gtm-strategy -> launch-execution | `market-validation` |
| Not sure what to do | Exploratory | brainstorming -> (pick workflow from above) | `engram-brainstorming` |

### Decision Tree

```
What do you need to do?
|
|-- Build something new -> Feature Development
|-- Fix a bug -> Debugging
|-- Design a system -> Architecture
|-- Review someone's code -> Review
|-- Prepare a release -> Release
|-- Verify compliance -> Compliance
|-- Multiple tasks at once -> Parallel
|-- Launch to market -> GTM Pipeline
|-- Unsure -> Brainstorm first, then re-evaluate
```

## Common Patterns

These patterns appear in every workflow. Learn them once, apply them everywhere.

### Research-Before-Acting

Always search engram before starting any phase. This prevents duplicating prior work and ensures you build on existing context.

```bash
engram ask query "<topic>"
engram task show <UUID>
```

### Task Hierarchy

One parent task for the goal. One subtask per workflow phase. The parent tracks overall status; subtasks track phase-level progress.

```bash
engram task create --title "<Goal: description>" --priority high --output json
engram task create --title "<Phase 1: research>" --parent <PARENT_UUID> --priority medium --output json
engram task create --title "<Phase 2: design>" --parent <PARENT_UUID> --priority medium --output json
engram task create --title "<Phase 3: implement>" --parent <PARENT_UUID> --priority high --output json
engram task create --title "<Phase 4: verify>" --parent <PARENT_UUID> --priority high --output json
```

### Reasoning Chain

Store why, not just what. Every phase produces a reasoning entity explaining the logic that led to its output. This makes the workflow auditable and resumable.

```bash
engram reasoning create \
  --title "<phase>: logic and conclusions" \
  --task-id <TASK_UUID> \
  --content "Inputs: <what we started with>\nAnalysis: <what we found>\nDecision: <what we chose and why>"
```

### Validation Gate

Run `engram validate check` before closing any task. Do not skip this. It catches lint errors, type errors, and configuration problems before they propagate.

```bash
engram validate check
engram task update <TASK_UUID> --status done --outcome "<summary>"
```

### Parallel Dispatch

When subtasks are independent, dispatch them simultaneously. This is the primary speed advantage of the workflow system.

```bash
engram task update <SUBTASK_A> --status in_progress
engram task update <SUBTASK_B> --status in_progress
# Tell both subagents their UUIDs. They run concurrently.
```

## Designing Custom Workflows

When none of the built-in workflows fit, design a custom one by following these steps.

### 1. Identify the Phases

Break the goal into sequential phases. Common phase shapes:

| Phase type | Purpose | Example skills |
|-----------|---------|----------------|
| Research | Gather information, explore options | brainstorming, spike-investigation, market-validation |
| Design | Make decisions, create plans | system-design, writing-plans, risk-assessment |
| Implement | Build, write code, execute | test-driven-development, subagent-driven-development |
| Verify | Test, review, validate | requesting-code-review, check-compliance, test-harness-review |
| Document | Record outcomes, transfer knowledge | changelog, runbooks, knowledge-transfer |

### 2. Pick One Skill Per Phase

Each phase maps to exactly one skill. Do not combine phases or skip them.

Example -- custom "Database Migration" workflow:

```
Phase 1: Research    -> spike-investigation (understand current schema)
Phase 2: Design      -> writing-plans (migration plan with rollback)
Phase 3: Implement   -> test-driven-development (write migration + tests)
Phase 4: Verify      -> check-compliance (data integrity check)
```

### 3. Define the Entity Flow

Specify what each phase creates and what the next phase consumes:

```
Phase 1 creates: context entities (current schema, constraints)
Phase 2 consumes: Phase 1 context
Phase 2 creates: task hierarchy (parent + subtasks per migration step)
Phase 3 consumes: Phase 2 task hierarchy
Phase 3 creates: code changes + reasoning entities
Phase 4 consumes: Phase 3 code changes
Phase 4 creates: compliance context + final reasoning
```

### 4. Set Up the Task Hierarchy

```bash
engram task create --title "Goal: Database migration for X" --priority high --output json
# PARENT_UUID = parent-001

engram task create --title "Research: current schema analysis" --parent parent-001 --priority medium --output json
# SUB_UUID = sub-002

engram task create --title "Design: migration plan with rollback" --parent parent-001 --priority medium --output json
# SUB_UUID = sub-003

engram task create --title "Implement: migration and tests" --parent parent-001 --priority high --output json
# SUB_UUID = sub-004

engram task create --title "Verify: data integrity compliance" --parent parent-001 --priority high --output json
# SUB_UUID = sub-005
```

### 5. Link Phase Outputs to Next Phase Inputs

After each phase completes, link its output entities to the next phase's task:

```bash
engram relationship create \
  --source-id <NEXT_PHASE_TASK_UUID> --source-type task \
  --target-id <OUTPUT_CONTEXT_UUID> --target-type context \
  --relationship-type depends_on --agent "<your-name>"
```

This makes the dependency graph explicit. If a future agent needs to understand why Phase 3 made certain choices, they traverse: Phase 3 task -> depends_on -> Phase 2 output -> explains -> Phase 1 context.

## Quick Reference: Skill Categories

| Category | Use Case | Entry Point Skill |
|----------|----------|-------------------|
| Meta | Orchestration, memory, delegation, workflows | `engram-orchestrator` |
| Workflow | Planning, brainstorming, code review | `engram-writing-plans` |
| Development | TDD, subagent-driven dev | `engram-test-driven-development` |
| Debugging | Systematic investigation | `engram-systematic-debugging` |
| Compliance | Framework checks, audit | `engram-check-compliance` |
| Testing | Test harness review | `test-harness-review` |
| Architecture | System design, API design, data modeling | `system-design` |
| Planning | Roadmaps, releases, capacity, spikes | `roadmap-planning` |
| Documentation | API docs, runbooks, ADRs, changelogs | `technical-writing` |
| Quality | Tech debt, performance, edge cases, accessibility | `tech-debt` |
| Review | Code quality, security, retrospectives, post-mortems | `code-quality` |
| Go-to-Market | Market validation, GTM strategy, launch | `market-validation` |

## Example: Full Feature Development Workflow

```
Goal: "Add user notifications system"

[Phase 1: Brainstorm]
engram ask query "user notifications"
engram task create --title "Goal: User notifications system" --priority high --output json
# PARENT_UUID = feat-001
engram task create --title "Brainstorm: notification requirements" --parent feat-001 --priority medium --output json
# BRAINSTORM_UUID = feat-002
engram task update feat-002 --status in_progress
# Run brainstorming skill -> produces context entities with requirements
engram reasoning create --title "Notification requirements summary" --task-id feat-001 --content "Push + in-app + email. Real-time via WebSocket. Preferences per user."
engram task update feat-002 --status done

[Phase 2: Plan]
engram task create --title "Plan: implementation breakdown" --parent feat-001 --priority medium --output json
# PLAN_UUID = feat-003
engram task update feat-003 --status in_progress
# Run writing-plans skill -> produces task hierarchy
engram task update feat-003 --status done

[Phase 3: Implement]
engram task create --title "Implement: notification service with TDD" --parent feat-001 --priority high --output json
# IMPL_UUID = feat-004
engram task update feat-004 --status in_progress
# Run test-driven-development skill -> produces code + reasoning
engram task update feat-004 --status done

[Phase 4: Review]
engram task create --title "Review: code review" --parent feat-001 --priority high --output json
# REVIEW_UUID = feat-005
engram task update feat-005 --status in_progress
# Run requesting-code-review skill -> produces review context
engram task update feat-005 --status done

[Close]
engram validate check
engram task update feat-001 --status done --outcome "User notifications system implemented and reviewed"
```

## Related Skills

- `engram-orchestrator` -- full execution loop for coordinating workflows
- `engram-use-engram-memory` -- command reference for storing and retrieving entities
- `engram-delegate-to-agents` -- agent catalog and delegation patterns
- `engram-dispatching-parallel-agents` -- running multiple subagents concurrently
- `engram-audit-trail` -- traceability patterns for workflow auditing
