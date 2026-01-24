---
name: engram-post-mortem
description: "Analyze incidents and outages with timeline reconstruction, identify root causes and contributing factors, create action items without blame."
---

# Post-Mortem Analysis (Engram-Integrated)

## Overview

Systematically analyze incidents, outages, and production failures to understand what happened, why it happened, and how to prevent recurrence. Reconstruct detailed timelines, identify root causes and contributing factors using techniques like Five Whys and Fishbone diagrams, and create actionable remediation plans. Store post-mortem analyses in Engram to build organizational learning and track incident patterns over time. Conduct post-mortems with blameless culture focusing on systems and processes, not individuals.

## When to Use

Use this skill when:
- Production incident or outage occurs
- Service degradation affects users
- Data loss or corruption incident
- Security breach or vulnerability exploitation
- Major bug escapes to production causing user impact
- Near-miss incident (could have been severe)
- Recurring incidents with same root cause
- After resolving critical issue to prevent recurrence

## The Pattern

### Step 1: Gather Incident Data and Reconstruct Timeline

Collect all relevant data to build accurate timeline:

```bash
engram context create \
  --title "Incident Post-Mortem: [Incident Name/ID]" \
  --content "## Incident Overview

**Incident ID:** [INC-XXXX]
**Incident Title:** [Brief description, e.g., Database Outage - Payment Processing Down]
**Severity:** [SEV-1: Critical / SEV-2: High / SEV-3: Medium / SEV-4: Low]
**Status:** Resolved
**Date:** [YYYY-MM-DD]

**Impact:**
- **Duration:** [N] hours [N] minutes (from detection to full resolution)
- **Users affected:** [N] users ([N%] of total)
- **Services affected:** [Service names]
- **Business impact:**
  - Revenue loss: $[N]
  - Failed transactions: [N]
  - Support tickets: [N]
  - SLA breach: [Yes/No]

**Incident Commander:** [Name]
**Communication Lead:** [Name]
**Technical Lead:** [Name]
**Participants:** [Names of responders]

## Timeline

All times in UTC.

### Detection Phase

**[HH:MM:SS]** - Incident begins (root cause occurs)
- [Description of what happened to cause incident]
- [System logs, metrics, or evidence]

**[HH:MM:SS]** - First symptom detected
- Monitoring alert: [Alert name] triggered
- Alert details: [Metric exceeded threshold]
- Source: [Monitoring system]

**[HH:MM:SS]** - Incident declared
- On-call engineer [Name] paged
- Initial assessment: [Brief description]
- Severity assigned: [SEV-N]

### Investigation Phase

**[HH:MM:SS]** - Investigation begins
- [Name] joined incident response
- Checked: [Systems/logs/metrics examined]
- Finding: [What was discovered]

**[HH:MM:SS]** - Initial hypothesis
- Suspected cause: [First theory]
- Reasoning: [Why this was suspected]

**[HH:MM:SS]** - Hypothesis disproven
- Evidence: [What disproved initial theory]
- New direction: [What to investigate next]

**[HH:MM:SS]** - Root cause identified
- Root cause: [Actual cause of incident]
- Evidence: [Logs, metrics, traces confirming root cause]
- Location: [System/service/code file:line]

### Mitigation Phase

**[HH:MM:SS]** - Mitigation started
- Action taken: [What was done to stop the bleeding]
- Executor: [Name]

**[HH:MM:SS]** - Partial restoration
- Status: [What services restored, what still broken]
- Metrics: [Error rate, latency improvements]

**[HH:MM:SS]** - Workaround deployed
- Workaround: [Temporary fix applied]
- Impact: [How this helped]

**[HH:MM:SS]** - Incident resolved
- Resolution action: [Final action that resolved incident]
- All services operational: [Confirmed healthy state]

### Communication Phase

**[HH:MM:SS]** - Initial customer communication
- Channel: [Status page, email, in-app notification]
- Message: [Summary of what was communicated]

**[HH:MM:SS]** - Update sent
- Message: [Progress update content]

**[HH:MM:SS]** - Resolution communicated
- Message: [Resolution announcement]
- Follow-up: [Promised post-incident report]

## Incident Metrics

**Detection:**
- Time from incident start to detection: [N] minutes
- Detection method: [Monitoring alert / Customer report / Manual discovery]

**Response:**
- Time from detection to first responder joined: [N] minutes
- Time from detection to root cause identified: [N] minutes
- Time from detection to mitigation started: [N] minutes
- Time from detection to full resolution: [N] minutes

**Communication:**
- Time from detection to first customer communication: [N] minutes
- Number of updates sent: [N]

**Impact:**
- Total downtime: [N] hours [N] minutes
- User-minutes of downtime: [N users × N minutes = N]
- Peak error rate: [N]% (normal: [N]%)
- Peak latency: [N]ms (normal: [N]ms)

## Monitoring and Observability Data

**Alerts triggered:**
1. [HH:MM:SS] - [Alert name]: [Description]
2. [HH:MM:SS] - [Alert name]: [Description]

**Key metrics during incident:**
- CPU: [N]% (normal: [N]%)
- Memory: [N]% (normal: [N]%)
- Database connections: [N] (max: [N])
- Queue depth: [N] (normal: [N])
- Error rate: [N]% (normal: [N]%)

**Relevant logs:**
\`\`\`
[Timestamp] ERROR [service]: [Log message]
[Timestamp] ERROR [service]: [Another key log]
\`\`\`

**Traces:**
- [Link to distributed trace showing failure path]

**Dashboards:**
- [Link to monitoring dashboard during incident]

## Artifacts

**Slack incident channel:** [#incident-XXXX]
**Incident Zoom recording:** [Link]
**Runbook executed:** [Link to runbook]
**Post-incident report:** [This document]" \
  --source "incident-post-mortem" \
  --tags "post-mortem,incident,[incident-id]"
```

### Step 2: Root Cause Analysis (Five Whys)

Use Five Whys technique to identify root cause:

```bash
engram reasoning create \
  --title "Root Cause Analysis: [Incident]" \
  --task-id [TASK_ID] \
  --content "## Five Whys Analysis

**Problem Statement:**
[Concise description of what went wrong]

**Why 1: Why did [problem] happen?**
Answer: [Direct cause]

**Why 2: Why did [answer 1] happen?**
Answer: [Cause of direct cause]

**Why 3: Why did [answer 2] happen?**
Answer: [Deeper cause]

**Why 4: Why did [answer 3] happen?**
Answer: [Even deeper cause]

**Why 5: Why did [answer 4] happen?**
Answer: [Root cause - typically a process or system issue]

**Root Cause:**
[The deepest cause identified - this is what we need to fix]

## Example Five Whys

**Problem Statement:**
Database ran out of connections, causing 500 errors for all users.

**Why 1: Why did database run out of connections?**
Answer: Connection pool exhausted (all 100 connections in use).

**Why 2: Why was connection pool exhausted?**
Answer: Connections were not being released after queries completed.

**Why 3: Why were connections not being released?**
Answer: Application code had a bug where exceptions caused early return without closing connection.

**Why 4: Why did buggy code reach production?**
Answer: Code review didn't catch the bug, and tests didn't cover the error path.

**Why 5: Why didn't tests cover the error path?**
Answer: No requirement for error path test coverage, no automated coverage checks in CI.

**Root Cause:**
Lack of enforced test coverage requirements for error paths allowed buggy code to reach production.

## Contributing Factors

Beyond root cause, what else contributed?

**Factor 1: Inadequate Monitoring**
- No alert on connection pool utilization
- Discovered incident only when users reported errors
- **Impact:** Delayed detection by [N] minutes

**Factor 2: Missing Runbook**
- No documented procedure for connection pool exhaustion
- Responders had to figure out mitigation from scratch
- **Impact:** Delayed mitigation by [N] minutes

**Factor 3: No Connection Timeout**
- Connections never timed out, remained stuck indefinitely
- **Impact:** Amplified the problem (stuck connections accumulated)

**Factor 4: [Another factor]**
- [Description]
- **Impact:** [How it worsened incident]

## Fishbone (Ishikawa) Diagram

Categorize contributing factors:

**People:**
- On-call engineer unfamiliar with database operations
- No backup engineer available (single point of failure)

**Process:**
- No enforced test coverage policy
- Code review checklist doesn't include error handling checks
- No incident response drills

**Technology:**
- Database connection pool too small for peak load
- No connection timeout configured
- Monitoring gaps (connection pool not monitored)

**Environment:**
- Production deployment on Friday evening (reduced team availability)
- No staging environment with production-like load

## Root Cause Classification

**Type:** [Human error / System failure / Process gap / External factor]

**Specific classification:**
- [ ] Configuration error
- [ ] Code bug
- [ ] Infrastructure failure
- [ ] Third-party service outage
- [ ] Capacity exceeded
- [ ] Security incident
- [ ] Data corruption
- [X] Process gap (missing test coverage requirement)
- [ ] Human error
- [ ] Unknown

**Preventability:** [Preventable / Partially preventable / Unpreventable]

**Recurrence risk:** [High / Medium / Low] - [Explanation]

## Systemic Issues Identified

Looking beyond immediate cause:

**Issue 1: Testing Culture**
- Observation: Tests focus on happy path, error paths often skipped
- Evidence: Average test coverage 65%, but error paths at 20%
- Scope: Organization-wide problem
- Recommendation: Establish testing standards, enforce in CI

**Issue 2: Monitoring Gaps**
- Observation: Many critical metrics not monitored
- Evidence: No alerts for connection pool, queue depth, disk space
- Scope: Infrastructure team
- Recommendation: Comprehensive monitoring audit

**Issue 3: [Another systemic issue]**
- [Description]

## Lessons Learned

**What went well:**
1. [Positive aspect of response]
2. [Another thing that worked well]
3. [Something to continue doing]

**What went poorly:**
1. [Aspect of response that needs improvement]
2. [Another problem area]
3. [Something to stop doing]

**What was lucky:**
1. [Thing that could have made incident worse but didn't]
2. [Another near-miss]" \
  --confidence 0.85 \
  --tags "post-mortem,root-cause,five-whys,[incident-id]"
```

### Step 3: Create Action Items

Develop concrete, actionable remediation plan:

```bash
engram reasoning create \
  --title "Post-Mortem Action Items: [Incident]" \
  --task-id [TASK_ID] \
  --content "## Action Items Summary

**Total action items:** [N]
- Prevent recurrence: [N]
- Improve detection: [N]
- Improve response: [N]
- Process improvements: [N]

**Priority breakdown:**
- P0 (Critical - do immediately): [N]
- P1 (High - do this sprint): [N]
- P2 (Medium - do next sprint): [N]
- P3 (Low - backlog): [N]

## P0: Critical Actions (Prevent Immediate Recurrence)

**Action 1: Fix Connection Leak Bug**
- **Type:** Code fix
- **Description:** Ensure database connections always released, even on exception
- **Owner:** [Name]
- **Due:** [YYYY-MM-DD] (2 days)
- **Tracking:** [Ticket ID]
- **Success criteria:** 
  - All query functions use try/finally or context manager
  - Code review confirms fix
  - Test coverage for error paths added
- **Completion status:** [ ] Not started / [ ] In progress / [ ] Complete

**Action 2: Add Connection Pool Monitoring**
- **Type:** Monitoring
- **Description:** Alert when connection pool >80% utilized
- **Owner:** [Name]
- **Due:** [YYYY-MM-DD] (3 days)
- **Tracking:** [Ticket ID]
- **Success criteria:**
  - Metric exported from application
  - Dashboard shows connection pool utilization
  - Alert configured (>80% for 5 minutes)
  - Alert tested in staging
- **Completion status:** [ ] Not started / [ ] In progress / [ ] Complete

**Action 3: Increase Connection Pool Size**
- **Type:** Configuration
- **Description:** Increase from 100 to 200 connections (interim fix while optimizing queries)
- **Owner:** [Name]
- **Due:** [YYYY-MM-DD] (1 day)
- **Tracking:** [Ticket ID]
- **Success criteria:**
  - Config updated in all environments
  - Database can handle 200 connections
  - Load test confirms no exhaustion
- **Completion status:** [ ] Not started / [ ] In progress / [ ] Complete

## P1: High Priority (Improve Detection and Response)

**Action 4: Add Connection Timeout**
- **Type:** Configuration
- **Description:** Set 30-second timeout on database connections
- **Owner:** [Name]
- **Due:** [YYYY-MM-DD] (1 week)
- **Tracking:** [Ticket ID]
- **Success criteria:**
  - Timeout configured
  - Long-running queries identified and optimized
  - Monitoring shows stuck connections are freed
- **Completion status:** [ ] Not started / [ ] In progress / [ ] Complete

**Action 5: Create Database Runbook**
- **Type:** Documentation
- **Description:** Document investigation and mitigation steps for database issues
- **Owner:** [Name]
- **Due:** [YYYY-MM-DD] (1 week)
- **Tracking:** [Ticket ID]
- **Contents:**
  - How to check connection pool status
  - How to identify source of connection leak
  - How to gracefully restart application
  - How to scale database connections
  - Rollback procedures
- **Success criteria:**
  - Runbook published in wiki
  - Team trained on runbook
  - Runbook tested in drill
- **Completion status:** [ ] Not started / [ ] In progress / [ ] Complete

**Action 6: Improve Error Path Test Coverage**
- **Type:** Testing
- **Description:** Increase error path coverage from 20% to 80%
- **Owner:** [Name]
- **Due:** [YYYY-MM-DD] (2 weeks)
- **Tracking:** [Ticket ID]
- **Success criteria:**
  - Tests added for all error paths
  - Coverage report shows 80%+ error path coverage
  - CI fails if error path coverage drops below 70%
- **Completion status:** [ ] Not started / [ ] In progress / [ ] Complete

## P2: Medium Priority (Process Improvements)

**Action 7: Enforce Test Coverage in CI**
- **Type:** Process
- **Description:** Block PRs if test coverage <80% or drops by >2%
- **Owner:** [Name]
- **Due:** [YYYY-MM-DD] (3 weeks)
- **Tracking:** [Ticket ID]
- **Success criteria:**
  - CI configured with coverage checks
  - Team notified of new requirement
  - Documentation updated
- **Completion status:** [ ] Not started / [ ] In progress / [ ] Complete

**Action 8: Update Code Review Checklist**
- **Type:** Process
- **Description:** Add error handling and resource cleanup to checklist
- **Owner:** [Name]
- **Due:** [YYYY-MM-DD] (1 week)
- **Tracking:** [Ticket ID]
- **Checklist items:**
  - [ ] All error paths handled
  - [ ] Resources (connections, files) properly released
  - [ ] Error paths tested
  - [ ] Timeout configured for external calls
- **Success criteria:**
  - Checklist updated in PR template
  - Team trained on new checklist items
- **Completion status:** [ ] Not started / [ ] In progress / [ ] Complete

**Action 9: Conduct Incident Response Drill**
- **Type:** Training
- **Description:** Practice responding to database connection exhaustion
- **Owner:** [Name]
- **Due:** [YYYY-MM-DD] (1 month)
- **Tracking:** [Ticket ID]
- **Drill scenario:**
  - Inject connection leak in staging
  - Responders investigate and mitigate using runbook
  - Evaluate response time and effectiveness
- **Success criteria:**
  - Drill conducted with 80% of on-call rotation
  - Response time <30 minutes
  - Runbook improvements identified
- **Completion status:** [ ] Not started / [ ] In progress / [ ] Complete

## P3: Low Priority (Long-term Improvements)

**Action 10: Comprehensive Monitoring Audit**
- **Type:** Monitoring
- **Description:** Audit all systems for monitoring gaps
- **Owner:** [Name]
- **Due:** [YYYY-MM-DD] (2 months)
- **Tracking:** [Ticket ID]
- **Scope:**
  - Database: connections, query time, locks, replication lag
  - Application: queue depth, cache hit rate, goroutines/threads
  - Infrastructure: CPU, memory, disk, network
- **Success criteria:**
  - Audit document lists all gaps
  - Priority assigned to each gap
  - Plan to address top 10 gaps
- **Completion status:** [ ] Not started / [ ] In progress / [ ] Complete

**Action 11: Staging Environment with Production Load**
- **Type:** Infrastructure
- **Description:** Create staging environment that mirrors production scale
- **Owner:** [Name]
- **Due:** [YYYY-MM-DD] (3 months)
- **Tracking:** [Ticket ID]
- **Requirements:**
  - Database with production-like data volume
  - Load testing simulating peak production traffic
  - Monitoring matching production
- **Success criteria:**
  - Staging environment deployed
  - Load tests running weekly
  - Performance issues caught before production
- **Completion status:** [ ] Not started / [ ] In progress / [ ] Complete

## Action Item Tracking

**Review cadence:** Weekly in team standup

**Tracking dashboard:** [Link to action item tracker]

**Escalation:** If P0 items not complete within 1 week, escalate to engineering manager

**Completion criteria:** All P0 and P1 items complete, P2 items scheduled

## Action Item Progress

**Week 1:**
- [Status update]

**Week 2:**
- [Status update]

**Week 3:**
- [Status update]

**Final Status:**
- P0: [N/N complete]
- P1: [N/N complete]
- P2: [N/N complete]
- P3: [N/N complete]

**Post-mortem closed:** [YYYY-MM-DD]" \
  --confidence 0.85 \
  --tags "post-mortem,action-items,[incident-id]"
```

### Step 4: Conduct Blameless Review

Facilitate blameless discussion focusing on systems and processes:

```bash
engram reasoning create \
  --title "Blameless Post-Mortem Discussion: [Incident]" \
  --task-id [TASK_ID] \
  --content "## Blameless Culture Principles

**Core tenets:**
1. Focus on systems and processes, not individuals
2. Assume everyone did their best with information available at the time
3. Seek to understand, not to judge
4. Learn from failures to prevent future incidents
5. Psychological safety - no punishment for speaking up

**Not blameless:** \"Who broke production?\"
**Blameless:** \"What system allowed this to reach production?\"

## Post-Mortem Discussion

**Date:** [YYYY-MM-DD]
**Attendees:** [Names]
**Facilitator:** [Name - neutral party, not incident responder]
**Duration:** [N] minutes

### Discussion Questions

**What happened?**
- [Factual description of incident]
- [Timeline of events]

**What were the contributing factors?**
- [System factors]
- [Process factors]
- [Environmental factors]

**What surprised us?**
- [Unexpected behaviors or outcomes]
- [Assumptions that proved incorrect]

**Where were we lucky?**
- [Things that could have made incident worse but didn't]
- [Near-misses]

**What questions do we have?**
- [Unanswered questions]
- [Areas needing more investigation]

## Psychological Safety Practices

**Avoid:**
- ❌ \"Why didn't you check X?\"
- ❌ \"You should have known better\"
- ❌ \"This was obviously wrong\"
- ❌ \"Who made this change?\"
- ❌ Naming individuals in negative context

**Encourage:**
- ✅ \"What information would have helped make a different decision?\"
- ✅ \"What could the system do to prevent this?\"
- ✅ \"How can we make the right thing easier to do?\"
- ✅ \"What did we learn?\"
- ✅ Focusing on actions, not actors

## Key Insights from Discussion

**Insight 1: [Theme]**
- Discussion: [Summary of what team discovered]
- Quote: \"[Memorable quote from discussion]\"
- Implication: [What this means for future work]

**Insight 2: [Another theme]**
- [Same structure]

**Insight 3: [Another theme]**
- [Same structure]

## Systemic Improvements Identified

Looking at the bigger picture:

**1. Testing practices need improvement**
- Current state: Optional error path testing, no enforcement
- Ideal state: Required error path coverage, enforced in CI
- Gap: No process to ensure quality
- Action: [Reference to action item]

**2. [Another systemic improvement]**
- [Same structure]

## Action Item Agreement

Team agreed on action items:
- P0 items: [List]
- P1 items: [List]
- P2 items: [List]

**Commitment:** [Team/organization commitment to completing action items]

## Incident Report Publication

**Internal report:**
- Audience: Engineering team
- Content: Full technical details, root cause analysis, action items
- Published: [Link to internal doc]

**External report (if customer-facing incident):**
- Audience: Customers
- Content: What happened, impact, resolution, prevention steps
- Tone: Transparent, honest, accountable (without assigning individual blame)
- Published: [Link to status page post-mortem]

**Example customer-facing summary:**
\"On [date], our payment processing service experienced an outage lasting [N] hours, affecting [N%] of transactions. The root cause was a connection pool exhaustion triggered by a code change that didn't properly release database connections under certain error conditions. We've fixed the immediate issue and are implementing additional monitoring and testing requirements to prevent similar incidents in the future.\"

## Lessons for Organization

**Knowledge captured:**
1. [Lesson learned applicable beyond this incident]
2. [Another transferable lesson]
3. [Another lesson]

**Knowledge sharing:**
- Present at engineering all-hands: [Date]
- Added to incident response training: [Date]
- Case study for new engineer onboarding: [Date]

## Follow-up

**Action item review:** Weekly until all P0/P1 complete
**Next review date:** [YYYY-MM-DD]
**Post-mortem retrospective:** [Date - review effectiveness of post-mortem process itself]" \
  --confidence 0.80 \
  --tags "post-mortem,blameless,discussion,[incident-id]"
```

### Step 5: Link Post-Mortem Entities to Task

```bash
# Link post-mortem analysis
engram relationship create \
  --source-id [TASK_ID] --source-type task \
  --target-id [INCIDENT_ID] --target-type context \
  --relationship-type references \
  --agent [AGENT]

engram relationship create \
  --source-id [TASK_ID] --source-type task \
  --target-id [ROOT_CAUSE_ID] --target-type reasoning \
  --relationship-type documents \
  --agent [AGENT]

engram relationship create \
  --source-id [TASK_ID] --source-type task \
  --target-id [ACTION_ITEMS_ID] --target-type reasoning \
  --relationship-type documents \
  --agent [AGENT]

engram relationship create \
  --source-id [TASK_ID] --source-type task \
  --target-id [DISCUSSION_ID] --target-type reasoning \
  --relationship-type documents \
  --agent [AGENT]
```

## Example

Production database runs out of connections, causing 2-hour outage affecting all users.

### Step 1: Reconstruct Timeline

Agent gathers data from logs, alerts, Slack:

"Timeline reconstructed from incident channel and monitoring:
- 14:23 UTC: Connection leak introduced in deploy
- 15:47 UTC: First monitoring alert (high latency)
- 15:52 UTC: Service completely down (connection pool exhausted)
- 15:53 UTC: On-call engineer paged
- 16:15 UTC: Root cause identified (connection leak)
- 16:45 UTC: Rollback deployed
- 17:05 UTC: Service fully restored

Total impact: 1h 13min full outage, 5000 users affected, $12K revenue loss."

### Step 2: Root Cause Analysis

Agent conducts Five Whys:

"Five Whys analysis:
1. Why did service go down? → Database connection pool exhausted
2. Why exhausted? → Connections not released after queries
3. Why not released? → Bug in error handling code path
4. Why did bug reach production? → Not caught in code review or tests
5. Why not caught? → No test coverage requirement for error paths

Root cause: Lack of enforced test coverage policy allowed buggy code to reach production.

Contributing factors: Missing monitoring, no runbook, Friday deployment."

### Step 3: Create Action Items

Agent proposes 11 action items:

"Action items created:
- P0 (3 items): Fix bug, add monitoring, increase pool size - Due: 3 days
- P1 (3 items): Add timeout, create runbook, improve test coverage - Due: 2 weeks
- P2 (3 items): Enforce coverage in CI, update review checklist, conduct drill - Due: 1 month
- P3 (2 items): Monitoring audit, staging environment - Due: 3 months

All items assigned owners and tracked in Engram."

### Step 4: Facilitate Blameless Discussion

Agent facilitates discussion with team:

"Blameless post-mortem discussion held with 8 engineers. Key insights:
- Testing culture focuses on happy path, team didn't realize error paths not covered
- Code reviewer not trained to check error handling
- No one felt comfortable deploying on Friday but no policy against it

Systemic improvements identified: Testing standards, reviewer training, deployment windows.

Team committed to all action items. Post-mortem report published internally and customer-facing summary on status page."

## Querying Post-Mortems

```bash
# Get incident post-mortems
engram context list | grep "Incident Post-Mortem:"

# Get root cause analyses
engram reasoning list | grep "Root Cause Analysis:"

# Get action items
engram reasoning list | grep "Post-Mortem Action Items:"

# Get blameless discussions
engram reasoning list | grep "Blameless Post-Mortem Discussion:"

# Get all post-mortem work for an incident
engram relationship connected --entity-id [TASK_ID] | grep -i "post-mortem"

# Find recurring incident patterns
engram context list | grep "Incident Post-Mortem:" | grep "Database"
```

## Related Skills

This skill integrates with:
- `engram-risk-assessment` - Incidents reveal risks not previously assessed
- `engram-security-review` - Security incidents require post-mortem analysis
- `engram-retrospective` - Similar techniques for team process improvement
- `engram-assumption-validation` - Incidents often reveal invalidated assumptions
- `engram-system-design` - Post-mortems inform future system design decisions
