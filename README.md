# Engram

**Engram is a distributed memory system for AI agents and human operators.**

It acts as a "second brain" for your software projects, allowing you to capture context, plan tasks, track reasoning, and manage knowledge alongside your code. Unlike a simple todo list or a wiki, Engram is designed to be machine-readable, allowing AI agents to understand the full context of your work.

> **Note for Developers**: If you are looking for build instructions, Rust implementation details, or contributing guidelines, please see [DEVELOPMENT.md](DEVELOPMENT.md).

## Why Engram?

*   **Context Persistence**: Don't lose the "why" behind your code changes. Store reasoning chains linked directly to tasks.
*   **Agent Ready**: Your plans and context are structured so AI agents can instantly onboard and contribute effectively.
*   **Theory Building**: Capture the mental model behind your code—concepts, design rationale, and invariants.
*   **Cognitive Dissonance**: Detect when theory conflicts with reality and evolve your understanding.
*   **Git-Native Storage**: Engram stores data directly in your `.git` database using custom references (`refs/engram/`). This means your project management is version-controlled without polluting your working directory.

## Quick Start

### 1. Setup

Run these commands in your project root to initialize Engram and set up your identity.

```bash
# Initialize workspace
engram setup workspace

# Create your profile (replace name as needed)
engram setup agent --name "Human Operator" --agent-type operator

# (Optional) Install git hook to enforce task linking
engram validate hook install
```

### 2. Core Workflow

The Engram workflow follows a simple cycle: **Plan -> Execute -> Remember**.

#### Plan
Break down your work into trackable tasks.

```bash
# Create a high-level task
engram task create --title "Add user authentication" --priority high
```

#### Execute & Document
As you work, save relevant information.

```bash
# Store context (e.g., docs)
engram context create --title "OAuth2 Spec" --source "https://oauth.net/2/"

# Link it to your task
engram relationship create --source-id <TASK_ID> --target-id <CONTEXT_ID> --type references
```

#### Remember (Reasoning)
Record *why* you made specific decisions.

```bash
# Record a decision
engram reasoning create --title "Chose JWT for stateless auth" --task-id <TASK_ID>
```

## Features at a Glance

*   **Tasks**: Hierarchical work items.
*   **Context**: Background info, docs, and snippets.
*   **Reasoning**: Decision logs and thought processes.
*   **Knowledge**: Reusable patterns and learnings.
*   **Theory Building**: Capture mental models from code (Naur, 1985).
*   **State Reflection**: Detect cognitive dissonance and evolve theories.
*   **Workflows**: Define state machines for your processes.
*   **Sessions**: Track work periods with theory binding.

## Documentation

*   [**User Guide**](docs/user-guide.md): A comprehensive guide for human operators.
*   [**Book (mdBook)**](book/index.html): Full documentation with Theory Building and State Reflection guides.
*   [**Using Engram (for Agents)**](docs/engram/skills/using-engram.md): How AI agents interact with the system.

## Theory Building

Based on Peter Naur's "Programming as Theory Building" (1985), Engram allows agents to capture the mental model behind code:

```bash
# Create a theory about a domain
engram theory create "User Authentication"

# Add concepts
engram theory update --id <ID> --concept "User: A person who authenticates to the system"

# Add system mappings
engram theory update --id <ID> --mapping "User: src/entities/user.rs (struct User)"

# Add invariants
engram theory update --id <ID> --invariant "User email must be unique"
```

## State Reflection

When code behavior conflicts with your theory, create a reflection:

```bash
# Create a reflection when dissonance is detected
engram reflect create --theory <THEORY_ID> --observed "Test failed" --trigger-type test_failure

# Record the dissonance
engram reflect record-dissonance --id <ID> --description "Theory claims X but code does Y"

# Check if theory mutation is required (dissonance >= 0.7)
engram reflect requires-mutation --id <ID>
```

## License

AGPL-3.0-or-later OR Commercial - dual-licensed for open source and commercial use
