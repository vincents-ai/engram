---
name: screenplay-character-developer
description: "Develop deep, consistent characters for any screenplay. Use when creating new characters, deepening existing ones, running character interviews, or checking voice consistency across scenes."
---

# Screenplay Character Developer

## Overview

A character development framework for any screenplay. Covers the core six-field profile, the character interview protocol for testing backstories, arc progression mapping, and a co-writer protocol for collaborative sessions.

## When to Use

- A new character needs to be introduced to the script
- A character's motivation feels thin or unclear
- Dialogue sounds "off" for a specific character
- A co-writer wants to add or change a character
- Checking character consistency across scenes

---

## Step 1: Load Existing Character Context

Before creating or editing any character, retrieve what is already known:

```bash
# Get all characters for this project
engram ask query "project:<slug> type:character"

# Get a specific character
engram ask query "project:<slug> type:character <character-name>"
```

Only load the character(s) relevant to the current task. Do not load the full beat sheet or world rules unless the character work requires it.

---

## Step 2: Build or Update the Core Profile

Every character needs all six fields before they appear in any scene:

```
WANT  (External Goal): What does this character consciously want in the story?
NEED  (Internal):      What do they actually need to grow as a person?
WOUND:                 What past event created their core fear or flaw?
FLAW:                  The one trait that keeps getting them in trouble
STRENGTH:              The one skill only they have
VOICE SIGNATURE:       2-3 specific speech patterns or habits unique to this character
```

If any field is missing, the character is not ready to appear in a scene.

---

## Step 3: Map the Arc

Map the change from Act I to Act III before writing any scene involving this character:

```
ACT I STATE:     Who they are at the start — defined by their FLAW
MIDPOINT SHIFT:  What forces them to change — usually a loss or revelation
ACT III STATE:   Who they become — their FLAW is overcome or accepted
```

---

## Step 4: Run the Character Interview

To test whether a character is fully formed, interview them in-character. Ask questions that reveal worldview, fears, and desires. If the answers feel generic or interchangeable with another character, they need more work.

Sample questions:
- "What do you want more than anything right now?"
- "What are you most afraid of losing?"
- "What do you believe that nobody else in this story believes?"
- "What's the one thing you would never do — and why?"

---

## Step 5: Scene Consistency Check

Before writing any scene with this character, verify:

- Would this character actually say this line? (Check VOICE SIGNATURE)
- Is this action consistent with their WANT or NEED?
- Does this scene move their arc forward?
- Are they serving the story, or just filling space?

---

## Co-Writer Collaboration Protocol

When a co-writer wants to add or change a character, ask the Four Magic Questions:

1. "What does this character **want** more than anything in the world?"
2. "What is something they are **really bad at**?"
3. "What is their **superpower** — the one thing only they can do?"
4. "How do they **talk**? Do they have a catchphrase or a weird habit?"

Never reject a co-writer's character idea. Every idea either goes in the script or into the ideas parking lot.

---

## Storing Character Data in Engram

When a character profile is created or meaningfully updated, store it:

```bash
engram context create \
  --title "<slug> character <name>" \
  --content "<full six-field profile + arc + any session notes>" \
  --tags "project:<slug>,type:character,character:<name>" \
  --relevance high \
  --agent "<agent-name>"
```

Link it to the active task:

```bash
engram relationship create \
  --source-id <TASK_UUID> --source-type task \
  --target-id <CONTEXT_UUID> --target-type context \
  --relationship-type explains --agent "<agent-name>"
```

**Tag convention:**

| Tag | Purpose |
|-----|---------|
| `project:<slug>` | Scopes all data to one film — always required |
| `type:character` | Marks this as character data |
| `character:<name>` | Allows retrieval of one specific character |

**Title convention:** `<slug> character <name>` — e.g. `heist character maya`

To update an existing profile rather than create a duplicate:

```bash
# Find the existing context ID first
engram ask query "project:<slug> type:character <name>"

# Then update it
engram context update <CONTEXT_UUID> --content "<updated profile>"
```
