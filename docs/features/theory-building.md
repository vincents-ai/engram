# Theory Building

> Based on Peter Naur's "Programming as Theory Building" (1985)

## Overview

A program is not just its source text; it is the living theory built in the mind of the developer. Without this theory, code becomes incomprehensible—even if every statement is understood.

Engram captures this by allowing agents to build formal **Theory** entities that represent their mental model of a system domain.

## What is a Theory?

A Theory consists of:

| Component | Description |
|-----------|-------------|
| **Domain** | The problem space being modeled |
| **Conceptual Model** | Core concepts (nouns/verbs) with definitions |
| **System Mapping** | How concepts map to code (file:line) |
| **Design Rationale** | The "why" behind design decisions |
| **Invariants** | What must always be true (the "laws") |

## CLI Usage

### Create a Theory

```bash
# Basic creation
./target/release/engram theory create "Workflow Engine"

# With agent and task
./target/release/engram theory create "Authentication" -a "the-architect" -t "task-123"

# From JSON
./target/release/engram theory create --json --json-file theory.json
```

### Add Concepts

```bash
./target/release/engram theory update --id <ID> --concept "User: A person who interacts with the system"
./target/release/engram theory update --id <ID> --concept "Session: A context for a user's interaction"
```

### Add System Mappings

```bash
./target/release/engram theory update --id <ID> --mapping "User: src/entities/user.rs:42 (struct User)"
./target/release/engram theory update --id <ID> --mapping "Session: src/entities/session.rs:28 (struct Session)"
```

### Add Design Rationale

```bash
./target/release/engram theory update --id <ID> --rationale "JWT Tokens: Stateless auth avoids session storage overhead"
```

### Add Invariants

```bash
./target/release/engram theory update --id <ID> --invariant "User email must be unique"
./target/release/engram theory update --id <ID> --invariant "Session expires after 24 hours"
```

### List and Show

```bash
# List all theories
./target/release/engram theory list

# Filter by agent
./target/release/engram theory list -a "the-architect"

# Show details with metrics
./target/release/engram theory show --id <ID> --show-metrics
```

## Agent Persona

Use the **The Theorist** agent to automatically extract theories from code:

```bash
# Use with your AI agent
prompts/agents/168-the-theorist.yaml
```

See [Pipeline: Theory Building](../pipelines/00-theory-building.yaml) for the full workflow.

## Integration with State Reflection

Theory entities work with [State Reflection](state-reflection.md) to detect when your mental model conflicts with reality.

When dissonance is detected (score ≥ 0.7), the theory must be updated before code fixes are attempted.
