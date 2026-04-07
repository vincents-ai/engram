---
name: engram-session-start
description: "Universal session start protocol. Run at the beginning of every agent session — syncs from remote, opens a session, loads prior context, and surfaces the next action."
---

# Engram Session Start

## Overview

This skill defines the universal protocol for starting an agent session. It ensures you are working with the latest engram state, opens a named session for grouping your work, loads prior context so you don't duplicate effort, and surfaces the highest-priority next action.

**Rule:** Run this protocol at the beginning of every agent session — no exceptions.

## When to Use

Use this skill when:
- Beginning any agent session (always)
- Resuming work after a break
- Starting a new subagent assigned to a task UUID
- Picking up work from a previous agent handoff

---

## The Protocol

### Step 0: Check for Configured Remotes and Conditionally Sync Pull

Before touching anything, check whether a remote is configured. If one exists, pull the latest state so you are not working with stale data.

```bash
# Check if any remotes are configured
engram sync list-remotes

# If output shows remotes: run pull
engram sync pull --remote origin --dry-run
engram sync pull --remote origin

# If output is empty (no remotes configured):
# → skip pull with note: "no remotes configured — skipping pull"
```

**Rule:** If no remote is configured, note it and continue — local-only work is valid.

---

### Step 1: Start a Named Session

Open a session that groups all work you do in this period under a single, identifiable name.

```bash
engram session start --name "<agent-role>-<brief-goal>"
# Examples:
#   "orchestrator-auth-feature"
#   "implementer-batch-task-cmd"
#   "reviewer-api-rate-limiting"

# Returns: SESSION_ID — save this, you need it for engram-session-end
```

Name format: `<role>-<goal>`. This makes the session queryable by the next agent.

---

### Step 2: Search for Prior Context

Before acting on anything, check what has already been done. Prior agents may have stored findings, decisions, and session summaries that are directly relevant to your goal.

```bash
engram ask query "<your goal or task area>"
# Read any summaries from prior sessions before touching anything

engram session list
# Scan for recent session summaries — look for sessions with matching names or goals
```

**Rule:** Never start work without querying prior session summaries. Prior agent context is in engram, not in the conversation.

---

### Step 3: Get the Next Priority Action

After loading context, surface the highest-priority pending task.

```bash
engram next
# Returns the highest-priority pending task
# Use this to orient your work if you don't have a specific task UUID
```

---

## Key Principles

1. **Always run session-start before any work** — no exceptions. An unopened session produces no summary, loses grouping, and breaks `engram next` context.
2. **Sync pull is step 0** — pull before creating anything so you don't duplicate work another agent has already done.
3. **Check for prior session summaries before acting** — `engram ask query` surfaces handoff context from previous sessions.
4. **Save the SESSION_ID** — you need it for `engram session end --id <SESSION_ID> --generate-summary` at the end of your session.
5. **If no remote is configured, note it and continue** — local-only work is valid. The absence of a remote is not an error.

---

## Worked Example: Orchestrator Starting a New Feature Session

```
Goal: Implement OAuth2 login flow

[Step 0: Check remotes]
engram sync list-remotes
# Output: origin → git@github.com:org/project-engram.git

engram sync pull --remote origin --dry-run
# Output: Would pull 3 new entities (2 context, 1 task)

engram sync pull --remote origin
# Output: ✅ Pulled 3 entities from origin/main

[Step 1: Start session]
engram session start --name "orchestrator-oauth2-login"
# SESSION_ID = sess-042
# → Save sess-042 for session end

[Step 2: Search for prior context]
engram ask query "OAuth2 login authentication"
# Returns: context node "ADR-7: Use OAuth2 over custom JWT" (uuid: ctx-018)
#          session summary "implementer-auth-spike" (uuid: sum-019)
# → Read both before proceeding — they contain design decisions already made

engram session list
# Shows: implementer-auth-spike (completed), researcher-oauth-libraries (completed)
# → Prior agents have done research and a spike — pull their findings before planning

[Step 3: Get next priority action]
engram next
# Returns: task "Implement: OAuth2 callback handler" (priority: high, uuid: abc-055)
# → This is the next unblocked high-priority task

[Begin work...]
# All engram activity from here is grouped under sess-042
```

---

## Related Skills

- `engram-session-end` — task close + summary generation + sync push + validate protocol
- `engram-sessions` — full session command reference and patterns
- `engram-sync` — remote sync command reference
- `engram-orchestrator` — orchestration loop that begins with this protocol
- `engram-use-engram-memory` — storing context, reasoning, and decisions during the session
