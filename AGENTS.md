# Agent Instructions

## North Star

`engram ask query "full-fidelity handoff"` — ADR-018

Any agent or human should be able to pick up exactly where another left off, with full fidelity, without the handoff party being present. Every decision, intent, and code change is traceable to engram.

## Starting a Session

1. Load the `engram-session-start` skill
2. Load the `engram-orchestrator` skill (if coordinating) or `engram-subagent-register` (if assigned a task UUID)
3. Run `engram ask query "<your task or goal>"` before touching anything

## Ending a Session

Load the `engram-session-end` skill. Never finish without it — unclosed sessions break `engram next` for the next agent.

## Commit Convention

```
<type>: <title> [<ENGRAM_TASK_UUID>]
```

The pre-commit hook rejects commits without a valid engram task UUID. Never use `--no-verify`.

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
