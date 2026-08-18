# Health

Health commands inspect repository quality, collaboration risk, and engram graph consistency.

## CLI Usage

```bash
# Run all health checks and compute an overall score
engram health audit
engram health audit --store

# Repository history signals
engram health churn --top 20
engram health bus-factor
engram health bug-clusters --top 20
engram health velocity
engram health firefighting
engram health commit-size
engram health test-signal

# Entity graph checks
engram health score
engram health orphans
engram health consistency

# Refresh knowledge decay weights and citation counts
engram health refresh-decay --lambda 0.01
```

## Checks

| Command | Purpose |
|---------|---------|
| `audit` | Runs a composite audit and reports an overall score. |
| `churn` | Finds files with the most changes. |
| `bus-factor` | Estimates contributor concentration risk. |
| `bug-clusters` | Finds files commonly touched by bug/fix commits. |
| `velocity` | Summarizes commit velocity over time. |
| `firefighting` | Detects revert, rollback, hotfix, and incident signals. |
| `commit-size` | Reports average change size. |
| `test-signal` | Reports test-related commit ratio. |
| `orphans` | Finds entities with no relationships. |
| `consistency` | Checks git-ref-backed entity storage consistency. |
| `refresh-decay` | Updates knowledge freshness/citation metadata. |

## Stored Audits

`engram health audit --store` records the audit result as an engram context entity so future agents can use it during planning and handoff.
