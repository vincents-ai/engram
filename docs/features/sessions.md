# Sessions

Sessions track work periods and can bind to theories for context-aware execution.

## CLI Usage

```bash
# Start session
engram session start --name "the-architect"

# Show session status (with optional metrics)
engram session status --id <SESSION_ID>
engram session status --id <SESSION_ID> --metrics

# End session (with optional summary generation)
engram session end --id <SESSION_ID>
engram session end --id <SESSION_ID> --generate-summary

# List sessions
engram session list
engram session list --agent "the-architect" --since 7d --limit 20

# Detect zombie sessions (started but never ended)
engram session zombies
engram session zombies --max-age-hours 48 --check-git

# Summarize recent sessions
engram session summaries
engram session summaries --agent "the-architect" --since 2024-01-01
```

## Session States

| Status | Description |
|--------|-------------|
| **Active** | Normal operation |
| **Paused** | Temporarily suspended |
| **Reflecting** | Theory must be updated before proceeding |
| **Completed** | Session ended normally |
| **Cancelled** | Session ended without completion |

## Theory Binding

When a session is bound to a theory:
- Theory invariants are injected into the session metadata
- Agents receive the theory context during execution
- Reflection state blocks code execution until theory is updated
