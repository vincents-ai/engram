---
name: engram-delegate-to-agents
description: "Break down work and delegate to specialized agents. Uses Agent 01 (The One) pattern and references agent prompts from ~/code/prompts/goose/agents/"
---

# Delegating to Agents

## Overview

Break work into atomic tasks and delegate to specialized sub-agents. Store delegation plans in engram. Pass only task UUIDs to subagents — they pull their own context from engram.

## When to Use

Use this skill when:
- Work can be parallelized
- Specialized expertise is needed
- Multiple independent tasks exist
- Orchestrating complex workflows

## The Pattern

### 0. Search First

Before creating any delegation plan, check what already exists:

```bash
engram ask query "<goal or feature name>"
engram task show <UUID>
```

### 1. Anchor Work

```bash
# No --status on create; update status separately
engram task create --title "<Goal: description>" --priority high --output json
# PARENT_UUID = ...
engram task update <PARENT_UUID> --status in_progress
```

### 2. Store the Delegation Plan

```bash
# --title is required on context create
engram context create \
  --title "Delegation plan: <goal>" \
  --content "## Goal\n<what needs to be done>\n\n## Task Breakdown\n1. Task: <name> → Agent: <agent-id> → Scope: <what to do>\n2. Task: <name> → Agent: <agent-id> → Scope: <what to do>\n\n## Dependencies\n- Task 2 depends on Task 1\n\n## Expected Outcome\n<what success looks like>" \
  --source "delegation-plan"
# PLAN_UUID = ...

engram relationship create \
  --source-id <PARENT_UUID> --source-type task \
  --target-id <PLAN_UUID> --target-type context \
  --relationship-type relates_to --agent "<your-name>"
```

### 3. Create Subtasks

One subtask per agent assignment. Use `--parent` to nest them:

```bash
engram task create \
  --title "<Subtask: what this agent does>" \
  --parent <PARENT_UUID> \
  --priority medium \
  --output json
# SUBTASK_UUID = ...
```

The `--parent` flag creates the hierarchy. You can also link explicitly:

```bash
engram relationship create \
  --source-id <PARENT_UUID> --source-type task \
  --target-id <SUBTASK_UUID> --target-type task \
  --relationship-type depends_on --agent "<your-name>"
```

### 4. Dispatch — Pass UUID Only

Assign the task and tell the subagent only their UUID. They retrieve context from engram themselves using `engram-subagent-register`.

```bash
engram task update <SUBTASK_UUID> --status in_progress

# Tell the subagent:
# "Your task UUID is <SUBTASK_UUID>.
# Run: engram task show <SUBTASK_UUID>
# Use the engram-subagent-register skill."
```

Do not paste instructions inline. The subagent must pull from engram.

### 5. Collect Results

When a subagent completes, retrieve what it stored:

```bash
engram relationship connected --entity-id <SUBTASK_UUID> --max-depth 2
engram ask query "<subtask topic> results"
```

### 6. Record Outcomes

```bash
engram reasoning create \
  --title "Outcome: <subtask title>" \
  --task-id <PARENT_UUID> \
  --content "Agent: <agent-id>\nStatus: Completed\nOutcome: <what happened>\nArtifacts: <files changed if any>"
# RSN_UUID = ...

engram relationship create \
  --source-id <PARENT_UUID> --source-type task \
  --target-id <RSN_UUID> --target-type reasoning \
  --relationship-type explains --agent "<your-name>"
```

### 7. Record Decisions

When synthesizing results into an architectural choice:

```bash
engram adr create \
  --title "<decision title>" \
  --number <N> \
  --context "<what situation led to this and what was decided>" \
  --agent "<your-name>"
# ADR_UUID = ...

engram relationship create \
  --source-id <PARENT_UUID> --source-type task \
  --target-id <ADR_UUID> --target-type adr \
  --relationship-type relates_to --agent "<your-name>"
```

### 8. Validate and Close

```bash
engram validate check
engram task update <PARENT_UUID> --status done --outcome "<summary>"
```

### 9. Get Next Action When Unsure

```bash
engram next
```

## Agent Catalog

```
~/code/prompts/goose/agents/
├── 01-the-one.yaml           # Orchestrator (you are this)
├── 02-the-sidekick.yaml      # General purpose
├── 03-the-architect.yaml     # Architecture
├── 15-the-tester.yaml        # Testing
├── 17-the-api-designer.yaml  # API design
├── 19-the-performance-tuner.yaml
├── 20-the-critical-roller.yaml  # Critical systems / security
├── 21-the-database-specialist.yaml
├── 37-the-prompt-engineer.yaml
├── 40-the-dependency-manager.yaml
├── 41-the-technical-writer.yaml
├── 70-the-rustacean.yaml     # Rust
├── 71-the-gopher.yaml        # Go
├── 72-the-type-safe.yaml     # TypeScript
└── ... 160+ more
```

See full catalog: `ls ~/code/prompts/goose/agents/`

| Work Type | Agent |
|-----------|-------|
| Architecture | 03-the-architect |
| API Design | 17-the-api-designer |
| Database | 21-the-database-specialist |
| Performance | 19-the-performance-tuner |
| Testing | 15-the-tester |
| Documentation | 41-the-technical-writer |
| Security | 20-the-critical-roller |
| Rust | 70-the-rustacean |
| Go | 71-the-gopher |
| TypeScript | 72-the-type-safe |

## Example

```
Goal: "Build a REST API for user management with auth"

[Search first]
engram ask query "user management API auth"

[Anchor]
engram task create --title "Goal: User Management API with Auth" --priority high --output json
# PARENT_UUID = abc-001
engram task update abc-001 --status in_progress

[Delegation plan]
engram context create \
  --title "Delegation plan: User Management API" \
  --content "## Tasks\n1. Architecture → 03-the-architect\n2. API Design → 17-the-api-designer\n3. Implementation → 70-the-rustacean\n4. Tests → 15-the-tester\n5. Docs → 41-the-technical-writer\n\n## Dependencies\n2 after 1, 3 after 2, 4 after 3, 5 after 3" \
  --source "delegation-plan"
# PLAN_UUID = ctx-002

engram relationship create \
  --source-id abc-001 --source-type task \
  --target-id ctx-002 --target-type context \
  --relationship-type relates_to --agent "orchestrator"

[Create subtask for architecture agent]
engram task create --title "Architecture: User Management API" --parent abc-001 --priority high --output json
# SUBTASK_UUID = abc-003
engram task update abc-003 --status in_progress

[Dispatch — UUID only]
# Tell 03-the-architect: "Your task UUID is abc-003. Use engram-subagent-register."

[Collect results]
engram relationship connected --entity-id abc-003 --max-depth 2

[Record outcome]
engram reasoning create \
  --title "Architecture agent outcome" \
  --task-id abc-001 \
  --content "Agent: 03-the-architect. Completed architecture design. Artifacts: ADR for layered auth, entity model stored."
# RSN_UUID = rsn-004

engram relationship create \
  --source-id abc-001 --source-type task \
  --target-id rsn-004 --target-type reasoning \
  --relationship-type explains --agent "orchestrator"

[Validate when all done]
engram validate check
engram task update abc-001 --status done --outcome "User Management API fully delegated and complete"
```

## Related Skills

- `engram-subagent-register` — what subagents use to claim tasks and report back
- `engram-orchestrator` — full orchestration execution loop
- `engram-dispatching-parallel-agents` — running multiple subagents concurrently
- `engram-audit-trail` — complete traceability of delegated work
