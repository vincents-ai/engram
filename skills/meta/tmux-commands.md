---
name: engram-tmux-commands
description: "Run shell commands safely using tmux sessions named after engram task IDs. Prevents cross-project session collisions when multiple agents run in parallel. Use whenever you need to run long-running or background commands."
---

# Running Commands with tmux and Engram Task IDs

## Overview

Multiple agents on multiple projects can run simultaneously and all have access to tmux. Generic session names like `main` or `build` cause commands to be sent to the **wrong session**. This skill eliminates that by naming every tmux session after the engram task UUID it belongs to.

## The Rule

**Every tmux session you create must be named `engram-<SHORT_TASK_ID>`** where `<SHORT_TASK_ID>` is the first 8 characters of the current engram task UUID.

Example: task `f75c9964-6ed4-4dac-807d-f0bd64df4317` → session `engram-f75c9964`

---

## When to Use tmux vs Direct Execution

| Situation | Method |
|---|---|
| Long-running command (build, test suite, server) | tmux session |
| Background process (watcher, dev server) | tmux session |
| Need to continue working while command runs | tmux session |
| Quick command where you need the output immediately | Direct bash |
| Single-line command that completes in <5s | Direct bash |

---

## Setup: Create the Session

Before running any command in tmux, ensure the session exists for the current task:

```bash
# Derive short ID from your engram task UUID (first 8 chars)
SHORT_ID="<first-8-chars-of-task-uuid>"
SESSION="engram-${SHORT_ID}"

# Create the session if it doesn't exist
tmux has-session -t "${SESSION}" 2>/dev/null || tmux new-session -d -s "${SESSION}"
```

---

## Running Commands

### Fire and Continue (non-blocking)

Use when you want to start a command and keep working:

```bash
SESSION="engram-<SHORT_ID>"

# Send the command
tmux send-keys -t "${SESSION}" "<your command here>" Enter

# Later: check output
tmux capture-pane -t "${SESSION}" -p
```

### Wait for Completion

Use when you need to know the result before proceeding:

```bash
SESSION="engram-<SHORT_ID>"

# Send command that signals completion with an exit marker
tmux send-keys -t "${SESSION}" "<your command> && echo '::DONE::' || echo '::FAILED::'" Enter

# Poll until marker appears
while ! tmux capture-pane -t "${SESSION}" -p | grep -qE "::(DONE|FAILED)::"; do
  sleep 2
done

# Capture full output
tmux capture-pane -t "${SESSION}" -p -S -200
```

### Run Multiple Independent Commands in Parallel

Create one window per command within the same session:

```bash
SESSION="engram-<SHORT_ID>"
tmux has-session -t "${SESSION}" 2>/dev/null || tmux new-session -d -s "${SESSION}"

# Window 0: already exists (created with new-session)
tmux send-keys -t "${SESSION}:0" "cargo build 2>&1 | tee /tmp/build-${SESSION}.log" Enter

# Window 1: new window in same session
tmux new-window -t "${SESSION}" -n "tests"
tmux send-keys -t "${SESSION}:tests" "cargo test 2>&1 | tee /tmp/test-${SESSION}.log" Enter
```

---

## Capturing Output

```bash
SESSION="engram-<SHORT_ID>"

# Last screenful
tmux capture-pane -t "${SESSION}" -p

# Larger history (last 500 lines)
tmux capture-pane -t "${SESSION}" -p -S -500

# Save to file for large output
tmux capture-pane -t "${SESSION}" -p -S -5000 > /tmp/output-${SESSION}.txt
```

---

## Cleanup

When the task is complete, kill the session:

```bash
SESSION="engram-<SHORT_ID>"
tmux kill-session -t "${SESSION}" 2>/dev/null || true
```

Do this as part of closing the engram task:

```bash
tmux kill-session -t "engram-<SHORT_ID>" 2>/dev/null || true
engram validate check
engram task update <TASK_UUID> --status done --outcome "<summary>"
```

---

## Full Example

```
Task: Run cargo build in the background while continuing to work
Engram task UUID: f75c9964-6ed4-4dac-807d-f0bd64df4317
Short ID: f75c9964
Session name: engram-f75c9964

[Step 1: Create session]
tmux has-session -t engram-f75c9964 2>/dev/null || tmux new-session -d -s engram-f75c9964

[Step 2: Start the build]
tmux send-keys -t engram-f75c9964 "cargo build 2>&1 | tee /tmp/build-f75c9964.log && echo '::BUILD_DONE::'" Enter

[Step 3: Continue other work while build runs]
# ... read files, run engram commands, etc. ...

[Step 4: Check build result]
tmux capture-pane -t engram-f75c9964 -p | grep -E "::(BUILD_DONE|error)::"

[Step 5: Read full output if needed]
cat /tmp/build-f75c9964.log

[Step 6: Cleanup on task done]
tmux kill-session -t engram-f75c9964
engram task update f75c9964 --status done --outcome "Build succeeded"
```

---

## Safety Rules

1. **Never use generic session names** — `main`, `build`, `test`, `default` are forbidden. Always use `engram-<SHORT_ID>`.
2. **Always check before creating** — `tmux has-session` before `tmux new-session` to avoid duplicate session errors.
3. **One task, one session** — don't reuse a session for a different task. Each engram task gets its own session.
4. **Fall back to direct execution** — if tmux is not available (`command -v tmux` fails), run commands directly. This skill's patterns are optional when tmux is absent.
5. **Never `tmux kill-server`** — this kills all sessions for all agents on the machine. Only kill the specific session you own.

---

## Checking tmux Availability

```bash
if command -v tmux >/dev/null 2>&1; then
  # Use tmux session pattern
  SESSION="engram-<SHORT_ID>"
  tmux has-session -t "${SESSION}" 2>/dev/null || tmux new-session -d -s "${SESSION}"
  tmux send-keys -t "${SESSION}" "<command>" Enter
else
  # Fall back to direct execution
  <command>
fi
```

---

## Related Skills

- `engram-use-engram-memory` — how to manage engram tasks and get task UUIDs
- `engram-orchestrator` — full agent execution loop that uses tmux for parallel subagent work
- `engram-systematic-debugging` — debugging loop; use tmux for reproducing errors in isolation
