---
name: engram-theory-building
description: "Capture and evolve your mental model of a codebase as a Theory entity. Use when starting work on an unfamiliar domain, before onboarding another agent, or any time tacit knowledge would otherwise be lost."
---

# Theory Building

## Overview

Peter Naur (1985) argued that the most important product of programming is not the code — it is the living theory in the programmer's mind: why the system is shaped the way it is, what real-world concepts it models, how those concepts map to structures in code, and what constraints must always hold.

When that theory exists only in someone's head, it dies when they leave. When an agent holds it only in its context window, it dies at the end of the session.

`engram theory` gives that theory a persistent, queryable home. A `Theory` entity captures the conceptual model, the code mappings, the design rationale, and the invariants for a named domain. Each time you learn something that changes your understanding, you update it and the iteration count increments — giving you a full evolution history.

## When to Use

- Starting work on an unfamiliar part of the codebase
- Before handing a domain off to another agent or developer
- After a significant surprise (something worked differently than you expected)
- Before a spike investigation — record your current theory first, then see what changes
- After a large refactor — the old theory needs to be replaced, not just patched

## The Pattern

### Step 1: Search for an existing theory

Before creating anything, check whether a theory for this domain already exists:

```bash
engram ask query "theory <domain-name>"
engram theory list --domain "<domain-name>"
```

If one exists, `engram theory show --id <ID>` to read it. Update rather than duplicate.

### Step 2: Create the theory

```bash
engram theory create "<domain-name>" --agent "<your-name>" --output json
# Returns: THEORY_ID
```

Use a domain name that describes the conceptual territory, not an implementation module name.
Good: `"entity-storage"`, `"task-lifecycle"`, `"auth-session"`.
Avoid: `"src/storage.rs"`, `"main module"`.

### Step 3: Populate concepts, mappings, rationale, invariants

Each flag can be repeated. Use `key:value` format for concepts, mappings, and rationale.

```bash
# Concepts — pure domain ideas, no implementation detail
engram theory update --id <THEORY_ID> \
  --concept "Theory:A named domain model with concepts, mappings, rationale, and invariants" \
  --concept "StateReflection:A recorded gap between observed behaviour and the current theory"

# System mappings — how each concept maps to actual code
engram theory update --id <THEORY_ID> \
  --mapping "Theory:src/entities/theory.rs::Theory struct" \
  --mapping "StateReflection:src/entities/state_reflection.rs::StateReflection struct"

# Design rationale — the *why* behind key choices
engram theory update --id <THEORY_ID> \
  --rationale "dissonance_score is f64 not enum:Allows gradual accumulation of evidence before a binary mutation decision is required" \
  --rationale "iteration_count on Theory:Tracks how many times the theory has evolved, giving a signal of domain stability"

# Invariants — facts that must always be true
engram theory update --id <THEORY_ID> \
  --invariant "A Theory must have at least one concept before it can be linked to a StateReflection" \
  --invariant "dissonance_score is always clamped to 0.0–1.0"
```

### Step 4: Link the theory to the active task

```bash
engram relationship create \
  --source-id <TASK_ID> --source-type task \
  --target-id <THEORY_ID> --target-type context \
  --relationship-type documents --agent "<your-name>"
```

### Step 5: Bind to the active session (optional but recommended)

```bash
engram session bind-theory --id <SESSION_ID> --theory <THEORY_ID>
```

Once bound, the session can enter a `Reflecting` status when dissonance is detected, which blocks further code changes until the theory is updated.

## Command Reference

| Command | Description |
|---|---|
| `engram theory create <DOMAIN>` | Create a new theory for a named domain |
| `engram theory list` | List all theories; filter with `--domain` or `--agent` |
| `engram theory show --id <ID>` | Show full theory detail |
| `engram theory show --id <ID> --show-metrics` | Show theory with counts for concepts/mappings/invariants |
| `engram theory update --id <ID>` | Add concepts, mappings, rationale, invariants |
| `engram theory apply-reflection --theory-id <ID> --reflection-id <ID> --updates-file <PATH>` | Apply a JSON map of theory updates derived from a StateReflection |
| `engram theory delete --id <ID>` | Delete a theory |

### `engram theory update` flags

| Flag | Format | Effect |
|---|---|---|
| `--concept "name:definition"` | `key:value` | Add a domain concept |
| `--mapping "concept:code-location"` | `key:value` | Map concept to implementation |
| `--rationale "decision:reason"` | `key:value` | Record a design rationale |
| `--invariant "statement"` | plain string | Add an invariant |

All flags can be repeated in a single command.

## When to Iterate

Bump the iteration by calling `engram theory update` whenever:

- You discover a concept that wasn't in the model
- A mapping changes (refactor, rename, move)
- A rationale turns out to be wrong
- An invariant is violated or newly established
- A `StateReflection` with `requires_theory_mutation` has been resolved

The iteration count is your signal of domain stability. A theory at iteration 1 after six months of active development is a warning sign.

## Example

Domain: the `session` subsystem.

```bash
# Step 1 — search
engram ask query "theory session"
# No results — create fresh

# Step 2 — create
engram theory create "session-lifecycle" --agent "orchestrator" --output json
# THEORY_ID = a1b2c3d4-...

# Step 3 — populate
engram theory update --id a1b2c3d4 \
  --concept "Session:A bounded unit of agent work with a defined start, active state, and end" \
  --concept "ActiveTheory:The Theory currently governing the session's domain understanding" \
  --concept "ReflectionStatus:Whether the session is currently blocked on a theory mutation"

engram theory update --id a1b2c3d4 \
  --mapping "Session:src/entities/session.rs::Session struct" \
  --mapping "ActiveTheory:src/entities/session.rs::active_theory_id field" \
  --mapping "ReflectionStatus:src/entities/session.rs::SessionStatus::Reflecting variant"

engram theory update --id a1b2c3d4 \
  --rationale "session holds theory_ids not just active_theory_id:Sessions evolve through multiple theory iterations; full history is retained for audit" \
  --rationale "Reflecting status blocks code changes:Prevents fixing symptoms before the underlying mental model is corrected"

engram theory update --id a1b2c3d4 \
  --invariant "A session in Reflecting status cannot have code changes committed against it" \
  --invariant "active_theory_id is always the most recently bound theory, not the highest-iteration one"

# Step 4 — link to task
engram relationship create \
  --source-id <CURRENT_TASK_ID> --source-type task \
  --target-id a1b2c3d4 --target-type context \
  --relationship-type documents --agent "orchestrator"

# Step 5 — bind to session
engram session bind-theory --id <SESSION_ID> --theory a1b2c3d4

# Verify
engram theory show --id a1b2c3d4 --show-metrics
```

## Related Skills

- `engram-state-reflection` — what to do when observed behaviour contradicts your theory
- `engram-use-engram-memory` — foundational memory discipline all skills build on
- `engram-audit-trail` — recording the full history of decisions and actions
- `engram-spike-investigation` — time-boxed research; always capture a theory before and after
- `engram-adr` — record architectural decisions that shaped the design rationale
