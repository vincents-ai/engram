---
name: engram-delegate-to-agents
description: "Break down work and delegate to specialized agents. Uses Agent 01 (The One) pattern and references agent prompts from ~/code/prompts/goose/agents/"
---

# Delegating to Agents

## Overview

Break work into atomic tasks and delegate to specialized sub-agents. Store delegation plans in engram for tracking.

## When to Use

Use this skill when:
- Work can be parallelized
- Specialized expertise needed
- Multiple independent tasks
- Orchestrating complex workflows

## The Pattern

### 1. Analyze and Decompose
Break the request into atomic tasks.

### 2. Identify Agents
Match tasks to specialized agents from the agent catalog:
```
~/code/prompts/goose/agents/
├── 01-the-one.yaml      # Orchestrator (you are this)
├── 02-the-sidekick.yaml # General purpose
├── 03-the-architect.yaml # Architecture
├── 15-the-tester.yaml   # Testing
├── 17-the-api-designer.yaml # API design
├── 18-the-integration-specialist.yaml # Integration
├── 19-the-performance-tuner.yaml # Performance
├── 20-the-critical-roller.yaml # Critical systems
├── 21-the-database-specialist.yaml # Database
├── 37-the-prompt-engineer.yaml # Prompts
├── 40-the-dependency-manager.yaml # Dependencies
├── 41-the-technical-writer.yaml # Documentation
├── 70-the-rustacean.yaml # Rust
├── 71-the-gopher.yaml # Go
├── 72-the-type-safe.yaml # TypeScript
└── ... 160+ more specialized agents
```

### 3. Create Delegation Plan
Store in engram:

```bash
engram context create \
  --title "Delegation Plan: [Work Description]" \
  --content "## Analysis\n[What needs to be done]\n\n## Task Breakdown\n1. **Task:** [Name] → **Agent:** [Agent ID] → **Instructions:** [What to do]\n2. **Task:** [Name] → **Agent:** [Agent ID] → **Instructions:** [What to do]\n\n## Dependencies\n- [Task 2 depends on Task 1]\n\n## Expected Outcome\n[What success looks like]" \
  --source "delegation"
```

### 4. Execute Delegation
Dispatch tasks to agents.

### 5. Review Results
Store outcomes in engram:

```bash
engram reasoning create \
  --title "Delegation Result: [Task Name]" \
  --task-id [TASK_ID] \
  --content "**Agent:** [Agent ID]\n**Status:** [Completed/Failed]\n**Outcome:** [What happened]\n**Artifacts:** [Files changed, if any]" \
  --confidence 1.0
```

## Example

```
User: "Build a REST API for user management with auth"

[Step 1: Analyze]
Requirements: CRUD users, JWT auth, role-based access, tests, docs

[Step 2: Identify agents]
- 03-the-architect.yaml → API architecture
- 17-the-api-designer.yaml → Endpoint design  
- 70-the-rustacean.yaml → Implementation (Rust)
- 15-the-tester.yaml → Tests
- 41-the-technical-writer.yaml → Docs

[Step 3: Create delegation plan]
engram context create \
  --title "Delegation Plan: User Management API" \
  --content "## Tasks\n1. Architecture → 03-the-architect\n2. API Design → 17-the-api-designer\n3. Implementation → 70-the-rustacean\n4. Tests → 15-the-tester\n5. Docs → 41-the-technical-writer\n\n## Dependencies\n2 depends on 1\n3 depends on 2\n4 depends on 3\n5 depends on 3"

[Step 4: Dispatch]
# Dispatch to agents in dependency order

[Step 5: Review and store]
engram reasoning create \
  --title "Delegation Complete: User Management API" \
  --content "All 5 tasks completed. API implemented, tests passing, docs written."
```

## Integration with Engram

All delegation is tracked in engram:
- **Context**: Delegation plans
- **Reasoning**: Task outcomes
- **Relationships**: Task dependencies
- **Tasks**: Individual delegated work

## Querying Delegation

```bash
# Get all delegated tasks
engram task list | grep -i delegation

# Get delegation plan
engram context list | grep "Delegation Plan:"

# Get outcomes
engram reasoning list | grep "Delegation Result"
```

## Agent Selection Guide

| Work Type | Agent |
|-----------|-------|
| Architecture | 03-the-architect |
| API Design | 17-the-api-designer |
| Database | 21-the-database-specialist |
| Performance | 19-the-performance-tuner |
| Testing | 15-the-tester |
| Documentation | 41-the-technical-writer |
| Security Review | 20-the-critical-roller |
| Integration | 18-the-integration-specialist |
| Rust Code | 70-the-rustacean |
| Go Code | 71-the-gopher |
| TypeScript | 72-the-type-safe |
| Dependency Updates | 40-the-dependency-manager |
| Prompt Engineering | 37-the-prompt-engineer |

See full catalog: `ls ~/code/prompts/goose/agents/`

## Related Skills

This skill integrates with:
- `engram-use-memory` - Store delegation context and outcomes
- `engram-plan-feature` - Break features into delegable tasks
- `engram-audit-trail` - Track delegation outcomes and artifacts
- `engram-dispatching-parallel-agents` - Execute parallel delegation patterns
- `engram-subagent-driven-development` - Systematic multi-agent development workflow
