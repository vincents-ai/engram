---
name: engram-setup
description: "Full setup sequence for engram in a new project or for a new agent joining an existing workspace. Covers bootstrap, agent registration, skills install, hook setup, and AGENTS.md authoring."
---

# Engram Setup

## Overview

Engram requires a one-time workspace initialisation per project and a one-time agent registration per agent. Once set up, the pre-commit hook enforces that every commit is linked to a valid engram task. This skill walks through the full bootstrap sequence, new-agent onboarding, skills installation, and the minimum viable AGENTS.md.

## When to Use

Use this skill when:
- Starting a brand-new project that needs engram
- Joining an existing project as a new agent
- Installing or re-installing engram skills for a specific AI tool (opencode, claude, goose)
- Verifying that an existing setup is healthy
- Authoring or updating an AGENTS.md / CLAUDE.md

---

## Pattern A: New Project Bootstrap

Run this sequence once, in order, when initialising a new repo.

```bash
# 1. Initialise the git repository (skip if already a repo)
git init

# 2. Create the engram workspace — stores metadata in .engram/
engram setup workspace

# 3. Register the agent working in this repo
#    --agent-type: coder | reviewer | planner  (default: coder)
engram setup agent \
  --name "your-agent-name" \
  --agent-type coder \
  --specialization "backend" \
  --email "you@example.com"

# 4. Install the git pre-commit hook
#    This validates that commits reference a live engram task UUID
engram validate hook install

# 5. Install skills for your AI tool
#    --tool: opencode | claude | goose  (default: no tool-specific install)
engram setup skills --tool opencode

# 6. Install standard prompts
engram setup prompts

# 7. Verify everything is wired up correctly
engram info
engram validate check
```

After this sequence, `engram info` should show the workspace name, registered agents, and hook status.

---

## Pattern B: New Agent Joining an Existing Project

The workspace already exists. You only need to register yourself and orient to current work.

```bash
# 1. Register as an agent in the existing workspace
engram setup agent --name "your-agent-name" --agent-type coder

# 2. Search for current context before touching anything
engram ask query "current tasks"

# 3. See the highest-priority next action
engram next
```

If the workspace is missing (`.engram/` does not exist), run Pattern A first.

---

## Pattern C: Installing Skills for a Specific Tool

Skills are markdown files copied into the tool's config directory so the AI can load them.

```bash
# Install for opencode  (writes to ~/.config/opencode/skills/)
engram setup skills --tool opencode

# Install for claude  (writes to ~/.claude/skills/ or equivalent)
engram setup skills --tool claude

# Install for goose  (writes to ~/.config/goose/skills/)
engram setup skills --tool goose

# Force overwrite existing skill files
engram setup skills --tool opencode --force

# Install from a non-default skills directory
engram setup skills --tool opencode --dir ./custom-skills
```

---

## Pattern D: Verifying Setup

Run these at any time to confirm the workspace is healthy.

```bash
# Show workspace metadata: name, agents, hook status, storage path
engram info

# Check git hook installation and commit-validation rules
engram validate check
```

A healthy workspace shows:
- Workspace name and creation date
- At least one registered agent
- Pre-commit hook: installed

---

## AGENTS.md Template

Every project should have an AGENTS.md (opencode reads this) or CLAUDE.md (Claude reads this) that tells the AI how to use engram in this repo.

**Where each tool reads it:**
| Tool | File |
|------|------|
| opencode | `AGENTS.md` in repo root |
| claude | `CLAUDE.md` in repo root |
| goose | `AGENTS.md` in repo root |

**Minimum viable AGENTS.md:**

```markdown
# Agent Instructions

## Memory — Engram

All persistent memory lives in engram, not in conversation context.

Always use the engram-use-engram-memory skill before starting work.

## Required Skills

Load these skills at the start of every session:
- engram-use-engram-memory
- engram-orchestrator (if coordinating subagents)
- engram-subagent-register (if you are a subagent receiving a task UUID)

## Commit Convention

Every commit message must follow this format:

  <type>: <title> [<ENGRAM_TASK_UUID>]

Examples:
  feat: add rate limiting middleware [abc-001]
  fix: normalise UTC timestamps in token validation [abc-002]

The pre-commit hook rejects commits that do not reference a valid engram task UUID.
Never use --no-verify to bypass the hook.

## Workflows

Available workflows (run `engram workflow list` to see all):
- <workflow-name>: <short description>

## Engram Quick Reference

  engram ask query "<text>"          # search before acting
  engram task create --title "..."   # anchor new work
  engram task update <UUID> --status in_progress
  engram next                        # get next priority action
  engram validate check              # verify setup before finishing
```

Customise the workflows section by running `engram workflow list` and listing the ones relevant to this project.

---

## Example: Full Bootstrap of a New Repo

```
$ git init my-project
$ cd my-project

$ engram setup workspace
✅ Workspace initialised at .engram/

$ engram setup agent --name "aria" --agent-type coder --specialization "api"
✅ Agent registered: aria (coder/api)

$ engram validate hook install
✅ Pre-commit hook installed at .git/hooks/pre-commit

$ engram setup skills --tool opencode
✅ Skills installed to ~/.config/opencode/skills/

$ engram setup prompts
✅ Prompts installed

$ engram info
Workspace: my-project
Agents: aria (coder)
Hook: installed
Storage: .engram/

$ engram validate check
✅ Hook: OK
✅ Workspace: OK
```

After bootstrap, create your first task before writing any code:

```bash
engram task create --title "Initial project scaffolding" --priority high
# Returns: TASK_UUID
engram task update <TASK_UUID> --status in_progress
```

Your first commit will then be:

```
feat: scaffold project layout [<TASK_UUID>]
```

---

## Related Skills

- `engram-use-engram-memory` — full command reference for context, reasoning, ADR, and relationships
- `engram-orchestrator` — agent coordination loop
- `engram-subagent-register` — how to claim and complete an assigned task UUID
- `engram-workflow-guide` — defining and running state machine workflows
- `engram-audit-trail` — traceability and complete record-keeping patterns
