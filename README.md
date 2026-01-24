# Engram

**Engram is a distributed memory system for AI agents and human operators.**

It acts as a "second brain" for your software projects, allowing you to capture context, plan tasks, track reasoning, and manage knowledge alongside your code. Unlike a simple todo list or a wiki, Engram is designed to be machine-readable, allowing AI agents to understand the full context of your work.

> **Note for Developers**: If you are looking for build instructions, Rust implementation details, or contributing guidelines, please see [DEVELOPMENT.md](DEVELOPMENT.md).

## Why Engram?

*   **Context Persistence**: Don't lose the "why" behind your code changes. Store reasoning chains linked directly to tasks.
*   **Agent Ready**: Your plans and context are structured so AI agents can instantly onboard and contribute effectively.
*   **Git-Backed**: Everything is stored as text files in your repository. It version controls your project management alongside your code.

## Quick Start

### Installation

(Assuming you have the binary available in your path or are running via `cargo run`)

```bash
# Initialize a new workspace
engram setup workspace

# Create your first agent identity
engram setup agent --name human --type operator
```

### Core Workflow

The Engram workflow follows a simple cycle: **Plan -> Execute -> Remember**.

#### 1. Plan
Break down your work into trackable tasks.

```bash
# Create a high-level task
engram task create --title "Add user authentication" --priority high

# Create subtasks
engram task create --title "Design database schema" --parent-id <TASK_ID>
```

#### 2. Execute & Document Context
As you work, save relevant information (snippets, docs, decisions).

```bash
# Store a piece of context (e.g., a documentation URL or snippet)
engram context create --title "OAuth2 Spec" --source "https://oauth.net/2/"

# Link it to your task
engram relationship create --source-id <TASK_ID> --target-id <CONTEXT_ID> --type references
```

#### 3. Remember (Reasoning)
Record *why* you made specific decisions. This is crucial for future maintainers (and AI agents).

```bash
# Record a decision
engram reasoning create --title "Chose JWT for stateless auth" --task-id <TASK_ID>
```

## Features at a Glance

*   **Tasks**: Hierarchical work items.
*   **Context**: Background info, docs, and snippets.
*   **Reasoning**: Decision logs and thought processes.
*   **Knowledge**: Reusable patterns and learnings.
*   **Workflows**: Define state machines for your processes.

## Documentation

*   [**User Guide**](docs/user-guide.md): A comprehensive guide for human operators.
*   [**Skills & Prompts**](SKILLS_AND_PROMPTS_INDEX.md): A library of AI agent capabilities and templates.
*   [**Using Engram (for Agents)**](docs/engram/skills/using-engram.md): How AI agents interact with the system.

## License

AGPL-3.0-or-later OR Commercial.
