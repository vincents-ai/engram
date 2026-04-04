---
name: screenplay-scene-writer
description: "Write individual scenes for any screenplay in proper Fountain format. Use when drafting a scene from the beat sheet, translating a verbal idea into script format, or rewriting a scene that isn't working."
---

# Screenplay Scene Writer

## Overview

A scene-writing framework for any screenplay. Covers mandatory Fountain format rules, the five-question scene construction protocol, writing style calibration by scene type, and a co-writer protocol for translating verbal ideas into formatted scenes.

## When to Use

- Writing any scene from scratch
- Expanding a beat into a fully formatted scene
- Translating a co-writer's verbal description into proper screenplay format
- Rewriting a scene that isn't working

---

## Step 1: Load the Context You Need

Before writing, retrieve only the context relevant to this scene. Do not load everything.

```bash
# Load the beat this scene belongs to
engram ask query "project:<slug> type:beat <beat-name-or-number>"

# Load only the characters in this scene
engram ask query "project:<slug> type:character <name>"

# Load world rules if the scene involves a location or system not yet written
engram ask query "project:<slug> type:world"
```

---

## Step 2: Answer the Five Construction Questions

Answer all five before writing a single line:

1. **WHERE are we?** — INT/EXT, location, time of day
2. **WHO is in this scene?** — only characters who need to be there
3. **What does the POV character WANT?** — their immediate goal in this scene
4. **What OBSTACLE prevents them?** — conflict: person, circumstance, or self
5. **What CHANGES by the end?** — every scene must end differently than it started

If you cannot answer all five, stop and resolve them before writing.

---

## Step 3: Write in Fountain Format

### Scene Heading (Slugline)
```
INT. LOCATION - TIME OF DAY
EXT. LOCATION - TIME OF DAY
```

### Action Lines
- Present tense, active verbs
- Maximum 4 lines before a visual break
- Describe only what the audience sees and hears — never internal thoughts
- New character name in ALL CAPS on first appearance only
- One page ≈ one minute of screen time

### Dialogue Block
```
                    CHARACTER NAME
          (parenthetical — only if essential)
     Dialogue text.
```

### Parentheticals
Use only when the reading of a line is genuinely counter-intuitive. Never for obvious emotions. Maximum one per 2-3 pages.

### Montage
```
MONTAGE:

- Short visual bullet.
- Short visual bullet.

END MONTAGE.
```

---

## Step 4: Apply Scene-Type Rules

### Action / Chase
Short sentence fragments. Ground danger in physical reality. No slow-motion in prose — that is the director's job.

### Emotional / Character
Let action do the work before dialogue. One character's guard dropping escalates the scene. Avoid on-the-nose emotion in dialogue.

### Technical Explanation
Use an analogy first, then the technical term. Visual demonstration beats verbal explanation. Cap technical dialogue at 3 lines before an action beat.

---

## Step 5: Store the Draft in Engram

When a scene draft is complete or significantly revised:

```bash
engram context create \
  --title "<slug> scene <act>-<sequence> <short-description>" \
  --content "<scene draft or summary of changes>" \
  --tags "project:<slug>,type:scene,act:<number>,beat:<beat-name>" \
  --relevance high \
  --agent "<agent-name>"
```

Link to the active task:

```bash
engram relationship create \
  --source-id <TASK_UUID> --source-type task \
  --target-id <CONTEXT_UUID> --target-type context \
  --relationship-type explains --agent "<agent-name>"
```

**Tag convention:**

| Tag | Purpose |
|-----|---------|
| `project:<slug>` | Scopes to one film — always required |
| `type:scene` | Marks this as a scene draft |
| `act:<number>` | Which act the scene belongs to |
| `beat:<beat-name>` | Which beat this scene fulfils |

**Title convention:** `<slug> scene <act>-<seq> <description>` — e.g. `heist scene 1-03 protagonist discovers the vault`

---

## Co-Writer Collaboration Protocol

When a co-writer dictates or describes a scene:

1. Capture exactly what they say first — do not filter or correct
2. Ask: "What does it look like? Describe it like a camera"
3. Ask: "What does [character] say? What does [other character] say back?"
4. Ask: "What happens that changes everything?"
5. Translate their words into Fountain format, keeping their ideas intact
6. Read the formatted scene back to them for approval before storing
