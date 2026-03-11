# Relationships

Relationships link entities together to create a knowledge graph.

## CLI Usage

```bash
# Create relationship
engram relationship create \
  --source-id <TASK_ID> \
  --target-id <CONTEXT_ID> \
  --type references

# Common types:
# - references: Task uses context
# - implements: Task fulfills requirement
# - depends_on: Task requires another task
# - justifies: Reasoning supports a decision
# - related: General connection

# List relationships
engram relationship list --source-id <ID>
engram relationship list --target-id <ID>
```

## Why Relationships Matter

- **Context graph**: See all context relevant to a task
- **Impact analysis**: Understand how changes affect other work
- **Traceability**: Link decisions to requirements
