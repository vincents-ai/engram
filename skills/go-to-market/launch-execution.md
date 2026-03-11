---
name: engram-launch-execution
description: "Build tactical launch plans with content calendar, distribution, outreach, and launch day runbook as engram task hierarchy."
---

# Launch Execution Plan (Engram-Integrated)

## Overview

Build the tactical, week-by-week launch plan. Store all launch artifacts as engram task hierarchy — content calendar, distribution plan, outreach templates, and launch day runbook.

## When to Use

- GTM strategy is defined (Pipeline 102 complete)
- Product is ready or nearly ready for launch
- You need a concrete, actionable plan to get first customers
- You're preparing for a specific launch event (Product Hunt, beta, public launch)

## Pipeline Reference

This skill implements Pipeline `103-launch-execution.yaml` from `prompts/ai/pipelines/`.

## Prerequisites

- Completed `engram-gtm-strategy` (Pipeline 102)
- Or: you have ICP, positioning, and pricing defined

## The Pattern

### Phase 1: Launch Scoping (Interactive)

Ask questions one at a time, waiting for answers between each:

1. **Launch Type** — Soft launch, beta, public launch, Product Hunt, etc.
2. **Timeline** — Hard date or flexible?
3. **Assets** — Demo, landing page, email list, social following
4. **Capacity** — Team size and availability for launch week
5. **Budget** — Paid promotion budget, if any

### Phase 2: Plan Formulation (Agent-Driven)

Dispatch agents for each plan component:

- **Agent 09 (Market Researcher)** — Best channels and timing for target audience
- **Agent 04 (Monetiser)** — Launch offers (early bird, free trials, limited deals)
- **Agent 01 (The One)** — Launch narrative and key messaging angles
- **Agent 41 (Technical Writer)** — Templates for announcement posts, emails, social copy

### Phase 3: Execution Plan

Synthesize into a week-by-week launch plan with day-of runbook.

## Engram Integration

### Step 1: Create Launch Scope Context

```bash
engram context create --title "Launch: [Product] - Launch Scope" \
  --content "**Launch Type:** [Soft/Beta/Public/Product Hunt]\n**Target Date:** [Date]\n**Available Assets:**\n- [Asset 1]\n- [Asset 2]\n**Team Capacity:** [People and hours]\n**Budget:** [Amount for paid promotion]" \
  --source "launch-execution" \
  --tags "launch,scope,[product-name]"
```

### Step 2: Create Parent Task for Launch Plan

```bash
PARENT_TASK=$(engram task create \
  --title "Launch: [Product Name]" \
  --description "**Pipeline:** 103-launch-execution\n**Goal:** Execute launch to acquire first [N] customers\n**Launch Type:** [Type]\n**Target Date:** [Date]" \
  --priority high \
  --agent default \
  --json | jq -r '.id')
```

### Step 3: Create Pre-Launch Subtasks

```bash
engram task create \
  --title "Launch: Pre-Launch Checklist" \
  --description "**Tasks:**\n- [ ] Landing page live\n- [ ] Demo/video ready\n- [ ] Email sequence drafted\n- [ ] Social posts scheduled\n- [ ] Analytics tracking verified\n- [ ] Beta feedback incorporated" \
  --parent $PARENT_TASK \
  --priority high \
  --agent default
```

### Step 4: Create Week-by-Week Subtasks

```bash
# T-4 weeks: Foundation
engram task create \
  --title "Launch: T-4 Weeks - Foundation" \
  --description "**Focus:** Build assets and seed audience\n\n**Tasks:**\n- Finalise landing page copy\n- Create demo video / GIF\n- Set up email capture / waitlist\n- Identify 20 influencers / communities for outreach\n- Draft announcement blog post" \
  --parent $PARENT_TASK \
  --priority high \
  --agent default

# T-3 weeks: Seed
engram task create \
  --title "Launch: T-3 Weeks - Seed" \
  --description "**Focus:** Build anticipation and gather early feedback\n\n**Tasks:**\n- Send beta invites to waitlist\n- Post in 3-5 relevant communities\n- Reach out to 10 influencers with personalised DMs\n- Publish teaser content (thread, short video)\n- Collect and address beta feedback" \
  --parent $PARENT_TASK \
  --priority high \
  --agent default

# T-2 weeks: Amplify
engram task create \
  --title "Launch: T-2 Weeks - Amplify" \
  --description "**Focus:** Build momentum and social proof\n\n**Tasks:**\n- Publish case study / beta user testimonial\n- Schedule all launch day social posts\n- Confirm influencer commitments\n- Prepare Product Hunt listing (if applicable)\n- Test all conversion flows end-to-end" \
  --parent $PARENT_TASK \
  --priority high \
  --agent default

# T-1 week: Final Prep
engram task create \
  --title "Launch: T-1 Week - Final Prep" \
  --description "**Focus:** Lock everything down\n\n**Tasks:**\n- Final landing page QA\n- Email sequence dry run\n- Prepare support / FAQ responses\n- Brief team on launch day runbook\n- Staging environment final test" \
  --parent $PARENT_TASK \
  --priority high \
  --agent default

# Launch Day: Runbook
engram task create \
  --title "Launch: Launch Day Runbook" \
  --description "**Hour-by-hour plan:**\n\n**06:00** - Final staging check\n**07:00** - Post announcement (blog, social, communities)\n**07:30** - Send launch email to full list\n**08:00** - Monitor analytics and respond to first comments\n**09:00** - Post in secondary channels (HN, Reddit, IndieHackers)\n**10:00** - DM influencers to amplify\n**12:00** - Midday check: metrics review, adjust messaging if needed\n**14:00** - Share early traction milestones publicly\n**16:00** - Respond to all comments and DMs\n**18:00** - End-of-day metrics report\n**20:00** - Plan T+1 follow-up actions" \
  --parent $PARENT_TASK \
  --priority high \
  --agent default

# Post-launch
engram task create \
  --title "Launch: T+1 to T+14 - Post-Launch" \
  --description "**Focus:** Sustain momentum and iterate\n\n**T+1:** Share results recap, follow up with signups, address feedback\n**T+3:** Publish 'what we learned' post, retargeting ads, second email\n**T+7:** Weekly metrics review, iterate conversion funnel\n**T+14:** Full retrospective, update GTM strategy, plan next growth phase" \
  --parent $PARENT_TASK \
  --priority high \
  --agent default
```

### Step 5: Create Content and Outreach Context Entities

```bash
engram context create --title "Launch: [Product] - Content Calendar" \
  --content "**Blog Posts:**\n- T-3w: [Teaser post]\n- T-1w: [Case study]\n- Launch: [Announcement]\n- T+1w: [Lessons learned]\n\n**Social Posts:**\n- T-2w: [Thread / video]\n- Launch: [Announcement posts]\n- T+1d: [Results recap]\n\n**Email Sequence:**\n- T-1w: [Beta invite]\n- Launch: [Launch announcement]\n- T+3d: [Non-converter follow-up]" \
  --source "launch-execution" \
  --tags "launch,content,[product-name]"
```

```bash
engram context create --title "Launch: [Product] - Outreach Templates" \
  --content "**Influencer DM:**\nHey [Name], I saw your post about [topic]. We built [product] that [value prop]. Would love to get your take — happy to give you early access. [Link]\n\n**Press Pitch:**\nSubject: [Product] launches [key differentiator] for [audience]\n\n[Opening hook]\n[Problem statement]\n[Solution description]\n[Traction / credibility]\n[CTA]\n\n**Community Post:**\nHey [community], we just launched [product] — [one-line description]. [Why relevant to this community]. Would love your feedback: [link]" \
  --source "launch-execution" \
  --tags "launch,outreach,[product-name]"
```

### Step 6: Create Launch Offer Reasoning Entity

```bash
engram reasoning create --title "Launch: [Product] - Launch Offers" \
  --task-id [TASK_ID] \
  --content "**Offer Options Considered:**\n\n1. **Early bird discount** (20% off for first 100 users)\n2. **Extended free trial** (30 days instead of 14)\n3. **Founder plan** (lifetime deal for early adopters)\n\n**Selected:** [Choice]\n\n**Reasoning:** [Why this offer maximises launch impact]" \
  --confidence [0.0-1.0] \
  --tags "launch,offers,[product-name]"
```

### Step 7: Link Everything

```bash
# Link subtasks to parent
engram relationship create --source-id $PARENT_TASK --target-id [SUBTASK_ID] --contains

# Link contexts to parent
engram relationship create --source-id $PARENT_TASK --source-type task \
  --target-id [CONTENT_CONTEXT_ID] --target-type context \
  --relationship-type references --agent [AGENT]
```

## After Launch

### Execution:
- Work through subtasks week by week
- Mark tasks complete as you go
- Create new tasks for pivots or adjustments

### Tracking:
- Update metrics context entity weekly
- Create reasoning entities for key decisions during launch

## Querying Launch Plan

```bash
# Get full launch plan
engram task show [PARENT_TASK_ID]

# Get all subtasks (week-by-week)
engram task list --parent [PARENT_TASK_ID]

# Get content calendar
engram context list | grep "Launch:.*Content"

# Get outreach templates
engram context list | grep "Launch:.*Outreach"
```

## Key Principles

- **Actionable over theoretical** — Every task is something you can do today
- **Time-boxed** — T-4 weeks to T+2 weeks, not open-ended
- **Launch day is a runbook** — Hour-by-hour, no ambiguity
- **Post-launch matters** — Most value comes after launch day
- **Persist to Engram** — Track what actually happened vs the plan

## Example

```
User: "We're launching our async standup tool on Product Hunt in 3 weeks"

[Store launch scope]
engram context create --title "Launch: AsyncStandups - Launch Scope" \
  --content "**Launch Type:** Product Hunt\n**Target Date:** April 19, 2026\n**Available Assets:** Landing page, demo video, 500 email waitlist\n**Team Capacity:** 2 people, 20 hrs/week\n**Budget:** £500 for paid promotion"

[Create task hierarchy]
PARENT=$(engram task create --title "Launch: AsyncStandups" ...)

engram task create --title "Launch: T-3 Weeks - Seed" \
  --description "Send beta invites, post in r/remotework and IndieHackers..." \
  --parent $PARENT

engram task create --title "Launch: Launch Day Runbook" \
  --description "06:00 Product Hunt submission, 07:00 social posts..." \
  --parent $PARENT

Agent: "Launch plan created. 6 subtasks from T-3 weeks to T+2 weeks. Ready to execute?"
```

## Related Skills

- `engram-gtm-strategy` — Prerequisite (Pipeline 102)
- `engram-market-validation` — Before strategy (Pipeline 101)
- `engram-writing-plans` — Convert launch tasks to detailed implementation plans
- `engram-audit-trail` — Track launch execution and decisions
- `engram-use-engram-memory` — Store launch data for future launches
