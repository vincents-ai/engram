# FAQ

## What is Engram?

Engram is a distributed memory system for AI agents and human operators. It stores context, reasoning, and knowledge in Git refs rather than files.

## How is data stored?

Data is stored in `.git/refs/engram/` as JSON files. This keeps your working directory clean while maintaining version control.

## Can I use Engram without Git?

Yes, use memory-only storage for testing or non-Git projects.

## What's the difference between Context and Knowledge?

- **Context**: Raw materials for a specific task (docs, snippets, logs)
- **Knowledge**: Reusable patterns that transcend tasks

## What is Theory Building?

Based on Peter Naur's "Programming as Theory Building" (1985). It captures the mental model developers have of their code—why it was designed a certain way, what invariants must hold, etc.

## How does State Reflection work?

When code behavior conflicts with an agent's theory, the agent creates a StateReflection. If dissonance is high (≥0.7), the theory must be updated before code can be fixed.

## How does sync work between agents?

`engram sync` uses `refs/engram/*` in git to share entities between agents. Each entity write creates a versioned sidecar ref. Pull merges remote changes with conflict resolution; push exports local refs. Use `engram sync status` to see per-type sync state.

## What does `engram health` check?

`engram health check` runs git-based workspace diagnostics: entity consistency validation, orphan ref detection, and workspace integrity checks. Use it to verify your engram data is clean after merges or manual edits.

## What is `engram doc`?

`engram doc` generates documentation from your engram data: entity docs, CLI reference, mdBook builds, and NLQ-powered doc search. It bridges the gap between stored knowledge and readable documentation.

## How do I use escalation?

`engram escalation create` requests permission for blocked operations (e.g., writes outside sandbox). Agents with approval rights can `engram escalation approve` or `deny`. This enables safe multi-agent workflows where not all agents have equal permissions.

## What are analytics and DORA metrics?

`engram analytics` reports on engineering effectiveness using DORA metrics: deployment frequency, lead time for changes, change failure rate, and mean time to restore. It also provides task duration reports and workflow stage analysis.

## What are personas and lessons?

Personas are structured expert profiles with CoV (Context of Value), FAP (Framing, Approach, Principle), and OV (Operational View) sections. Lessons capture teachable insights linked to personas. Use `engram persona create/list/show` and `engram lesson create/list/show` to manage them.

## How does task archiving work?

`engram task archive <ID>` moves completed tasks out of active queries, reducing noise in `engram next` and list results. Archived tasks are still queryable with `--include-archived`.

## What is the Locus TUI?

Locus is a terminal UI for browsing engram data without leaving your terminal. It provides views for tasks, contexts, reasoning, ADRs, theories, and sync status. Navigate with keyboard shortcuts and auto-refresh to see live changes.

## How do I use the commit-msg hook?

`engram validate hook install` adds a git commit-msg hook that rejects commits without a valid engram task UUID. This ensures every commit is traceable to a task. Never use `--no-verify` to bypass it.
