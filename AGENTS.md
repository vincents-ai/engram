# Agent Instructions

## Memory — Engram

All persistent memory lives in engram, not in conversation context.

Always use the engram-use-engram-memory skill before starting work.

Always use the engram-session-start skill before starting work and engram-session-end when finishing.

## Required Skills

Load these skills at the start of every session:
- engram-use-engram-memory: Full command reference for context, reasoning, ADR, and relationships
- engram-session-start: Run at the start of every session — syncs from remote, opens session, loads context
- engram-session-end: Run at the end of every session — closes tasks, generates summary, syncs to remote
- engram-orchestrator (if coordinating subagents)
- engram-subagent-register (if you are a subagent receiving a task UUID)

## Session Management

Always use the engram-session-start skill before starting work and engram-session-end when finishing.

- **Start**: `engram-session-start` — pulls from remote, opens a named session, loads prior context, surfaces next action
- **End**: `engram-session-end` — closes open tasks, generates handoff summary, pushes to remote, validates state

Never start work without running the session-start protocol. Never finish work without running the session-end protocol. An unclosed session loses its summary and breaks `engram next` context for the next agent.

## Commit Convention

Every commit message must follow this format:

  <type>: <title> [<ENGRAM_TASK_UUID>]

Examples:
  feat: add rate limiting middleware [abc-001]
  fix: normalise UTC timestamps in token validation [abc-002]

The pre-commit hook rejects commits that do not reference a valid engram task UUID.
Never use --no-verify to bypass the hook.

## Workflows

Available workflows (run `engram workflow list` to see all):
- <workflow-name>: <short description>

## Engram Quick Reference

  engram ask query "<text>"          # search before acting
  engram task create --title "..."   # anchor new work
  engram task update <UUID> --status in_progress
  engram next                        # get next priority action
  engram validate check              # verify setup before finishing
  engram sync list-remotes           # check if remote is configured
  engram sync pull --remote origin   # pull before starting work
  engram sync push --remote origin   # push after ending session
