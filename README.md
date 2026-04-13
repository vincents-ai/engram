# Engram

**Your AI agents forget everything between sessions. Engram fixes that.**

Every time you start a new AI coding session, your agent starts from zero. It doesn't remember why you chose that architecture, what you tried and rejected, or what's left to do. You re-explain the same context, re-answer the same questions, and re-trace the same dead ends.

Engram gives your AI agents **persistent memory** — a structured knowledge graph that lives alongside your code. Plans, decisions, reasoning chains, and context are stored in your git repo. When a new session starts, your agent queries engram and picks up exactly where the last one left off.

## What does it look like?

A new AI agent joins your project and runs this:

```
$ engram ask query "what's the current state of authentication work"

→ Task #a3f1 "Add OAuth2 login flow" — status: in_progress, priority: high
  Parent: #b7c2 "User authentication"
  Assigned to: Claude (implementation)

→ Reasoning #d4e9: "Chose JWT over sessions — stateless, scales horizontally"
  Linked to: Task #a3f1

→ Context #c2b8: "OAuth2 RFC 6749 spec summary" (linked to task)

→ Reasoning #e1f0: "Decided against Auth0 — cost, vendor lock-in. Rolling own with jose crate."
  Linked to: Task #a3f1

→ Task #a3f1 has 2 open subtasks:
  • #f6a3 "Implement token refresh rotation" — status: todo
  • #g7b4 "Add PKCE flow for mobile clients" — status: todo

Next priority: #f6a3 "Implement token refresh rotation"
```

No onboarding doc. No wiki. No "let me explain the codebase to you." The agent reads engram and gets to work.

## Why not just use a wiki / Notion / docs folder?

| | Wiki / Docs | Engram |
|---|---|---|
| **Who reads it?** | Humans only | Humans and AI agents |
| **How do you find things?** | Browse or full-text search | Natural language queries across a knowledge graph |
| **How does it stay current?** | Someone remembers to update it | Agents write to it as part of their workflow |
| **Where does it live?** | Separate tool / separate repo | In your git repo (`.git/refs/engram/`) |
| **Can agents pick up where others left off?** | No | Yes — that's the point |

## Install

**One line (Linux, macOS):**

```bash
curl -fsSL https://github.com/vincents-ai/engram/releases/latest/download/install.sh | bash
```

**Or manually:**

| Platform | Command |
|----------|---------|
| Linux (x86_64) | `curl -L …/engram-linux-amd64.tar.gz \| tar xz && chmod +x engram && sudo mv engram /usr/local/bin/` |
| Linux (musl/static) | `curl -L …/engram-linux-musl-amd64.tar.gz \| tar xz && chmod +x engram && sudo mv engram /usr/local/bin/` |
| macOS (Apple Silicon) | `curl -L …/engram-macos-arm64.tar.gz \| tar xz && chmod +x engram && sudo mv engram /usr/local/bin/` |
| macOS (Intel) | `curl -L …/engram-macos-amd64.tar.gz \| tar xz && chmod +x engram && sudo mv engram /usr/local/bin/` |
| Windows | Download `engram-windows-amd64.zip` from [releases](https://github.com/vincents-ai/engram/releases/latest) |
| Nix | `nix run github:vincents-ai/engram -- --help` |
| Cargo | `cargo install engram` |

Replace `…` with `https://github.com/vincents-ai/engram/releases/latest/download` in the manual commands above.

```bash
engram --version   # verify
```

## Quick Start (5 minutes)

```bash
# 1. Initialize engram in your project
cd your-project
engram setup workspace

# 2. Create your agent profile
engram setup agent --name "Your Name" --agent-type operator

# 3. Install the commit hook (requires task UUID in every commit message)
engram validate hook install
```

Now use it:

```bash
# Create a task
engram task create --title "Add user authentication" --priority high

# Store why you made a decision
engram reasoning create \
  --title "Chose JWT over sessions" \
  --task-id <TASK_ID> \
  --content "Stateless, scales horizontally, no server-side session store."

# Save reference material
engram context create --title "OAuth2 Spec" --source "https://oauth.net/2/" --content "..."

# Ask a question across all your stored knowledge
engram ask query "why did we choose JWT?"

# What should I work on next?
engram next
```

Every entity is linked. Ask a question about authentication, and you get the task, the reasoning, the context, the open subtasks, and who worked on it last.

## Setting up an AI agent

If you use an AI coding tool (Claude Code, OpenCode, Goose, etc.), engram ships **skills** that teach your agent how to use it:

```bash
# Core engram skills (14 skills — search before acting, store everything, etc.)
engram skills setup

# All skills (44 — planning, architecture, review, debugging, compliance, TDD)
engram setup skills
```

Minimal agent setup: `engram setup workspace` → `engram setup agent` → `engram skills setup`.

Your agent now knows to search engram before acting, store decisions after making them, and link everything together. When a new session starts, it runs `engram ask query "full-fidelity handoff"` and gets the full state of the project.

### Skills included

| Skill | What it teaches your agent |
|-------|---------------------------|
| `engram-use-engram-memory` | Core loop: search before acting, store everything |
| `engram-orchestrator` | Full agent execution loop with task management |
| `engram-subagent-register` | How a subagent claims a task and reports back |
| `engram-dispatching-parallel-agents` | Coordinate multiple agents on independent tasks |
| `engram-writing-plans` | Store implementation plans as task hierarchies |
| `engram-systematic-debugging` | Root cause investigation with reasoning chains |
| `engram-test-driven-development` | TDD with engram checkpoints |
| `engram-brainstorming` | Store design sessions as engram entities |
| `engram-audit-trail` | Every decision stored, linked, and traceable |
| `engram-check-compliance` | Compliance audits as engram entities |
| `engram-requesting-code-review` | Review dispatch via task UUID |
| `engram-plan-feature` | Template-based feature planning pipeline |
| `engram-delegate-to-agents` | Single-agent delegation pattern |
| `engram-subagent-driven-development` | Execute plans with review gates |

Skills are embedded in the binary — they're always version-matched to your CLI.

---

## What's inside

**Core entities** — the things you store and query:

- **Tasks** — Hierarchical work items with priority, status, and parent/child linking
- **Context** — Background info, docs, and code snippets linked to tasks
- **Reasoning** — Decision logs and thought chains linked to tasks and context
- **Relationships** — Typed graph edges connecting any two entities
- **Knowledge** — Reusable patterns that transcend individual tasks
- **ADRs** — Architecture Decision Records with numbered sequence

**Advanced capabilities:**

- **Theory Building** — Capture the mental model behind your code (concepts, design rationale, invariants) per Naur's "Programming as Theory Building" (1985)
- **State Reflection** — Detect when theory conflicts with reality and evolve your understanding
- **Workflows** — State machines with transitions, guards, side-effects, and commit policies
- **Sessions** — Work periods with theory binding, summaries, and zombie detection
- **NLQ** — Natural language queries with deep graph traversal via `engram ask query`
- **Sync** — Multi-agent coordination via `refs/engram/*` with conflict resolution
- **Analytics** — DORA metrics, task duration reports, workflow analytics
- **Locus TUI** — Terminal UI for browsing entities, relationships, and theories
- **Escalation** — Permission management with create/approve/deny workflow

## Theory Building in action

Engram implements Peter Naur's insight that programming is building a theory of the problem domain, not just writing code:

```bash
engram theory create "User Authentication"
engram theory update --id <ID> --concept "User: A person who authenticates to the system"
engram theory update --id <ID> --mapping "User: src/entities/user.rs (struct User)"
engram theory update --id <ID> --invariant "User email must be unique"

# When reality contradicts theory
engram reflect create --theory <THEORY_ID> --observed "Test failed" --trigger-type test_failure
engram reflect record-dissonance --id <ID> --description "Theory claims X but code does Y"
```

## Documentation

- [User Guide](docs/user-guide.md) — Comprehensive guide for human operators
- [Using Engram (for Agents)](engram/skills/using-engram.md) — How AI agents interact with the system
- [DEVELOPMENT.md](DEVELOPMENT.md) — Build instructions and contributing guidelines
- [CHANGELOG.md](CHANGELOG.md) — Release history

## License

AGPL-3.0-or-later OR Commercial — dual-licensed for open source and commercial use.
