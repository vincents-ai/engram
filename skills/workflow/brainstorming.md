---
name: engram-brainstorming
description: "Engram-integrated version. Use before any creative work - creates engram context/reasoning entities instead of markdown files."
---

# Brainstorming Ideas Into Designs

## Overview

Turn ideas into fully formed designs through collaborative dialogue. Store all design artifacts in engram — not markdown files — so they are queryable by future agents.

## When to Use

Use before any creative or design work:
- Starting a new feature
- Exploring architectural approaches
- Resolving ambiguity before planning
- Recording approach trade-offs before implementation

## The Process

### 0. Search First

Before brainstorming, check what already exists:

```bash
engram ask query "<feature or topic name>"
engram task show <UUID>
```

### 1. Anchor Work

```bash
engram task create --title "Design: <feature name>"
# TASK_UUID = ...
engram task update <TASK_UUID> --status in_progress
```

### 2. Understand the Idea

- Check current project state (files, recent commits) by running shell commands directly
- Ask one question at a time — not multiple at once
- Prefer multiple-choice questions when possible
- Focus on: purpose, constraints, success criteria

```bash
# Run these directly in your shell:
git log --oneline -10
ls -la src/
```

### 3. Explore Approaches

- Propose 2–3 approaches with trade-offs
- Lead with your recommendation and explain why
- Store each approach as context:

```bash
engram context create \
  --title "Approach: <name>" \
  --content "Approach: <name>\nPros: <list>\nCons: <list>\nRecommendation: <yes/no and why>" \
  --source "brainstorming"
# APPROACH_UUID = ...

engram relationship create \
  --source-id <TASK_UUID> --source-type task \
  --target-id <APPROACH_UUID> --target-type context \
  --relationship-type relates_to --agent "<name>"
```

### 3b. Ask-First Refinement Protocol

Before finalising any stored entity, use engram to ask the user refinement questions rather than storing half-baked ideas as final.

**Protocol:**
1. Create all engram entities immediately as drafts (with `[DRAFT]` prefix in title)
2. Ask the user targeted refinement questions (one at a time)
3. Update/replace draft entities with finalised content once answers are received

```bash
# Step 1: Store a draft immediately
engram context create \
  --title "[DRAFT] Approach: <name>" \
  --content "<initial thoughts — mark clearly as draft>" \
  --source "brainstorming"
# DRAFT_UUID = ...

# Step 2: Ask the user a refinement question (one at a time)
# "Should this approach use X or Y? Recommendation: X because..."
# Wait for their answer before proceeding.

# Step 3: Update with final content once answer received
engram context update <DRAFT_UUID> \
  --title "Approach: <name>" \
  --content "<finalised content incorporating user's answer>"
```

**Rule:** Never store speculative content as final. Draft first, ask, then finalise. This prevents garbage data accumulating in the knowledge graph.

**CLI vs UI split:** `engram` CLI commands are for agent interaction only. Locus is the human interface. Do not mix these — new commands belong in `engram` (for agents) or Locus (for humans), never both.

### 4. Record the Trade-off Reasoning

```bash
engram reasoning create \
  --title "Trade-off: <A> vs <B>" \
  --task-id <TASK_UUID> \
  --content "Approach <A> vs <B>: chose <A> because <rationale>. <B> rejected: <reason>."
# RSN_UUID = ...

engram relationship create \
  --source-id <TASK_UUID> --source-type task \
  --target-id <RSN_UUID> --target-type reasoning \
  --relationship-type explains --agent "<name>"
```

### 5. Record the Decision as an ADR

When the user selects an approach, this is an architectural decision — use ADR:

```bash
engram adr create \
  --title "Design decision: <feature> — <approach name>" \
  --number <N> \
  --context "<what situation led to this choice and what was decided, including alternatives rejected>" \
  --agent "<name>"
# ADR_UUID = ...

engram relationship create \
  --source-id <TASK_UUID> --source-type task \
  --target-id <ADR_UUID> --target-type adr \
  --relationship-type relates_to --agent "<name>"
```

### 6. Present and Validate the Design

Break into 200–300 word sections. After each section, ask the user to validate:

- Architecture
- Components
- Data Flow
- Error Handling
- Testing Strategy

Store each validated section:

```bash
engram context create \
  --title "Design section: <section name>" \
  --content "<200-300 words of validated design content for this section>" \
  --source "brainstorming"
# DESIGN_UUID = ...

engram relationship create \
  --source-id <TASK_UUID> --source-type task \
  --target-id <DESIGN_UUID> --target-type context \
  --relationship-type relates_to --agent "<name>"
```

### 7. Validate and Next Step

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
User: "I need authentication for the API"

[Search first]
engram ask query "authentication API decisions"

[Anchor]
engram task create --title "Design: API Authentication"
# TASK_UUID = task-001
engram task update task-001 --status in_progress

[Explore project state — run directly in shell]
git log --oneline -5

[Ask one question]
"What authentication method? A) JWT tokens B) Session cookies C) OAuth2"
User: "JWT tokens"

[Store approach]
engram context create \
  --title "Approach: Stateless JWT + refresh tokens" \
  --content "Approach: Stateless JWT with short expiry + refresh tokens\nPros: scales horizontally, refresh allows revocation\nCons: slightly more complex client handling\nRecommendation: YES" \
  --source "brainstorming"
# APPROACH_UUID = ctx-002

engram relationship create \
  --source-id task-001 --source-type task \
  --target-id ctx-002 --target-type context \
  --relationship-type relates_to --agent "designer"

[Store reasoning]
engram reasoning create \
  --title "Trade-off: JWT vs sessions" \
  --task-id task-001 \
  --content "JWT stateless chosen over session cookies (stateful, hard to scale) and long-lived JWT (no revocation path)."
# RSN_UUID = rsn-003

engram relationship create \
  --source-id task-001 --source-type task \
  --target-id rsn-003 --target-type reasoning \
  --relationship-type explains --agent "designer"

[Record ADR when approach selected]
engram adr create \
  --title "Use stateless JWT with refresh token rotation for API auth" \
  --number 1 \
  --context "Need stateless auth that scales horizontally with revocation support. JWT access tokens (15min expiry) + refresh token rotation chosen. Sessions rejected as stateful." \
  --agent "designer"
# ADR_UUID = adr-004

engram relationship create \
  --source-id task-001 --source-type task \
  --target-id adr-004 --target-type adr \
  --relationship-type relates_to --agent "designer"

[Present architecture section]
"Here's the architecture section..."
User: "Looks good"

[Store validated section]
engram context create \
  --title "Architecture: JWT auth" \
  --content "JWT-based auth with stateless tokens. TokenService handles issuance/validation. AuthController exposes /auth/login and /auth/refresh. Middleware validates Bearer tokens on protected routes." \
  --source "brainstorming"
# ARCH_UUID = ctx-005

engram relationship create \
  --source-id task-001 --source-type task \
  --target-id ctx-005 --target-type context \
  --relationship-type relates_to --agent "designer"

# ... continue for each section ...

[Finish]
engram validate check
engram next
```

## Key Principles

- **Search first** — `engram ask query` before brainstorming anything
- **Draft first, ask, then finalise** — create entities with `[DRAFT]` prefix, ask the user refinement questions, update to final only after answers are received; never store half-baked ideas as final
- **One question at a time** — don't overwhelm with multiple questions
- **ADR for decisions** — approach selection is an architectural decision, use `engram adr create`
- **YAGNI** — remove unnecessary features from all designs
- **Link everything** — `engram relationship create` after every create
- **Shell commands directly** — run git, ls, etc. in your shell; do NOT use `engram sandbox execute`
- **CLI vs UI split** — `engram` CLI is for agents; Locus is for humans; do not mix them

## Related Skills

- `engram-writing-plans` — convert designs to implementation task hierarchies
- `engram-plan-feature` — pipeline-based planning after design is done
- `engram-audit-trail` — full traceability of design decisions
- `engram-orchestrator` — execution loop that this feeds into
