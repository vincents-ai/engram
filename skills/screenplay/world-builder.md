---
name: screenplay-world-builder
description: "Establish and maintain consistent world-building rules for any screenplay. Use before writing any scene in a new location or situation, when introducing new systems, or when checking scene consistency against established rules. Also covers genre conventions."
---

# Screenplay World Builder

## Overview

A world-building framework for any screenplay. Covers the six-field world document, genre tone conventions, universal consistency rules, the process for introducing new world elements without breaking established rules, and engram storage for all world data.

## When to Use

- At the start of a project — establish world rules before writing any scenes
- Before writing any scene that introduces a new location, technology, or social system
- When checking whether a scene's backdrop violates established rules
- When a co-writer asks "what would actually happen if..."
- After a major story change, to check whether world rules need updating

---

## Step 1: Load Existing World Context

```bash
# Load all world rules for this project
engram ask query "project:<slug> type:world"

# Load a specific aspect of the world
engram ask query "project:<slug> type:world <aspect>"
# e.g. "project:<slug> type:world economy"
```

---

## Step 2: Build the World Document

Every screenplay world needs these six fields documented before scene writing begins:

```
THE STATUS QUO (or THE EVENT):
  What is the central situation that defines this world?
  What caused it? (Or is the cause deliberately left unknown?)

WHAT STILL WORKS:
  What systems, technologies, and social structures are intact?

WHAT HAS CHANGED OR FAILED:
  What is unavailable, broken, or different from the audience's familiar world?

THE TIMELINE:
  How did the world reach its current state?
  What is the rate of change — sudden shock or slow drift?

THE ECONOMY:
  What do people trade, need, and hoard?
  What has become newly scarce or newly worthless?

THE GEOGRAPHY:
  What are the key locations and what makes each narratively distinct?
  What are the distances and travel times between them?
  Keep these consistent — check every scene against them.
```

---

## Step 3: Genre Tone Conventions

Genre shapes what the audience expects and what structure needs to deliver. Establish the genre before writing and check scenes against it.

| Genre | Structural expectation | Tone markers |
|---|---|---|
| Family Adventure | Clear moral stakes; protagonist is young or underestimated; adults are flawed but not villains | Wonder, danger, earned triumph |
| Sci-Fi / Thriller | World rules must be airtight; logic holes break audience trust more than in other genres | Tension, escalating stakes, "what if" taken seriously |
| Drama | Character arc is the plot; external events are catalysts for internal change | Emotional honesty, restraint, earned catharsis |
| Comedy | Escalation of a central comic premise; character flaws are the engine | Specificity over generality; character-based jokes outlast situation jokes |
| Horror | Threat must be established and respected; rules of the threat must hold | Dread over shock; what the audience imagines is worse than what you show |

For any genre, verify:
- Does the opening image immediately signal the correct genre tone?
- Do the stakes match what this genre's audience came to feel?
- Does the climax deliver the emotional payoff the genre promises?

---

## Step 4: Universal Consistency Rules

1. **Establish rules before using them** — the audience accepts any rule if it is set up clearly before it matters
2. **No magic solutions** — every fix must use only resources the world has established as available
3. **Scarcity is real** — if something is rare in this world, it stays rare; do not conjure it when convenient
4. **Time is honest** — solutions take the time they would plausibly take; do not compress for convenience
5. **Geography matters** — distances and travel times must be consistent; check every scene
6. **Characters are products of their world** — their skills, fears, and assumptions should reflect what this world would produce

---

## Step 5: Introducing New World Elements

When adding a new location, technology, or social system mid-script:

1. Check whether any established rule constrains or enables it
2. Decide: has it always existed (retroactive) or is it new to the story (introduced now)?
3. If retroactive: check whether any prior scenes need updating for consistency
4. If new: establish it clearly the first time it appears, before it becomes plot-critical
5. Store the addition in engram and link it to the world context

---

## Step 6: The "What Would Actually Happen" Test

Before writing any scene that depends on the world behaving a certain way, ask: what would this situation actually produce in real life? If the answer differs from what the script needs, either:

- Adjust the scene to match reality, or
- Establish a specific in-world reason why this situation is an exception to real-world logic

Never skip this test for scenes that hinge on logistics, technology, or social behaviour.

---

## Storing World Data in Engram

Store world rules at a granular enough level that individual aspects can be retrieved without loading everything:

```bash
# Store a specific world aspect
engram context create \
  --title "<slug> world <aspect>" \
  --content "<rules, constraints, examples for this aspect>" \
  --tags "project:<slug>,type:world,world:<aspect>" \
  --relevance high \
  --agent "<agent-name>"

# Store the full world document snapshot
engram context create \
  --title "<slug> world full" \
  --content "<all six fields + genre conventions>" \
  --tags "project:<slug>,type:world,world:full" \
  --relevance high \
  --agent "<agent-name>"
```

Update rather than duplicate when a rule changes:

```bash
engram ask query "project:<slug> type:world <aspect>"
engram context update <CONTEXT_UUID> --content "<updated rules>"
```

**Tag convention:**

| Tag | Purpose |
|-----|---------|
| `project:<slug>` | Scopes to one film — always required |
| `type:world` | Marks this as world-building data |
| `world:<aspect>` | Allows retrieval of one aspect (e.g. `world:economy`, `world:geography`) |

**Title convention:** `<slug> world <aspect>` — e.g. `heist world economy`
