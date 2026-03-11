---
name: engram-gtm-strategy
description: "Define ICP, positioning, pricing, channels, and success metrics for validated products. Stores strategy as engram context/reasoning entities."
---

# Go-to-Market Strategy (Engram-Integrated)

## Overview

Define the complete GTM strategy for a validated product idea. Store all strategy components in Engram — ideal customer profiles, positioning, pricing model, channel strategy, and success metrics.

## When to Use

- Market validation returned "Go" verdict
- You have a product ready or nearly ready to bring to market
- You need to define who to sell to, how to position, and where to find customers
- You're planning pricing and distribution channels

## Pipeline Reference

This skill implements Pipeline `102-gtm-strategy.yaml` from `prompts/ai/pipelines/`.

## Prerequisites

- Completed `engram-market-validation` (Pipeline 101) with a Go verdict
- Or: you have sufficient market understanding to skip validation

## The Pattern

### Phase 1: Positioning Interview (Interactive)

Ask questions one at a time, waiting for answers between each:

1. **Ideal Customer** — Demographics, behaviours, pain triggers, job titles
2. **Value Proposition** — What makes this different from alternatives?
3. **Pricing Expectations** — Free, freemium, subscription, one-time?
4. **Customer Locations** — Where do they congregate online/offline?
5. **Acquisition Budget** — What can you spend per customer?

### Phase 2: Strategy Formulation (Agent-Driven)

Dispatch agents for each strategy component:

- **Agent 09 (Market Researcher)** — Refine ICP with market data and segment sizing
- **Agent 04 (Monetiser)** — Design pricing tiers, revenue model, conversion funnels
- **Agent 01 (The One)** — Craft positioning statement and channel strategy
- **Agent 05 (Deconstructor)** — Map competitor positioning to find whitespace

### Phase 3: Strategy Document

Synthesize into a comprehensive GTM strategy.

## Engram Integration

### Step 1: Create ICP Context Entity

```bash
engram context create --title "GTM: [Product] - Ideal Customer Profile" \
  --content "**Primary ICP:**\n- **Role:** [Job title/function]\n- **Company size:** [Range]\n- **Industry:** [Sectors]\n- **Pain triggers:** [What makes them look for a solution]\n- **Budget authority:** [Can they buy? How much?]\n\n**Persona 1:** [Name] - [Description]\n**Persona 2:** [Name] - [Description]\n**Persona 3:** [Name] - [Description]" \
  --source "gtm-strategy" \
  --tags "gtm,icp,[product-name]"
```

### Step 2: Create Positioning Context Entity

```bash
engram context create --title "GTM: [Product] - Positioning" \
  --content "**Positioning Statement:**\nFor [target customer] who [need], [product] is [category] that [key benefit]. Unlike [competitors], we [differentiator].\n\n**Messaging Hierarchy:**\n1. **Primary:** [Core message]\n2. **Supporting:** [Proof point 1]\n3. **Supporting:** [Proof point 2]\n\n**Tagline:** [One-liner]" \
  --source "gtm-strategy" \
  --tags "gtm,positioning,[product-name]"
```

### Step 3: Create Pricing Context Entity

```bash
engram context create --title "GTM: [Product] - Pricing Model" \
  --content "**Model:** [Freemium/Subscription/One-time/Usage-based]\n\n**Tiers:**\n| Tier | Price | Features |\n|------|-------|----------|\n| Free | £0 | [Features] |\n| Pro | £X/mo | [Features] |\n| Team | £X/mo | [Features] |\n\n**Rationale:** [Why this model and pricing]\n\n**Conversion Funnel:**\n1. [Awareness stage]\n2. [Trial/signup stage]\n3. [Conversion trigger]\n4. [Expansion trigger]" \
  --source "gtm-strategy" \
  --tags "gtm,pricing,[product-name]"
```

### Step 4: Create Channel Strategy Context Entity

```bash
engram context create --title "GTM: [Product] - Channel Strategy" \
  --content "**Primary Channels:**\n1. [Channel 1] - [Why] - [Expected CAC]\n2. [Channel 2] - [Why] - [Expected CAC]\n\n**Secondary Channels:**\n1. [Channel] - [Why]\n2. [Channel] - [Why]\n\n**Channels to Avoid:**\n- [Channel] - [Why not]" \
  --source "gtm-strategy" \
  --tags "gtm,channels,[product-name]"
```

### Step 5: Create Metrics Context Entity

```bash
engram context create --title "GTM: [Product] - Success Metrics" \
  --content "**North Star Metric:** [Metric] - [Why this one]\n\n**KPIs:**\n- **Acquisition:** [Metric] - [Target]\n- **Activation:** [Metric] - [Target]\n- **Retention:** [Metric] - [Target]\n- **Revenue:** [Metric] - [Target]\n- **Referral:** [Metric] - [Target]\n\n**90-Day Milestones:**\n- Week 1-4: [Milestone]\n- Week 5-8: [Milestone]\n- Week 9-12: [Milestone]" \
  --source "gtm-strategy" \
  --tags "gtm,metrics,[product-name]"
```

### Step 6: Create Trade-off Reasoning Entities

For pricing and channel decisions:

```bash
engram reasoning create --title "Trade-off: [Product] - Pricing Model Selection" \
  --task-id [TASK_ID] \
  --content "**Options Considered:**\n\n1. **Freemium:** [Pros/Cons]\n2. **Free trial:** [Pros/Cons]\n3. **Paid only:** [Pros/Cons]\n\n**Selected:** [Choice]\n\n**Reasoning:** [Why this model wins]" \
  --confidence [0.0-1.0] \
  --tags "trade-off,gtm,pricing,[product-name]"
```

### Step 7: Link Everything to Task

```bash
engram relationship create --source-id [TASK_ID] --source-type task \
  --target-id [CONTEXT_ID] --target-type context \
  --relationship-type references --agent [AGENT]

engram relationship create --source-id [TASK_ID] --source-type task \
  --target-id [REASONING_ID] --target-type reasoning \
  --relationship-type documents --agent [AGENT]
```

## After Strategy

### Next Step:
- Proceed to `engram-launch-execution` (Pipeline 103)
- Use ICP, channels, and pricing as input for launch plan

### Feeds Into:
- `engram-plan-feature` — Feature prioritisation based on ICP needs
- `prompts/ai/pipelines/09-monetisation-implementation.yaml` — Technical implementation of pricing model

## Querying Strategy

```bash
# Get all GTM contexts
engram context list | grep "GTM: [Product]"

# Get ICP
engram context list | grep "GTM:.*ICP"

# Get pricing model
engram context list | grep "GTM:.*Pricing"

# Get full strategy
engram relationship connected --entity-id [TASK_ID]
```

## Key Principles

- **ICP before channels** — Know who before figuring out where
- **Position before pricing** — Value proposition drives price, not cost
- **Test channels cheaply** — Validate channel assumptions before scaling spend
- **One North Star** — Avoid metric overload; pick one guiding metric
- **Persist to Engram** — Strategy evolves; store it for iteration

## Example

```
User: "We validated async standups — it's a Go. Now I need a GTM strategy."

[Positioning Interview]
Agent: "Who is the ideal customer for this?"
User: "Engineering managers at remote-first companies, 10-100 people"

Agent: "What's your unique value proposition vs Slack threads and Loom?"
User: "We structure the updates, surface blockers automatically, and nobody has to watch a video"

[Store ICP]
engram context create --title "GTM: AsyncStandups - Ideal Customer Profile" \
  --content "**Primary ICP:**\n- **Role:** Engineering Manager\n- **Company size:** 10-100 (remote-first)\n- **Pain triggers:** Standup fatigue, missed blockers\n**Persona 1:** Sarah - EM at Series A startup, manages 8 engineers"

[Agent Research]
Agent 04: "Recommend freemium: free for teams under 5, £8/user/mo for Pro, £15/user/mo for Team..."

[Store pricing]
engram context create --title "GTM: AsyncStandups - Pricing Model" \
  --content "**Model:** Freemium\n| Free | £0 | Up to 5 users |\n| Pro | £8/mo | Unlimited users, integrations |\n| Team | £15/mo | Analytics, custom workflows |"

Agent: "GTM strategy complete. 5 context entities stored. Ready for launch execution?"
```

## Related Skills

- `engram-market-validation` — Prerequisite (Pipeline 101)
- `engram-launch-execution` — Next step after strategy (Pipeline 103)
- `engram-brainstorming` — Design product features aligned with ICP
- `engram-plan-feature` — Plan feature implementation using pipeline templates
- `engram-use-engram-memory` — Store strategy for iteration
