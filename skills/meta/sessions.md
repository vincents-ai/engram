---
name: engram-sessions
description: "Manage engram sessions to group work, enable agent handoff, and make engram next context-aware. Use at the start and end of every work session."
---

# Engram Sessions

## Overview

Sessions group all work done in a period of time — tasks created, context stored, reasoning recorded — under a named session. They make `engram next` context-aware, enable clean handoff between agents, and produce summaries that the next agent can query to understand what happened.

**Rule:** Start a session before doing any work. End it with `--generate-summary` when you are done.

## When to Use

Use this skill when:
- Beginning a work session as an agent (always)
- Handing off to another agent
- Resuming work after a break — check session status first
- You want `engram next` to return context-relevant suggestions

## Command Reference

```bash
# Start a session — required: --name; optional: --auto-detect
engram session start --name "<agent-name-or-session-description>"

# Check the current session mid-work
engram session status

# End a session and generate a summary for handoff
# required: --id; optional: --generate-summary
engram session end --id <SESSION_ID> --generate-summary

# List all sessions
engram session list
```

## The Pattern

### 1. Start a Session

At the very beginning of any work, start a named session:

```bash
engram session start --name "implementer-auth-feature"
# Returns: SESSION_ID
```

The session name should identify who is working and what they are working on. Once started, all engram activity is grouped under this session.

### 2. Do Work — Store Everything in Engram

While the session is active, store all findings, reasoning, and decisions in engram as normal. The session automatically groups them:

```bash
engram context create --title "Found: rate limit is 100 req/s" --content "..." --source "src/api/client.rs"
engram reasoning create --title "Why we chose exponential backoff" --task-id <TASK_UUID> --content "..."
engram task update <TASK_UUID> --status in_progress
```

Check session status at any point:

```bash
engram session status
# Returns: active session name, ID, start time, linked entities
```

### 3. End with Summary

When work is complete, end the session and generate a summary:

```bash
engram session end --id <SESSION_ID> --generate-summary
```

The generated summary is stored in engram and is queryable by the next agent.

### 4. Agent Handoff

The next agent starts their own session, then queries the previous session's summary before doing anything:

```bash
# Next agent: start their session first
engram session start --name "reviewer-auth-feature"

# Then find what the previous agent did
engram ask query "auth feature session summary"
# Returns: summary UUID and content from previous agent's session
```

**Rule:** Never start work without querying prior session summaries. Prior agent context is in engram, not in the conversation.

## Example Workflow

```
[Agent: implementer]

# 1. Start session
engram session start --name "implementer-2026-auth"
# SESSION_ID = sess-001

# 2. Do work, store findings
engram task create --title "Implement JWT auth" --priority high --output json
# TASK_UUID = abc-001
engram task update abc-001 --status in_progress

engram context create \
  --title "JWT library selected: jsonwebtoken 9.x" \
  --content "jsonwebtoken 9.x chosen: actively maintained, supports RS256, no unsafe deps." \
  --source "Cargo.toml"

# ... implement, test, etc. ...

engram task update abc-001 --status done --outcome "JWT auth implemented and tested"

# 3. End session with summary
engram session end --id sess-001 --generate-summary

---

[Agent: reviewer]

# 1. Start own session
engram session start --name "reviewer-2026-auth"

# 2. Find prior work
engram ask query "implementer auth session summary"
# Returns: summary of sess-001

# 3. Review, store findings, end session
engram session end --id <NEW_SESSION_ID> --generate-summary
```

## Key Principles

1. **Always start a session** — before any work, `engram session start`.
2. **Name sessions meaningfully** — include your agent role and the feature or goal.
3. **End with `--generate-summary`** — the summary is the handoff artifact.
4. **Query summaries before starting** — `engram ask query` for prior session summaries.
5. **Sessions make `engram next` smarter** — context-aware suggestions depend on an active session.

## Related Skills

- `engram-use-engram-memory` — storing context, reasoning, ADRs within a session
- `engram-orchestrator` — orchestration loop that uses sessions for each dispatch
- `engram-subagent-register` — subagent session registration on task claim
- `engram-audit-trail` — full traceability using session grouping
