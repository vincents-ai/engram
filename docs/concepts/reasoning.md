# Reasoning

Reasoning captures the *decision-making process*—the "why" behind your choices.

## CLI Usage

```bash
# Record a decision
engram reasoning create \
  --title "Chose JWT for stateless auth" \
  --description "Session storage adds overhead. JWT allows stateless verification." \
  --task-id <TASK_ID>

# Add structured steps, optionally with IBIS metadata
engram reasoning add-step <REASONING_ID> \
  --content "JWT keeps API nodes stateless" \
  --ibis-type idea
engram reasoning add-step <REASONING_ID> \
  --content "Token revocation needs explicit design" \
  --ibis-type con \
  --ibis-polarity con

# Conclude and inspect
engram reasoning conclude <REASONING_ID> --conclusion "Use JWT with short TTL and refresh rotation"
engram reasoning list
engram reasoning list --task-id <TASK_ID>
engram reasoning show <REASONING_ID>
engram reasoning history <REASONING_ID>
engram reasoning export <REASONING_ID> --output reasoning.json

# Search by IBIS metadata or keyword
engram reasoning search --ibis-type question
engram reasoning search --polarity pro --keyword jwt
```

## IBIS Support

Reasoning steps can carry Issue-Based Information System metadata:

| Field | Meaning |
|-------|---------|
| `ibis_type` | `question`, `idea`, `pro`, `con`, `reference`, or `note` |
| `ibis_polarity` | `pro` or `con` for argumentative positions |
| `ibis_parent_id` | Optional parent step ID for a reasoning hierarchy |

The entity model also supports `ibis_mode` and captured `positions`; positions are flattened into normal reasoning steps for compatibility with existing consumers.

## Provenance and Event History

Reasoning chains maintain event history for important lifecycle changes. Events include automatic storage activity and agent attribution, so later agents can inspect who changed a chain and why.

## Why It Matters

- **Context preservation**: Future developers understand why decisions were made
- **Agent onboarding**: AI agents can understand project history
- **Audit trail**: Complete record of decision rationale
- **Argument mapping**: IBIS metadata separates questions, candidate ideas, supporting arguments, objections, references, and notes
