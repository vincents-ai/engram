---
name: screenplay-theme-developer
description: "Establish and develop the central theme for any screenplay before structural work begins. Use when the story lacks a compass, when the protagonist's arc feels arbitrary, or when scenes don't connect to a larger meaning."
---

# Screenplay Theme Developer

## Overview

Theme is the engine that drives a story and gives it meaning. It is not the plot. It is the question the story is asking, or the argument the story is making. It is what the protagonist's arc is actually about beneath the surface events. Established early, theme becomes the compass that resolves every structural decision that follows.

## When to Use

- Before the beat sheet or outline — theme should precede structure
- When the story feels directionless or the protagonist's arc feels arbitrary
- When scenes are disconnected from each other despite plot logic being intact
- When a co-writer asks "what is this story really about?"
- When a structural problem cannot be solved by plot alone — theme usually resolves it

---

## Step 1: Find the Theme Question

A theme is most useful as a question the story raises and answers through action — not through dialogue.

Ask these questions about the story:

- What does the protagonist believe at the start that is wrong, incomplete, or incomplete?
- What will they have to give up, accept, or understand to grow?
- What is the tension between two things both of which have value? (freedom vs. security; ambition vs. loyalty; self-preservation vs. community)

The theme lives in that tension. It is not a moral lesson. It is a genuine question the story wrestles with.

---

## Step 2: State the Theme in One Sentence

Write the theme as a single declarative sentence — an argument, not a question:

```
"[Something] is more powerful / more important / more dangerous than [something else]."
"The cost of [value] is [sacrifice]."
"[Flaw] prevents [goal] until [the character learns / accepts / releases] [truth]."
```

Examples:
- "Knowledge shared is more powerful than knowledge hoarded."
- "The instinct to control is itself the source of chaos."
- "You cannot outrun grief — you can only carry it."

If you cannot state the theme in one sentence, it is not specific enough yet.

---

## Step 3: Plant the Theme Stated Beat

The theme must be spoken aloud in the script — once, early, by someone other than the protagonist, who does not fully understand what they are saying. This is Beat 2 in the Save the Cat! framework (page 5).

The protagonist hears it, ignores it or dismisses it, and then spends the rest of the film learning it is true.

The Theme Stated line should:
- Sound like ordinary conversation, not a speech
- Be said by a supporting character, not the hero
- Not be recognised as significant by anyone in the scene
- Be recognisable in retrospect — when the audience looks back at the end

---

## Step 4: Map Theme to the Protagonist Arc

The protagonist's arc is the theme made visible through action. Map them together:

```
ACT I:     Protagonist embodies the WRONG BELIEF the theme challenges
           (Their flaw = the thematic error)

ACT II-A:  Protagonist pursues their goal using the wrong belief — partial success
           (The wrong belief seems to be working)

MIDPOINT:  The wrong belief is tested — it produces a false victory or false defeat

ACT II-B:  The consequences of the wrong belief accumulate
           (The flaw causes the losses that lead to All Is Lost)

ACT III:   Protagonist abandons or overcomes the wrong belief
           (The climax requires the correct belief to succeed)

FINAL IMAGE: The theme is visible in the contrast with the Opening Image
```

---

## Step 5: Test Every Scene Against Theme

Once the theme is established, use it as a filter for every scene:

- Does this scene reinforce, complicate, or test the theme?
- If a scene neither advances plot nor touches theme — it is a candidate for cutting
- When stuck on a structural problem, ask: what does the theme say should happen here?

---

## Step 6: Theme Is Not Message

Theme is not a moral that the story proves. It is a genuine question the story takes seriously from both sides. The antagonist should be the living argument against the theme — not evil, but wrong in a coherent way that the protagonist has to actually defeat, not just outlast.

A story where the theme is obvious from page one and never genuinely challenged is a sermon, not a screenplay.

---

## Storing Theme Data in Engram

```bash
engram context create \
  --title "<slug> theme" \
  --content "<theme statement + tension it embodies + how it maps to protagonist arc + Theme Stated beat candidate>" \
  --tags "project:<slug>,type:theme" \
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

Update rather than duplicate when the theme is refined:

```bash
engram ask query "project:<slug> type:theme"
engram context update <CONTEXT_UUID> --content "<revised theme statement + rationale>"
```

**Tag convention:**

| Tag | Purpose |
|-----|---------|
| `project:<slug>` | Scopes to one film — always required |
| `type:theme` | The single theme record for this project |

**Title convention:** `<slug> theme` — one per project, updated in place.
