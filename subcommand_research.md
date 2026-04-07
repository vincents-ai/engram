engram CLI — Complete Reference
Version: engram 0.3.0
Global flags (available on the root command):
| Flag | Description |
|------|-------------|
| --json | Enable JSON I/O mode |
| -h, --help | Print help |
| -V, --version | Print version |
---
Changelog Summary
| Version | Date | Notable Additions |
|---------|------|-------------------|
| 0.3.0 | 2026-03-10 | theory entity (Naur 1985 mental models), reflect entity (cognitive dissonance detection), session enhancements (Reflecting status, bind_theory), mdBook docs |
| 0.1.2 | 2026-01-20 | Agent sandboxing + escalation, engram next, workflow validator, perkeep integration, engram info, analytics, vector search |
| 0.1.1 | 2026-01-19 | Initial implementation: core entities, Git-backed storage, basic CLI, workflow engine foundation, commit validation |
| 0.1.0 | 2026-01-17 | Initial release: Task/Context/Reasoning/Relationship entities, basic CLI, Git storage |
---
Top-Level Commands
engram setup — Initialize workspace or agent
Flags: --json, -h, --help
| Sub-subcommand | Usage | Flags |
|----------------|-------|-------|
| workspace | Initialize workspace | --json, -h |
| agent | Initialize agent profile | -n/--name <NAME> (required), -a/--agent-type <TYPE> (default: coder), --specialization, --email, --json, -h |
| skills | Install skills | -f/--force, -d/--dir <DIR>, -t/--tool <TOOL> (opencode\|claude\|goose; default output: ~/.config/engram/skills/), --json, -h |
| prompts | Install prompts | -p/--path <PATH> (default: ./prompts), --json, -h |
---
engram convert — Convert from other formats (EXPERIMENTAL — not yet implemented)
Usage: engram convert --from <FROM> --file <FILE>
| Flag | Description |
|------|-------------|
| -o/--from <FROM> | Source format: openspec, beads, github (required) |
| -f/--file <FILE> | Source file path (required) |
| --json | |
| -h | |
---
engram import — Import entities from structured markdown files
Flags: --json, -h
| Sub-subcommand | Usage | Flags |
|----------------|-------|-------|
| import | Import from an Engram Markdown (EMD) file | -f/--file <FILE> (required), -v/--verbose, --dry-run, --force, -j/--json, -h |
---
engram git — Run Git commands safely (blocks --no-verify)
Flags: --json, -h
(Passes through to git; no sub-subcommands exposed via --help)
---
engram test — Run test suite
Flags: --json, -h
---
engram task — Create/manage work items
Flags: --json, -h
| Sub-subcommand | Usage | Flags |
|----------------|-------|-------|
| create | Create a new task | -t/--title, -d/--description, -p/--priority (default: medium), -a/--agent, --parent, --tags, --output (json\|text, default: text), --title-stdin, --title-file, --description-stdin, --description-file, --json (input format), --json-file, -h |
| list | List tasks | -a/--agent, -s/--status, -l/--limit, --json, -h |
| show | Show task details | <ID> (positional, required), --json, -h |
| update | Update task status | <ID> (required), -s/--status <STATUS> (required): todo\|in_progress\|done\|blocked\|cancelled, --outcome, --reason, --json, -h |
| archive | Archive a task (soft delete) | <ID> (required), --reason, --json, -h |
| resolve | Resolve a blocked task | <ID> (required), -m/--message, --json, -h |
---
engram context — Background information and documentation
Flags: --json, -h
| Sub-subcommand | Usage | Flags |
|----------------|-------|-------|
| create | Create a new context | -t/--title, -c/--content, -s/--source, -r/--relevance (default: medium), --source-id, -a/--agent, --tags, --title-stdin, --title-file, --content-stdin, --content-file, --json, --json-file, -h |
| list | List contexts | -a/--agent, -r/--relevance, -l/--limit, --json, -h |
| show | Show context details | <ID> (required), --json, -h |
| update | Update context content | <ID> (required), -c/--content <CONTENT> (required), --json, -h |
| delete | Delete a context | <ID> (required), --json, -h |
---
engram ask — Natural Language Query Interface
Flags: --json, -h
| Sub-subcommand | Usage | Flags |
|----------------|-------|-------|
| query | Ask a NL query about Engram data | <QUERY> (positional, required), -c/--context, -v/--verbose, -j/--json, -h |
---
engram reasoning — Decision chains and rationale
Flags: --json, -h
| Sub-subcommand | Usage | Flags |
|----------------|-------|-------|
| create | Create a new reasoning chain | -t/--title, --task-id, -a/--agent, -c/--confidence (0.0–1.0), --content, --tags, --title-stdin, --title-file, --content-stdin, --content-file, --json, --json-file, -h |
| add-step | Add a reasoning step | <ID> (required), -d/--description, -c/--conclusion, -f/--confidence (required, 0.0–1.0), --description-stdin, --description-file, --conclusion-stdin, --conclusion-file, --json, -h |
| conclude | Set final conclusion | <ID> (required), -c/--conclusion, -f/--confidence (required), --conclusion-stdin, --conclusion-file, --json, -h |
| list | List reasoning chains | -a/--agent, -t/--task-id, -l/--limit, --json, -h |
| show | Show reasoning details | <ID> (required), --json, -h |
| delete | Delete reasoning | <ID> (required), --json, -h |
---
engram knowledge — Knowledge base management
Flags: --json, -h
| Sub-subcommand | Usage | Flags |
|----------------|-------|-------|
| create | Create a new knowledge item | -t/--title, -c/--content, -k/--knowledge-type (fact\|pattern\|rule\|concept\|procedure\|heuristic, default: fact), -f/--confidence (default: 0.8), -s/--source, -a/--agent, --tags, --title-stdin, --title-file, --content-stdin, --content-file, --json, --json-file, -h |
| list | List knowledge items | -a/--agent, -k/--kind, -l/--limit, --json, -h |
| show | Show knowledge details | -i/--id <ID> (required), --json, -h |
| update | Update knowledge item | -i/--id (required), -f/--field (required): content\|confidence\|type, -v/--value (required), --json, -h |
| delete | Delete knowledge item | -i/--id (required), --json, -h |
---
engram session — Coding session tracking
Flags: --json, -h
| Sub-subcommand | Usage | Flags |
|----------------|-------|-------|
| start | Start a new session | -n/--name <NAME> (required), --auto-detect, --json, -h |
| status | Show session status | -i/--id <ID> (required), --metrics, --json, -h |
| end | End current session | -i/--id <ID> (required), --generate-summary, --json, -h |
| list | List all sessions | -a/--agent, -l/--limit, --json, -h |
---
engram compliance — Compliance requirements
Flags: --json, -h
| Sub-subcommand | Usage | Flags |
|----------------|-------|-------|
| create | Create a compliance requirement | -t/--title (required), --category (required), --severity (default: medium), --description (required), -a/--agent, --json, -h |
| list | List compliance requirements | -a/--agent, --category, -l/--limit, --json, -h |
| show | Show compliance requirement details | <ID> (required), --json, -h |
| update | Update compliance requirement | -i/--id (required), -f/--field (required): status\|severity\|description, -v/--value (required), --json, -h |
| delete | Delete compliance requirement | -i/--id (required), --json, -h |
---
engram rule — Rules and policies
Flags: --json, -h
| Sub-subcommand | Usage | Flags |
|----------------|-------|-------|
| create | Create a new rule | -t/--title (required), --description, --rule-type (validation\|transformation\|enforcement\|notification, default: validation), --priority (default: medium), --entity-types, --condition (JSON, default: {}), --action (JSON, default: {}), -a/--agent, --json, -h |
| get | Get rule details | <ID> (required), --json, -h |
| update | Update rule | <ID> (required), --title, --description, --rule-type, --priority, --entity-types, --condition, --action, --status (active\|inactive\|deprecated), --json, -h |
| delete | Delete rule | <ID> (required), --json, -h |
| list | List rules | --rule-type, --priority, --entity-type, --status, --search, --limit (default: 20), --offset (default: 0), --json, -h |
| execute | Execute rule | <ID> (required), --entity-id (required), --entity-type (required), --json, -h |
---
engram standard — Coding standards
Flags: --json, -h
| Sub-subcommand | Usage | Flags |
|----------------|-------|-------|
| create | Create a new standard | -t/--title (required), --description, --category (coding\|testing\|documentation\|security\|performance\|process\|architecture, default: process), --version (default: 1.0), --effective-date (ISO 8601), -a/--agent, --json, -h |
| get | Get standard details | <ID> (required), --json, -h |
| update | Update standard | <ID> (required), --title, --description, --category, --version, --status (draft\|active\|deprecated\|superseded), --effective-date, --superseded-by, --json, -h |
| delete | Delete standard | <ID> (required), --json, -h |
| list | List standards | --category, --status, --search, --limit (default: 20), --offset (default: 0), --json, -h |
| add-requirement | Add requirement to standard | <ID> (required), --title (required), --description (required), --mandatory, --priority (default: medium), --evidence-required, --json, -h |
---
engram adr — Architectural decisions
Flags: --json, -h
| Sub-subcommand | Usage | Flags |
|----------------|-------|-------|
| create | Create a new ADR | -t/--title (required), --number (required), --context (required), -a/--agent, --json, -h |
| get | Get ADR details | <ID> (required), --json, -h |
| update | Update ADR | <ID> (required), --title, --status (proposed\|accepted\|deprecated\|superseded), --context, --decision, --consequences, --implementation, --superseded-by, --json, -h |
| delete | Delete ADR | <ID> (required), --json, -h |
| list | List ADRs | --status, --search, --limit (default: 20), --offset (default: 0), --json, -h |
| accept | Accept an ADR | <ID> (required), --decision (required), --consequences (required), --json, -h |
| add-alternative | Add alternative to ADR | <ID> (required), --description (required), --json, -h |
| add-stakeholder | Add stakeholder to ADR | <ID> (required), --stakeholder (required), --json, -h |
---
engram workflow — State machines and process flows
Flags: --json, -h
| Sub-subcommand | Usage | Flags |
|----------------|-------|-------|
| create | Create a new workflow | -t/--title (required), --description (required), --entity-types, -a/--agent, --json, -h |
| get | Get workflow details | <ID> (required), --json, -h |
| update | Update workflow | <ID> (required), --title, --description, --status (active\|inactive\|draft\|archived), --entity-types, --initial-state, --json, -h |
| delete | Delete workflow | <ID> (required), --json, -h |
| list | List workflows | --status, --search, --limit (default: 20), --offset (default: 0), --json, -h |
| add-state | Add state to workflow | <ID> (required), --name (required), --state-type (start\|in_progress\|review\|done\|blocked, default: in_progress), --description (required), --is-final, --json, -h |
| add-transition | Add transition to workflow | <ID> (required), --name (required), --from-state (required), --to-state (required), --transition-type (automatic\|manual\|conditional\|scheduled, default: manual), --description (required), --json, -h |
| activate | Activate workflow | <ID> (required), --json, -h |
| start | Start a workflow instance | <WORKFLOW_ID> (required), --entity-id, --entity-type, -a/--agent (required), --variables (key=value pairs), --context-file (JSON), --json, -h |
| transition | Execute a transition in an instance | <INSTANCE_ID> (required), -t/--transition (required), -a/--agent (required), --context-file, --json, -h |
| status | Get workflow instance status | <INSTANCE_ID> (required), --json, -h |
| instances | List active workflow instances | --workflow-id, --agent, --running-only, --json, -h |
| cancel | Cancel a workflow instance | <INSTANCE_ID> (required), -a/--agent (required), --reason, --json, -h |
| execute-action | Execute an action | --action-type (required): external_command\|notification\|update_entity, --command, --args, --working-directory, --environment, --timeout-seconds, --message, --entity-id, --entity-type, --json, -h |
| query-actions | Query available actions/guards/checks | <WORKFLOW_ID> (required), --state-id, --json, -h |
---
engram relationship — Link entities
Flags: --json, -h
| Sub-subcommand | Usage | Flags |
|----------------|-------|-------|
| create | Create a relationship | --source-id (required), --source-type (required), --target-id (required), --target-type (required), --relationship-type (required), --direction (unidirectional\|bidirectional\|inverse, default: unidirectional), --strength (weak\|medium\|strong\|critical\|0.0–1.0, default: medium), --description, --agent (required), --json, -h |
| list | List relationships | --entity-id, --source-id, --target-id, --relationship-type, --direction, --active-only, --agent, --json, -h |
| get | Show relationship details | <ID> (required), --json, -h |
| delete | Delete relationship | <ID> (required), --agent (required), --json, -h |
| find-path | Find paths between entities | --source-id (required), --target-id (required), --algorithm (bfs\|dfs\|dijkstra, default: dijkstra), --max-depth, --json, -h |
| connected | Get all connected entities | --entity-id (required), --algorithm (bfs\|dfs, default: bfs), --max-depth, --json, -h |
| stats | Show relationship statistics | --json, -h |
---
engram validate — Git commit validation and pre-commit hooks
Flags: --json, -h
| Sub-subcommand | Usage | Flags |
|----------------|-------|-------|
| commit | Validate a commit | -m/--message <MESSAGE> (required), --dry-run, --json, -h |
| hook | Manage git hooks | (has sub-subcommands — see below) |
| check | Check validation setup | --json, -h |
engram validate hook sub-subcommands:
| Sub-sub-subcommand | Description | Flags |
|--------------------|-------------|-------|
| install | Install pre-commit hook | --json, -h |
| uninstall | Uninstall pre-commit hook | --json, -h |
| status | Show hook status | --json, -h |
---
engram sandbox — Agent sandbox security and resource management
Flags: --json, -h
| Sub-subcommand | Usage | Flags |
|----------------|-------|-------|
| create | Create sandbox config | -a/--agent, -l/--level (default: standard), --created-by, --stdin, --file, --json, -h |
| list | List all sandbox configs | --agent-id, --level, -a/--agent, --json, -h |
| get | Get sandbox config details | <ID> (required), --json, -h |
| update | Update sandbox config | <ID> (required), --level, --stdin, --file, --json, -h |
| delete | Delete sandbox config | <ID> (required), --force, --json, -h |
| validate | Validate an operation against constraints | -a/--agent-id, -o/--operation, -r/--resource-type, --stdin, --file, --json, -h |
| stats | Show sandbox statistics | -a/--agent-id, --json, -h |
| reset | Reset sandbox config to defaults | <AGENT_ID> (required), --force, --json, -h |
---
engram escalation — Escalation requests for sandbox permission denied operations
Flags: --json, -h
| Sub-subcommand | Usage | Flags |
|----------------|-------|-------|
| create | Create escalation request | -a/--agent, -o/--operation-type, --operation, --block-reason, -j/--justification, -p/--priority (default: normal), --impact, --reviewer, --stdin, --file, --json, -h |
| list | List escalation requests | --agent-id, --status (pending\|approved\|denied\|expired\|cancelled), --priority (low\|normal\|high\|critical), --operation-type, --expired-only, --actionable-only, -a/--agent, --json, -h |
| get | Get escalation request details | <ID> (required), --json, -h |
| review | Review an escalation request | <ID> (required), -s/--status (approved\|denied), -r/--reason, --reviewer-id, --reviewer-name, --duration, --create-policy, --notes, --stdin, --file, --json, -h |
| cancel | Cancel an escalation request | <ID> (required), -r/--reason, --force, --json, -h |
| cleanup | Mark expired escalation requests | --apply (default is dry run), --json, -h |
| stats | Show escalation statistics | -a/--agent-id, --days (default: 30), --json, -h |
---
engram sync — Synchronize between agents
Flags: --json, -h
| Sub-subcommand | Usage | Flags |
|----------------|-------|-------|
| sync | Synchronize agents locally | -a/--agents (required), -s/--strategy (default: merge_with_conflict_resolution), --dry-run, --json, -h |
| add-remote | Add remote repository | <NAME> <URL> (both positional, required), --branch (default: main), --auth-type, --username, --password, --ssh-key, --json, -h |
| list-remotes | List configured remotes | --json, -h |
| status | Show sync status with remote | --remote, --json, -h |
| pull | Pull from remote | --remote (required), --branch, --agents, --auth-type, --username, --password, --ssh-key, --dry-run, --json, -h |
| push | Push to remote | --remote (required), --branch, --agents, --auth-type, --username, --password, --ssh-key, --dry-run, --json, -h |
| create-branch | Create a new branch for agent isolation | <NAME> (required), --agent, --from, --json, -h |
| switch-branch | Switch to a different branch | <NAME> (required), --create, --json, -h |
| list-branches | List all branches | --all, --current, --json, -h |
| delete-branch | Delete a branch | <NAME> (required), --force, --json, -h |
---
engram next — Get next task and generate prompt
Usage: engram next [OPTIONS]
| Flag | Description |
|------|-------------|
| -i/--id <ID> | Optional specific task ID |
| --format <FORMAT> | Output format: markdown (default) or json |
| --json | |
| -h | |
---
engram info — Display workspace and storage information
Flags: --json, -h
---
engram migration — Migrate from dual-repository to Git refs storage
Flags: --json, -h
---
engram perkeep — Perkeep backup and restore operations
Flags: --json, -h
| Sub-subcommand | Usage | Flags |
|----------------|-------|-------|
| backup | Backup entities to Perkeep | --entity-type, --include-relationships, --description, --json, -h |
| restore | Restore entities from Perkeep | --blobref, -a/--agent, --dry-run, --json, -h |
| list | List available backups | --detailed, --json, -h |
| health | Check Perkeep server health | --json, -h |
| config | Configure Perkeep settings | -s/--server, --auth-token, --save, --json, -h |
---
engram guide — Help and onboarding commands
Flags: --json, -h
| Sub-subcommand | Usage | Flags |
|----------------|-------|-------|
| onboarding | Show onboarding information | --json, -h |
| getting-started | Get started guide | --json, -h |
| examples | Show examples | --json, -h |
---
engram skills — List and manage skills
Flags: --json, -h
| Sub-subcommand | Usage | Flags |
|----------------|-------|-------|
| setup | Install skills | -f/--force, -d/--dir, -t/--tool (opencode\|claude\|goose; default output: ~/.config/engram/skills/), -s/--source (default: ./skills), --json, -h |
| list | List all available skills | -f/--format (short\|full, default: short), -v/--verbose, --json, -h |
| show | Show skill details | <NAME> (required), --json, -h |
---
engram prompts — List and manage prompts from ENGRAM_PROMPTS_PATH
Flags: --json, -h
| Sub-subcommand | Usage | Flags |
|----------------|-------|-------|
| list | List all available prompts | -c/--category (agents\|ai\|compliance), -f/--format (short\|full, default: short), -v/--verbose, --json, -h |
| show | Show prompt details | <NAME> (required), --json, -h |
| validate | Validate all prompts for evidence-based validation requirements | -c/--category, -f/--fix, --json, -h |
---
engram schema — Generate JSON Schema for entity types
Flags: --json, -h
| Sub-subcommand | Usage | Flags |
|----------------|-------|-------|
| workflow | Generate JSON Schema for workflow entity | -o/--output <OUTPUT> (file path; stdout if omitted), --json, -h |
---
engram theory — Domain theories and mental models (Naur, 1985)
Flags: --json, -h
| Sub-subcommand | Usage | Flags |
|----------------|-------|-------|
| create | Create a theory | [DOMAIN] (positional, optional), -a/--agent, -t/--task, -j/--json, --json-file, -h |
| list | List theories | -a/--agent, -d/--domain, -l/--limit, --json, -h |
| show | Show theory details | -i/--id (required), --show-metrics, --json, -h |
| update | Update theory | -i/--id (required), --concept, --mapping, --rationale, --invariant, --json, -h |
| delete | Delete theory | -i/--id (required), --json, -h |
| apply-reflection | Apply reflection updates to theory | -t/--theory-id (required), -r/--reflection-id (required), --updates-file (required), --json, -h |
---
engram reflect — State reflections and cognitive dissonance detection
Flags: --json, -h
| Sub-subcommand | Usage | Flags |
|----------------|-------|-------|
| create | Create a new state reflection | -t/--theory <THEORY> (required), --context <CONTEXT> (required), -o/--observed <OBSERVED> (required), --trigger-type (test_failure\|runtime_error\|unexpected_output\|...), -a/--agent, --json, --json-file, -h |
| list | List state reflections | -t/--theory, --trigger-type, --unresolved, -l/--limit, --json, -h |
| show | Show state reflection details | -i/--id (required), --json, -h |
| record-dissonance | Record cognitive dissonance in a reflection | -i/--id (required), --description (required), --score <SCORE> (0.0–1.0, required), --json, -h |
| propose-update | Propose a theory update | -i/--id (required), --update <UPDATE> (required), --json, -h |
| resolve | Resolve a reflection (mark resolved with new theory ID) | -i/--id (required), --new-theory <NEW_THEORY> (required), --json, -h |
| delete | Delete state reflection | -i/--id (required), --json, -h |
| requires-mutation | Check if theory mutation is required | -i/--id (required), --threshold (default: 0.7), --json, -h |
---
Summary Statistics
- Version: 0.3.0
- Top-level subcommands: 30 (including help)
- Total sub-subcommands: ~120+
- Depth: 3 levels maximum (engram validate hook install)
- Universal flags: Every command supports --json (machine-readable I/O) and -h/--help
- Only validate hook has a 3rd level of nesting (install / uninstall / status)
