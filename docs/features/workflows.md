# Workflows

Workflows define state machines for processes, enabling complex multi-step operations.

## CLI Usage

### Workflow Definitions

```bash
# Create workflow
engram workflow create --title "Code Review" --description "Code review process"

# Add states
engram workflow add-state <ID> --name pending --description "Pending review" --state-type start
engram workflow add-state <ID> --name review --description "Under review" --state-type in_progress
engram workflow add-state <ID> --name approved --description "Approved" --state-type done --is-final
engram workflow add-state <ID> --name rejected --description "Rejected" --state-type done --is-final

# Add transitions
engram workflow add-transition <ID> --name "start_review" --from-state <FROM> --to-state <TO>
engram workflow add-transition <ID> --name "approve" --from-state <FROM> --to-state <TO> --transition-type automatic

# Activate workflow
engram workflow activate <ID>
```

### Workflow Instances

```bash
# Start a workflow instance
engram workflow start <WORKFLOW_ID> --agent "the-architect"
engram workflow start <WORKFLOW_ID> --agent "the-architect" --entity-id <ID> --entity-type task
engram workflow start <WORKFLOW_ID> --agent "the-architect" --variables "key1=val1,key2=val2"
engram workflow start <WORKFLOW_ID> --agent "the-architect" --context-file context.json

# Execute a transition
engram workflow transition <INSTANCE_ID> --transition "approve" --agent "the-architect"

# Get instance status
engram workflow status <INSTANCE_ID>

# List instances
engram workflow instances
engram workflow instances --workflow-id <ID> --agent "the-architect" --running-only

# Cancel an instance
engram workflow cancel <INSTANCE_ID> --agent "the-architect" --reason "No longer needed"
```

### Actions and Queries

```bash
# Execute an action
engram workflow execute-action --action-type external_command --command "make test"

# Query available actions, guards, and transitions
engram workflow query-actions <WORKFLOW_ID>
engram workflow query-actions <WORKFLOW_ID> --state-id <STATE_ID>
```

## Use Cases

- Code review processes
- Incident response stages
- Deployment pipelines
- Task approval flows
