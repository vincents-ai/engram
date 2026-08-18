# Health

Health commands inspect repository and engram graph quality signals.

## CLI Usage

```bash
# Run all health checks and compute an overall score
engram health audit
engram health audit --store

# Repository signals
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

# Refresh knowledge metadata
engram health refresh-decay --lambda 0.01
```

## Output

Health commands emit human-readable tables by default. `audit --store` records the audit result as engram context so agents can use it in future planning.
