---
name: screenplay-outliner
description: "Build a scene-level outline from a beat sheet before writing any draft pages. Use as the bridge between structural planning and first-draft writing. Fixes structural problems at the outline stage, where changes are cheap."
---

# Screenplay Outliner

## Overview

The outline is the bridge between the beat sheet and the first draft. It works at scene level — one entry per scene — describing what happens, who is in it, and what changes. Structural problems fixed at outline stage cost nothing. The same problems found in a completed draft cost weeks.

## When to Use

- After the beat sheet is complete and before any draft pages are written
- When resuming work after a break — re-read the outline to reorient
- When a scene is not working — check what the outline says it was supposed to do
- When a co-writer proposes adding or changing a scene — place it in the outline first

---

## Step 1: Load Beat Sheet and Characters

```bash
# Load the full beat sheet
engram ask query "project:<slug> type:beat"

# Load all characters
engram ask query "project:<slug> type:character"

# Load world rules for location and logic checks
engram ask query "project:<slug> type:world"
```

Do not start outlining until the beat sheet is complete and the main characters have profiles.

---

## Step 2: The Scene Entry Format

Each scene in the outline gets one entry. Keep entries short — this is not a draft, it is a map.

```
SCENE:       Sequential number (e.g. 1-01, 1-02, 2-01...)
BEAT:        Which of the 15 beats this scene belongs to (or "connector")
LOCATION:    INT/EXT, place, time of day
CHARACTERS:  Who is present (only those who need to be there)
WANT:        What the POV character wants in this scene
OBSTACLE:    What prevents them
CHANGE:      What is different at the end of this scene vs. the beginning
CAUSE→EFFECT: How this scene leads to the next ("therefore" or "but")
```

Every scene must have a CHANGE. If nothing changes, the scene does not earn its place.

---

## Step 3: Build Act by Act

Work through the outline act by act, not scene by scene in isolation:

**Act I (scenes 1 through approximately 8-10):**
- Opening image scene establishes world and protagonist in one visual moment
- Setup scenes reveal flaw, gift, and key relationships through action — not description
- Catalyst scene is a single clear event — not a gradual build
- Debate scenes show internal resistance, not just external hesitation
- Break Into Two is a decision the protagonist actively makes

**Act II-A (scenes through Midpoint):**
- Each scene escalates the new world — things mostly going right with growing complications
- B Story scene introduces the emotional/relationship strand early
- Fun and Games scenes deliver the premise promise — this section should be the most enjoyable to outline
- Midpoint scene changes the direction — what was ascending starts descending, or vice versa

**Act II-B (scenes from Midpoint through All Is Lost):**
- Cause-and-effect chain must be tightest here — each scene makes things worse in a direct causal way
- Team/relationship fractures happen here — not randomly, but as a consequence of the Midpoint shift
- All Is Lost is a concrete, external loss — not just an emotional low

**Act III:**
- Dark Night of the Soul is quiet — one scene, no plot movement, full internal reckoning
- Break Into Three is a decision that synthesises the A and B stories
- Finale scenes use the protagonist's growth — the solution must require who they became, not who they were
- Final image scene is the visual mirror of the opening image

---

## Step 4: The Outline Structural Checks

Before moving to a first draft, verify:

- [ ] Every scene has a CHANGE — no scene ends the same way it started
- [ ] Every scene transition has a "therefore" or "but" — no "and then" chains
- [ ] The inciting incident is a single scene at or before scene 5
- [ ] The Midpoint genuinely reverses the story's direction
- [ ] The All Is Lost is an external loss, not just a feeling
- [ ] The climax requires the protagonist's growth to work
- [ ] The final image visually mirrors the opening image

---

## Step 5: Fix Structure at Outline Stage

When a structural problem is found in the outline, fix it here — do not carry it into the draft.

Common outline fixes:
- **Scene with no change**: merge it with the preceding or following scene, or cut it
- **Broken cause-effect chain**: add a beat that creates the causal link, or reorder scenes
- **Act II sag**: insert a new complication scene, or escalate an existing one
- **Climax requires no growth**: rewrite the obstacle so it specifically defeats the protagonist's old flaw

---

## Storing the Outline in Engram

Store the outline in act-sized chunks so individual acts can be retrieved without loading the full document:

```bash
# Store one act's outline
engram context create \
  --title "<slug> outline act <number>" \
  --content "<all scene entries for this act>" \
  --tags "project:<slug>,type:outline,act:<number>" \
  --relevance high \
  --agent "<agent-name>"
```

Update when scenes are added, removed, or restructured:

```bash
engram ask query "project:<slug> type:outline act:<number>"
engram context update <CONTEXT_UUID> --content "<revised act outline>"
```

**Tag convention:**

| Tag | Purpose |
|-----|---------|
| `project:<slug>` | Scopes to one film — always required |
| `type:outline` | Marks this as outline data |
| `act:<number>` | Allows retrieval of one act's outline |

**Title convention:** `<slug> outline act <number>` — e.g. `heist outline act 1`
