---
name: brainstorming-engram
description: "Engram-integrated version. Use before any creative work - creates engram context/reasoning entities instead of markdown files."
---

# Brainstorming Ideas Into Designs (Engram-Integrated)

## Overview

Help turn ideas into fully formed designs and specs through natural collaborative dialogue, storing all design artifacts in Engram for persistent, queryable memory.

## Key Changes from Original

**Original:** Writes validated design to `docs/plans/YYYY-MM-DD-<topic>-design.md`
**Engram-integrated:** Creates engram context entities for design sections and reasoning entities for trade-off analysis.

## The Process

### Understanding the idea:
- Check out the current project state first (files, docs, recent commits)
- Ask questions one at a time to refine the idea
- Prefer multiple choice questions when possible, but open-ended is fine too
- Only one question per message - if a topic needs more exploration, break it into multiple questions
- Focus on understanding: purpose, constraints, success criteria

### Exploring approaches:
- Propose 2-3 different approaches with trade-offs
- Present options conversationally with your recommendation and reasoning
- Lead with your recommended option and explain why

### Presenting the design:
- Once you believe you understand what you're building, present the design
- Break it into sections of 200-300 words
- Ask after each section whether it looks right so far
- Cover: architecture, components, data flow, error handling, testing
- Be ready to go back and clarify if something doesn't make sense

## Engram Integration

### Step 1: Create Design Context Entities

After user validates each design section, create an engram context entity:

```bash
engram context create --title "Design: [Feature Name] - [Section Name]" \
  --content "[200-300 words of validated design content]" \
  --source "brainstorming" \
  --tags "design,brainstorming,[feature-name]"
```

**Sections to create:**
- Design: [Feature] - Architecture
- Design: [Feature] - Components
- Design: [Feature] - Data Flow
- Design: [Feature] - Error Handling
- Design: [Feature] - Testing Strategy

### Step 2: Create Trade-off Reasoning Entities

For each approach explored, create reasoning entities:

```bash
engram reasoning create --title "Trade-off Analysis: [Approach Name]" \
  --task-id [TASK_ID] \
  --content "**Approach:** [Description]\n\n**Pros:**\n- [Pro 1]\n- [Pro 2]\n\n**Cons:**\n- [Con 1]\n- [Con 2]\n\n**Recommendation:** [Your recommendation with reasoning]" \
  --confidence [0.0-1.0] \
  --tags "trade-off,brainstorming,[feature-name]"
```

### Step 3: Create Decision Reasoning Entity

After user selects an approach:

```bash
engram reasoning create --title "Design Decision: [Feature Name] - Approach Selection" \
  --task-id [TASK_ID] \
  --content "**Selected Approach:** [Approach Name]\n\n**Reasoning:** [Why this approach was chosen]\n\n**Alternatives Considered:**\n- [Alternative 1]: Rejected because [reason]\n- [Alternative 2]: Rejected because [reason]" \
  --confidence 0.9 \
  --tags "decision,brainstorming,[feature-name]"
```

### Step 4: Link Everything to Task

Create relationships to connect all entities:

```bash
# Link design contexts to task
engram relationship create --source-id [TASK_ID] --source-type task \
  --target-id [CONTEXT_ID] --target-type context \
  --relationship-type references --agent [AGENT]

# Link trade-off reasoning to task
engram relationship create --source-id [TASK_ID] --source-type task \
  --target-id [REASONING_ID] --target-type reasoning \
  --relationship-type documents --agent [AGENT]
```

## After the Design

### Documentation:
- All design content is now in engram context entities
- All decisions are in engram reasoning entities
- Everything is queryable via `engram relationship connected --entity-id [TASK_ID]`

### Implementation (if continuing):
- Ask: "Ready to set up for implementation?"
- Use superpowers:using-git-worktrees to create isolated workspace
- Use superpowers:writing-plans to create detailed implementation plan

## Key Principles

- **One question at a time** - Don't overwhelm with multiple questions
- **Multiple choice preferred** - Easier to answer than open-ended when possible
- **YAGNI ruthlessly** - Remove unnecessary features from all designs
- **Explore alternatives** - Always propose 2-3 approaches before settling
- **Incremental validation** - Present design in sections, validate each
- **Be flexible** - Go back and clarify when something doesn't make sense
- **Persist to Engram** - All design content goes to engram, not markdown

## Example Workflow

```
User: "I need authentication for the API"

[Understanding phase - ask questions]
Agent: "What authentication method do you prefer? A) JWT tokens B) Session cookies C) OAuth2"
User: "JWT tokens"

[Exploring approaches]
Agent: "For JWT, we have options:\n1. Stateless JWT with short expiry + refresh tokens\n2. Stateless JWT with long expiry (simpler)\n3. Hybrid: JWT for API, sessions for web\n\nI recommend option 1 for security..."
User: "Go with option 1"

[Presenting design - Section 1]
Agent: "Here's the architecture section..."
User: "Looks good"

[Create engram context entity]
engram context create --title "Design: Auth API - Architecture" \
  --content "JWT-based authentication with stateless tokens..." \
  --source "brainstorming"

[Presenting design - Section 2]
Agent: "Here's the components section..."
User: "Looks good"

[Create engram context entity]
engram context create --title "Design: Auth API - Components" \
  --content "TokenService, AuthController, Middleware..." \
  --source "brainstorming"

[Create trade-off reasoning]
engram reasoning create --title "Trade-off: JWT Stateless vs Stateful" \
  --content "**Stateless:** Pros - simpler, no storage needed..." \
  --confidence 0.85

[Create decision reasoning]
engram reasoning create --title "Design Decision: Auth - JWT Stateless" \
  --content "Selected stateless JWT with refresh tokens..." \
  --confidence 0.9

[Link to task]
engram relationship create --source-id [TASK] --target-id [CONTEXT] --references
engram relationship create --source-id [TASK] --target-id [REASONING] --documents

Agent: "Design complete and stored in Engram. Ready to implement?"
```

## Querying Design Context

After brainstorming, agents can retrieve the design via:

```bash
# Get all design contexts for a task
engram relationship connected --entity-id [TASK_ID] --relationship-type references

# Get all reasoning (decisions, trade-offs)
engram relationship connected --entity-id [TASK_ID] --relationship-type documents

# Search for specific design content
engram context list | grep "Design: Auth"
```

## Related Skills

This skill integrates with:
- `engram-use-memory` - Store design decisions persistently
- `engram-writing-plans` - Convert designs to implementation plans
- `engram-plan-feature` - Use pipeline templates for implementation
- `engram-audit-trail` - Track design decisions over time
- `engram-delegate-to-agents` - Delegate implementation to specialized agents
