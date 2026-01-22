---
name: engram-plan-feature
description: "Plan feature implementation using pipeline templates from ~/code/prompts/goose/ai/pipelines/ and store as engram task hierarchy."
---

# Planning Feature Implementation

## Overview

Create detailed implementation plans using existing pipeline templates. Store as engram task hierarchy for tracking.

## When to Use

Use this skill when:
- Starting new feature work
- Breaking down complex work
- Planning implementation steps
- Estimating effort

## The Pattern

### 1. Select Pipeline Template
Choose from available pipelines:
```
~/code/prompts/goose/ai/pipelines/
├── 01-greenfield-feature-launch.yaml    # New features
├── 02-ui-overhaul-refresh.yaml          # UI changes
├── 03-bug-hunt-triage.yaml              # Bug fixes
├── 04-api-modernisation.yaml            # API changes
├── 05-database-migration.yaml           # DB changes
├── 06-mobile-app-port.yaml              # Mobile
├── 11-nixos-immutable-deploy.yaml       # Nix deployments
├── 12-chaos-engineering-drill.yaml      # Resilience
├── 13-security-penetration-test.yaml    # Security
├── build-verify.yaml                    # CI/CD
└── ... 90+ more pipelines
```

### 2. Analyze Feature
Break down using pipeline stages.

### 3. Create Task Hierarchy
Store in engram:

```bash
# Create parent task (the feature)
PARENT=$(engram task create \
  --title "[Feature Name]" \
  --description "**Pipeline:** [Pipeline Template]\n**Goal:** [What this achieves]" \
  --priority [high/medium/low] \
  --json | jq -r '.id')

# Create subtasks for each pipeline stage
STAGE1=$(engram task create \
  --title "[Feature] Stage 1: [Stage Name]" \
  --description "[Detailed instructions from pipeline]" \
  --parent $PARENT \
  --priority high \
  --json | jq -r '.id')

# Continue for all stages...
```

### 4. Link to Pipeline
Store pipeline reference:

```bash
engram context create \
  --title "Pipeline: [Feature Name]" \
  --content "**Template:** [Pipeline file]\n**Stages:**\n1. [Stage 1]\n2. [Stage 2]\n3. [Stage 3]" \
  --source "pipeline"
```

## Example

```
Feature: "Add user authentication to API"

[Step 1: Select pipeline]
01-greenfield-feature-launch.yaml

[Step 2: Create hierarchy]
engram task create \
  --title "User Authentication Feature" \
  --description "Pipeline: 01-greenfield-feature-launch\nGoal: JWT-based auth with refresh tokens"

# Returns: TASK_ID=auth-feature

[Step 3: Create subtasks]
engram task create \
  --title "Auth: Requirements & Architecture" \
  --parent auth-feature \
  --description "From pipeline: Define requirements, design architecture, identify dependencies"

engram task create \
  --title "Auth: Implementation" \
  --parent auth-feature \
  --description "From pipeline: Core implementation, tests, documentation"

engram task create \
  --title "Auth: Review & Merge" \
  --parent auth-feature \
  --description "From pipeline: Code review, testing, deployment"

[Step 4: Link pipeline]
engram context create \
  --title "Pipeline: User Authentication" \
  --content "Template: 01-greenfield-feature-launch.yaml\nStages: Requirements, Implementation, Review"

[Step 5: Link all to parent]
engram relationship create --source-id auth-feature --target-id [SUBTASK1] --contains
engram relationship create --source-id auth-feature --target-id [SUBTASK2] --contains
engram relationship create --source-id auth-feature --target-id [SUBTASK3] --contains
```

## Integration with Engram

All planning stored in engram:
- **Parent Task**: Feature overview
- **Subtasks**: Pipeline stages
- **Context**: Pipeline template reference
- **Relationships**: Task hierarchy

## Querying Plans

```bash
# Get feature task
engram task show [PARENT_ID]

# Get all stages
engram task list --parent [PARENT_ID]

# Get pipeline reference
engram context list | grep "Pipeline:"

# Get full plan
engram relationship connected --entity-id [PARENT_ID] --relationship-type contains
```

## Common Pipelines

| Feature Type | Pipeline |
|--------------|----------|
| New feature | 01-greenfield-feature-launch |
| UI changes | 02-ui-overhaul-refresh |
| Bug fix | 03-bug-hunt-triage |
| API change | 04-api-modernisation |
| DB migration | 05-database-migration |
| Nix deployment | 11-nixos-immutable-deploy |
| Security testing | 13-security-penetration-test |
| CI/CD | build-verify.yaml |

See full catalog: `ls ~/code/prompts/goose/ai/pipelines/`
