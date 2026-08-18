# Analytics

Analytics commands compute delivery and task-flow metrics from engram entities and repository history.

## CLI Usage

```bash
# DORA metrics over a time window
engram analytics dora --window-days 30

# Task duration report
engram analytics report

# Slowest tasks and bottlenecks
engram analytics bottleneck --top 10
```

## Metrics

- **Deployment frequency** — how often changes are delivered.
- **Lead time for changes** — time from task start to completion.
- **Change failure rate** — share of changes associated with failure signals.
- **Mean time to recovery** — time to recover from incidents or regressions.
- **Task duration and bottlenecks** — elapsed task time and slow-flow areas.
