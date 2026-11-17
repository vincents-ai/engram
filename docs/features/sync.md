# Sync

Engram sync coordinates memory state across agents, branches, and configured remotes. It is designed for distributed agent work where each agent may maintain local state and later reconcile with shared refs.

## CLI Usage

```bash
# Synchronize local agent state
engram sync sync --agents alice,bob --strategy latest_wins
engram sync sync --agents alice,bob --strategy intelligent_merge --dry-run

# Configure and inspect remotes
engram sync add-remote origin https://example.com/org/repo.git --branch main
engram sync list-remotes
engram sync status --remote origin
engram sync status --remote origin --json

# Pull, push, or do both in safe order
engram sync pull --remote origin --branch main
engram sync push --remote origin --branch main
engram sync both --remote origin --branch main

# Branch helpers
engram sync create-branch feature/my-agent --agent alice
engram sync switch-branch feature/my-agent
engram sync list-branches --all
engram sync delete-branch feature/my-agent --force

# Import existing git remotes into engram sync config
engram sync import-git-remotes

# Resolve reported remote conflicts
engram sync resolve --remote origin --strategy latest_wins
```

## Conflict Strategies

| Strategy | Use when |
|----------|----------|
| `latest_wins` | Timestamp order is authoritative. |
| `intelligent_merge` | Compatible entity fields can be merged. |
| `merge_with_conflict_resolution` | Backward-compatible alias used by older automation. |
| `manual` | A human/agent should inspect conflicts explicitly. |

## Remote Status

`engram sync status --remote <name>` reports local/remote divergence, pending pushes, pending pulls, and conflicts. Use `--json` for automation.

## Recommended Flow

1. `engram sync pull --remote origin`
2. Resolve conflicts if reported.
3. Run local validation.
4. `engram sync push --remote origin`

`engram sync both --remote origin` performs the pull-then-push sequence for routine use.
