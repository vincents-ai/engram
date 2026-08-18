# Sync

Engram sync commands coordinate memory state across agents, branches, and configured remotes.

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

# Pull, push, or do both in order
engram sync pull --remote origin --branch main
engram sync push --remote origin --branch main
engram sync both --remote origin --branch main

# Branch helpers
engram sync create-branch feature/my-agent --agent alice
engram sync switch-branch feature/my-agent
engram sync list-branches --all
engram sync delete-branch feature/my-agent --force
```

## Conflict Strategies

- `latest_wins` — keep the newest entity update.
- `intelligent_merge` / `merge_with_conflict_resolution` — merge compatible changes and report conflicts.
- `manual` — surface conflicts for explicit resolution.

## Notes

Pull before pushing shared state so remote updates are incorporated before publishing local changes.
