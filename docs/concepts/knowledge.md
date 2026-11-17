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

# Inspect/update/delete
engram knowledge show <KNOWLEDGE_ID>
engram knowledge update <KNOWLEDGE_ID> --content "Updated guidance"
engram knowledge delete <KNOWLEDGE_ID>
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

## Decay, Usage, and Citations

Knowledge entities track freshness and usefulness metadata:

| Field | Meaning |
|-------|---------|
| `usage_count` | Number of times the knowledge item has been referenced |
| `citation_count` | Number of inbound citations/relationships |
| `last_used_at` | Last time the knowledge was used |
| `decay_weight` | Optional exponential decay score, clamped between `0.0` and `1.0` |

Refresh decay metadata with:

```bash
engram health refresh-decay --lambda 0.01
```

A lower decay weight indicates older or less recently used knowledge. Agents should prefer high-confidence, highly cited, recently used items when multiple knowledge entries conflict.
