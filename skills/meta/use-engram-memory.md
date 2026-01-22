---
name: engram-use-memory
description: "Use engram as persistent memory for all work. Store context, decisions, and reasoning in engram entities instead of losing them in conversation."
---

# Using Engram Memory

## Overview

Engram is your persistent memory system. Every significant piece of context, decision, and reasoning should be stored in engram entities for future retrieval.

## The Rule

**Before completing any task, ensure all important context is stored in engram.**

## When to Use

Use this skill when:
- Making design decisions
- Writing code that implements a feature
- Debugging issues
- Planning work
- Reviewing code
- Any work that should be remembered

## Engram Entity Types

### Context
Background information, design docs, requirements, architecture decisions.
```bash
engram context create --title "Design: [Feature] - [Section]" --content "[detailed content]"
```

### Reasoning
Decision chains, trade-off analysis, hypothesis testing, justification for choices.
```bash
engram reasoning create --title "Decision: [Choice]" --task-id [ID] --content "[rationale]" --confidence 0.9
```

### Tasks
Work items, implementation steps, subtasks.
```bash
engram task create --title "[Task description]" --parent [PARENT_ID] --priority high
```

### Relationships
Links between entities for traversal.
```bash
engram relationship create --source-id [TASK] --target-id [CONTEXT] --references
```

## The Pattern

### 1. Capture Context
When starting work:
```bash
# Create context for the work
engram context create --title "Context: [Work Description]" \
  --content "[What you're working on, why, constraints]"
```

### 2. Store Decisions
When making decisions:
```bash
# Store decision with reasoning
engram reasoning create --title "Decision: [What was decided]" \
  --task-id [TASK_ID] \
  --content "**Decision:** [What]\n**Rationale:** [Why]\n**Alternatives:** [What else was considered]\n**Confidence:** 0.9" \
  --confidence 0.9
```

### 3. Link Everything
Connect entities:
```bash
# Link context to task
engram relationship create --source-id [TASK] --target-id [CONTEXT] --references

# Link reasoning to task
engram relationship create --source-id [TASK] --target-id [REASONING] --documents
```

### 4. Retrieve Later
Query memory:
```bash
# Get all context for a task
engram relationship connected --entity-id [TASK] --relationship-type references

# Get all reasoning for a task
engram relationship connected --entity-id [TASK] --relationship-type documents

# Search for specific content
engram context list | grep "[search term]"
```

## Example Workflow

```
User: "Add authentication to the API"

[Step 1: Create context]
engram context create --title "Context: Auth Feature Implementation" \
  --content "Adding JWT-based authentication to REST API.\nRequirements: stateless, refresh tokens, role-based access."

[Step 2: Create task]
engram task create --title "Implement Auth Feature" --priority high
# Returns: TASK_ID=abc123

[Step 3: Store design decision]
engram reasoning create --title "Decision: JWT with Refresh Tokens" \
  --task-id abc123 \
  --content "**Chosen:** JWT access tokens + refresh token rotation\n**Why:** Stateless, scales horizontally, refresh tokens allow revocation\n**Alternatives considered:** Session cookies (stateful, harder to scale), API keys (no expiration)" \
  --confidence 0.9

[Step 4: Link to task]
engram relationship create --source-id abc123 --target-id [CONTEXT_ID] --references

[Later: Another agent can retrieve]
engram relationship connected --entity-id abc123 --relationship-type references
# Returns: Design context, architecture decisions
```

## Key Principles

1. **Always Store Decisions** - Not just outcomes, but the reasoning
2. **Link Everything** - Tasks without relationships are orphans
3. **Use Confidence Scores** - Reasoning entities have confidence levels
4. **Query Before Acting** - `engram relationship connected` before starting work
5. **Persist, Don't Ponder** - Memory is external, not in your context window

## Integration with Other Skills

This skill integrates with:
- `engram-plan-feature` - Plan stores as task hierarchy
- `engram-delegate-agents` - Delegation plans stored in context
- `engram-check-compliance` - Compliance evidence stored in context
- `engram-run-pipeline` - Pipeline results stored as reasoning

## Query Patterns

```bash
# Get full context for a feature
engram relationship connected --entity-id [FEATURE_TASK] --relationship-type references

# Get decision history
engram relationship connected --entity-id [FEATURE_TASK] --relationship-type documents

# Search for similar work
engram reasoning list | grep -i "[keyword]"

# Find related tasks
engram task list | grep "[search term]"
```
