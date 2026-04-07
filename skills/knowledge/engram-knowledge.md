---
name: engram-knowledge
description: "Store and query durable, reusable project-wide knowledge in engram. Use when you discover facts, patterns, rules, concepts, procedures, or heuristics that should persist beyond a single task."
---

# Engram Knowledge

## Overview

Knowledge in engram is durable, reusable, project-wide information that should outlive any individual task or session. It is distinct from context (task-scoped observations) and reasoning (task-scoped logic chains).

**Rule:** When you discover something that any future agent should know — store it as knowledge.

## When to Use

Use this skill when:
- You discover a concrete fact about the system ("The rate limit is 100 req/s")
- You observe a recurring pattern ("Auth failures always log to /var/log/auth.log")
- You establish a rule that must be enforced ("Never commit without a task UUID")
- You define a domain concept ("Engram uses git refs for storage")
- You document a repeatable procedure ("How to rotate API keys")
- You identify a heuristic ("If a test takes >5s it is probably hitting the network")

## Command Reference

```bash
# Create a knowledge record
# required: --title, --content
# optional: --knowledge-type (fact|pattern|rule|concept|procedure|heuristic, default: fact)
#           --confidence (0.0-1.0, default: 0.8)
#           --source, --agent, --tags
engram knowledge create \
  --title "<short title>" \
  --content "<full content>" \
  --knowledge-type <TYPE> \
  --confidence <0.0-1.0> \
  --source "<file/url/observation>" \
  --agent "<your-name>" \
  --tags "<tag1,tag2>"

# List knowledge records; optional: --agent, --kind, --limit
engram knowledge list
engram knowledge list --kind rule
engram knowledge list --agent implementer --limit 20

# Show a specific knowledge record
engram knowledge show <UUID>

# Update a knowledge record; same flags as create
engram knowledge update <UUID> --confidence 0.95 --content "<updated content>"

# Delete a knowledge record
engram knowledge delete <UUID>
```

## Knowledge Types

Choose the type that best matches what you have discovered:

| Type | When to use | Example |
|------|-------------|---------|
| `fact` | A concrete, discovered truth about the system | "The API rate limit is 100 req/s" |
| `pattern` | A recurring pattern observed in the codebase or system | "All auth failures log to /var/log/auth.log" |
| `rule` | An enforceable rule that must always be followed | "Never commit without a task UUID" |
| `concept` | A domain concept that agents need to understand | "Engram uses git refs for storage" |
| `procedure` | A repeatable sequence of steps for a known operation | "How to rotate API keys: 1. generate... 2. update..." |
| `heuristic` | A rule of thumb that is usually right but not always | "If a test takes >5s it is probably hitting the network" |

Default is `fact` when no type is specified.

## Knowledge vs Context vs Reasoning

| | Knowledge | Context | Reasoning |
|---|---|---|---|
| **Scope** | Project-wide, permanent | Task-scoped | Task-scoped |
| **Reuse** | Any agent, any task | Usually one task | Usually one task |
| **Examples** | Facts, rules, procedures | Error logs, code snippets, observations | Logic chains, deductions, rationale |
| **Command** | `engram knowledge create` | `engram context create` | `engram reasoning create` |

**Use knowledge** when the information should be available to any future agent working on any future task.

**Use context** when you are storing a task-specific observation (a log output, a specific error message, a snippet).

**Use reasoning** when you are recording your logic chain for a specific decision on a specific task.

## The Pattern

### 1. Discover — Identify What Is Worth Preserving

Not every observation becomes knowledge. Ask:
- Would a future agent need to know this independently of the current task?
- Is this true across the project, not just in this task?
- Would forgetting this cause a future agent to make a mistake?

If yes to any of these — store it as knowledge.

### 2. Create with the Right Type and Confidence

```bash
engram knowledge create \
  --title "API rate limit: 100 req/s per key" \
  --content "The external payments API enforces 100 requests per second per API key. Exceeding this returns HTTP 429. Implement exponential backoff with jitter." \
  --knowledge-type fact \
  --confidence 0.95 \
  --source "src/payments/client.rs:47" \
  --agent "implementer" \
  --tags "api,rate-limit,payments"
# Returns: KNOWLEDGE_UUID
```

Set `--confidence` to reflect how certain you are:
- `1.0` — verified by test or official documentation
- `0.9` — strong evidence, observed directly
- `0.7` — inferred, likely correct
- `0.5` — uncertain, needs verification

### 3. Link to Task

Connect the knowledge record to the task where it was discovered:

```bash
engram relationship create \
  --source-id <TASK_UUID> --source-type task \
  --target-id <KNOWLEDGE_UUID> --target-type knowledge \
  --relationship-type relates_to \
  --agent "<your-name>"
```

### 4. Query Later

```bash
# List all rules (e.g., for a code review agent checking compliance)
engram knowledge list --kind rule

# List all facts from a specific agent
engram knowledge list --kind fact --agent implementer

# Retrieve a specific record
engram knowledge show <UUID>

# Natural language search across all knowledge
engram ask query "rate limit payments API"
```

## Example Workflow

```
[Task: debug slow tests]

# 1. Discover during investigation
# Tests taking >10s are all hitting a real Redis instance in CI

# 2. Create a heuristic (future agents should know this)
engram knowledge create \
  --title "Slow tests (>5s) are likely hitting real Redis in CI" \
  --content "Any test taking more than 5 seconds in CI is almost certainly connecting to the real Redis instance instead of a mock. Check for missing TestRedis mock setup in test fixtures." \
  --knowledge-type heuristic \
  --confidence 0.9 \
  --source "tests/integration/cache_test.rs" \
  --agent "debugger" \
  --tags "testing,redis,ci,performance"
# KNOWLEDGE_UUID = kno-001

# 3. Link to current task
engram relationship create \
  --source-id <TASK_UUID> --source-type task \
  --target-id kno-001 --target-type knowledge \
  --relationship-type relates_to --agent "debugger"

# 4. Future agent queries it
engram knowledge list --kind heuristic
engram ask query "slow tests Redis CI"
# Returns: kno-001 — actionable immediately
```

## Key Principles

1. **Store to share** — knowledge is for future agents, not just the current task.
2. **Choose the right type** — type determines how knowledge is queried and applied.
3. **Set confidence accurately** — low confidence (`0.5`) signals something needs verification.
4. **Link to tasks** — unlinked knowledge cannot be found by graph traversal.
5. **Update, don't duplicate** — use `engram knowledge update` when facts change.

## Related Skills

- `engram-use-engram-memory` — context and reasoning for task-scoped information
- `engram-orchestrator` — querying knowledge before dispatching agents
- `engram-systematic-debugging` — knowledge records preserve debugging findings
- `engram-audit-trail` — knowledge + reasoning + context = full traceability
