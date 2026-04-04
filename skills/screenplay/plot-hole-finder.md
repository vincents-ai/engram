---
name: screenplay-plot-hole-finder
description: "Systematically identify and fix logical inconsistencies, plot holes, and structural weaknesses in any screenplay. Use after completing any act, beat cluster, or major story change."
---

# Screenplay Plot Hole Finder

## Overview

A systematic framework for finding and fixing plot holes in any screenplay. Covers four analysis layers (logic, character motivation, structure, theme), the cause-and-effect chain audit, the fix documentation protocol, and engram storage conventions for tracked holes.

## When to Use

- After completing any draft of an act or scene cluster
- When something "feels off" but you cannot name it
- Before starting a new act, to verify the previous one is solid
- When a co-writer asks "but why would [character] do that?"
- After any major story change, to check for cascading inconsistencies

---

## Step 1: Load the Context You Need

Do not load everything. Load only what is relevant to the section being checked.

```bash
# Load the beats covering the section under review
engram ask query "project:<slug> type:beat act:<number>"

# Load characters involved in the section
engram ask query "project:<slug> type:character <name>"

# Load world rules if checking logic holes
engram ask query "project:<slug> type:world"

# Load any previously logged holes for this project
engram ask query "project:<slug> type:plot-hole"
```

---

## Step 2: Run the Cause-and-Effect Chain Audit

Before running the four analysis layers, verify the connective tissue between beats:

For each beat transition in the section, ask:

> Does this follow because of what came before? ("therefore")
> Or does this complicate what came before? ("but")

If the only answer is "and then..." — the chain is broken. Flag it before going deeper.

---

## Step 3: The Four Analysis Layers

### Layer 1: Logic Holes (Physical World)

- Does anything require a resource, technology, or system the world has established as unavailable?
- Are timelines consistent? Do events happen at a plausible speed?
- Are distances and travel times consistent with the geography established?
- Are the established rules of this world being followed consistently?

### Layer 2: Character Holes (Motivation Logic)

- Why does each character make their specific choice in this scene?
- Is that choice consistent with their FLAW, WANT, and arc position?
- **The "Why Doesn't X Just..." Test**: state the simpler solution. If it is valid, either have the character try it and fail for a specific reason, or have another character explain why it will not work.
- Is any character behaving randomly — doing something that serves the plot but not their own logic?

### Layer 3: Structural Holes (Story Logic)

- Act I: Is the protagonist's flaw visible before the inciting incident?
- Act I: Does the inciting incident happen by page 12 at the latest?
- Act II: Does the Midpoint genuinely change the story's direction — not just raise stakes?
- Act II: Is everything getting worse in a causally connected way before the Act III break?
- Act III: Does the climax require the protagonist to use their **growth** — not just a skill or gadget they had from page one?
- Ending: Is the resolution earned by what came before, or does it arrive from outside the story?

### Layer 4: Theme Holes

- Does this scene reinforce, complicate, or contradict the story's central theme?
- Is the protagonist's arc — from flaw toward resolution — consistently visible throughout?
- Does the final image make the theme visible without dialogue?
- Is the theme stated early (by someone other than the hero) and then tested throughout?

---

## Step 4: Document Before Fixing

When a hole is found, record it before touching the script:

```
HOLE TYPE:     Logic / Character / Structure / Theme
LOCATION:      Beat name / scene / act / page range
DESCRIPTION:   What is inconsistent or broken
SEVERITY:      Minor (cosmetic) / Medium (affects scene) / Major (breaks act)
PROPOSED FIX:  The specific change to make
SIDE EFFECTS:  What else this fix might affect — check other beats and characters
```

---

## Step 5: Store Logged Holes in Engram

```bash
engram context create \
  --title "<slug> plot-hole <act>-<short-description>" \
  --content "<full hole record: type, location, description, severity, fix, side effects>" \
  --tags "project:<slug>,type:plot-hole,act:<number>,status:open" \
  --relevance high \
  --agent "<agent-name>"
```

When a hole is resolved, update its status:

```bash
engram context update <CONTEXT_UUID> \
  --content "<original record + resolution notes>"
# Update the tag manually in the next create if needed, or note resolution in content
```

**Tag convention:**

| Tag | Purpose |
|-----|---------|
| `project:<slug>` | Scopes to one film — always required |
| `type:plot-hole` | Marks this as a tracked hole |
| `act:<number>` | Which act the hole is in |
| `status:open` / `status:resolved` | Current state of the hole |

**Title convention:** `<slug> plot-hole <act>-<description>` — e.g. `heist plot-hole 2-vault-access-motive`

---

## Co-Writer Review Protocol

When reviewing with a young or non-technical co-writer, frame every finding as a question rather than a correction:

- "Does this make sense? If you were [character], would you really do that?"
- "Is there an easier way [character] could solve this? Why didn't they try it?"
- "Does this feel fair? Would [antagonist] really make that choice?"

Every finding surfaced by a co-writer goes into the hole log, even if it turns out not to be a real problem.
