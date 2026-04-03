---
name: engram-author-skill
description: "Create a new engram skill correctly. Introspects the live binary before writing any commands, tests every command in a sandbox, stores the design in engram, and validates the finished skill before publishing."
---

# Author a New Skill

## Overview

Guide for writing a new engram skill from scratch. The core rule: **never write a command you haven't run**. Every `engram` command that appears in the finished skill must be tested against the live binary in a sandbox before it is written into the file.

Stores the skill design as engram entities so future agents can query why the skill was designed the way it was.

## When to Use

- You need a repeatable workflow that doesn't already exist as a skill
- An existing skill needs to be extended significantly
- After a binary upgrade, when existing skills need to be brought up to date

## Before You Start

Check if the skill already exists:

```bash
engram skills list
engram ask query "<what the skill should do>"
```

If something close exists, consider extending it rather than writing a new one.

## The Pattern

### 1. Anchor in Engram

```bash
engram task create --title "Author skill: engram-<skill-name>"
# AUTHOR_TASK_UUID = ...
engram task update <AUTHOR_TASK_UUID> --status in_progress
```

### 2. Define the Skill

Answer these four questions before touching the binary. Store answers as context:

1. **What situation triggers this skill?** (the `When to Use` section)
2. **What is the outcome?** What does "done" look like for an agent following this skill?
3. **Which engram entity types does the workflow create?** (task / context / reasoning / adr / relationship — which are needed and why?)
4. **Does the workflow need `engram workflow` or `engram escalation`?**

```bash
engram context create \
  --title "Skill spec: engram-<skill-name>" \
  --content "Trigger: <when to use>\nOutcome: <what done looks like>\nEntities created: <list>\nWorkflow steps: <N>\nNeeds workflow subsystem: yes/no\nNeeds escalation: yes/no" \
  --source "skill-authoring"
# SPEC_UUID = ...

engram relationship create \
  --source-id <AUTHOR_TASK_UUID> --source-type task \
  --target-id <SPEC_UUID> --target-type context \
  --relationship-type relates_to --agent "<your-agent-name>"
```

### 3. Introspect the Binary

For every `engram` subcommand the skill will reference, run `--help` and read the output before writing a single command. This is the step that prevents broken skills.

```bash
# Run --help for every subcommand you plan to use
engram task --help
engram task create --help
engram task update --help
engram context create --help
engram reasoning create --help
engram adr create --help
engram relationship create --help
engram ask query --help
engram validate check --help
engram next --help

# If using workflow:
engram workflow --help
engram workflow create --help
engram workflow add-state --help
engram workflow add-transition --help
engram workflow start --help
engram workflow transition --help

# If using escalation:
engram escalation create --help
```

Store the flags for any subcommand whose flags are non-obvious:

```bash
engram context create \
  --title "CLI introspection: engram <subcommand>" \
  --content "<paste full --help output here>" \
  --source "binary-introspection"

engram relationship create \
  --source-id <AUTHOR_TASK_UUID> --source-type task \
  --target-id <INTROSPECT_UUID> --target-type context \
  --relationship-type relates_to --agent "<your-agent-name>"
```

Key flags to confirm before using:
- `engram task create` — no `--status` flag; use `engram task update` to set status
- `engram reasoning create` — `--task-id` is required
- `engram relationship create` — all five flags required: `--source-id`, `--source-type`, `--target-id`, `--target-type`, `--relationship-type`, `--agent`
- `engram adr create` — `--title`, `--number`, `--context` required; no `--status` or `--decision`
- `engram escalation create` — `--block-reason` is required
- `engram ask` — subcommand is `engram ask query "<text>"`, not `engram ask "<text>"`

### 4. Test Every Command in a Sandbox

Create a temporary workspace and run every command the skill will contain before writing the skill file:

```bash
SANDBOX=$(mktemp -d)
cd "$SANDBOX"
git init -q
engram setup workspace
```

Run each command with real values. If a command requires a UUID, create the entity first:

```bash
# Example: test that reasoning create works
PROBE_TASK=$(engram task create --title "author-probe" --output json \
  | grep '"id"' | sed 's/.*"id": "\([^"]*\)".*/\1/')

engram reasoning create \
  --title "test reasoning" \
  --task-id "$PROBE_TASK" \
  --content "test content"
# Must exit 0 before this command goes in the skill

# Test relationship create
PROBE_CTX=$(engram context create --title "test-ctx" --content "probe" \
  | grep "^Context '" | sed "s/Context '\\([^']*\\)'.*/\1/")

engram relationship create \
  --source-id "$PROBE_TASK" --source-type task \
  --target-id "$PROBE_CTX" --target-type context \
  --relationship-type relates_to --agent "author"
# Must exit 0
```

**Rule: if a command exits non-zero due to wrong syntax or a missing required flag, fix it now. Do not put it in the skill.**

Clean up after testing:

```bash
engram task archive "$PROBE_TASK" --reason "authoring probe"
rm -rf "$SANDBOX"
```

### 5. Write the Skill File

Every engram skill follows this structure:

```markdown
---
name: engram-<skill-name>
description: "<one sentence: what it does and when to use it>"
---

# <Skill Title>

## Overview
<2–3 sentences: what it does, what it produces, what makes it engram-integrated>

## When to Use
<bullet list of specific situations>

## The Pattern

### 0. Search First
<always include this — check for prior work before starting>

### 1. Anchor in Engram
<task create + task update in_progress>

### 2–N. <Steps>
<each step: prose explanation + engram commands + shell commands>

## Example
<concrete end-to-end walkthrough with real-looking UUIDs>

## Related Skills
<list of skill names + one-line descriptions>
```

**Mandatory sections in every skill:**

- **Search First** — `engram ask query` before any action
- **Anchor** — `engram task create` + `engram task update in_progress`
- **Store results** — every significant output goes into a `context create` or `reasoning create`
- **Link everything** — `engram relationship create` after every `create`
- **Validate and close** — `engram validate check` + `engram task update done`

**Prohibited patterns** (commonly hallucinated — all wrong):

```bash
# WRONG — 'ask' requires 'query' subcommand
engram ask "my question"

# WRONG — task create has no --status flag
engram task create --title "foo" --status in_progress

# WRONG — reasoning create --based-on does not exist
engram reasoning create --based-on <UUID>

# WRONG — adr create has no --status or --decision flags
engram adr create --status "Accepted" --decision "..."

# WRONG — sandbox execute does not exist
engram sandbox execute --command "ls"

# WRONG — knowledge traverse does not exist
engram knowledge traverse <UUID> --depth 2

# WRONG — escalation request does not exist (use escalation create)
engram escalation request --reason "..."

# WRONG — workflow step-complete does not exist
engram workflow step-complete <UUID>

# WRONG — relationship create shorthand does not exist
engram relationship create <SRC> <TGT> --fulfills

# WRONG — escalation create missing required --block-reason
engram escalation create --agent "x" --operation-type "y" --operation "z" --justification "w"
```

**Place the file in the appropriate category directory:**

```
skills/meta/          — skills about the engram/skill system itself
skills/workflow/      — planning, design, review workflows
skills/development/   — implementation, TDD, multi-agent dev
skills/debugging/     — bug investigation
skills/compliance/    — regulatory and audit workflows
skills/architecture/  — system design
skills/documentation/ — docs, ADRs, runbooks
skills/planning/      — project planning
skills/quality/       — code quality, performance
skills/review/        — code review, retrospectives
skills/testing/       — test execution and coverage
skills/go-to-market/  — market validation, launch
```

### 6. Record the Design Decision

```bash
engram adr create \
  --title "New skill: engram-<skill-name>" \
  --number <N> \
  --context "Created <skill-name> skill to handle <trigger situation>. Workflow: <N> steps. Entities: <list>. Alternatives considered: <any rejected approaches>." \
  --agent "<your-agent-name>"
# ADR_UUID = ...

engram relationship create \
  --source-id <AUTHOR_TASK_UUID> --source-type task \
  --target-id <ADR_UUID> --target-type adr \
  --relationship-type relates_to --agent "<your-agent-name>"
```

### 7. Register the Skill in the Binary

The skill is embedded in the binary at compile time via `include_str!()`. To make it installable via `engram skills setup`, it must be registered in two source files:

**File 1:** `src/cli/skills.rs` — the `engram skills setup` command (installs the 14 core skills).

Add an entry to the skills vector:

```rust
SkillDefinition {
    name: "engram-<skill-name>".to_string(),
    content: include_str!("../../skills/<category>/<skill-name>.md").to_string(),
},
```

**File 2:** `src/cli/setup.rs` — the `engram setup skills` command (installs all 44+ skills).

Add a matching entry in the same format.

After adding both entries, rebuild:

```bash
cargo build --bin engram
```

### 8. Install and Validate

Install the newly built binary's skills:

```bash
engram skills setup
```

Then run `engram-validate-skill` on the new skill:

```bash
# The validate skill will:
# 1. Read ~/.config/opencode/skills/engram-<skill-name>/SKILL.md
# 2. Run every engram command in a sandbox
# 3. Report PASS/FAIL per command
# 4. Store the result as engram context
```

If the validate skill reports any FAILs, fix the skill source file and repeat from step 7.

### 9. Close the Task

```bash
engram reasoning create \
  --title "Skill authoring complete: engram-<skill-name>" \
  --task-id <AUTHOR_TASK_UUID> \
  --content "Skill written and validated. Commands tested: <N>. All pass. Registered in skills.rs and setup.rs. Installed via engram skills setup."

engram relationship create \
  --source-id <AUTHOR_TASK_UUID> --source-type task \
  --target-id <REASONING_UUID> --target-type reasoning \
  --relationship-type explains --agent "<your-agent-name>"

engram validate check
engram task update <AUTHOR_TASK_UUID> --status done \
  --outcome "engram-<skill-name> written, validated, and registered"
```

## Example

```
[Check if skill exists]
engram skills list
engram ask query "deploy workflow automation"
# Not found — proceed

[Anchor]
engram task create --title "Author skill: engram-deploy-workflow"
# AUTHOR_TASK_UUID = task-001
engram task update task-001 --status in_progress

[Define]
engram context create \
  --title "Skill spec: engram-deploy-workflow" \
  --content "Trigger: deploying a service to staging or production\nOutcome: deploy tracked in engram with context, reasoning, and relationships\nEntities: task, context (×3), reasoning (×1), adr (×1), relationships\nWorkflow steps: 6\nNeeds workflow subsystem: no\nNeeds escalation: yes — elevated deploy permissions" \
  --source "skill-authoring"
# SPEC_UUID = ctx-002

engram relationship create \
  --source-id task-001 --source-type task \
  --target-id ctx-002 --target-type context \
  --relationship-type relates_to --agent "author"

[Introspect]
engram escalation create --help
# Confirms: --block-reason is required

[Test in sandbox]
SANDBOX=$(mktemp -d) && cd "$SANDBOX" && git init -q && engram setup workspace
PROBE=$(engram task create --title "probe" --output json | grep '"id"' | sed 's/.*"id": "\([^"]*\)".*/\1/')

engram escalation create \
  --agent "deployer" \
  --operation-type "deployment" \
  --operation "deploy service to production" \
  --block-reason "production deployment requires human approval" \
  --justification "passing all staging checks, ready for prod"
# Exit 0 — confirmed, include this exact form in the skill

engram task archive "$PROBE" --reason "authoring probe"
rm -rf "$SANDBOX"

[Write skill file]
# skills/workflow/deploy-workflow.md
# ... (follows canonical format with confirmed commands)

[Register in binary]
# Add to src/cli/skills.rs and src/cli/setup.rs
# cargo build --bin engram

[Install and validate]
engram skills setup
# Run engram-validate-skill on engram-deploy-workflow

[Record ADR]
engram adr create \
  --title "New skill: engram-deploy-workflow" \
  --number 5 \
  --context "Deploy workflows need tracking in engram so deploys are queryable. Escalation used for prod gating. Six-step process: anchor, pre-deploy checks, escalation, deploy, post-deploy verify, close." \
  --agent "author"
# ADR_UUID = adr-003

engram relationship create \
  --source-id task-001 --source-type task \
  --target-id adr-003 --target-type adr \
  --relationship-type relates_to --agent "author"

[Close]
engram reasoning create \
  --title "Skill authoring complete: engram-deploy-workflow" \
  --task-id task-001 \
  --content "Skill written and validated. 18 commands tested in sandbox, all pass. Registered in skills.rs and setup.rs."

engram validate check
engram task update task-001 --status done \
  --outcome "engram-deploy-workflow written, validated, and registered"
```

## Checklist

Before considering the skill done:

- [ ] Every `engram` command was run in a sandbox and exited correctly
- [ ] No prohibited patterns used (see Step 5)
- [ ] Skill has: Search First, Anchor, Store, Link, Validate, Close sections
- [ ] YAML frontmatter has `name` and `description`
- [ ] Registered in `src/cli/skills.rs` (if it's a core skill)
- [ ] Registered in `src/cli/setup.rs`
- [ ] `engram-validate-skill` run on the installed skill — all commands PASS
- [ ] Design stored as ADR in engram
- [ ] Author task marked done

## Related Skills

- `engram-validate-skill` — validates every command in a skill file against the live binary; called in step 8
- `engram-brainstorming` — design the skill's workflow before authoring
- `engram-use-engram-memory` — reference for engram entity creation patterns
- `engram-audit-trail` — full traceability of authoring decisions
