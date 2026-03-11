# Reasoning

Reasoning captures the *decision-making process*—the "why" behind your choices.

## CLI Usage

```bash
# Record a decision
engram reasoning create \
  --title "Chose JWT for stateless auth" \
  --description "Session storage adds overhead. JWT allows stateless verification." \
  --task-id <TASK_ID>

# List reasoning
engram reasoning list
engram reasoning list --task-id <TASK_ID>
```

## Why It Matters

- **Context preservation**: Future developers understand why decisions were made
- **Agent onboarding**: AI agents can understand project history
- **Audit trail**: Complete record of decision rationale
