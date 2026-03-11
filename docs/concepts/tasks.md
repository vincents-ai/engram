# Tasks

Tasks are the fundamental unit of work in Engram. They are hierarchical and stateful.

## CLI Usage

```bash
# Create a task
engram task create --title "Implement OAuth2 Login" --priority high

# Create subtask
engram task create --title "Design DB Schema" --parent-id <PARENT_TASK_ID>

# Update status
engram task update --id <TASK_ID> --status in_progress

# List tasks
engram task list
engram task list --status pending
```

## Attributes

- **title**: Short description
- **description**: Detailed requirements
- **priority**: high, medium, low
- **status**: pending, in_progress, completed, blocked
- **parent_id**: For hierarchical tasks
