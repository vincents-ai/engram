---
name: engram-session-end
description: "Universal session end protocol. Run at the end of every agent session — closes open tasks, generates a handoff summary, syncs to remote, and validates state."
---

# Engram Session End

## Overview

This skill defines the universal protocol for ending an agent session. It closes any open tasks, generates a summary that the next agent can query, syncs to remote so the state is available to other agents, and validates that nothing is broken.

**Rule:** Run this protocol at the end of every agent session — always. An unclosed session loses its summary and breaks `engram next` context for the next agent.

## When to Use

Use this skill when:
- Finishing a work session (always)
- Handing off to another agent
- Pausing work you will resume later
- Completing a subagent task and returning control to the orchestrator

---

## The Protocol

### Step 1: Close Any Remaining Open Tasks

Before generating the summary, resolve all in-progress tasks. A task left in `in_progress` without a summary confuses the next agent.

```bash
# List all tasks still in progress
engram task list --status in_progress

# Mark done if completed
engram task update <UUID> --status done --outcome "<what was accomplished>"

# Or block with reason if incomplete — tell the next agent what they need to know
engram task update <UUID> --status blocked --reason "<why blocked, what next agent needs to know>"
```

**Rule:** Do not leave dangling in-progress tasks. Either close them or block them with a clear reason.

---

### Step 2: End the Session with a Summary

Generate the session summary — this is the handoff artifact for the next agent.

```bash
engram session end --id <SESSION_ID> --generate-summary
# The summary is stored in engram and queryable by the next agent via:
# engram ask query "<your session name> summary"
```

The `SESSION_ID` is the value returned when you ran `engram session start` at the beginning of this session (see `engram-session-start`).

**Rule:** Always use `--generate-summary`. Without it, no handoff artifact is created and the next agent cannot orient themselves.

---

### Step 3: Conditionally Sync Push

After generating the summary, push to remote so other agents can pull the latest state — including the summary you just generated.

```bash
# Check if any remotes are configured
engram sync list-remotes

# If remotes exist:
engram sync push --remote origin --dry-run
engram sync push --remote origin

# If no remotes configured:
# → skip push with note: "no remotes configured — skipping push"
```

**Why this order matters:** The summary must be generated before pushing. Pushing before `session end --generate-summary` means the next agent who pulls will receive entity data but not the summary narrative. Always end the session first, then push.

---

### Step 4: Validate

Confirm nothing is broken before exiting.

```bash
engram validate check
```

**Rule:** `engram validate check` must pass before you exit. If it fails, fix the issue before leaving.

---

## Key Principles

1. **Always end sessions** — an unclosed session loses its summary and breaks `engram next` context for the next agent.
2. **Generate the summary** — it is the handoff artifact; without it the next agent has no narrative context.
3. **Sync push is the last step** — the summary must be generated before pushing so the remote has the full handoff.
4. **Close or block all in-progress tasks** — don't leave dangling state; the next agent needs to know what is done and what is blocked.
5. **Validate before exiting** — `engram validate check` confirms nothing is broken and the workspace is clean.

---

## Worked Example: Implementer Finishing a Feature Session

```
[Wrapping up: OAuth2 callback handler implementation]

[Step 1: Close open tasks]
engram task list --status in_progress
# Returns:
#   abc-055 — Implement: OAuth2 callback handler (in_progress)
#   abc-056 — Write: integration tests for OAuth2 (in_progress)

engram task update abc-055 --status done --outcome "OAuth2 callback handler implemented, returns JWT on success"

# abc-056 was not completed — block it with context for next agent
engram task update abc-056 --status blocked --reason "Integration tests require staging OAuth2 provider — credentials not yet provisioned. Next agent: request credentials from ops team (ticket OPS-142) before proceeding."

[Step 2: End session with summary]
engram session end --id sess-042 --generate-summary
# Generates summary of all work done under sess-042
# Summary stored as engram entity — queryable by next agent:
# engram ask query "orchestrator-oauth2-login summary"

[Step 3: Sync push]
engram sync list-remotes
# Output: origin → git@github.com:org/project-engram.git

engram sync push --remote origin --dry-run
# Output: Would push 12 entities (3 context, 4 reasoning, 2 task, 1 adr, 1 session, 1 summary)

engram sync push --remote origin
# Output: ✅ Pushed 12 entities to origin/main

[Step 4: Validate]
engram validate check
# Output: ✅ Hook: OK  ✅ Workspace: OK

[Session complete — next agent can resume with:]
# engram sync pull --remote origin
# engram ask query "orchestrator-oauth2-login summary"
```

---

## Related Skills

- `engram-session-start` — sync pull + session open + context load protocol (the complement to this skill)
- `engram-sessions` — full session command reference and patterns
- `engram-sync` — remote sync command reference
- `engram-orchestrator` — orchestration loop that ends with this protocol
