---
name: engram-plan-feature
description: "Plan feature implementation using pipeline templates from ~/code/prompts/goose/ai/pipelines/ and store as engram task hierarchy."
---

# Planning Feature Implementation

## Overview

Create structured implementation plans using pipeline templates. Store as an engram task hierarchy — not a markdown file — so any agent can retrieve it with `engram task show <UUID>`.

## When to Use

Use this skill when:
- Starting new feature work with a well-defined pipeline type
- Breaking down complex work into pipeline stages
- Planning implementation steps against a standard process

## The Pattern

### 0. Search First

```bash
engram ask query "<feature name> plan"
engram task show <UUID>
```

### 1. Select a Pipeline Template

```
~/code/prompts/goose/ai/pipelines/
├── 01-greenfield-feature-launch.yaml
├── 02-ui-overhaul-refresh.yaml
├── 03-bug-hunt-triage.yaml
├── 04-api-modernisation.yaml
├── 05-database-migration.yaml
├── 06-mobile-app-port.yaml
├── 11-nixos-immutable-deploy.yaml
├── 12-chaos-engineering-drill.yaml
├── 13-security-penetration-test.yaml
├── build-verify.yaml
└── ... 90+ more
```

| Feature Type | Pipeline |
|---|---|
| New feature | 01-greenfield-feature-launch |
| UI changes | 02-ui-overhaul-refresh |
| Bug fix | 03-bug-hunt-triage |
| API change | 04-api-modernisation |
| DB migration | 05-database-migration |
| Nix deployment | 11-nixos-immutable-deploy |
| Security testing | 13-security-penetration-test |

See full catalog: `ls ~/code/prompts/goose/ai/pipelines/`

### 2. Create the Parent Task

```bash
engram task create --title "<Feature Name>"
# PARENT_UUID = ...
engram task update <PARENT_UUID> --status in_progress

# Store pipeline reference as context
engram context create \
  --title "Pipeline reference: <feature name>" \
  --content "Pipeline: <pipeline-file>\nGoal: <what this achieves>\nStages: <list of stages from pipeline>" \
  --source "pipeline-template"
# PIPELINE_UUID = ...

engram relationship create \
  --source-id <PARENT_UUID> --source-type task \
  --target-id <PIPELINE_UUID> --target-type context \
  --relationship-type relates_to --agent "<name>"
```

### 3. Create Subtasks for Each Pipeline Stage

```bash
engram task create --title "<Feature> Stage 1: <Stage Name>"
# STAGE1_UUID = ...

engram relationship create \
  --source-id <PARENT_UUID> --source-type task \
  --target-id <STAGE1_UUID> --target-type task \
  --relationship-type depends_on --agent "<name>"

# Store detailed stage instructions
engram context create \
  --title "Stage 1 detail: <stage name>" \
  --content "<Detailed instructions from pipeline for this stage>" \
  --source "<pipeline-file>"
# DETAIL_UUID = ...

engram relationship create \
  --source-id <STAGE1_UUID> --source-type task \
  --target-id <DETAIL_UUID> --target-type context \
  --relationship-type relates_to --agent "<name>"
```

Repeat for each stage.

### 4. Verify the Hierarchy

```bash
engram relationship connected --entity-id <PARENT_UUID> --max-depth 2
```

### 5. Validate

```bash
engram validate check
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
Feature: "Add user authentication to API"

[Search first]
engram ask query "user authentication API plan"

[Anchor]
engram task create --title "User Authentication Feature"
# PARENT_UUID = abc-001
engram task update abc-001 --status in_progress

[Pipeline reference]
engram context create \
  --title "Pipeline reference: User Auth" \
  --content "Pipeline: 01-greenfield-feature-launch.yaml\nGoal: JWT-based auth with refresh tokens\nStages: Requirements & Architecture, Implementation, Review & Merge" \
  --source "01-greenfield-feature-launch.yaml"
# PIPELINE_UUID = ctx-002

engram relationship create \
  --source-id abc-001 --source-type task \
  --target-id ctx-002 --target-type context \
  --relationship-type relates_to --agent "planner"

[Stage 1]
engram task create --title "Auth Stage 1: Requirements & Architecture"
# STAGE1_UUID = abc-003

engram relationship create \
  --source-id abc-001 --source-type task \
  --target-id abc-003 --target-type task \
  --relationship-type depends_on --agent "planner"

engram context create \
  --title "Stage 1 detail: Requirements & Architecture" \
  --content "From pipeline: Define JWT requirements, design token structure, identify dependencies (jsonwebtoken, redis), write architecture ADR." \
  --source "01-greenfield-feature-launch.yaml"
# DETAIL1_UUID = ctx-004

engram relationship create \
  --source-id abc-003 --source-type task \
  --target-id ctx-004 --target-type context \
  --relationship-type relates_to --agent "planner"

[Stage 2]
engram task create --title "Auth Stage 2: Implementation"
# STAGE2_UUID = abc-005

engram relationship create \
  --source-id abc-001 --source-type task \
  --target-id abc-005 --target-type task \
  --relationship-type depends_on --agent "planner"

engram context create \
  --title "Stage 2 detail: Implementation" \
  --content "From pipeline: TDD cycle for login, register, refresh, logout endpoints. Run tests after each. Commit per endpoint." \
  --source "01-greenfield-feature-launch.yaml"
# DETAIL2_UUID = ctx-006

engram relationship create \
  --source-id abc-005 --source-type task \
  --target-id ctx-006 --target-type context \
  --relationship-type relates_to --agent "planner"

[Stage 3]
engram task create --title "Auth Stage 3: Review & Merge"
# STAGE3_UUID = abc-007

engram relationship create \
  --source-id abc-001 --source-type task \
  --target-id abc-007 --target-type task \
  --relationship-type depends_on --agent "planner"

[Verify]
engram relationship connected --entity-id abc-001 --max-depth 2
# Shows abc-001 → abc-003, abc-005, abc-007

[Validate]
engram validate check
engram next
```

## Subagents Retrieve Stage Details Via

```bash
engram task show <STAGE_UUID>
engram relationship connected --entity-id <STAGE_UUID> --max-depth 1
```

## Related Skills

- `engram-brainstorming` — design the feature before planning
- `engram-writing-plans` — finer-grained TDD-style planning for simpler features
- `engram-subagent-driven-development` — execute stages with agent-per-stage
- `engram-dispatching-parallel-agents` — execute independent stages in parallel
