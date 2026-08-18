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

## Reports

| Command | Output |
|---------|--------|
| `dora` | Deployment frequency, lead time for changes, change failure rate, and MTTR. |
| `report` | Task duration summary from task lifecycle timestamps. |
| `bottleneck` | Slowest tasks and blocked-flow indicators. |

## Metrics

- **Deployment frequency** — how often changes are delivered.
- **Lead time for changes** — elapsed time from change/task start to delivery.
- **Change failure rate** — share of changes associated with failure signals.
- **Mean time to recovery** — time to recover from incidents or regressions.
- **Task duration and bottlenecks** — elapsed task time and slow-flow areas.

Analytics output is intended for both human review and agent planning: agents can use bottleneck and DORA signals to prioritize remediation work.
