---
name: screenplay-session-start
description: "Start a new writing session for any screenplay project. Use at the beginning of every session — loads the right context, applies project conventions, and orients the AI before any writing begins."
---

# Screenplay Session Start

## Overview

Run this at the start of every writing session. Retrieves all project state from engram, establishes what to work on, and enforces co-writer rules before any writing begins.

Replace `<slug>` throughout with your project's short identifier (e.g. `heist`, `roadtrip`, `orbit`).

---

## Step 1: Find the Project Task

```bash
# Find the main project task
engram ask query "project:<slug>"

# Or list recent tasks to locate it
engram task list
```

Note the main project task UUID — you will use it to store all session work.

---

## Step 2: Pull All Project Context From Engram

```bash
# Get all reasoning linked to the project task
engram reasoning list --task-id <PROJECT_TASK_UUID>

# Get all context entities for this project
engram ask query "project:<slug>"
```

The entries you will typically need most at session start:

| Query | What it contains |
|-------|-----------------|
| `project:<slug> type:beat` | Current position in beat sheet + what to write next |
| `project:<slug> co-writer rules` | Rules governing the session — read these first |
| `project:<slug> type:theme` | Theme, story invariants, writing principles |
| `project:<slug> type:world` | World facts, rules, and systems |
| `project:<slug> type:character <name>` | Individual character profiles |

To retrieve any entry's full content:

```bash
engram reasoning show <SHORT_ID>
engram context show <SHORT_ID>
```

---

## Step 3: Always Read These Two First

Before touching any script or character file, retrieve and read:

1. **Co-writer rules** — what agreements govern this collaboration
2. **Beat sheet progress** — exactly which beat and scene to work on next

```bash
engram ask query "project:<slug> co-writer rules"
engram ask query "project:<slug> type:beat progress"
```

These answer: what are the rules, and what do we write next.

---

## Step 4: Load Only the Characters in This Scene

Do not load all character files. Identify which characters appear in the next beat, then retrieve only those:

```bash
engram ask query "project:<slug> type:character <name>"
```

**Rule:** If a character is not in the scene being written, do not load their entry.

---

## Step 5: Load World Rules Only If Needed

Only retrieve world rules if the scene involves a location, timeline moment, or system not yet established in the current context window:

```bash
engram ask query "project:<slug> type:world"
engram ask query "project:<slug> type:theme"
```

---

## Step 6: Confirm the Session Goal

Before writing anything, state clearly:

- **Which beat** are we working on?
- **Which characters** are in those scenes? (load only those profiles)
- **What changes** by the end of the session?

Get the co-writer's confirmation before opening any script file.

---

## Step 7: Recommended File Conventions

A consistent project layout keeps sessions focused:

```
characters/               ← one .md per character — detail only
characters/CHARACTER_PROFILES.md  ← index/table of contents only
docs/STORY_BIBLE.md       ← world rules source of truth
docs/LOGLINE_AND_THEME.md ← theme and logline
docs/WORKFLOW.md          ← session protocol reference
outlines/BEAT_SHEET.md    ← full 15-beat structure
brainstorm/IDEAS_PARKING_LOT.md  ← parked ideas, nothing discarded
scripts/ACT_ONE.fountain  ← Act I script (in progress)
scripts/scenes/           ← individual scene drafts
```

All scripts should use `.fountain` format.

---

## Step 8: Store Before Returning Output

**Store in engram BEFORE returning any output to the co-writer.** Every meaningful action must be persisted as part of that action — not after. If the session ends before you respond, the work is already saved.

### After every scene written — store before showing the scene

```bash
engram context create \
  --title "<slug> scene <act>-<seq> <short description>" \
  --content "<what was written, key beats hit, co-writer approvals, dialogue decisions>" \
  --tags "project:<slug>,type:scene,act:<number>" \
  --relevance high \
  --agent "<agent-name>"

engram relationship create \
  --source-id <PROJECT_TASK_UUID> --source-type task \
  --target-id <NEW_ID> --target-type context \
  --relationship-type explains --agent "<agent-name>"
```

### After every character decision — store before continuing

```bash
engram context create \
  --title "<slug> character <name> <what changed>" \
  --content "<the decision, why, what it changes downstream>" \
  --tags "project:<slug>,type:character,character:<name>" \
  --relevance high \
  --agent "<agent-name>"
```

### After every parked idea — store before moving on

```bash
engram context create \
  --title "<slug> idea <short description>" \
  --content "<the idea verbatim, potential placement in structure, reason for parking>" \
  --tags "project:<slug>,type:idea" \
  --relevance medium \
  --agent "<agent-name>"
```

Also write it to `brainstorm/IDEAS_PARKING_LOT.md`.

### After every completed beat — update beat progress before moving on

```bash
engram context create \
  --title "<slug> beat sheet progress and next steps" \
  --content "<beats completed, current beat, next beat, open questions>" \
  --tags "project:<slug>,type:beat,beat:progress" \
  --relevance high \
  --agent "<agent-name>"

engram relationship create \
  --source-id <PROJECT_TASK_UUID> --source-type task \
  --target-id <NEW_ID> --target-type context \
  --relationship-type explains --agent "<agent-name>"
```

The previous progress entry remains as history. The newest entry with this title pattern is the current state.

---

## Related Skills

| Skill | When to Use |
|-------|-------------|
| `screenplay-scene-writer` | Writing or drafting a scene |
| `screenplay-character-developer` | Developing a character further |
| `screenplay-dialogue-refiner` | Tightening dialogue after a draft |
| `screenplay-beat-sheet-builder` | Adjusting story structure |
| `screenplay-plot-hole-finder` | Checking logic or consistency |
| `screenplay-world-builder` | Adding or checking world rules |
