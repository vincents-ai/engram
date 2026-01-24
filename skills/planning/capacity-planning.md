---
name: engram-capacity-planning
description: "Estimate effort, identify bottlenecks, balance workload, track velocity trends, and forecast completion dates."
---

# Capacity Planning (Engram-Integrated)

## Overview

Systematically estimate team capacity, identify resource bottlenecks, and forecast delivery timelines based on historical velocity. Store capacity models, velocity metrics, and forecasts in Engram to support data-driven planning and staffing decisions.

## When to Use

Use this skill when:
- Planning a release or major milestone with multiple features
- A stakeholder asks "when will it be done?"
- Team composition changes (new hires, departures, part-time assignments)
- Multiple projects compete for the same resources
- You need to justify hiring decisions or timeline extensions
- Sprint planning requires realistic workload distribution

## The Pattern

### Step 1: Assess Current Capacity

Calculate available capacity considering:

**Time Availability:**
- Work days in period (accounting for holidays, PTO)
- Percentage of time on project vs other work
- On-call rotations and support burden
- Meetings and overhead (typically 20-30% of time)

**Team Composition:**
- Individual skill levels (junior, mid, senior)
- Domain expertise (who knows which systems)
- Part-time vs full-time allocation
- Planned absences or departures

```bash
engram context create \
  --title "Capacity Assessment: [Team Name] - [Time Period]" \
  --content "## Time Period\n[Start Date] to [End Date] ([N weeks])\n\n## Team Composition\n\n### Engineers\n1. [Name] - Senior Backend - 100% allocated - 8 weeks available\n   - Skills: [Rust, PostgreSQL, distributed systems]\n   - Velocity: [N story points/week] (historical avg)\n   \n2. [Name] - Mid Frontend - 70% allocated - 5.6 weeks available\n   - Skills: [React, TypeScript, CSS]\n   - Velocity: [N story points/week] (historical avg)\n   \n3. [Name] - Junior Full-stack - 100% allocated - 8 weeks available\n   - Skills: [Python, JavaScript, learning phase]\n   - Velocity: [N story points/week] (historical avg)\n\n## Time Reductions\n\n### Holidays and PTO\n- [Date range]: [Holiday name] - All team members\n- [Name]: PTO [dates] - [N days]\n- Total person-days lost: [N]\n\n### Other Commitments\n- On-call rotation: [N days per person per period]\n- Support tickets: Estimate [M hours/week]\n- Meetings: Estimate 20% of time (standups, planning, retros)\n- Context switching: 10% overhead between projects\n\n### Production Support\n- Incidents: Historical average [N hours/week]\n- Bug fixes: [M% of capacity reserved]\n- Tech debt: [P% of capacity reserved]\n\n## Effective Capacity\n\n**Total Theoretical Hours:**\n[Team size] × [Weeks] × [40 hours/week] = [N hours]\n\n**Adjustments:**\n- Holidays/PTO: -[N hours]\n- Part-time allocation: -[M hours]\n- Meetings/overhead: -20% = -[P hours]\n- Support burden: -[Q hours]\n\n**Total Available Hours:** [X hours]\n**Available Story Points:** [Y points] (based on historical velocity)\n\n## Bottlenecks\n\n### Skill Bottlenecks\n- [Skill area]: Only [Name] has expertise - [Max N points/week]\n- [System]: Only [Name] and [Name] familiar - Risk if one absent\n\n### Dependency Bottlenecks\n- [External team]: Delivers API by [date] - Blocks [N points of work]\n- [Approval process]: Requires [N days] - Blocks deployment\n\n### Infrastructure Bottlenecks\n- [Environment]: Only [M] test environments available - Limits parallel testing\n- [Tool]: Only [N] licenses - Limits concurrent dev work" \
  --source "capacity-planning" \
  --tags "capacity,capacity-planning,[team-name],[period]"
```

### Step 2: Calculate Historical Velocity

Track completed work over past periods:

```bash
engram reasoning create \
  --title "Velocity Trend Analysis: [Team Name]" \
  --task-id [TASK_ID] \
  --content "## Historical Velocity (Last 6 Sprints)\n\n**Sprint 1 ([Dates]):**\n- Committed: [N points]\n- Completed: [M points]\n- Completion rate: [M/N]%\n- Notes: [Holiday week, reduced capacity]\n\n**Sprint 2 ([Dates]):**\n- Committed: [N points]\n- Completed: [M points]\n- Completion rate: [M/N]%\n- Notes: [New team member onboarding]\n\n**Sprint 3 ([Dates]):**\n- Committed: [N points]\n- Completed: [M points]\n- Completion rate: [M/N]%\n- Notes: [Incident consumed 20% capacity]\n\n[Continue for all sprints...]\n\n## Velocity Statistics\n\n**Average Velocity:** [X points/sprint]\n**Median Velocity:** [Y points/sprint] (more stable metric)\n**Standard Deviation:** [Z points]\n**Range:** [Min]-[Max] points\n\n**Trends:**\n- Increasing: Velocity up [N%] over period (team maturing)\n- Stable: Velocity within [M points] (predictable)\n- Decreasing: Velocity down [P%] over period (investigate causes)\n\n## Factors Affecting Velocity\n\n**Positive Factors:**\n- Team stability: Same members for [N sprints]\n- Improved tooling: CI/CD reduced deployment time by [M%]\n- Reduced tech debt: Refactoring paid off\n\n**Negative Factors:**\n- High support burden: [N incidents] consumed [M% capacity]\n- New team members: Onboarding overhead [P weeks]\n- Cross-team dependencies: Blocked on [External team] [Q times]\n\n## Velocity Forecast\n\n**Conservative (p90):** [Low] points/sprint\n**Most Likely (p50):** [Med] points/sprint\n**Optimistic (p10):** [High] points/sprint\n\n**Recommendation:** Use [Conservative/Most Likely] for commitments.\n\n**Confidence:** [0.0-1.0]\n\n**Assumptions:**\n- Team composition remains stable\n- Support burden similar to historical average\n- No major architectural changes requiring research\n- Dependencies deliver on schedule" \
  --confidence [0.0-1.0] \
  --tags "capacity,velocity,capacity-planning,[team-name]"
```

### Step 3: Estimate Upcoming Work

Break down planned work into estimable units:

```bash
engram reasoning create \
  --title "Work Estimation: [Feature/Release Name]" \
  --task-id [TASK_ID] \
  --content "## Work Breakdown\n\n### Feature 1: [Name]\n\n**User Stories:**\n1. [Story description] - Estimate: [N points]\n   - Complexity: [High/Medium/Low]\n   - Skills required: [Backend, Database]\n   - Dependencies: [None/Blocked by X]\n   \n2. [Story description] - Estimate: [M points]\n   - Complexity: [High/Medium/Low]\n   - Skills required: [Frontend, API integration]\n   - Dependencies: [Story 1 must complete]\n\n**Total for Feature 1:** [Sum points]\n\n### Feature 2: [Name]\n\n[Similar breakdown...]\n\n### Feature 3: [Name]\n\n[Similar breakdown...]\n\n## Total Work Estimate\n\n**All Features:** [Total points]\n\n**Estimation Confidence:**\n- High confidence (familiar domain): [N points]\n- Medium confidence (some unknowns): [M points]\n- Low confidence (research needed): [P points]\n\n**Recommended Buffer:**\n- Add 20% for unknowns: +[Buffer points]\n- Add [Q days] for spike investigations\n\n**Total with Buffer:** [Total + Buffer] points\n\n## Skill Distribution\n\n**Backend work:** [N points] - Requires: [Name], [Name]\n**Frontend work:** [M points] - Requires: [Name]\n**Database work:** [P points] - Requires: [Name] (bottleneck)\n**DevOps work:** [Q points] - Requires: [External team]\n\n**Skill Imbalance:**\n- [Backend] has [X%] more work than capacity\n- [Frontend] has capacity for [Y%] more work\n\n**Recommendations:**\n- Cross-train [Name] on backend to reduce bottleneck\n- Consider contracting frontend work to external team\n- Descope [Feature] to balance workload" \
  --confidence [0.0-1.0] \
  --tags "capacity,estimation,capacity-planning,[feature-name]"
```

### Step 4: Forecast Completion Date

Calculate timeline based on capacity and velocity:

```bash
engram reasoning create \
  --title "Delivery Forecast: [Feature/Release Name]" \
  --task-id [TASK_ID] \
  --content "## Forecast Calculation\n\n**Total Work:** [N points] (including buffer)\n**Team Velocity:** [M points/sprint] (p50 median)\n**Sprints Required:** [N / M] = [X sprints]\n\n**Start Date:** [Date]\n**End Date (p50):** [Date + X sprints]\n\n## Confidence Intervals\n\n**Conservative (p90 - 90% confident):**\n- Velocity: [Low points/sprint]\n- Sprints: [N / Low] = [Y sprints]\n- Completion: [Date + Y sprints]\n\n**Most Likely (p50 - 50% confident):**\n- Velocity: [Med points/sprint]\n- Sprints: [N / Med] = [X sprints]\n- Completion: [Date + X sprints]\n\n**Optimistic (p10 - 10% confident):**\n- Velocity: [High points/sprint]\n- Sprints: [N / High] = [Z sprints]\n- Completion: [Date + Z sprints]\n\n## Risks to Timeline\n\n**High Probability:**\n- [Risk]: Could add [N days] delay\n- [Risk]: Could add [M days] delay\n\n**Medium Probability:**\n- [Risk]: Could add [P days] if materializes\n\n**Mitigation:**\n- Start [Task] early to reduce critical path\n- Add buffer sprint for unknowns\n- Plan parallel work streams where possible\n\n## Staffing Options\n\n**Scenario 1: Current Team (No Changes)**\n- Completion: [Date p50]\n- Risk: [Medium/High] - bottlenecks in [skill area]\n\n**Scenario 2: Add Contractor (+1 Frontend)**\n- Completion: [Date - N weeks]\n- Cost: [Estimate]\n- Risk: [Low/Medium] - reduces bottleneck\n\n**Scenario 3: Descope Feature [X]**\n- Completion: [Date - M weeks]\n- Cost: [Zero]\n- Risk: [Low] - feature not on critical path\n- Trade-off: Lose [business value]\n\n## Recommendation\n\n**Best Path:** [Scenario X]\n\n**Rationale:**\n[Why this balances timeline, cost, and risk]\n\n**Confidence:** [0.0-1.0]" \
  --confidence [0.0-1.0] \
  --tags "capacity,forecast,capacity-planning,[feature-name]"
```

### Step 5: Track and Update

Monitor progress against forecast:

```bash
# At end of each sprint/period
engram reasoning create \
  --title "Capacity Review: [Team Name] - [Sprint/Period]" \
  --task-id [TASK_ID] \
  --content "**Period:** [Dates]\n\n**Forecast vs Actual:**\n- Forecasted: [N points]\n- Actual: [M points]\n- Variance: [M - N] points ([%]%)\n\n**Reasons for Variance:**\n- [Reason 1]: [e.g., Incident consumed 15% capacity]\n- [Reason 2]: [e.g., Dependency delayed by 3 days]\n- [Reason 3]: [e.g., Stories larger than estimated]\n\n**Velocity Update:**\n- Previous average: [X points/sprint]\n- New average: [Y points/sprint]\n- Trend: [Improving/Stable/Declining]\n\n**Forecast Adjustment:**\n- Original completion: [Date]\n- Updated completion: [Date]\n- Change: [+/- N days]\n\n**Actions Taken:**\n- [Action to address bottleneck]\n- [Action to improve velocity]\n\n**Lessons Learned:**\n[What to improve in next planning cycle]" \
  --confidence 1.0 \
  --tags "capacity,review,capacity-planning,[team-name]"
```

### Step 6: Link Capacity Entities

```bash
# Link all capacity planning to task
engram relationship create \
  --source-id [TASK_ID] --source-type task \
  --target-id [CAPACITY_ASSESSMENT_ID] --target-type context \
  --relationship-type references \
  --agent [AGENT]

engram relationship create \
  --source-id [TASK_ID] --source-type task \
  --target-id [VELOCITY_ANALYSIS_ID] --target-type reasoning \
  --relationship-type documents \
  --agent [AGENT]

engram relationship create \
  --source-id [TASK_ID] --source-type task \
  --target-id [WORK_ESTIMATION_ID] --target-type reasoning \
  --relationship-type documents \
  --agent [AGENT]

engram relationship create \
  --source-id [TASK_ID] --source-type task \
  --target-id [FORECAST_ID] --target-type reasoning \
  --relationship-type documents \
  --agent [AGENT]
```

## Example

User wants to plan capacity for Q2 release with 3 major features.

### Step 1: Assess Team Capacity

```bash
CAPACITY=$(engram context create \
  --title "Capacity Assessment: Backend Team - Q2 2026" \
  --content "## Time Period\nApril 1 - June 30, 2026 (13 weeks)\n\n## Team Composition\n\n### Engineers\n1. Alice Chen - Senior Backend - 100% allocated - 13 weeks available\n   - Skills: Rust, PostgreSQL, distributed systems, 5 years experience\n   - Velocity: 13 story points/week (historical avg)\n   \n2. Bob Wilson - Mid Backend - 70% allocated - 9.1 weeks available\n   - Skills: Python, Redis, API design, 2 years experience\n   - Velocity: 8 story points/week (historical avg)\n   - Note: 30% on Platform team project\n   \n3. Carol Davis - Junior Full-stack - 100% allocated - 13 weeks available\n   - Skills: JavaScript, Node.js, PostgreSQL, 6 months experience\n   - Velocity: 5 story points/week (historical avg, ramping up)\n\n## Time Reductions\n\n### Holidays and PTO\n- April 18-22: Spring break - Carol out (1 week)\n- May 27: Memorial Day - All team members\n- June 15-19: Alice PTO (1 week)\n- Total person-days lost: 17 days (~3.4 weeks)\n\n### Other Commitments\n- On-call rotation: 1 week per person per quarter (3 weeks total)\n- Support tickets: Average 5 hours/week per person\n- Meetings: 20% of time (standups, planning, 1:1s, retros)\n- Bob's Platform work: 30% allocation (already factored above)\n\n### Production Support\n- Incidents: Historical average 10 hours/week team-wide\n- Bug fixes: Reserve 15% of capacity\n- Tech debt: Reserve 10% of capacity (paying down auth system)\n\n## Effective Capacity\n\n**Total Theoretical Hours:**\n3 engineers × 13 weeks × 40 hours/week = 1,560 hours\n\n**Adjustments:**\n- Bob part-time (30% other): -156 hours\n- Holidays/PTO: -136 hours\n- Meetings/overhead: -20% = -280 hours\n- Support burden: -130 hours (10 hr/wk × 13 wk)\n- Bug fixes: -15% = -149 hours\n- Tech debt: -10% = -99 hours\n\n**Total Available Hours:** 610 hours for feature work\n**Available Story Points:** 213 points (based on historical velocity)\n\n## Bottlenecks\n\n### Skill Bottlenecks\n- Distributed systems: Only Alice has expertise - Max 13 points/week\n- Database migrations: Only Alice and Bob comfortable - Risk if both absent\n- Frontend work: Only Carol has deep React knowledge - Max 5 points/week\n\n### Dependency Bottlenecks\n- Frontend team: Delivers new component library by May 1 - Blocks UI work\n- DevOps team: Kubernetes upgrade April 15-20 - Blocks deployment testing\n\n### Infrastructure Bottlenecks\n- Staging environment: Only 1 available - Limits parallel integration testing\n- Load testing: Requires coordination with DevOps - 3 day lead time" \
  --source "capacity-planning" \
  --tags "capacity,capacity-planning,backend-team,q2-2026" \
  --json | jq -r '.id')

echo "Capacity assessment created: $CAPACITY"
```

### Step 2: Analyze Historical Velocity

```bash
VELOCITY=$(engram reasoning create \
  --title "Velocity Trend Analysis: Backend Team" \
  --task-id a3f8b2c1-1234-5678-90ab-cdef12345678 \
  --content "## Historical Velocity (Last 6 Sprints)\n\n**Sprint 1 (Jan 1-14):**\n- Committed: 45 points\n- Completed: 42 points\n- Completion rate: 93%\n- Notes: Holiday week Jan 1, reduced capacity\n\n**Sprint 2 (Jan 15-28):**\n- Committed: 50 points\n- Completed: 38 points\n- Completion rate: 76%\n- Notes: Major incident Jan 20 consumed 25% capacity\n\n**Sprint 3 (Jan 29-Feb 11):**\n- Committed: 48 points\n- Completed: 47 points\n- Completion rate: 98%\n- Notes: Clean sprint, no incidents\n\n**Sprint 4 (Feb 12-25):**\n- Committed: 50 points\n- Completed: 51 points\n- Completion rate: 102%\n- Notes: Carol velocity improving, stories smaller than estimated\n\n**Sprint 5 (Feb 26-Mar 10):**\n- Committed: 52 points\n- Completed: 49 points\n- Completion rate: 94%\n- Notes: Bob split between projects, lower effective capacity\n\n**Sprint 6 (Mar 11-24):**\n- Committed: 50 points\n- Completed: 48 points\n- Completion rate: 96%\n- Notes: Stable performance\n\n## Velocity Statistics\n\n**Average Velocity:** 46 points/sprint\n**Median Velocity:** 48 points/sprint (more stable metric)\n**Standard Deviation:** 4.7 points\n**Range:** 38-51 points\n**Completion Rate:** 93% average\n\n**Trends:**\n- Stable: Velocity within 5 points over 6 sprints (predictable)\n- Carol ramping: +2 points/sprint as she gains experience\n- Bob constraint: 30% allocation limits team capacity\n\n## Factors Affecting Velocity\n\n**Positive Factors:**\n- Team stability: Same core members for 6 months\n- Improved CI/CD: Deployment time down from 45min to 12min\n- Carol maturing: Velocity increased 60% since onboarding\n- Better estimation: Completion rate improved from 80% to 95%\n\n**Negative Factors:**\n- Bob split allocation: Effectively 0.7 FTE, reduces team velocity\n- Incidents: Average 2 per sprint consuming 10-25% capacity\n- Tech debt in auth: Slows new feature development\n- Dependency wait: Frontend team blocked us 2 times\n\n## Velocity Forecast for Q2\n\n**Conservative (p90):** 42 points/sprint\n- Accounts for: 1-2 incidents, dependency delays, estimation errors\n- Use for: External commitments, executive reporting\n\n**Most Likely (p50):** 46 points/sprint\n- Based on: Median historical velocity\n- Use for: Internal planning, sprint commitments\n\n**Optimistic (p10):** 50 points/sprint\n- Assumes: No incidents, all dependencies deliver, Carol continues growth\n- Use for: Best-case scenario planning only\n\n**Recommendation:** Use 46 points/sprint (p50) for Q2 planning, with 42 points as floor for commitments.\n\n**Confidence:** 0.80\n\n**Assumptions:**\n- Team composition remains stable (Alice, Bob, Carol)\n- Bob continues 70% allocation (no change from Platform team)\n- Incident rate similar to historical (2 per sprint)\n- Carol continues velocity growth trajectory\n- No major architectural changes requiring research\n- Dependencies deliver on schedule (Frontend component library)" \
  --confidence 0.80 \
  --tags "capacity,velocity,capacity-planning,backend-team" \
  --json | jq -r '.id')

echo "Velocity analysis created: $VELOCITY"
```

### Step 3: Estimate Q2 Work

```bash
ESTIMATION=$(engram reasoning create \
  --title "Work Estimation: Q2 2026 Release" \
  --task-id a3f8b2c1-1234-5678-90ab-cdef12345678 \
  --content "## Work Breakdown\n\n### Feature 1: Payment Processing Integration\n\n**User Stories:**\n1. Design payment schema and API - Estimate: 5 points\n   - Complexity: Medium\n   - Skills required: Backend (Alice), Database\n   - Dependencies: None\n   \n2. Implement Stripe SDK integration - Estimate: 8 points\n   - Complexity: Medium\n   - Skills required: Backend (Bob)\n   - Dependencies: Story 1 complete\n   \n3. Build payment webhook handler - Estimate: 5 points\n   - Complexity: Low\n   - Skills required: Backend (Bob or Carol)\n   - Dependencies: Story 2 complete\n   \n4. Add payment UI components - Estimate: 8 points\n   - Complexity: Medium\n   - Skills required: Frontend (Carol), requires component library\n   - Dependencies: Frontend team delivers library May 1\n   \n5. Integration testing and edge cases - Estimate: 8 points\n   - Complexity: High\n   - Skills required: Full team\n   - Dependencies: Stories 1-4 complete\n\n**Total for Feature 1:** 34 points\n\n### Feature 2: Real-time Notification System\n\n**User Stories:**\n1. Set up WebSocket server infrastructure - Estimate: 13 points\n   - Complexity: High\n   - Skills required: Backend (Alice), DevOps coordination\n   - Dependencies: None\n   \n2. Implement notification delivery logic - Estimate: 8 points\n   - Complexity: Medium\n   - Skills required: Backend (Bob)\n   - Dependencies: Story 1 complete\n   \n3. Build client-side WebSocket handlers - Estimate: 8 points\n   - Complexity: Medium\n   - Skills required: Frontend (Carol)\n   - Dependencies: Story 1 complete\n   \n4. Add notification persistence to DB - Estimate: 5 points\n   - Complexity: Low\n   - Skills required: Backend (Bob or Carol)\n   - Dependencies: Story 2 complete\n   \n5. Load testing and scalability validation - Estimate: 8 points\n   - Complexity: High\n   - Skills required: Backend (Alice), DevOps\n   - Dependencies: Stories 1-4 complete\n\n**Total for Feature 2:** 42 points\n\n### Feature 3: Advanced Search and Filtering\n\n**User Stories:**\n1. Design search indexing strategy - Estimate: 5 points\n   - Complexity: Medium\n   - Skills required: Backend (Alice)\n   - Dependencies: None\n   \n2. Implement Elasticsearch integration - Estimate: 13 points\n   - Complexity: High\n   - Skills required: Backend (Alice, Bob)\n   - Dependencies: Story 1 complete\n   \n3. Build search API endpoints - Estimate: 8 points\n   - Complexity: Medium\n   - Skills required: Backend (Bob)\n   - Dependencies: Story 2 complete\n   \n4. Create search UI with filters - Estimate: 13 points\n   - Complexity: High\n   - Skills required: Frontend (Carol)\n   - Dependencies: Story 3 complete\n   \n5. Index backfill and performance tuning - Estimate: 8 points\n   - Complexity: High\n   - Skills required: Backend (Alice)\n   - Dependencies: Stories 2-4 complete\n\n**Total for Feature 3:** 47 points\n\n### Supporting Work\n\n**Tech Debt:**\n- Auth system refactor: 21 points (10% capacity reserved)\n\n**Bug Fixes:**\n- Ongoing: 32 points (15% capacity reserved)\n\n**DevOps/Infrastructure:**\n- Kubernetes migration support: 8 points\n- Monitoring improvements: 5 points\n\n**Total Supporting:** 66 points\n\n## Total Work Estimate\n\n**Feature Work:** 123 points (34 + 42 + 47)\n**Supporting Work:** 66 points\n**Grand Total:** 189 points\n\n**Estimation Confidence:**\n- High confidence (familiar domain): 87 points (payment, notifications)\n- Medium confidence (some unknowns): 56 points (search, WebSocket scaling)\n- Low confidence (new territory): 46 points (Elasticsearch, complex UI)\n\n**Recommended Buffer:**\n- Add 15% for unknowns: +18 points\n- Add 5 days for spike investigation (search indexing): +8 points\n\n**Total with Buffer:** 215 points\n\n## Skill Distribution\n\n**Alice (Senior) work:** 76 points - Requires distributed systems, architecture\n**Bob (Mid) work:** 68 points - API development, integrations\n**Carol (Junior) work:** 45 points - Frontend, simpler backend tasks\n**Shared work:** 26 points - Testing, documentation\n\n**Alice Capacity:** 13 points/week × 12 weeks (1 PTO) = 156 points ✓\n**Bob Capacity:** 8 points/week × 13 weeks × 0.7 = 73 points ✓\n**Carol Capacity:** 5 points/week × 12 weeks (1 PTO) = 60 points ✓\n\n**Skill Balance:** Relatively balanced, Alice has headroom, Carol near capacity\n\n**Bottleneck Risk:** Carol is bottleneck for frontend work (45 points, 60 capacity). If any frontend work grows, will delay features.\n\n**Recommendations:**\n- Prioritize Feature 1 (payments) - highest business value\n- Consider descoping advanced filters from Feature 3 if capacity tight\n- Prepare to shift some of Bob's work to Alice if Carol frontend work expands" \
  --confidence 0.75 \
  --tags "capacity,estimation,capacity-planning,q2-release" \
  --json | jq -r '.id')

echo "Work estimation created: $ESTIMATION"
```

### Step 4: Forecast Delivery

```bash
FORECAST=$(engram reasoning create \
  --title "Delivery Forecast: Q2 2026 Release" \
  --task-id a3f8b2c1-1234-5678-90ab-cdef12345678 \
  --content "## Forecast Calculation\n\n**Total Work:** 215 points (including 15% buffer)\n**Team Velocity:** 46 points/sprint (p50 median)\n**Sprint Length:** 2 weeks\n**Sprints in Q2:** 6.5 sprints (13 weeks)\n\n**Capacity Available:** 6.5 × 46 = 299 points\n**Work Planned:** 215 points\n**Headroom:** 84 points (28%)\n\n**Sprints Required:** 215 / 46 = 4.7 sprints = ~9.4 weeks\n\n**Start Date:** April 1, 2026\n**End Date (p50):** June 8, 2026 (with 3 weeks buffer until end of Q2)\n\n## Confidence Intervals\n\n**Conservative (p90 - 90% confident):**\n- Velocity: 42 points/sprint\n- Sprints: 215 / 42 = 5.1 sprints\n- Duration: ~10.2 weeks\n- Completion: June 15, 2026 (2 weeks buffer)\n- Risk: Low - accounts for incidents and delays\n\n**Most Likely (p50 - 50% confident):**\n- Velocity: 46 points/sprint\n- Sprints: 215 / 46 = 4.7 sprints\n- Duration: ~9.4 weeks\n- Completion: June 8, 2026 (3 weeks buffer)\n- Risk: Medium - assumes historical velocity holds\n\n**Optimistic (p10 - 10% confident):**\n- Velocity: 50 points/sprint\n- Sprints: 215 / 50 = 4.3 sprints\n- Duration: ~8.6 weeks\n- Completion: May 30, 2026 (4 weeks buffer)\n- Risk: High - requires everything going smoothly\n\n## Feature Delivery Timeline\n\n**Feature 1: Payment Processing (34 points)**\n- Start: Sprint 1 (April 1)\n- End: Sprint 2 (April 28)\n- Dependency: None, can start immediately\n\n**Feature 2: Real-time Notifications (42 points)**\n- Start: Sprint 1 (April 1) - infrastructure in parallel\n- End: Sprint 3 (May 12)\n- Dependency: WebSocket infrastructure complete first\n\n**Feature 3: Advanced Search (47 points)**\n- Start: Sprint 2 (April 15) - after spike investigation\n- End: Sprint 4 (May 26)\n- Dependency: Elasticsearch spike complete\n\n**Integration and Polish:** Sprints 5-6 (May 26 - June 22)\n\n## Risks to Timeline\n\n**High Probability:**\n- Frontend component library delay: Could add 1 sprint (2 weeks) to Feature 1\n- Elasticsearch learning curve: Spike may reveal complexity, add 5-8 points\n\n**Medium Probability:**\n- Major incident during Q2: Historical average 2 per sprint, already factored\n- Carol PTO extended: Could reduce capacity 5 points/week\n\n**Low Probability:**\n- Bob pulled to Platform full-time: Would lose 8 points/week (crisis scenario)\n- Alice departure: Would require hiring, 3+ month delay\n\n**Mitigation:**\n- Start Elasticsearch spike immediately (April 1) to reduce uncertainty\n- Coordinate with Frontend team weekly on component library progress\n- Cross-train Bob on Alice's work to reduce single-point-of-failure\n- Identify descope options if timeline compresses\n\n## Staffing Options\n\n**Scenario 1: Current Team (No Changes)**\n- Completion: June 8, 2026 (p50)\n- Cost: Current burn rate\n- Risk: Medium - Carol is bottleneck for frontend\n- Headroom: 84 points (28%) buffer for unknowns\n\n**Scenario 2: Add Contract Frontend Developer (0.5 FTE)**\n- Completion: May 25, 2026 (2 weeks earlier)\n- Cost: +$15K for 10 weeks\n- Risk: Low - removes Carol bottleneck\n- Trade-off: Onboarding overhead (~5 points), but unlocks parallel work\n\n**Scenario 3: Descope Feature 3 (Advanced Search)**\n- Completion: May 19, 2026 (3 weeks earlier)\n- Cost: Zero\n- Risk: Low - Features 1 and 2 fit comfortably in 4 sprints\n- Trade-off: Lose search functionality (defer to Q3)\n\n**Scenario 4: Descope Advanced Filters from Feature 3**\n- Completion: June 1, 2026 (1 week earlier)\n- Cost: Zero\n- Risk: Low - Deliver basic search, iterate in Q3\n- Trade-off: Reduced scope (-13 points), keep search foundation\n\n## Recommendation\n\n**Best Path:** Scenario 1 (Current Team) with Scenario 4 (Descope Advanced Filters) as contingency\n\n**Rationale:**\n- Current forecast shows 3 weeks buffer with existing team\n- 28% headroom is healthy for unknowns and scope creep\n- Carol bottleneck is manageable if frontend work doesn't expand\n- Cost of contractor ($15K) not justified given healthy buffer\n- Descoping advanced filters (-13 points) available if timeline compresses\n\n**Recommendation to Stakeholders:**\n\"We can deliver all 3 features by June 8 with 90% confidence (conservative forecast: June 15). This leaves 2-3 weeks buffer before end of Q2. If timeline becomes critical, we can descope advanced filters from search feature to gain 1 week.\"\n\n**Confidence:** 0.85\n\n**Next Steps:**\n1. Start Elasticsearch spike April 1 to validate search estimates\n2. Coordinate with Frontend team on component library delivery date\n3. Review forecast after Sprint 2 (April 28) to validate velocity assumptions\n4. Prepare descope decision for advanced filters by May 15 if needed" \
  --confidence 0.85 \
  --tags "capacity,forecast,capacity-planning,q2-release" \
  --json | jq -r '.id')

echo "Delivery forecast created: $FORECAST"
```

### Step 5: Link All Entities

```bash
# Link capacity planning entities to task
engram relationship create \
  --source-id a3f8b2c1-1234-5678-90ab-cdef12345678 --source-type task \
  --target-id $CAPACITY --target-type context \
  --relationship-type references \
  --agent default

engram relationship create \
  --source-id a3f8b2c1-1234-5678-90ab-cdef12345678 --source-type task \
  --target-id $VELOCITY --target-type reasoning \
  --relationship-type documents \
  --agent default

engram relationship create \
  --source-id a3f8b2c1-1234-5678-90ab-cdef12345678 --source-type task \
  --target-id $ESTIMATION --target-type reasoning \
  --relationship-type documents \
  --agent default

engram relationship create \
  --source-id a3f8b2c1-1234-5678-90ab-cdef12345678 --source-type task \
  --target-id $FORECAST --target-type reasoning \
  --relationship-type documents \
  --agent default
```

### Step 6: Communicate to Stakeholders

Agent presents capacity plan:

"I've analyzed team capacity for Q2. With current team (Alice, Bob 70%, Carol), we have 299 points available vs 215 points planned (28% buffer). Forecast completion: June 8 (p50) with 3 weeks buffer. Key constraint: Carol is bottleneck for frontend work. Recommend proceeding with current team, with option to descope advanced filters if needed. All forecasts in Engram. Questions?"

## Querying Capacity Plans

After creating capacity plan, agents can retrieve information:

```bash
# Get capacity assessment
engram context list | grep "Capacity Assessment"

# Get velocity trends
engram reasoning list | grep "Velocity Trend"

# Get work estimates
engram reasoning list | grep "Work Estimation"

# Get delivery forecasts
engram reasoning list | grep "Delivery Forecast"

# Get all capacity planning for a team
engram relationship connected --entity-id [TASK_ID] | grep -E "Capacity|Velocity|Forecast"
```

## Related Skills

This skill integrates with:
- `engram-roadmap-planning` - Feed capacity constraints into roadmap
- `engram-backlog-refinement` - Estimate stories during refinement
- `engram-release-planning` - Validate release dates against capacity
- `engram-dependency-mapping` - Factor dependencies into timeline
- `engram-risk-assessment` - Identify capacity risks and constraints
