# CLI Reference

## Core Commands

### Task
```bash
engram task create --title <TEXT> [--priority high|medium|low] [--parent-id <ID>]
engram task list [--status <STATUS>] [--agent <NAME>]
engram task show --id <ID>
engram task update --id <ID> --status <STATUS>
engram task delete --id <ID>
```

### Context
```bash
engram context create --title <TEXT> [--source <URL>] [--content <TEXT>]
engram context list [--agent <NAME>]
engram context show --id <ID>
engram context delete --id <ID>
```

### Reasoning
```bash
engram reasoning create --title <TEXT> --description <TEXT> [--task-id <ID>]
engram reasoning list [--task-id <ID>]
engram reasoning show --id <ID>
engram reasoning delete --id <ID>
```

### Knowledge
```bash
engram knowledge create --title <TEXT> [--content <TEXT>] [--type fact|pattern|rule|concept|procedure|heuristic]
engram knowledge list [--type <TYPE>] [--agent <NAME>]
engram knowledge show --id <ID>
engram knowledge update --id <ID> --field <FIELD> --value <VALUE>
engram knowledge delete --id <ID>
```

### Theory
```bash
engram theory create <DOMAIN> [--agent <NAME>] [--task <ID>] [--json]
engram theory list [--agent <NAME>] [--domain <DOMAIN>] [--limit <N>]
engram theory show --id <ID> [--show-metrics]
engram theory update --id <ID> [--concept X:Y] [--mapping X:Y] [--rationale X:Y] [--invariant X]
engram theory delete --id <ID>
```

### Reflection
```bash
engram reflect create --theory <ID> --observed <TEXT> --trigger-type <TYPE> [--agent <NAME>]
engram reflect list [--agent <NAME>] [--severity <LEVEL>]
engram reflect show --id <ID>
engram reflect record-dissonance --id <ID> --description <TEXT>
engram reflect propose-update --id <ID> --update <TEXT>
engram reflect resolve --id <ID> [--new-theory-id <ID>]
engram reflect requires-mutation --id <ID>
```

### Session
```bash
engram session start --agent <NAME>
engram session list
engram session show --id <ID>
engram session bind-theory --id <SESSION_ID> --theory <THEORY_ID>
engram session trigger-reflection --id <ID>
engram session resolve-reflection --id <ID>
engram session end --id <ID>
```

### Workflow
```bash
engram workflow create --title <TEXT> [--description <TEXT>]
engram workflow add-state --name <NAME> --workflow-id <ID>
engram workflow add-transition --from <NAME> --to <NAME> --workflow-id <ID>
engram workflow transition --to <NAME> --id <ID>
engram workflow list
engram workflow show --id <ID>
```

### Relationship
```bash
engram relationship create --source-id <ID> --target-id <ID> --type <TYPE>
engram relationship list [--source-id <ID>] [--target-id <ID>]
engram relationship delete --id <ID>
```

## Setup Commands
```bash
engram setup workspace
engram setup agent --name <NAME> --type <TYPE>
engram validate hook install
engram validate hook status
```
