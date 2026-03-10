# Knowledge

Knowledge represents reusable patterns and learnings that transcend individual tasks.

## CLI Usage

```bash
# Create knowledge
engram knowledge create \
  --title "PostgreSQL Connection Pooling" \
  --content "Use r2d2 pool for PostgreSQL connections. Max 10 connections per worker." \
  --type pattern

# Knowledge types: fact, pattern, rule, concept, procedure, heuristic

# List knowledge
engram knowledge list
engram knowledge list --type pattern
```

## Types

| Type | Description |
|------|-------------|
| **fact** | Verifiable statements |
| **pattern** | Recurring solutions |
| **rule** | Constraints or requirements |
| **concept** | Domain definitions |
| **procedure** | Step-by-step processes |
| **heuristic** | Guidelines based on experience |
