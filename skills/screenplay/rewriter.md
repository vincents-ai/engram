---
name: screenplay-rewriter
description: "Guide the rewrite process for any screenplay draft. Use after completing a first draft. Covers the sequential pass system, taking a break, working with feedback from collaborators, and knowing which notes to act on."
---

# Screenplay Rewriter

## Overview

All writing is rewriting. A first draft's job is to exist — to give you material to work with. The real screenplay is found in the rewrite. This skill covers the sequential pass system (scene order → character arcs → dialogue), how to take and process feedback from collaborators, and how to know when a draft is ready to move forward.

## When to Use

- After completing any full draft of an act or the whole script
- When resuming a rewrite after a break
- Before sharing a draft with a co-writer or collaborator for feedback
- When feedback has been received and needs to be processed
- When a pass is complete and the next pass needs to begin

---

## Step 1: Take a Break First

Before beginning any rewrite, step away from the draft. The minimum is a few days; ideally a week or more for a full draft. The distance makes structural problems visible that were invisible during writing.

Do not begin a rewrite pass while still emotionally attached to the most recent writing session.

---

## Step 2: Load the Current State

```bash
# Load the beat sheet to check structural intent
engram ask query "project:<slug> type:beat"

# Load the outline to check scene-level intent
engram ask query "project:<slug> type:outline act:<number>"

# Load any logged plot holes
engram ask query "project:<slug> type:plot-hole status:open"

# Load character profiles to verify arc consistency
engram ask query "project:<slug> type:character"
```

---

## Step 3: The Sequential Pass System

Run passes in this order. Do not mix passes. Each pass has one job.

### Pass 1: Scene Order (Structure Lock)

Read the full draft for structure only. Do not touch dialogue or description yet.

Ask of every scene:
- Does this scene belong where it is, or does it belong earlier or later?
- Does this scene earn its place — does something change by the end?
- Is the cause-and-effect chain intact from the previous scene?

Actions: cut scenes that earn no change, reorder scenes that are in the wrong position, flag scenes that are doing too much or too little. Update the outline in engram as structure is locked.

### Pass 2: Character Arc Pass

Once scene order is locked, read for each major character individually — one character at a time, not the whole script at once.

For each character, ask:
- Is their flaw visible early and consistent throughout?
- Does the Midpoint shift something in their arc?
- Does the All Is Lost cost them something that matters?
- Do they reach their Act III state through earned change, not coincidence?
- Is their voice consistent across all their scenes?

Load that character's profile from engram before their pass:

```bash
engram ask query "project:<slug> type:character <name>"
```

Update the character profile if the draft has changed who they are.

### Pass 3: Dialogue Pass

Run this pass last. By the time the dialogue pass begins, the structure and arcs must be locked.

For every line:
- Read it aloud. Would a real person say this?
- Could another character say this line? If yes, rewrite until it is unmistakably this character.
- Is it doing work (revealing character, advancing conflict, adding subtext) or just filling space?
- Is there exposition that could be cut, compressed, or made more indirect?

Apply the five dialogue diseases from `screenplay-dialogue-refiner` during this pass.

### Pass 4: Description Pass (Optional but High Value)

After dialogue is clean, read action lines only:

- Are they in present tense with active verbs?
- Do any exceed 4 lines? Break them up.
- Is there anything described that the camera cannot film (internal thoughts, backstory)?
- Does the opening image scene's description make the theme visible without words?

---

## Step 4: Working With Feedback

### Soliciting Notes

Get notes from people with different perspectives — a director reads differently from an actor, who reads differently from another writer. Each will find different problems. Seek at least two different viewpoint types.

Do not get notes from people who dislike the genre. Their discomfort with the form will read as story problems.

### Processing Notes

When notes arrive, sort them before acting on any:

```
NOTE TYPE:    Specific (line/scene) / Structural / Tonal / Character
FREQUENCY:   Did more than one reader flag this?
VALIDITY:    Is this a real story problem, or a preference?
THE NOTE BEHIND THE NOTE: What is the reader actually responding to — even if their suggested fix is wrong?
```

The most important skill in processing notes: separate the **problem** from the **proposed solution**. A reader's suggested fix is often wrong. Their identification of a problem is usually right. Find the problem, solve it your own way.

### Notes Hierarchy

Act on notes in this order:

1. Notes that multiple readers gave independently — highest signal
2. Notes that identify a problem you already suspected
3. Notes from readers whose taste closely matches the intended audience
4. Notes that suggest a problem but propose a solution you disagree with — fix the problem, ignore the solution
5. Notes that are purely personal preference with no story logic behind them — log but do not act

### Free Pass Limits

For collaborative projects, establish upfront how many free revision passes are reasonable before a new pass requires a new agreement. This protects the work and the relationship.

---

## Step 5: Store Rewrite State in Engram

After each pass, store a brief note on what was done and what remains:

```bash
engram context create \
  --title "<slug> rewrite pass <number> <pass-type>" \
  --content "<what was changed, what was found, what remains open>" \
  --tags "project:<slug>,type:rewrite,pass:<pass-type>" \
  --relevance medium \
  --agent "<agent-name>"
```

**Tag convention:**

| Tag | Purpose |
|-----|---------|
| `project:<slug>` | Scopes to one film — always required |
| `type:rewrite` | Marks this as a rewrite pass record |
| `pass:<pass-type>` | Which pass: `structure`, `character`, `dialogue`, `description` |

**Title convention:** `<slug> rewrite pass <number> <type>` — e.g. `heist rewrite pass 1 structure`

---

## Step 6: Knowing When to Stop

A draft is ready to move forward when:

- All logged plot holes are resolved
- Every scene has a change
- Every character's arc is consistent
- Dialogue reads as distinct per character when read aloud
- The opening and final images mirror each other
- You can state what the story is about in one sentence and the script delivers it

If any of these are not true, there is another pass to do.
