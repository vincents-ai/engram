# Workflows

Workflows define state machines for processes, enabling complex multi-step operations.

## CLI Usage

```bash
# Create workflow
engram workflow create --title "Code Review" --description "Code review process"

# Add states
engram workflow add-state --name pending --workflow-id <ID>
engram workflow add-state --name review --workflow-id <ID>
engram workflow add-state --name approved --workflow-id <ID>
engram workflow add-state --name rejected --workflow-id <ID>

# Add transitions
engram workflow add-transition --from pending --to review --workflow-id <ID>
engram workflow add-transition --from review --to approved --workflow-id <ID>
engram workflow add-transition --from review --to rejected --workflow-id <ID>

# Update state
engram workflow transition --to review --id <ID>
```

## Use Cases

- Code review processes
- Incident response stages
- Deployment pipelines
- Task approval flows
