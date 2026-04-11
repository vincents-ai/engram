# Agent Instructions — engram

**Read the workspace AGENTS.md at `../AGENTS.md` first.** This file extends it with repo-specific detail for `engram`. Platform-wide rules (engram protocol, licensing, build conventions, commit format) are defined there and apply here without exception.

---

## Repo Role in ADP

`engram` is the distributed AI memory layer for the entire ADP platform. It is the single source of truth for all work across `agentic-repos`, `adp-web-ui`, and this repo itself. Every agent — human-driven or autonomous — stores tasks, reasoning, decisions, context, and session state here.

Breaking changes to the engram CLI surface or storage schema affect every other repo. Treat them as platform-breaking changes and document them as ADRs.

---

## North Star

`engram ask query "full-fidelity handoff"` — ADR-018

Any agent or human should be able to pick up exactly where another left off, with full fidelity, without the handoff party being present. Every decision, intent, and code change is traceable to engram.

---

## Starting a Session

1. Load the `engram-session-start` skill
2. Load the `engram-orchestrator` skill (if coordinating) or `engram-subagent-register` (if assigned a task UUID)
3. Run `engram ask query "<your task or goal>"` before touching anything

## Ending a Session

Load the `engram-session-end` skill. Never finish without it — unclosed sessions break `engram next` for the next agent.

---

## Commit Convention

```
<type>: <title> [<ENGRAM_TASK_UUID>]
```

The pre-commit hook rejects commits without a valid engram task UUID. Never use `--no-verify`.

---

## Key Queries

```bash
engram ask query "<text>"                  # search before acting — always
engram next                                # next priority action for this session
engram task show <UUID>                    # full task detail + relationships
engram relationship connected --entity-id <UUID> --max-depth 2
```

## Workflows

```bash
engram workflow list                       # SDLC, Feature Development, Regression Testing
```

## Sync

```bash
engram sync pull --remote origin           # pull before starting
engram sync push --remote origin           # push after ending session
```

---

## Key Conventions

- New skills go in `engram/skills/` — follow existing skill file structure
- New agent personas go in `engram/prompts/agents/`
- The `engram` binary and `locus` TUI are both produced from this crate
- CLI breaking changes require an ADR and a CHANGELOG entry
- Storage schema changes (git refs format) require a migration in `src/migration.rs` and an ADR
- Use `tracing` for all diagnostics — no `println!` / `eprintln!`
- All public API surface requires tests; use BDD scenarios in `tests/` for user-facing behaviour
