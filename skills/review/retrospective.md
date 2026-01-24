---
name: engram-retrospective
description: "Facilitate team retrospectives using frameworks like Start/Stop/Continue, 4Ls, and Sailboat. Track action items and improvements over time."
---

# Team Retrospective (Engram-Integrated)

## Overview

Facilitate effective team retrospectives to reflect on processes, identify improvements, and foster continuous learning. Use structured frameworks like Start/Stop/Continue, 4Ls (Liked/Learned/Lacked/Longed For), Sailboat, and Mad/Sad/Glad to guide discussions. Create actionable improvement items, track progress over time, and build psychological safety for honest feedback. Store retrospective outcomes and action items in Engram to maintain team memory and measure improvement velocity.

## When to Use

Use this skill when:
- End of sprint or iteration (agile teams)
- End of project or major milestone
- After significant team change (new members, reorganization)
- Team velocity or morale declining
- Recurring problems need to be addressed
- Building new team and establishing norms
- Quarterly or monthly team health check
- After completing a challenging project

## The Pattern

### Step 1: Prepare Retrospective

Set up retrospective session for success:

```bash
engram context create \
  --title "Retrospective: [Sprint/Project Name] - [Date]" \
  --content "## Retrospective Setup

**Type:** [Sprint retrospective / Project retrospective / Quarterly review]
**Team:** [Team name]
**Timeframe covered:** [Sprint N / Date range / Project duration]
**Date:** $(date -u +%Y-%m-%dT%H:%M:%SZ)
**Facilitator:** [Name - ideally not team lead]
**Participants:** [Names]
**Duration:** [N] minutes (recommended: 45-90 minutes)
**Location:** [Conference room / Zoom link]

## Retrospective Framework

**Framework chosen:** [Start/Stop/Continue / 4Ls / Sailboat / Mad-Sad-Glad / Timeline]

**Why this framework:**
[Rationale for choosing this framework - e.g., Start/Stop/Continue for focused action items, Sailboat for exploring team dynamics]

## Retrospective Principles

**Prime Directive:**
\"Regardless of what we discover, we understand and truly believe that everyone did the best job they could, given what they knew at the time, their skills and abilities, the resources available, and the situation at hand.\"
- Norm Kerth

**Ground Rules:**
1. Psychological safety - no blame, no judgment
2. What's said here stays here (unless action items)
3. One conversation at a time
4. Assume positive intent
5. Focus on actions we can control
6. Be specific with examples
7. Everyone participates

**Expected outcomes:**
1. Team alignment on what's working and what's not
2. [N] actionable improvement items
3. Appreciation and celebration of wins
4. Strengthened team relationships

## Pre-Retrospective Data

**Metrics for context:**
- Velocity: [N] story points (last sprint: [N])
- Completed stories: [N] of [N] planned ([N%])
- Bugs found: [N] (last sprint: [N])
- Cycle time: [N] days avg (last sprint: [N])
- Team satisfaction: [N]/10 (last: [N])

**Notable events:**
- [Event 1: e.g., Production incident on Tuesday]
- [Event 2: e.g., Sprint goal changed mid-sprint]
- [Event 3: e.g., New team member onboarded]

**Previous retrospective action items:**
- [Action item 1]: [Completed / In progress / Not started]
- [Action item 2]: [Status]
- [Action item 3]: [Status]

**Action item completion rate:** [N%] (target: >80%)

## Preparation

**Materials:**
- Virtual whiteboard (Miro, Mural, FigJam) or physical board
- Sticky notes (virtual or physical)
- Timer
- Previous action items list
- Sprint/project metrics

**Pre-work (optional):**
- Send framework template to team 1 day before
- Ask team to think about experiences to share
- Share sprint metrics for context" \
  --source "retrospective-prep" \
  --tags "retrospective,preparation,[team-name],[sprint-id]"
```

### Step 2: Facilitate Retrospective Session

Guide team through structured reflection:

```bash
engram reasoning create \
  --title "Retrospective Session Notes: [Sprint/Project]" \
  --task-id [TASK_ID] \
  --content "## Retrospective Session

**Framework:** [Start/Stop/Continue]
**Duration:** [N] minutes
**Participants:** [N] present, [N] absent

## Agenda

**1. Set the Stage (5 min)**
- Welcome and appreciation
- Review ground rules and prime directive
- State goal: Improve as a team

**2. Gather Data (15 min)**
- Silent individual reflection using framework
- Each person writes thoughts on sticky notes
- No discussion yet, just capturing thoughts

**3. Generate Insights (20 min)**
- Share and cluster similar items
- Discuss themes and patterns
- Ask \"why?\" to deepen understanding

**4. Decide What to Do (15 min)**
- Dot voting on highest priority items
- Create concrete action items
- Assign owners and deadlines

**5. Close Retrospective (5 min)**
- Summarize action items
- Appreciation shoutouts
- Retrospective feedback

---

## Framework: Start/Stop/Continue

### START (What should we start doing?)

**Theme: Improve Testing Practices**
- Start: Write tests before code (TDD) [5 votes]
- Start: Pair programming on complex features [4 votes]
- Start: Architecture discussions before big changes [3 votes]

**Theme: Better Communication**
- Start: Daily async standup in Slack [6 votes]
- Start: Demo new features to stakeholders [2 votes]
- Start: Documenting decisions in ADRs [2 votes]

**Other suggestions:**
- Start: Code review checklist [1 vote]
- Start: Weekly learning sessions [1 vote]

**Top START item (voted):**
Daily async standup in Slack (6 votes)

**Discussion notes:**
- Synchronous standups hard with distributed team across timezones
- Text updates allow flexibility, async review
- Concern: Will people actually read/respond?
- Mitigation: Try for 2 weeks, retrospect again

### STOP (What should we stop doing?)

**Theme: Reduce Interruptions**
- Stop: Pinging individual developers, use team channel [7 votes]
- Stop: Meetings without agenda [5 votes]
- Stop: Context switching between multiple stories [3 votes]

**Theme: Process Overhead**
- Stop: Writing tickets for every tiny bug, batch them [4 votes]
- Stop: Mandatory attendance at all sprint meetings [2 votes]

**Other suggestions:**
- Stop: Working late to hit arbitrary deadlines [1 vote]

**Top STOP item (voted):**
Pinging individual developers, use team channel (7 votes)

**Discussion notes:**
- Individual pings break focus, cause context switching
- Team channel allows anyone to respond, spreads knowledge
- Concern: What about urgent issues?
- Agreement: Still OK to ping for P0 incidents, but use judgment

### CONTINUE (What should we keep doing?)

**Theme: Team Culture**
- Continue: Celebrating wins in team channel [8 votes]
- Continue: Blameless incident reviews [7 votes]
- Continue: Flexible work hours [6 votes]

**Theme: Technical Practices**
- Continue: Thorough code reviews [5 votes]
- Continue: Friday afternoon tech talks [4 votes]
- Continue: Automated testing in CI [4 votes]

**Other suggestions:**
- Continue: Monthly team lunch [3 votes]
- Continue: Rotating on-call schedule [2 votes]

**Top CONTINUE items:**
- Celebrating wins (8 votes)
- Blameless incident reviews (7 votes)
- Flexible work hours (6 votes)

**Discussion notes:**
- Celebrating wins boosts morale, reinforces positive behaviors
- Blameless culture encourages honesty, learning over blame
- Flexible hours critical for work-life balance

---

## Themes Identified

**1. Communication gaps causing misalignment**
- Evidence: Multiple instances where teammates had different understanding
- Contributing factors: Distributed team, async work, lack of documentation
- Impact: Rework, frustration, delays

**2. Testing quality needs improvement**
- Evidence: 3 production bugs from untested edge cases
- Contributing factors: Pressure to ship fast, testing seen as optional
- Impact: Incident response time, customer trust

**3. Interruptions hurting focus**
- Evidence: Developers report frequent context switches
- Contributing factors: Unclear communication norms, lack of designated focus time
- Impact: Reduced velocity, increased cycle time

**4. Process working well overall**
- Evidence: Many CONTINUE items, team morale high
- Strengths: Blameless culture, code review, flexibility

## Insights and Discoveries

**Insight 1: Async communication underutilized**
- Discussion: Team realized synchronous meetings not necessary for standups
- Quote: \"I dread the 9am standup because I'm not a morning person, but I'd happily write an update at 11am\"
- Experiment: Try async standups for 2 weeks

**Insight 2: Testing debt accumulating**
- Discussion: Short-term speed (skip tests) creates long-term slowdown (bugs)
- Quote: \"We're going faster but arriving later\"
- Agreement: Testing is not optional, invest upfront

**Insight 3: Individual pings creating interruption culture**
- Discussion: Normalized to ping anyone anytime, expectation of immediate response
- Impact: Developers feel they can never focus deeply
- Agreement: Shift to team channels, respect focus time

## Appreciation

**Shoutouts:**
- [Name]: \"Thank you for staying late to help debug the payment issue\"
- [Name]: \"Awesome code review feedback that caught the security bug\"
- [Name]: \"Great job onboarding our new team member\"
- [Name]: \"Your patience explaining the architecture was really helpful\"

**Team strengths celebrated:**
- Collaborative and supportive culture
- Willingness to help each other
- Strong technical skills
- Openness to feedback and change

## Action Items (Next Sprint)

Generated from START and STOP themes:

**Priority 1 (Must do):**

**Action 1: Implement Async Standup**
- Owner: [Name]
- Due: Start next sprint (Monday)
- Description: Create Slack workflow for daily standup questions, post by 11am local time
- Success criteria: 80% team participation daily
- **Tracking:** [Ticket ID]

**Action 2: Establish Communication Norms**
- Owner: [Name]
- Due: This week
- Description: Document when to use team channel vs individual ping, post in wiki
- Success criteria: Team agrees on norms, posted visibly
- **Tracking:** [Ticket ID]

**Priority 2 (Should do):**

**Action 3: Add Code Review Checklist**
- Owner: [Name]
- Due: Next week
- Description: Create checklist in PR template including test coverage requirement
- Success criteria: Checklist in place, team trained
- **Tracking:** [Ticket ID]

**Action 4: Schedule Architecture Discussions**
- Owner: [Name]
- Due: Next sprint
- Description: 1-hour weekly slot for discussing upcoming architectural changes
- Success criteria: Calendar invite sent, first meeting held
- **Tracking:** [Ticket ID]

**Priority 3 (Nice to have):**

**Action 5: Experiment with Pairing**
- Owner: [Name]
- Due: Next sprint
- Description: Try pair programming on one complex feature
- Success criteria: At least one pairing session, retrospective on experience
- **Tracking:** [Ticket ID]

## Retrospective Feedback

**What went well about THIS retrospective:**
- Good participation, everyone contributed
- Framework worked well for surfacing issues
- Felt psychologically safe
- Concrete action items created

**What to improve for NEXT retrospective:**
- Could use more time (felt rushed)
- Try different framework for variety
- Ensure remote participants fully included

## Commitments

**Team commitments:**
1. Complete Priority 1 action items before next retrospective
2. Review action item progress in next sprint's standup
3. Bring action item status to next retrospective

**Facilitator commitments:**
1. Share retrospective notes within 24 hours
2. Create tracking tickets for action items
3. Send reminder about action items mid-sprint
4. Follow up on incomplete action items from last retrospective" \
  --confidence 0.85 \
  --tags "retrospective,session-notes,start-stop-continue,[team-name],[sprint-id]"
```

### Step 3: Alternative Framework - 4Ls

Provide alternative retrospective framework:

```bash
engram reasoning create \
  --title "Retrospective Framework: 4Ls (Liked, Learned, Lacked, Longed For)" \
  --task-id [TASK_ID] \
  --content "## 4Ls Retrospective Framework

**When to use:**
- Focusing on learning and growth
- Team wants to explore emotions and desires
- After completing learning-heavy project
- When Start/Stop/Continue feels stale

**Duration:** 60 minutes

## The 4Ls

### LIKED (What went well?)

Focus on positives, celebrations, and successes.

**Examples:**
- \"I liked how we came together during the incident\"
- \"I liked the new CI pipeline, it catches issues early\"
- \"I liked working with [Name] on the refactoring\"
- \"I liked that we shipped on time despite challenges\"

**Facilitator questions:**
- What are you proud of?
- What made you smile this sprint?
- What should we celebrate?

### LEARNED (What did we learn?)

Focus on new knowledge, skills, insights, and discoveries.

**Examples:**
- \"I learned how to use Kubernetes properly\"
- \"I learned that our database can't handle 10K concurrent connections\"
- \"I learned that early testing catches bugs cheaper\"
- \"I learned that async communication works better for our distributed team\"

**Facilitator questions:**
- What surprised you?
- What will you do differently next time because of what you learned?
- What knowledge can we share with other teams?

### LACKED (What was missing?)

Focus on gaps, missing resources, or absent elements.

**Examples:**
- \"We lacked clear requirements at the start\"
- \"We lacked test coverage on the legacy code\"
- \"We lacked documentation for the deployment process\"
- \"We lacked time for refactoring technical debt\"

**Facilitator questions:**
- What would have made your work easier?
- What tools, resources, or support did we need?
- What information was missing?

### LONGED FOR (What did we desire?)

Focus on aspirations, wishes, and future improvements.

**Examples:**
- \"I longed for more pair programming opportunities\"
- \"I longed for better observability into production\"
- \"I longed for dedicated focus time without meetings\"
- \"I longed for clearer priorities from leadership\"

**Facilitator questions:**
- What would make the next sprint better?
- What's your ideal working environment?
- What changes would you love to see?

## Example 4Ls Retrospective

**LIKED:**
- Collaborative debugging session (6 people mentioned)
- New documentation we created
- Friday tech talks continuing
- Flexible work schedule
- Team's supportive culture

**LEARNED:**
- Redis can't handle 1M keys efficiently (need to shard)
- Early code review catches bugs 80% cheaper
- Customer feedback sessions are valuable
- Terraform state locking prevents conflicts
- Importance of incremental releases over big bang

**LACKED:**
- Clear acceptance criteria on 3 stories
- Monitoring for new microservice
- Test environment matching production
- Knowledge of legacy authentication code
- Time to address technical debt

**LONGED FOR:**
- Dedicated focus time (2-hour blocks)
- Better tooling for debugging distributed systems
- More frequent 1:1s with manager
- Cross-team collaboration on shared services
- Clearer product roadmap

## Converting to Action Items

From the 4Ls, identify action items:

**From LACKED:**
- Action: Write acceptance criteria template
- Action: Add monitoring to microservice
- Action: Upgrade test environment

**From LONGED FOR:**
- Action: Block calendar for focus time (10am-12pm daily)
- Action: Evaluate distributed tracing tools (Jaeger, Zipkin)
- Action: Schedule cross-team architecture sync

**From LEARNED:**
- Action: Document Redis sharding strategy
- Action: Share customer feedback insights with product team
- Action: Write wiki page on Terraform best practices

**From LIKED:**
- Continue: Friday tech talks (already working)
- Continue: Collaborative debugging (reinforce this behavior)

## Template for Team

**4Ls Retrospective Template**

Share with team 1 day before retrospective:

\`\`\`
## 4Ls Retrospective Prep

Please reflect on the past sprint/project:

### LIKED
What went well? What are you proud of?
- [Your thoughts]

### LEARNED
What new knowledge, skills, or insights did you gain?
- [Your thoughts]

### LACKED
What was missing? What would have helped?
- [Your thoughts]

### LONGED FOR
What do you wish we had? What would improve things?
- [Your thoughts]
\`\`\`" \
  --confidence 0.80 \
  --tags "retrospective,framework,4ls,[team-name]"
```

### Step 4: Alternative Framework - Sailboat

Provide metaphorical retrospective framework:

```bash
engram reasoning create \
  --title "Retrospective Framework: Sailboat (Wind, Anchor, Rocks, Island)" \
  --task-id [TASK_ID] \
  --content "## Sailboat Retrospective Framework

**Metaphor:** Team is a sailboat traveling toward an island (goal).

**When to use:**
- Visual/creative team
- Want to explore team dynamics
- Identifying risks and impediments
- Making progress toward a goal

**Duration:** 60 minutes

## Sailboat Elements

### ISLAND (Our Goal)

The destination, what we're working toward.

**Questions:**
- What's our sprint/project goal?
- What does success look like?
- Why are we doing this work?

**Example:**
\"Ship payment processing v2 with 99.9% uptime\"

### WIND (What's Helping Us)

Forces propelling us toward the island.

**Examples:**
- Strong collaboration between developers
- Automated testing catching bugs early
- Clear product vision from leadership
- Team member's expertise in payments
- Supportive stakeholders

**Facilitator questions:**
- What's going well?
- What's accelerating our progress?
- What strengths can we leverage?

### ANCHOR (What's Holding Us Back)

Weight dragging us down, slowing progress.

**Examples:**
- Technical debt in legacy code
- Unclear requirements changing frequently
- Too many meetings interrupting flow
- Slow CI/CD pipeline (30 min builds)
- Insufficient test coverage

**Facilitator questions:**
- What's slowing us down?
- What frustrations do you have?
- What would you remove if you could?

### ROCKS (Risks/Obstacles)

Dangers ahead that could sink the boat.

**Examples:**
- Key team member leaving next month
- Third-party payment API has reliability issues
- Regulatory compliance deadline approaching
- Security vulnerabilities in dependencies
- Scaling concerns as user base grows

**Facilitator questions:**
- What could go wrong?
- What keeps you up at night?
- What are our biggest risks?

## Example Sailboat Retrospective

**ISLAND (Goal):**
Launch new dashboard with 50ms p95 latency by end of Q2

**WIND (Helping):**
- New database indexes improved query speed 10x [6 votes]
- Team has strong frontend skills [5 votes]
- Stakeholder excited and supportive [4 votes]
- Automated performance tests in CI [3 votes]
- Pair programming on complex features [2 votes]

**ANCHOR (Holding back):**
- Legacy API slow, can't change it easily [8 votes]
- Frequent context switching between projects [7 votes]
- Flaky tests failing randomly [5 votes]
- Code review taking 2+ days [4 votes]
- Insufficient documentation on data model [3 votes]

**ROCKS (Risks):**
- Database might not scale to 10K concurrent users [7 votes]
- Third-party charting library has bugs, vendor slow to respond [5 votes]
- Tight deadline, might cut corners on quality [4 votes]
- New team member still ramping up [2 votes]
- Marketing launch might bring unexpected traffic spike [2 votes]

## Discussion and Insights

**Insight 1: Legacy API is biggest impediment**
- Anchor item with most votes
- Discussion: Can't rewrite it, need to work around
- Ideas: Cache responses, minimize calls, async where possible
- Action: Spike on caching strategy

**Insight 2: Scaling risk needs attention NOW**
- Rock item with most votes
- Discussion: Haven't load tested, don't know limits
- Risk: Discover problem too late
- Action: Load test this sprint, identify bottlenecks

**Insight 3: Context switching killing productivity**
- Anchor item with many votes
- Discussion: Working on dashboard + supporting production + other projects
- Impact: Nothing gets deep focus, everything takes longer
- Action: Dedicate 2 people full-time to dashboard, others handle other work

## Converting to Action Items

**Address ANCHORS:**
- Action: Implement API response caching (reduce legacy API calls)
- Action: Fix flaky tests (allocate 1 day)
- Action: Set code review SLA (24 hours, escalate if breached)
- Action: Dedicate focus time, minimize context switching

**Mitigate ROCKS:**
- Action: Load test with 10K concurrent users this sprint
- Action: Evaluate alternative charting libraries (backup plan)
- Action: Define \"done\" criteria (prevent quality shortcuts)
- Action: Pair new team member with mentor

**Amplify WIND:**
- Continue: Performance testing in CI
- Continue: Pair programming
- Celebrate: 10x query speedup with indexes

## Template for Team

**Sailboat Retrospective Board**

\`\`\`
                 Island
              (Our Goal)
                  🏝️


        Wind             Rocks
    (Helping Us)      (Risks)
         ⛵             🪨🪨
        /\\\\
       /  \\\\
      /    \\\\
     /______\\\\
        ||
        ||
        ⚓
      Anchor
  (Holding Us Back)
\`\`\`" \
  --confidence 0.80 \
  --tags "retrospective,framework,sailboat,[team-name]"
```

### Step 5: Track Action Items Over Time

Monitor retrospective action item completion:

```bash
engram reasoning create \
  --title "Retrospective Action Item Tracking: [Team] [Quarter]" \
  --task-id [TASK_ID] \
  --content "## Action Item Tracking

**Team:** [Team name]
**Period:** [Q1 2026]
**Retrospectives held:** [N]
**Total action items created:** [N]
**Completion rate:** [N%]

## Action Items by Retrospective

### Retro 1: Sprint 23 (Jan 15, 2026)

**Action 1: Implement async standup**
- Owner: [Name]
- Due: Jan 17
- Status: ✅ Completed (Jan 17)
- Outcome: 85% daily participation, team prefers async format
- Impact: Eliminated 5 hours/week of synchronous meeting time

**Action 2: Create code review checklist**
- Owner: [Name]
- Due: Jan 22
- Status: ✅ Completed (Jan 20)
- Outcome: Checklist in PR template, training conducted
- Impact: Code reviews catching more issues (20% increase)

**Action 3: Document communication norms**
- Owner: [Name]
- Due: Jan 18
- Status: ✅ Completed (Jan 19)
- Outcome: Norms documented in wiki, team channel pings reduced 60%
- Impact: Fewer interruptions, improved focus

**Completion rate:** 3/3 (100%)

### Retro 2: Sprint 24 (Jan 29, 2026)

**Action 4: Load test dashboard**
- Owner: [Name]
- Due: Feb 2
- Status: ✅ Completed (Feb 1)
- Outcome: Identified bottleneck in query, fixed before production
- Impact: Prevented potential incident

**Action 5: Fix flaky tests**
- Owner: [Name]
- Due: Feb 5
- Status: ⏳ In Progress (50% complete)
- Blocker: More flaky tests than expected, need 2 more days
- Updated ETA: Feb 7

**Action 6: Implement API response caching**
- Owner: [Name]
- Due: Feb 12
- Status: ❌ Not Started
- Reason: Deprioritized for P0 bug fix
- Plan: Will start next sprint

**Action 7: Evaluate alternative charting libraries**
- Owner: [Name]
- Due: Feb 12
- Status: ✅ Completed (Feb 10)
- Outcome: Found better library, migrating next sprint
- Impact: Will eliminate vendor bug issues

**Completion rate:** 2/4 complete, 1/4 in progress, 1/4 not started (50% on-time)

### Retro 3: Sprint 25 (Feb 12, 2026)

**Action 8: [Action item]**
- Owner: [Name]
- Due: [Date]
- Status: [Not started / In progress / Completed / Cancelled]

[Continue for all action items...]

## Completion Metrics

**Overall:**
- Created: [N] action items
- Completed on time: [N] ([N%])
- Completed late: [N] ([N%])
- In progress: [N] ([N%])
- Not started: [N] ([N%])
- Cancelled: [N] ([N%])

**By Priority:**
- P1 completion rate: [N%]
- P2 completion rate: [N%]
- P3 completion rate: [N%]

**Average time to complete:** [N] days

## Patterns and Insights

**Success patterns:**
- Action items with single clear owner complete 90% of the time
- Action items <2 days effort complete 95% of the time
- Action items tied to pain points get prioritized

**Failure patterns:**
- Action items without owner rarely complete (20% completion)
- Vague action items (\"improve testing\") don't complete
- P3 low-priority items often deprioritized for urgent work

**Recurring themes:**
- Testing quality (3 retrospectives)
- Communication norms (2 retrospectives)
- Technical debt (ongoing)

## Recommendations

**Process improvements:**
1. Limit action items to 3-5 per retrospective (current avg: 4.5, good)
2. Ensure every action item has clear owner and deadline
3. Review previous action items at start of each retrospective
4. Make action items specific and measurable

**Team focus areas:**
Based on recurring themes, dedicate next quarter to:
1. Improve test coverage to 80% (currently 65%)
2. Reduce technical debt (allocate 20% sprint capacity)
3. Maintain communication improvements (working well)

## Action Item Velocity

**Sprint 23:** 3 created, 3 completed (100%)
**Sprint 24:** 4 created, 2 completed (50%)
**Sprint 25:** 5 created, 4 completed (80%)

**Trend:** [Improving / Stable / Declining]

**Recommendation:** [Based on trend]" \
  --confidence 0.80 \
  --tags "retrospective,tracking,action-items,[team-name]"
```

### Step 6: Link Retrospective Entities to Task

```bash
# Link retrospective analysis
engram relationship create \
  --source-id [TASK_ID] --source-type task \
  --target-id [PREP_ID] --target-type context \
  --relationship-type references \
  --agent [AGENT]

engram relationship create \
  --source-id [TASK_ID] --source-type task \
  --target-id [SESSION_ID] --target-type reasoning \
  --relationship-type documents \
  --agent [AGENT]

engram relationship create \
  --source-id [TASK_ID] --source-type task \
  --target-id [TRACKING_ID] --target-type reasoning \
  --relationship-type documents \
  --agent [AGENT]
```

## Example

Team completes sprint 23, time for retrospective to reflect and improve.

### Step 1: Prepare Retrospective

Agent sets up retrospective:

"Preparing Sprint 23 retrospective for 8-person team. Checking previous action items:
- Async standup: ✅ Completed
- Code review checklist: ✅ Completed  
- Communication norms: ✅ Completed

100% completion rate from last retro. Choosing Start/Stop/Continue framework. Prepared virtual board, sent calendar invite, shared metrics (velocity up 15%, 2 production bugs this sprint)."

### Step 2: Facilitate Session

Agent facilitates 60-minute retrospective:

"Retrospective completed with full team participation.

START:
- Daily async standups (6 votes) - Experiment approved
- Architecture discussions before changes (3 votes)

STOP:
- Individual pings, use team channel (7 votes) - Team agreed
- Meetings without agenda (5 votes)

CONTINUE:
- Celebrating wins (8 votes) - Team loves this
- Blameless incident reviews (7 votes)
- Flexible work hours (6 votes)

Key insight: Communication gaps causing misalignment. Team prefers async over synchronous.

Created 5 action items, all with owners and deadlines. Team committed to Priority 1 items."

### Step 3: Track Progress

Agent tracks action items over next 2 weeks:

"Action item progress check:
- Async standup: ✅ Complete (85% participation)
- Communication norms: ✅ Complete (wiki page created)
- Code review checklist: ✅ Complete (in PR template)
- Architecture discussions: ⏳ In progress (first meeting scheduled)
- Pairing experiment: ⏳ In progress (pairing session tomorrow)

4/5 on track for completion. 80% completion rate expected.

Retrospective action items stored in Engram. Ready for next retro to review progress and identify new improvements."

## Querying Retrospectives

```bash
# Get retrospective preparation
engram context list | grep "Retrospective:" | grep "Preparation"

# Get retrospective session notes
engram reasoning list | grep "Retrospective Session Notes:"

# Get retrospective frameworks
engram reasoning list | grep "Retrospective Framework:"

# Get action item tracking
engram reasoning list | grep "Retrospective Action Item Tracking:"

# Get all retrospectives for a team
engram relationship connected --entity-id [TASK_ID] | grep -i "retrospective"

# Find retrospectives by framework
engram reasoning list | grep "4Ls"
engram reasoning list | grep "Sailboat"
```

## Related Skills

This skill integrates with:
- `engram-post-mortem` - Similar reflection techniques for incidents
- `engram-brainstorming` - Retrospectives generate improvement ideas
- `engram-risk-assessment` - Retrospectives surface risks
- `engram-assumption-validation` - Retrospectives test team assumptions
- `engram-writing-plans` - Retrospective insights inform future planning
