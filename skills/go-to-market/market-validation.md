---
name: engram-market-validation
description: "Validate product ideas through interactive discovery and agent-driven market research. Stores findings as engram context/reasoning entities with go/no-go recommendation."
---

# Market Validation (Engram-Integrated)

## Overview

Validate product ideas through interactive discovery and agent-driven market research. Store all validation artifacts in Engram for persistent, queryable memory — problem-solution fit, competitive gaps, demand signals, and a clear Go/No-Go/Pivot recommendation.

## When to Use

- You have an idea but aren't sure if it's worth building
- You need evidence before committing development resources
- You want to understand competitive gaps before investing
- You're deciding between multiple ideas and need a comparison

## Pipeline Reference

This skill implements Pipeline `101-market-validation.yaml` from `prompts/ai/pipelines/`.

## The Pattern

### Phase 1: Discovery Interview (Interactive)

Ask questions one at a time, waiting for answers between each:

1. **The Problem** — What problem are you solving? Who has it? How painful is it?
2. **Existing Alternatives** — What do users currently do? What products do they use?
3. **Demand Evidence** — What signals suggest people want this? (Anecdotes, data, personal experience)
4. **Unfair Advantage** — What's your unique insight or edge?
5. **Success Criteria** — What does "validated" look like to you?

### Phase 2: Agent-Driven Research

Dispatch agents for independent analysis:

- **Agent 09 (Market Researcher)** — Estimate TAM, SAM, SOM and identify market trends
- **Agent 05 (Deconstructor)** — Break down top 3-5 competitor offerings and find gaps
- **Agent 08 (Magic 8-Ball)** — Surface risks, failure modes, and blind spots

### Phase 3: Synthesis and Verdict

Combine interview insights with research into a structured validation report.

## Engram Integration

### Step 1: Create Discovery Context Entities

After the interview, store each finding:

```bash
engram context create --title "Validation: [Idea] - Problem Definition" \
  --content "**Problem:** [Problem statement]\n**Who has it:** [Target users]\n**Pain level:** [1-10 with reasoning]\n**Current alternatives:** [What they do now]" \
  --source "market-validation" \
  --tags "validation,discovery,[idea-name]"
```

```bash
engram context create --title "Validation: [Idea] - Demand Signals" \
  --content "**Evidence:** [Anecdotes, data, experience]\n**Strength:** [Weak/Moderate/Strong]\n**Reasoning:** [Why this assessment]" \
  --source "market-validation" \
  --tags "validation,demand,[idea-name]"
```

### Step 2: Create Research Context Entities

After agents complete their analysis:

```bash
engram context create --title "Validation: [Idea] - Market Size" \
  --content "**TAM:** [Total Addressable Market]\n**SAM:** [Serviceable Available Market]\n**SOM:** [Serviceable Obtainable Market]\n**Trends:** [Key market trends]" \
  --source "market-validation" \
  --tags "validation,market-size,[idea-name]"
```

```bash
engram context create --title "Validation: [Idea] - Competitive Gaps" \
  --content "**Competitors:**\n1. [Competitor 1] - [Strengths/Weaknesses]\n2. [Competitor 2] - [Strengths/Weaknesses]\n3. [Competitor 3] - [Strengths/Weaknesses]\n\n**Gaps:** [Where incumbents fail]" \
  --source "market-validation" \
  --tags "validation,competition,[idea-name]"
```

### Step 3: Create Risk Reasoning Entities

For each identified risk:

```bash
engram reasoning create --title "Risk: [Risk Description]" \
  --task-id [TASK_ID] \
  --content "**Risk:** [Description]\n\n**Likelihood:** [Low/Medium/High]\n**Impact:** [Low/Medium/High]\n\n**Mitigation:** [How to address it]" \
  --confidence [0.0-1.0] \
  --tags "risk,validation,[idea-name]"
```

### Step 4: Create Verdict Reasoning Entity

```bash
engram reasoning create --title "Validation Verdict: [Idea Name]" \
  --task-id [TASK_ID] \
  --content "**Problem-Solution Fit:** [Score 1-10]\n\n**Competitive Gap:** [Strength of differentiation]\n\n**Demand Signal:** [Weak/Moderate/Strong]\n\n**Top Risks:**\n- [Risk 1]\n- [Risk 2]\n- [Risk 3]\n\n**Recommendation:** [Go / No-Go / Pivot]\n\n**Conditions:** [What must be true for Go]" \
  --confidence [0.0-1.0] \
  --tags "verdict,validation,[idea-name]"
```

### Step 5: Link Everything to Task

```bash
engram relationship create --source-id [TASK_ID] --source-type task \
  --target-id [CONTEXT_ID] --target-type context \
  --relationship-type references --agent [AGENT]

engram relationship create --source-id [TASK_ID] --source-type task \
  --target-id [REASONING_ID] --target-type reasoning \
  --relationship-type documents --agent [AGENT]
```

## After Validation

### If Go:
- Proceed to `engram-gtm-strategy` (Pipeline 102)
- Use the competitive gaps and demand signals as input for positioning

### If No-Go:
- All findings stored in engram for future reference
- Can revisit if market conditions change

### If Pivot:
- Create new validation task with the pivoted idea
- Link back to original validation for comparison

## Querying Validation Results

```bash
# Get all validation contexts
engram context list | grep "Validation: [Idea]"

# Get the verdict
engram reasoning get --task-id [TASK_ID] | grep "Verdict"

# Get all risks
engram reasoning list --tags "risk,validation,[idea-name]"

# Get full validation picture
engram relationship connected --entity-id [TASK_ID]
```

## Key Principles

- **Evidence over intuition** — Every claim needs backing data or reasoning
- **One question at a time** — Don't overwhelm during the interview
- **Independent agent research** — Agents work without interview bias
- **Clear verdict** — Always produce Go/No-Go/Pivot with conditions
- **Persist to Engram** — All findings stored for future reference and comparison

## Example

```
User: "I want to build a tool that helps remote teams do async standups"

[Discovery Interview]
Agent: "What problem do current standup tools fail to solve for remote teams?"
User: "They're synchronous — everyone has to be online at the same time, or the bot posts into a void and nobody reads the updates"

Agent: "What do teams currently do instead?"
User: "Slack threads, Loom videos, or just skip standups entirely"

Agent: "What evidence do you have that teams want this solved?"
User: "I've seen 50+ posts on r/remotework complaining about standup fatigue, and 3 YC startups in this space got funded last year"

[Store discovery findings]
engram context create --title "Validation: Async Standups - Problem Definition" \
  --content "**Problem:** Remote standups require synchronicity or become ignored noise\n**Who has it:** Remote teams of 5-50 people\n**Pain level:** 7/10 - teams skip standups or resent mandatory syncs\n**Current alternatives:** Slack threads, Loom, skip entirely"

[Agent Research]
Agent 09: "TAM: $2.1B team communication market. SAM: $340M async tools. SOM: $12M addressable in year 1..."

[Store research]
engram context create --title "Validation: Async Standups - Market Size" \
  --content "**TAM:** $2.1B (team communication)\n**SAM:** $340M (async-first tools)\n**SOM:** $12M (year 1 target)"

[Verdict]
engram reasoning create --title "Validation Verdict: Async Standups" \
  --content "**Problem-Solution Fit:** 8/10\n**Recommendation:** Go with conditions\n**Conditions:** Must integrate with Slack/Teams, not replace them"
```

## Related Skills

- `engram-gtm-strategy` — Next step after Go verdict (Pipeline 102)
- `engram-brainstorming` — Design the product before validating
- `engram-use-engram-memory` — Store validation for long-term reference
- `engram-audit-trail` — Track validation decisions over time
- `engram-plan-feature` — Use pipeline templates for implementation after validation
