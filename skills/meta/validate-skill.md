---
name: engram-validate-skill
description: "Validate any engram skill file by running every CLI command in it against a live binary. Use after writing a new skill, after a binary upgrade, or to audit existing skills."
---

# Validate a Skill

## Overview

Run every `engram` command in a skill file against a live binary and report PASS/FAIL per command. Stores the validation result as an engram context entity so it is queryable by future agents.

This skill works on any installed skill — newly written or pre-existing — and is portable to any project that uses engram.

## When to Use

- After writing a new skill with `engram-author-skill`
- After upgrading the `engram` binary, to check existing skills still work
- Before merging a skill into the repository
- Any time you suspect a skill's commands may be outdated

## The Pattern

### 0. Search for Prior Validation

```bash
engram ask query "validation result <skill-name>"
```

If a recent validation result exists, read it before re-running.

### 1. Anchor the Validation Run

```bash
engram task create --title "Validate skill: <skill-name>"
# VALIDATE_TASK_UUID = ...
engram task update <VALIDATE_TASK_UUID> --status in_progress
```

### 2. Locate the Skill File

Skills are installed by the binary. Find the file:

```bash
# List installed skills
engram skills list

# The skill file is at:
# ~/.config/engram/skills/engram-<skill-name>/SKILL.md
# Typical location: ~/.config/engram/skills/engram-<skill-name>/SKILL.md
```

Read the full skill file before extracting commands.

### 3. Extract All `engram` Commands

From the skill file, collect every bash code block that contains an `engram` command. Ignore lines that are comments (`#`) or shell variable assignments without an engram call.

Patterns to extract:
- `engram <subcommand> ...`
- Lines with `$(engram ...)` subshell captures
- Lines that call engram via a variable like `$ENGRAM_BIN` — treat as engram calls

Keep the commands in document order. Number them for the report.

### 4. Prepare a Validation Workspace

Create a temporary isolated workspace so probes do not pollute the user's project:

```bash
SANDBOX=$(mktemp -d)
cd "$SANDBOX"
git init -q
engram setup workspace
```

### 5. Run Each Command

For each extracted command, follow the substitution protocol before running:

**Substitution protocol:**

Skill files use placeholder tokens like `<UUID>`, `<TASK_UUID>`, `<PARENT_UUID>`, `<CONTEXT_UUID>`, `<REASONING_UUID>`, `<ADR_UUID>`, `<WORKFLOW_UUID>`, `<AGENT_NAME>`, `<SKILL_NAME>`, `<N>`.

Before running a command that needs a UUID:
1. Check if you already created a real entity of that type during this run
2. If not, create one now and capture its ID:

```bash
# Create a probe task (reuse for all TASK_UUID substitutions)
PROBE_TASK=$(engram task create --title "validator-probe-task" --output json | grep '"id"' | sed 's/.*"id": "\([^"]*\)".*/\1/')

# Create a probe context
PROBE_CTX=$(engram context create --title "validator-probe-context" --content "probe" | grep "^Context '" | sed "s/Context '\\([^']*\\)'.*/\1/")

# Create a probe reasoning (requires --task-id)
PROBE_RSN=$(engram reasoning create --title "validator-probe-reasoning" --task-id "$PROBE_TASK" --content "probe" | grep "^Reasoning '" | sed "s/Reasoning '\\([^']*\\)'.*/\1/")
```

Substitute tokens before running:
- `<UUID>` / `<TASK_UUID>` / `<PARENT_UUID>` / `<SUBTASK_UUID>` → `$PROBE_TASK`
- `<CONTEXT_UUID>` → `$PROBE_CTX`
- `<REASONING_UUID>` → `$PROBE_RSN`
- `<AGENT_NAME>` / `<name>` → `"validator"`
- `<N>` (ADR number) → `99`
- `<SKILL_NAME>` → `"probe"`
- Free-text placeholders in `--title`, `--content`, `--context` → replace with `"probe"`

**PASS criteria:**
- Exit code 0
- Exit code non-zero for a data reason: "not found", "no results", "no connected entities" — these mean the command syntax is valid but the data isn't there

**FAIL criteria:**
- `error: unrecognized subcommand`
- `error: unexpected argument`
- `error: unknown flag`
- Usage/help text printed because of wrong invocation (indicates wrong flags)
- `Entity validation error` (indicates a required flag is missing)

Record each result:

```
Command N: engram <subcommand> ...
Exit: <code>
Result: PASS / FAIL
Reason: <one line>
```

### 6. Clean Up Probes

After all commands are run, archive the probe entities to keep the workspace clean:

```bash
engram task archive "$PROBE_TASK" --reason "validation probe"
```

Remove the sandbox directory:

```bash
rm -rf "$SANDBOX"
```

### 7. Store the Result in Engram

```bash
engram context create \
  --title "Validation result: <skill-name> — <PASS|FAIL>" \
  --content "Skill: <skill-name>\nDate: <date>\nBinary: $(engram --version)\nCommands tested: <N>\nPass: <X>\nFail: <Y>\n\nFailures:\n<list each FAIL with command and error>" \
  --source "skill-validation"
# RESULT_UUID = ...

engram relationship create \
  --source-id <VALIDATE_TASK_UUID> --source-type task \
  --target-id <RESULT_UUID> --target-type context \
  --relationship-type relates_to --agent "validator"
```

### 8. Close the Task

If all commands pass:

```bash
engram task update <VALIDATE_TASK_UUID> --status done \
  --outcome "All <N> commands in <skill-name> passed"
```

If any fail:

```bash
engram task update <VALIDATE_TASK_UUID> --status blocked \
  --reason "Skill has <Y> failing commands — see context <RESULT_UUID>"
```

## Reporting

Present results as a table:

```
| # | Command | Exit | Result | Reason |
|---|---------|------|--------|--------|
| 1 | engram task create ... | 0 | PASS | |
| 2 | engram ask query "..." | 0 | PASS | no results, expected |
| 3 | engram escalation create ... | 1 | FAIL | Entity validation error: Block reason is required |
```

Then a summary line:

```
Skill: engram-<name> | Commands: N | Pass: X | Fail: Y | Overall: PASS/FAIL
```

## Example

```
[Search first]
engram ask query "validation result engram-brainstorming"
# No prior result found

[Anchor]
engram task create --title "Validate skill: engram-brainstorming"
# VALIDATE_TASK_UUID = abc-001
engram task update abc-001 --status in_progress

[Locate skill]
# ~/.config/engram/skills/engram-brainstorming/SKILL.md

[Create sandbox]
SANDBOX=$(mktemp -d)
cd "$SANDBOX" && git init -q && engram setup workspace

[Create probes]
PROBE_TASK=$(engram task create --title "validator-probe-task" --output json \
  | grep '"id"' | sed 's/.*"id": "\([^"]*\)".*/\1/')
PROBE_CTX=$(engram context create --title "validator-probe-context" --content "probe" \
  | grep "^Context '" | sed "s/Context '\\([^']*\\)'.*/\1/")
PROBE_RSN=$(engram reasoning create --title "validator-probe-reasoning" \
  --task-id "$PROBE_TASK" --content "probe" \
  | grep "^Reasoning '" | sed "s/Reasoning '\\([^']*\\)'.*/\1/")

[Run commands — substituting probes for placeholders]
# Command 1: engram ask query "<feature or topic name>"
engram ask query "test query"
# Exit 0, PASS

# Command 2: engram task create --title "Design: <feature name>"
engram task create --title "Design: probe"
# Exit 0, PASS

# Command 3: engram task update <TASK_UUID> --status in_progress
engram task update "$PROBE_TASK" --status in_progress
# Exit 0, PASS

# ...continue for all commands in the skill...

# Command 15: engram escalation create --agent "..." --operation-type "..."
engram escalation create --agent "validator" --operation-type "shell" \
  --operation "probe" --justification "probe"
# Exit 1, FAIL — Entity validation error: Block reason is required

[Clean up]
engram task archive "$PROBE_TASK" --reason "validation probe"
rm -rf "$SANDBOX"

[Store result]
engram context create \
  --title "Validation result: engram-brainstorming — FAIL" \
  --content "Skill: engram-brainstorming\nBinary: engram 0.3.0\nCommands tested: 15\nPass: 14\nFail: 1\n\nFailures:\n- Command 15: escalation create missing --block-reason flag" \
  --source "skill-validation"
# RESULT_UUID = ctx-002

engram relationship create \
  --source-id abc-001 --source-type task \
  --target-id ctx-002 --target-type context \
  --relationship-type relates_to --agent "validator"

engram task update abc-001 --status blocked \
  --reason "Skill has 1 failing command — see context ctx-002"
```

## Notes

- Run this skill from **inside a git repository** that has `engram setup workspace` completed — the binary requires a workspace to store entities
- If the skill references `engram workflow transition`, also check `engram workflow add-state --help` and `engram workflow add-transition --help` — workflow commands require pre-configured states and transitions before `workflow start` and `workflow transition` will succeed
- `engram compliance list` may return empty even with records — this is a known display bug; treat exit 0 as PASS regardless of output

## Related Skills

- `engram-author-skill` — writes a new skill; calls this skill as its final validation gate
- `engram-use-engram-memory` — core pattern for storing results in engram
- `engram-audit-trail` — broader audit trail for all work
