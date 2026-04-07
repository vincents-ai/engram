# Engram

**Engram is a distributed memory system for AI agents and human operators.**

It acts as a "second brain" for your software projects — capturing context, plans, reasoning chains, and knowledge alongside your code. Unlike a wiki or a todo list, Engram is machine-readable: AI agents can query it with natural language, traverse its knowledge graph, and store decisions in a persistent, version-controlled store.

> **Developers**: For build instructions and contributing guidelines see [DEVELOPMENT.md](DEVELOPMENT.md).

## Why Engram?

- **Context Persistence** — Don't lose the "why" behind your code. Reasoning chains are stored, linked, and queryable.
- **Agent Ready** — Plans and context are structured so AI agents can onboard instantly and operate without losing state across sessions.
- **Theory Building** — Capture the mental model behind your code: concepts, design rationale, invariants (Naur, 1985).
- **Cognitive Dissonance** — Detect when theory conflicts with reality and evolve your understanding.
- **Git-Native Storage** — Data lives in `.git` using `refs/engram/`. Version-controlled, no directory pollution.

---

## Installing Engram for an LLM Agent

This section is for setting up Engram in a project so an AI agent (Claude, GPT-4, etc.) can use it as persistent memory. The install script below detects your OS, asks what you want to set up, and then runs only what you confirm.

### One-line install

```bash
curl -fsSL https://github.com/vincents-ai/engram/releases/latest/download/install.sh | bash
```

Or install manually for your platform:

**Linux (x86\_64)**
```bash
curl -L https://github.com/vincents-ai/engram/releases/latest/download/engram-linux-amd64.tar.gz | tar xz
chmod +x engram && sudo mv engram /usr/local/bin/
```

**Linux (static musl — for containers, NixOS, Alpine)**
```bash
curl -L https://github.com/vincents-ai/engram/releases/latest/download/engram-linux-musl-amd64.tar.gz | tar xz
chmod +x engram && sudo mv engram /usr/local/bin/
```

**macOS (Apple Silicon)**
```bash
curl -L https://github.com/vincents-ai/engram/releases/latest/download/engram-macos-arm64.tar.gz | tar xz
chmod +x engram && sudo mv engram /usr/local/bin/
```

**macOS (Intel)**
```bash
curl -L https://github.com/vincents-ai/engram/releases/latest/download/engram-macos-amd64.tar.gz | tar xz
chmod +x engram && sudo mv engram /usr/local/bin/
```

**Windows (x86\_64)**

Download `engram-windows-amd64.zip` from the [latest release](https://github.com/vincents-ai/engram/releases/latest), extract it, and add the directory to your `PATH`.

**Nix**
```bash
nix run github:vincents-ai/engram -- --help
```

**Cargo**
```bash
cargo install engram
```

### Verify the install

```bash
engram --version
```

---

### Bootstrap for an AI agent (interactive)

Once the binary is installed, run the bootstrap sequence in your project root. Each step asks for confirmation before running — nothing is applied automatically.

```bash
# 1. Initialise the workspace (creates .engram/ config in your project)
engram setup workspace

# 2. Register yourself or your agent (replace the name and type as appropriate)
#    Types: operator | implementation | quality_assurance | architecture
engram setup agent --name "Claude" --agent-type implementation

# 3. Install LLM skills (14 engram-specific skills for AI coding tools (OpenCode, Claude, Goose, etc.))
engram skills setup

# 4. Install all skills — 44 skills across planning, architecture, review, debugging, and more
engram setup skills

# 5. Install prompt library (agents/, pipelines/, compliance/)
#    Requires the prompts/ directory — clone with --recurse-submodules or supply --path
engram setup prompts

# 6. Install the commit-msg hook (enforces task linkage on every commit)
engram validate hook install
```

You do not need to run all of these. A minimal agent setup is steps 1–3. Step 6 is strongly recommended if humans are also committing to the repo.

**What each step does:**

| Step | Command | What it installs |
|------|---------|-----------------|
| 1 | `engram setup workspace` | `.engram/` directory + `config.yaml` with default agent roles |
| 2 | `engram setup agent` | Agent profile YAML in `.engram/agents/` |
| 3 | `engram skills setup` | 14 core engram skills to `~/.config/engram/skills/` |
| 4 | `engram setup skills` | 44 skills (all categories) to `~/.config/engram/skills/` |
| 5 | `engram setup prompts` | Agent, pipeline, and compliance prompts to `~/.config/engram/prompts/` |
| 6 | `engram validate hook install` | `commit-msg` hook that rejects commits without a task UUID |

### Skills installed by `engram skills setup`

These 14 skills are the core engram agent loop. They teach the LLM how to use engram correctly:

| Skill | Purpose |
|-------|---------|
| `engram-use-engram-memory` | Core memory pattern — search before acting, store everything |
| `engram-orchestrator` | Full agent execution loop |
| `engram-subagent-register` | How a subagent claims a task UUID and reports back |
| `engram-dispatching-parallel-agents` | Coordinate multiple agents on independent tasks |
| `engram-subagent-driven-development` | Execute plans one subagent per task with review gates |
| `engram-audit-trail` | Traceability — every decision stored and linked |
| `engram-delegate-to-agents` | Single-agent delegation pattern |
| `engram-brainstorming` | Design sessions stored as engram entities |
| `engram-writing-plans` | Implementation plans stored as task hierarchies |
| `engram-plan-feature` | Pipeline-template-based feature planning |
| `engram-requesting-code-review` | Review dispatch via task UUID |
| `engram-systematic-debugging` | Root cause investigation with reasoning chains |
| `engram-test-driven-development` | TDD with engram checkpoints at each phase |
| `engram-check-compliance` | Compliance audits stored as engram entities |

Skills are embedded in the binary at compile time. Running `engram skills setup` installs the version that shipped with the binary you downloaded — skills and CLI commands are always in sync.

---

## Quick Start (human operators)

```bash
# Initialize
engram setup workspace
engram setup agent --name "Your Name" --agent-type operator
engram validate hook install

# Plan
engram task create --title "Add user authentication" --priority high

# Document
engram context create --title "OAuth2 Spec" --source "https://oauth.net/2/" --content "..."
engram relationship create \
  --source-id <TASK_ID> --source-type task \
  --target-id <CONTEXT_ID> --target-type context \
  --relationship-type relates_to --agent "Your Name"

# Record reasoning
engram reasoning create \
  --title "Chose JWT for stateless auth" \
  --task-id <TASK_ID> \
  --content "JWT chosen: stateless, scales horizontally. Sessions rejected: stateful."

# Search
engram ask query "authentication design decisions"

# What to work on next
engram next
```

---

## Features

- **Tasks** — Hierarchical work items with priority, status, and parent/child linking
- **Context** — Background info, docs, code snippets, linked to tasks
- **Reasoning** — Decision logs and thought processes, linked to tasks and context
- **Relationships** — Typed graph edges connecting any two entities
- **ADRs** — Architecture Decision Records with numbered sequence
- **Theory Building** — Capture mental models (Naur, 1985): concepts, mappings, invariants
- **State Reflection** — Detect when theory conflicts with reality
- **Workflows** — State machines with transitions and guards
- **Sessions** — Work periods with theory binding
- **Skills** — Embedded agent playbooks installed to your LLM tool
- **Validation** — Quality gates and commit-message enforcement
- **NLQ** — Natural language queries via `engram ask query`

---

## Theory Building

Based on Peter Naur's "Programming as Theory Building" (1985):

```bash
engram theory create "User Authentication"
engram theory update --id <ID> --concept "User: A person who authenticates to the system"
engram theory update --id <ID> --mapping "User: src/entities/user.rs (struct User)"
engram theory update --id <ID> --invariant "User email must be unique"
```

## State Reflection

When code behaviour conflicts with theory:

```bash
engram reflect create --theory <THEORY_ID> --observed "Test failed" --trigger-type test_failure
engram reflect record-dissonance --id <ID> --description "Theory claims X but code does Y"
engram reflect requires-mutation --id <ID>
```

---

## Documentation

- [User Guide](user-guide.md) — Comprehensive guide for human operators
- [Using Engram (for Agents)](engram/skills/using-engram.md) — How AI agents interact with the system
- [DEVELOPMENT.md](DEVELOPMENT.md) — Build instructions and contributing guidelines
- [CHANGELOG.md](CHANGELOG.md) — Release history

---

## License

AGPL-3.0-or-later OR Commercial — dual-licensed for open source and commercial use.
