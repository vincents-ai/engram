# Sessions

Sessions track work periods and can bind to theories for context-aware execution.

## CLI Usage

```bash
# Start session
engram session start --agent "the-architect"

# Bind theory to session
engram session bind-theory <SESSION_ID> --theory <THEORY_ID>

# Trigger reflection (when dissonance detected)
engram session trigger-reflection <SESSION_ID>

# Resolve reflection
engram session resolve-reflection <SESSION_ID>

# End session
engram session end <SESSION_ID>
```

## Session States

| Status | Description |
|--------|-------------|
| **Active** | Normal operation |
| **Reflecting** | Theory must be updated before proceeding |
| **Completed** | Session ended |

## Theory Binding

When a session is bound to a theory:
- Theory invariants are injected into the session metadata
- Agents receive the theory context during execution
- Reflection state blocks code execution until theory is updated
