---
name: engram-project-bootstrap
description: "Bootstrap a new project with engram: initialise the workspace, register agents, create the initial task hierarchy, record the first ADR, establish project context, and configure validation rules. Use at the start of any brand-new git repository."
---

# Engram Project Bootstrap

## Overview

When a new project starts, engram must be initialised before any code is written. This skill walks through the complete bootstrap sequence — from `git init` to a verified, commit-ready workspace with a populated task hierarchy, project context, and architectural decision record.

**Rule:** Run this skill once per project, on the very first session. After bootstrap, use `engram-sessions` to manage ongoing work.

## When to Use

Use this skill when:
- Starting a brand-new git repository that needs engram
- Setting up engram for an existing repo that has never been initialised
- Verifying or repairing an incomplete bootstrap

---

## The Bootstrap Sequence

Run these steps in order. Each step depends on the previous one.

### Step 1: Initialise the Git Repo and Engram Workspace

```bash
# Skip if the repo already exists
git init

# Initialise the engram workspace — creates storage in git refs
engram setup workspace

# Verify the workspace was created
engram info
```

`engram info` should show workspace metadata and zero entities. If it shows an error, stop and re-run `engram setup workspace`.

---

### Step 2: Register Agent Configs

Every agent working in the repo must be registered. `--agent-type` is free-form.

```bash
# Register the primary working agent
engram setup agent \
  --name "<agent-name>" \
  --agent-type coder \
  --specialization "<domain, e.g. backend, frontend, infra>" \
  --email "<optional>"

# Register additional agents as needed — agent-type is a free-form string
engram setup agent --name "orchestrator" --agent-type orchestrator
engram setup agent --name "reviewer" --agent-type reviewer
engram setup agent --name "planner" --agent-type planner
```

**Common agent types:** `coder`, `reviewer`, `planner`, `orchestrator`, `tester`, `deployer`, `security-auditor`. Any string is valid.

---

### Step 3: Install the Git Pre-commit Hook

This enforces that every commit references a valid engram task UUID.

```bash
engram validate hook install
```

After this, every commit message must include `[<TASK_UUID>]`. Never use `--no-verify` to bypass this.

```
# Valid commit format:
feat: add user authentication [abc-001]
fix: resolve race condition in job queue [abc-002]
```

---

### Step 4: Install Skills and Prompts

```bash
# Install skills for your AI tool (opencode | claude | goose)
engram setup skills --tool opencode

# Install standard prompts
engram setup prompts
```

---

### Step 5: Create the Initial Task Hierarchy

Build the goal → milestone → task structure before writing any code.

```bash
# 1. Create the top-level goal task (no --parent)
engram task create \
  --title "Goal: <project goal in one line>" \
  --priority high \
  --output json
# Returns: GOAL_UUID

engram task update <GOAL_UUID> --status in_progress

# 2. Create milestone tasks (--parent chains them under the goal)
engram task create \
  --title "Milestone: <first deliverable>" \
  --parent <GOAL_UUID> \
  --priority high \
  --output json
# Returns: MILESTONE_1_UUID

engram task create \
  --title "Milestone: <second deliverable>" \
  --parent <GOAL_UUID> \
  --priority medium \
  --output json
# Returns: MILESTONE_2_UUID

# 3. Create initial work tasks under the first milestone
engram task create \
  --title "<first concrete task>" \
  --parent <MILESTONE_1_UUID> \
  --priority high \
  --output json
# Returns: TASK_1_UUID
```

**Hierarchy rule:** Every commit must reference a leaf task UUID (the concrete work item), not the goal or milestone. Use the leaf task UUID in commit messages.

---

### Step 6: Create the Project Context Entity

Store the project's goals, constraints, and non-obvious context in engram so all agents can find them.

```bash
engram context create \
  --title "Project: <project name> — Goals and Constraints" \
  --content "<Describe: what the project does, who uses it, key constraints (tech stack, performance targets, compliance requirements), non-obvious decisions already made, links to external docs>" \
  --source "AGENTS.md" \
  --relevance high \
  --tags "project,goals,constraints,bootstrap"
# Returns: CONTEXT_UUID

# Link the context to the goal task
engram relationship create \
  --source-id <GOAL_UUID> --source-type task \
  --target-id <CONTEXT_UUID> --target-type context \
  --relationship-type references \
  --agent "<agent-name>"
```

---

### Step 7: Record the First ADR

Every project starts with at least one architectural decision — even if it is as simple as the choice of language or framework. Record it now so it is findable later.

```bash
engram adr create \
  --title "<short title: what was decided>" \
  --number 1 \
  --context "<Situation: describe the context and problem. Decision: what was chosen. Rationale: why this over alternatives.>" \
  --agent "<agent-name>"
# Returns: ADR_UUID

# Link the ADR to the goal task
engram relationship create \
  --source-id <GOAL_UUID> --source-type task \
  --target-id <ADR_UUID> --target-type adr \
  --relationship-type explains \
  --agent "<agent-name>"
```

**Rule:** Use sequential integers for `--number`. ADR 1 is the first architectural decision. Increment for each subsequent ADR.

---

### Step 8: Set Up Validation Rules (Optional)

Add project-specific validation rules to encode team conventions. These are enforced by engram's rule engine.

```bash
# Example: enforce that all tasks have descriptions before being marked done
engram rule create \
  --title "Tasks must have descriptions" \
  --description "No task can be marked done without a description" \
  --rule-type validation \
  --priority medium \
  --entity-types task \
  --agent "<agent-name>"

# Example: enforce that all ADRs are linked to a task
engram rule create \
  --title "ADRs must be linked to a task" \
  --description "Every ADR must have a relationship to the task that triggered it" \
  --rule-type enforcement \
  --priority high \
  --entity-types adr \
  --agent "<agent-name>"
```

---

### Step 9: Verify Bootstrap is Complete

```bash
# Check workspace, hook, and validation setup
engram validate check

# Get the next recommended action (confirms task hierarchy is readable)
engram next

# Review workspace totals
engram info
```

A successful bootstrap shows:
- `engram validate check`: all systems working
- `engram next`: returns a prompt referencing the first concrete task
- `engram info`: workspace name, at least one registered agent, entities > 0

---

## Recommended First-Session Checklist

Run through this list top to bottom before writing any project code:

```
[ ] git init (if new repo)
[ ] engram setup workspace
[ ] engram info — confirms workspace is live
[ ] engram setup agent --name "..." --agent-type coder
[ ] engram validate hook install
[ ] engram setup skills --tool <tool>
[ ] engram setup prompts
[ ] engram session start --name "<agent>-bootstrap"
[ ] Goal task created and set to in_progress
[ ] At least 2 milestone tasks created under goal
[ ] At least 1 concrete work task created under first milestone
[ ] Project context entity created and linked to goal
[ ] ADR 1 created and linked to goal
[ ] engram validate check — passes clean
[ ] engram next — returns first task prompt
[ ] AGENTS.md written in repo root (see template below)
[ ] engram session end --id <SESSION_ID> --generate-summary
```

---

## AGENTS.md Template

Write this file in the repo root so all agents load the correct conventions on startup.

```markdown
# Agent Instructions

## Memory — Engram

All persistent memory lives in engram. Search before acting.

  engram ask query "<text>"          # search before acting
  engram task create --title "..."   # anchor new work
  engram task update <UUID> --status in_progress
  engram next                        # get next priority action
  engram validate check              # verify setup before finishing

## Required Skills

Load these at the start of every session:
- engram-use-engram-memory
- engram-orchestrator (if coordinating subagents)
- engram-subagent-register (if you are a subagent receiving a task UUID)
- engram-sessions (always)

## Commit Convention

Every commit message must follow this format:

  <type>: <title> [<ENGRAM_TASK_UUID>]

Examples:
  feat: add authentication middleware [abc-001]
  fix: resolve UTC offset in session tokens [abc-002]

The pre-commit hook rejects commits missing a valid task UUID.
NEVER use --no-verify to bypass the hook.

## Engram Rules

- Always run `engram ask query` before creating any new task
- Always link entities with `engram relationship create` after creating them
- Always run `engram validate check` before marking any task done
- Never commit without a live engram task UUID
```

---

## Full Bootstrap Example

```
$ git init my-project && cd my-project

$ engram setup workspace
✅ Workspace initialised

$ engram setup agent --name "aria" --agent-type coder --specialization "api"
✅ Agent registered: aria (coder/api)

$ engram validate hook install
✅ Pre-commit hook installed at .git/hooks/pre-commit

$ engram setup skills --tool opencode
$ engram setup prompts

$ engram session start --name "aria-bootstrap"
# SESSION_ID = sess-001

$ engram task create --title "Goal: Build event-driven notification service" \
    --priority high --output json
# GOAL_UUID = task-001
$ engram task update task-001 --status in_progress

$ engram task create --title "Milestone: Core delivery pipeline" \
    --parent task-001 --priority high --output json
# MILESTONE_UUID = task-002

$ engram task create --title "Design message schema and queue topology" \
    --parent task-002 --priority high --output json
# TASK_UUID = task-003

$ engram context create \
    --title "Project: Notification service — Goals and Constraints" \
    --content "Async event-driven service. Constraints: Rust only, zero unsafe, Kafka for transport, p99 < 50ms. Non-negotiable: idempotent delivery." \
    --source "AGENTS.md" --relevance high --tags "project,goals,bootstrap"
# CONTEXT_UUID = ctx-001

$ engram relationship create \
    --source-id task-001 --source-type task \
    --target-id ctx-001 --target-type context \
    --relationship-type references --agent "aria"

$ engram adr create \
    --title "Use Kafka for message transport" \
    --number 1 \
    --context "Need durable, replayable event stream. Kafka chosen over RabbitMQ: team experience, retention policy, consumer group semantics match our fan-out requirements." \
    --agent "aria"
# ADR_UUID = adr-001

$ engram relationship create \
    --source-id task-001 --source-type task \
    --target-id adr-001 --target-type adr \
    --relationship-type explains --agent "aria"

$ engram validate check
🎉 All validation systems are working correctly!

$ engram next
# Returns prompt for task-003: "Design message schema and queue topology"

$ engram session end --id sess-001 --generate-summary

# First commit (references the concrete work task):
git add AGENTS.md
git commit -m "chore: bootstrap engram workspace and project structure [task-003]"
```

---

## Key Principles

1. **Workspace before code** — `engram setup workspace` is the first command in any new repo.
2. **Hook before committing** — `engram validate hook install` before the first commit.
3. **Task before work** — create a task in engram before touching any file.
4. **Link everything** — run `engram relationship create` after every `context create`, `adr create`, and `reasoning create`.
5. **Validate before closing** — `engram validate check` must pass before marking any task done.
6. **Session wraps bootstrap** — start a session, do the work, end with `--generate-summary`.

---

## Related Skills

- `engram-setup` — full setup reference including new-agent onboarding and tool-specific skills install
- `engram-sessions` — managing sessions, handoffs, and `engram next` context
- `engram-use-engram-memory` — full command reference for context, reasoning, ADR, and relationships
- `engram-orchestrator` — agent coordination loop for multi-agent projects
- `engram-adr` — full ADR authoring and decision record patterns
- `engram-roadmap-planning` — building a multi-quarter roadmap after bootstrap
- `engram-writing-plans` — creating detailed task hierarchies from specs
- `engram-audit-trail` — full traceability and record-keeping patterns
