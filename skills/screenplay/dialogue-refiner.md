---
name: screenplay-dialogue-refiner
description: "Polish and authenticate dialogue for any screenplay. Use after any scene draft to eliminate exposition dumps, on-the-nose emotion, and generic voice, and to ensure each character sounds unmistakably themselves."
---

# Screenplay Dialogue Refiner

## Overview

A dialogue refinement framework for any screenplay. Covers the three-step refinement process, the five common dialogue diseases and their cures, voice profile construction, and how to store and retrieve per-character voice data in engram.

## When to Use

- After any scene draft is written — run a dialogue pass
- When a line feels "writerly" or unnatural
- When a character sounds like every other character
- When dialogue is doing too much explaining
- When a co-writer says "that doesn't sound right"

---

## Step 1: Load the Voice Profiles You Need

Load only the characters whose dialogue is being refined. Do not load the full beat sheet or world rules.

```bash
# Get the voice profile for a specific character
engram ask query "project:<slug> type:dialogue character:<name>"

# Get all voice profiles for this project (only if doing a full pass)
engram ask query "project:<slug> type:dialogue"
```

---

## Step 2: Run the Three-Step Refinement Process

### Step 2a: Read Aloud Test
Read every line out loud. Flag anything that:
- Trips on the tongue
- Sounds like a textbook
- No real person would say
- Explains what the audience already knows

### Step 2b: Character Voice Test
For each line: could any other character in this script say this line? If yes — rewrite it until it is unmistakably this character's voice. Use their VOICE SIGNATURE from the character profile.

```bash
# Fetch the character's voice signature if not already loaded
engram ask query "project:<slug> type:character <name>"
```

### Step 2c: Subtext Layer
Good dialogue is about what characters do not say:
- What does the character **want** to say?
- What are they **actually** saying?
- What are they **hiding**?

The gap between those three is the subtext. If all three are the same, the line is on-the-nose.

---

## Step 3: Check for the Five Dialogue Diseases

### Disease 1: The Exposition Dump
**Symptom**: Characters explaining backstory to each other that they would both already know.
**Cure**: Give exposition only to a character who genuinely does not know it yet.

### Disease 2: On-The-Nose Emotion
**Symptom**: "I'm scared." / "This is really hard for me."
**Cure**: Show the emotion through action lines. The dialogue says something else entirely.

### Disease 3: Too Much Chit-Chat
**Symptom**: Characters greeting each other, being polite, saying goodbye.
**Cure**: Cut it. Enter the scene at the conflict, not the hello.

### Disease 4: Everyone Sounds The Same
**Symptom**: Any character could swap lines with any other.
**Cure**: Each character's voice profile must be distinct. One character per page should be identifiable by voice alone, with no name attached.

### Disease 5: Dialogue That Describes Action
**Symptom**: "Look out — that car is heading straight for us!"
**Cure**: Put it in the action line. Dialogue is for what characters choose to say, not what they can observe.

---

## Step 4: Build or Update a Voice Profile

For each major character, maintain a voice profile:

```
SPEECH RHYTHM:       Fast/slow, complete sentences or fragments, interrupts or waits?
VOCABULARY:          Technical, simple, figurative, literal?
DEFLECTION:          How do they avoid saying what they mean?
TELL:                The word, phrase, or habit that appears when under pressure
WHAT THEY NEVER SAY: The direct emotional statement they are always circling
```

---

## Step 5: Store Voice Profile Data in Engram

When a voice profile is created or updated:

```bash
engram context create \
  --title "<slug> dialogue <character-name>" \
  --content "<full voice profile + any before/after examples from this session>" \
  --tags "project:<slug>,type:dialogue,character:<name>" \
  --relevance high \
  --agent "<agent-name>"
```

To update rather than duplicate:

```bash
# Find the existing context ID
engram ask query "project:<slug> type:dialogue character:<name>"

engram context update <CONTEXT_UUID> --content "<updated voice profile>"
```

**Tag convention:**

| Tag | Purpose |
|-----|---------|
| `project:<slug>` | Scopes to one film — always required |
| `type:dialogue` | Marks this as voice/dialogue data |
| `character:<name>` | Allows retrieval of one character's voice data |

**Title convention:** `<slug> dialogue <character-name>` — e.g. `heist dialogue maya`

---

## Parenthetical Rules

Use only when the reading of a line is genuinely counter-intuitive to what is on the page. Never for obvious emotions. Maximum one per 2-3 pages.
