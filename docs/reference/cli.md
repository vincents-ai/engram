# CLI Reference

This reference reflects the current top-level `engram` CLI command surface. Run `engram <command> --help` for full flag detail and current defaults.

## Setup and Onboarding

```bash
engram setup workspace
engram setup agent --name <NAME> --type <TYPE>
engram setup skills
engram setup prompts

engram guide onboarding
engram guide getting-started
engram guide examples

engram info
engram next
```

## Core Memory Entities

```bash
# Tasks
engram task create --title <TEXT> [--priority high|medium|low|critical]
engram task list [--status <STATUS>] [--agent <NAME>]
engram task show <ID>
engram task update <ID> --status <STATUS>
engram task archive <ID>
engram task archive-bulk [--status done]
engram task resolve <ID>
engram task create-batch --file <JSON_FILE>

# Context
engram context create --title <TEXT> [--source <URL>] [--content <TEXT>]
engram context list [--agent <NAME>]
engram context show <ID>
engram context update <ID> [--title <TEXT>] [--content <TEXT>]
engram context delete <ID>

# Reasoning
engram reasoning create --title <TEXT> [--description <TEXT>] [--task-id <ID>]
engram reasoning add-step <ID> --content <TEXT>
engram reasoning conclude <ID> --conclusion <TEXT>
engram reasoning list [--task-id <ID>]
engram reasoning show <ID>
engram reasoning history <ID>
engram reasoning export <ID> --output <FILE>
engram reasoning search [--ibis-type question|idea|pro|con|reference|note] [--polarity pro|con]
engram reasoning log <ID> --event-type <TYPE> --content <TEXT>
engram reasoning delete <ID>

# Knowledge
engram knowledge create --title <TEXT> [--content <TEXT>] [--type fact|pattern|rule|concept|procedure|heuristic]
engram knowledge list [--type <TYPE>] [--agent <NAME>]
engram knowledge show <ID>
engram knowledge update <ID> [--title <TEXT>] [--content <TEXT>]
engram knowledge delete <ID>

# Relationships
engram relationship create --source-id <ID> --target-id <ID> --type <TYPE>
engram relationship list [--source-id <ID>] [--target-id <ID>]
engram relationship get <ID>
engram relationship delete <ID>
engram relationship find-path --from <ID> --to <ID>
engram relationship connected --entity-id <ID> [--max-depth <N>]
engram relationship stats
```

## Process, Governance, and Decisions

```bash
# ADRs
engram adr create --title <TEXT> --context <TEXT>
engram adr list
engram adr get <ID>
engram adr update <ID>
engram adr accept <ID>
engram adr add-alternative <ID>
engram adr add-stakeholder <ID>
engram adr delete <ID>

# Workflows
engram workflow create --title <TEXT> [--description <TEXT>]
engram workflow add-state <WORKFLOW_ID> --name <NAME>
engram workflow add-transition <WORKFLOW_ID> --from-state <FROM> --to-state <TO>
engram workflow activate <WORKFLOW_ID>
engram workflow start <WORKFLOW_ID> [--entity-id <ID>] [--agent <NAME>]
engram workflow transition <INSTANCE_ID> --transition <NAME>
engram workflow status <INSTANCE_ID>
engram workflow instances [--workflow-id <ID>] [--running-only]
engram workflow cancel <INSTANCE_ID>
engram workflow execute-action --action-type <TYPE>
engram workflow query-actions <WORKFLOW_ID>

# Rules, standards, compliance
engram rule create|get|update|delete|list|execute
engram standard create|get|update|delete|list|add-requirement
engram compliance create|list|show|update|delete
```

## Sessions, Theories, and Reflection

```bash
engram session start --name <NAME> [--auto-detect]
engram session status --id <SESSION_ID> [--metrics]
engram session end --id <SESSION_ID> [--generate-summary]
engram session list [--agent <NAME>] [--since <DATE_OR_DURATION>]
engram session zombies [--max-age-hours <N>] [--check-git]
engram session summaries [--agent <NAME>]

engram theory create <DOMAIN> [--agent <NAME>] [--task <ID>]
engram theory list [--agent <NAME>] [--domain <DOMAIN>]
engram theory show --id <ID> [--show-metrics]
engram theory update --id <ID>
engram theory apply-reflection --id <ID> --reflection <ID>
engram theory history --id <ID>
engram theory decay [--max-weight <F64>]
engram theory delete --id <ID>

engram reflect create --theory <ID> --observed <TEXT> --trigger-type <TYPE>
engram reflect list [--agent <NAME>] [--severity <LEVEL>]
engram reflect show --id <ID>
engram reflect record-dissonance --id <ID> --description <TEXT>
engram reflect propose-update --id <ID> --update <TEXT>
engram reflect resolve --id <ID> [--new-theory-id <ID>]
engram reflect requires-mutation --id <ID>
engram reflect delete --id <ID>
```

## Git, Sync, Validation, and Migration

```bash
# gix-backed git porcelain: no production shell-outs
engram git status
engram git log
engram git checkpoint --message <TEXT>
engram git verify-history

# Multi-agent and remote sync
engram sync sync --agents alice,bob --strategy latest_wins
engram sync add-remote origin <URL> --branch main
engram sync list-remotes
engram sync status --remote origin [--json]
engram sync pull --remote origin [--branch main]
engram sync push --remote origin [--branch main]
engram sync both --remote origin [--branch main]
engram sync resolve --remote origin --strategy <STRATEGY>
engram sync import-git-remotes

# Validation hooks and checks
engram validate hook install
engram validate hook status
engram validate commit --message "feat: title [<TASK_UUID>]"
engram validate check

# Storage migration/repair
engram migration
engram migrate triple-nesting
```

## Documentation, Schemas, Skills, and Prompts

```bash
engram doc build --output docs
engram doc topics list
engram doc chunk list <TOPIC>
engram doc write <TOPIC> <CHUNK_ID> --title <TEXT> --stdin
engram doc status
engram doc refs
engram doc fetch

engram schema generate --entity task --output task.schema.json
engram schema generate --entity reasoning
engram schema publish
engram schema workflow --output workflow.schema.json

engram skills setup
engram skills list
engram skills show <NAME>

engram prompts list [--category <NAME>]
engram prompts show <NAME>
engram prompts validate
```

## Security, Backup, Analytics, and Evolution

```bash
engram sandbox create|list|get|update|delete|validate|stats|check|reset
engram escalation create|list|get|review|cancel|cleanup|approve|deny|stats

engram perkeep backup|restore|list|health|config

engram analytics dora --window-days 30
engram analytics report
engram analytics bottleneck --top 10

engram health audit [--store]
engram health churn --top 20
engram health bus-factor
engram health bug-clusters --top 20
engram health velocity
engram health firefighting
engram health commit-size
engram health test-signal
engram health score
engram health orphans
engram health consistency
engram health refresh-decay --lambda 0.01

engram evo ingest|evaluate|optimize|replay|loop|report
```

## Experimental Commands

```bash
engram convert
engram test
```

`convert` is present in the command surface and marked experimental/not yet implemented by the CLI help.
