---
name: screenplay-logline-writer
description: "Craft and test the logline for any screenplay. Use before outlining begins — the logline is the compass for all structural decisions that follow. Also use when the story loses direction mid-draft."
---

# Screenplay Logline Writer

## Overview

A logline is 1-2 sentences that define the protagonist, the inciting incident, and the direction of action. It is not a tagline, a summary, or a question. It is the story's spine — written before the beat sheet, used to test every structural decision, and returned to whenever the script loses focus.

## When to Use

- Before any outlining or scene writing begins
- When the story feels directionless mid-draft
- When a co-writer describes a new idea — test it against the logline first
- When preparing to pitch or share the project

---

## Step 1: Load Existing Theme Context

Before writing the logline, check whether a theme has already been established:

```bash
engram ask query "project:<slug> type:theme"
```

The logline should be consistent with the theme. If no theme exists yet, use the `screenplay-theme-developer` skill first.

---

## Step 2: The Logline Formula

A strong logline contains all three elements:

```
PROTAGONIST:        Who the story is about — described by their defining trait,
                    not their name. Name means nothing to someone who hasn't read it.

INCITING INCIDENT:  The event that forces the protagonist into action.
                    This is not the setup — it is the moment the story begins.

ACTION / DIRECTION: What the protagonist must now do, pursue, or survive.
                    This implies the central conflict and the stakes.
```

Example structure:

> A [protagonist defined by trait], forced by [inciting incident], must [action] before [consequence or opposition].

---

## Step 3: Logline Rules

- **Present tense** — always. "A woman discovers..." not "A woman discovered..."
- **No character names** — describe who they are, not what they are called
- **No taglines** — "In a world where..." is a tagline, not a logline
- **No questions** — "What happens when...?" is not a logline
- **1-2 sentences maximum** — if it needs more, the story is not focused enough yet
- **Mention time period** only if it is essential to understanding the stakes
- **Include a hint of genre tone** — word choice should signal whether this is a thriller, comedy, family adventure, etc.

---

## Step 4: The Logline Tests

Run all four tests before accepting a logline:

### Test 1: The Swap Test
Could this logline describe another well-known film? If yes, it is too generic. Add the specific detail that makes this story and no other story.

### Test 2: The Stakes Test
What happens if the protagonist fails? If the answer is "not much" — the stakes are not in the logline yet.

### Test 3: The Conflict Test
Is there a clear opposing force (person, system, circumstance, or self)? If the protagonist has no obstacle implied, it is a premise, not a logline.

### Test 4: The Tone Test
Read the logline aloud. Does the word choice feel like the genre? A horror logline should feel different from a family adventure logline even at sentence level.

---

## Step 5: Use the Logline as a Compass

Once the logline is locked, use it to evaluate every structural decision:

- Does this beat serve the protagonist defined in the logline?
- Does this scene follow from the inciting incident the logline describes?
- Does this climax resolve the action the logline set up?

If a beat or scene cannot be connected back to the logline, it either belongs in the B story or should be cut.

---

## Step 6: Practice on Known Films

Before writing the project logline, practice by writing loglines for 3 films in the same genre. Compare word choice, protagonist description, and stakes framing. This calibrates the tone register before writing the real one.

---

## Storing the Logline in Engram

```bash
engram context create \
  --title "<slug> logline" \
  --content "<logline text + any rejected variants and reasons>" \
  --tags "project:<slug>,type:logline" \
  --relevance critical \
  --agent "<agent-name>"
```

Link to the active task:

```bash
engram relationship create \
  --source-id <TASK_UUID> --source-type task \
  --target-id <CONTEXT_UUID> --target-type context \
  --relationship-type explains --agent "<agent-name>"
```

Update rather than duplicate when the logline is refined:

```bash
engram ask query "project:<slug> type:logline"
engram context update <CONTEXT_UUID> --content "<revised logline + rationale for change>"
```

**Tag convention:**

| Tag | Purpose |
|-----|---------|
| `project:<slug>` | Scopes to one film — always required |
| `type:logline` | The single logline record for this project |

**Title convention:** `<slug> logline` — one per project, updated in place.
