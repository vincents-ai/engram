---
name: screenplay-beat-sheet-builder
description: "Build and maintain a master beat sheet for any screenplay. Use to plan the structural skeleton of a script, place new ideas into structure, check pacing, and track cause-and-effect chains across all three acts."
---

# Screenplay Beat Sheet Builder

## Overview

A structural framework for any screenplay. Covers the Save the Cat! 15-beat system, cause-and-effect beat chaining to prevent Act II sag, emotional through-line tracking alongside plot beats, pacing checks, and engram storage conventions for the living beat sheet.

## When to Use

- At the very start of a project — before writing any scenes
- At the start of any writing session to orient where you are
- After a brainstorm session to place new ideas into structure
- When a co-writer proposes a new plot element
- To check pacing or find where the story is losing energy
- Before starting a new act, to verify the previous one holds

---

## Step 1: Load Existing Beat Sheet Context

```bash
# Get the full beat sheet for this project
engram ask query "project:<slug> type:beat"

# Get a specific beat or act
engram ask query "project:<slug> type:beat act:<number>"
engram ask query "project:<slug> type:beat <beat-name>"
```

---

## Step 2: The Save the Cat! 15-Beat Framework

| Beat | Name | Typical Page | What It Does |
|------|------|-------------|--------------|
| 1 | Opening Image | p1 | Visual statement of the theme and starting world |
| 2 | Theme Stated | p5 | Someone other than the hero states the theme without knowing it |
| 3 | Setup | p1-10 | Establish the hero's ordinary world, flaw, gift, and key relationships |
| 4 | Catalyst | p12 | The inciting incident — the world changes |
| 5 | Debate | p12-25 | Hero resists the call — internal conflict |
| 6 | Break Into Two | p25 | Hero makes an active choice — enters the new world |
| 7 | B Story | p30 | The emotional/relationship story that carries the theme |
| 8 | Fun & Games | p30-55 | The promise of the premise — what the audience came to see |
| 9 | Midpoint | p55 | False victory or false defeat — stakes raised, direction shifts |
| 10 | Bad Guys Close In | p55-75 | External pressure mounts; internal doubt and team fractures |
| 11 | All Is Lost | p75 | The worst moment — the hero loses what matters most |
| 12 | Dark Night of the Soul | p75-80 | Hero at their lowest; the old way is dead |
| 13 | Break Into Three | p80 | Hero finds the solution — A and B stories synthesise |
| 14 | Finale | p80-95 | Hero executes the plan using their growth, not just their skill |
| 15 | Final Image | p95+ | Mirror of the opening image — shows how much has changed |

---

## Step 3: Build with Cause-and-Effect Chains

Beats must connect causally, not just sequentially. Test every transition with:

> **"This happened, therefore…"** → the next beat follows as a consequence.

> **"This happened, but…"** → the next beat is a complication or reversal.

If a beat transition can only be described as "and then…" the connective tissue is weak and the story will feel episodic. Rework the beat until it has a causal relationship to what precedes and follows it.

---

## Step 4: Track the Emotional Through Line

Alongside each plot beat, note the protagonist's **emotional state**:

```
BEAT:           [beat name and page]
PLOT EVENT:     What happens externally
PROTAGONIST FEELS: How they are responding emotionally
ARC PROGRESS:   Are they closer to or further from who they need to become?
```

The emotional through line must be as continuous as the plot line. If there are beats where we don't know how the protagonist feels about what just happened, that is a gap to fill.

---

## Step 5: Check the First 10 Pages

Industry professionals judge a script within the first 10 pages. Verify:

- [ ] The protagonist is introduced with a character-revealing action (not just a description)
- [ ] The world and tone are established clearly
- [ ] The protagonist's flaw and gift are visible
- [ ] The inciting incident has happened by page 12 at the latest
- [ ] The reader wants to know what happens next

If any of these are missing, the opening needs work before the rest of the script is written.

---

## Step 6: Opening Image vs. Final Image Check

Before finalising the beat sheet, verify the bookends:

- Does the final image directly mirror and invert the opening image?
- Can you see the entire theme in the visual difference between them — without dialogue?

If not, one or both images need rethinking.

---

## Step 7: Pacing Check

| Section | Pages | Should Feel Like |
|---------|-------|-----------------|
| Act I | 1-25 | Setup, curiosity, a world we want to live in |
| Act II-A | 25-55 | Adventure, momentum, things going (mostly) right |
| Act II-B | 55-75 | Pressure, betrayal, things going wrong |
| Act III | 75-95+ | Tense → triumphant → earned emotional release |

If a section drags: find a scene to cut, compress, or replace with a stronger cause-and-effect beat.
If a section rushes: find a scene to add or expand, or a relationship beat that needs more time.

---

## How to Place a New Idea

When a co-writer proposes a new plot element:

1. Ask: "Where in the story does this happen — beginning, middle, or end?"
2. Map it to the nearest beat in the table above
3. Test: does it replace or enrich the existing beat?
4. Test: does adding it break any cause-and-effect chains?
5. Test: does it create plot holes? (use `screenplay-plot-hole-finder`)
6. Update the stored beat sheet in engram

---

## Storing Beat Sheet Data in Engram

Store each beat — or a cluster of beats — as a separate context so you can retrieve them individually:

```bash
# Store a single beat
engram context create \
  --title "<slug> beat <number> <beat-name>" \
  --content "<beat description, scene ideas, cause-effect links, open questions>" \
  --tags "project:<slug>,type:beat,act:<number>,beat:<beat-name>" \
  --relevance high \
  --agent "<agent-name>"

# Store the full living beat sheet snapshot
engram context create \
  --title "<slug> beat sheet full" \
  --content "<full 15-beat overview with statuses>" \
  --tags "project:<slug>,type:beat,beat:full-sheet" \
  --relevance high \
  --agent "<agent-name>"
```

Update rather than duplicate when a beat changes:

```bash
engram ask query "project:<slug> type:beat <beat-name>"
engram context update <CONTEXT_UUID> --content "<revised beat>"
```

**Tag convention:**

| Tag | Purpose |
|-----|---------|
| `project:<slug>` | Scopes to one film — always required |
| `type:beat` | Marks this as structural beat data |
| `act:<number>` | Which act this beat belongs to |
| `beat:<beat-name>` | Allows retrieval of one specific beat |

**Title convention:** `<slug> beat <number> <name>` — e.g. `heist beat 9 midpoint`
