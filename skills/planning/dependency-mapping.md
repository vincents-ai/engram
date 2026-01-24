---
name: engram-dependency-mapping
description: "Map technical and organizational dependencies to identify critical paths, blockers, and coordination needs."
---

# Dependency Mapping (Engram-Integrated)

## Overview

Systematically identify and map technical, organizational, and external dependencies before implementation. Store dependency graphs in Engram as context and reasoning entities to track critical paths, coordinate work, and surface blockers early.

## When to Use

Use this skill when:
- Starting work that touches multiple systems or teams
- Planning parallel work streams that may have hidden dependencies
- Estimating timelines and need to identify critical path
- Coordinating with other teams or external vendors
- A feature requires changes in upstream or downstream systems
- You need to answer "what's blocking us?" or "who do we need?"

## The Pattern

### Step 1: Identify Dependency Categories

Examine these dependency types:

**Technical Dependencies:**
- Libraries, frameworks, or platform versions
- APIs or services (internal or external)
- Database schemas or data pipelines
- Shared infrastructure or deployment systems

**Work Dependencies:**
- Tasks that must complete before others can start
- Parallel tasks that converge (integration points)
- Tasks blocked by decisions or approval
- Critical path items that delay everything else

**Organizational Dependencies:**
- Other teams that must deliver features or APIs
- Stakeholders who must review or approve
- External vendors or contractors
- Subject matter experts for domain knowledge

**Data Dependencies:**
- Migration or data backfill required
- Test data or fixtures needed
- Access to production data or logs
- Compliance or privacy review needed

### Step 2: Create Dependency Context

Map out all dependencies in a structured context:

```bash
engram context create \
  --title "Dependency Map: [Feature Name]" \
  --content "## Technical Dependencies\n\n### Required Before Starting\n- [Library X v2.0+]: Provides [capability] - Status: [Available/Blocked/Unknown]\n- [API Y endpoint]: Needed for [purpose] - Owner: [Team/Person]\n\n### Required During Implementation\n- [Service Z]: Integration point - Contact: [Person]\n- [Database migration]: Schema change needed - Coordination: [DBA team]\n\n### Required Before Release\n- [Infrastructure update]: Load balancer config - Owner: [DevOps]\n\n## Work Dependencies\n\n### Blocking (must complete first)\n1. [Task A]: [Description] - Status: [Todo/In Progress/Done]\n   - Blocks: [Task B, Task C]\n   - Owner: [Person]\n   - ETA: [Date]\n\n### Parallel (can work simultaneously)\n- [Task D] and [Task E] can proceed in parallel\n- Converge at: [Integration point F]\n\n### Critical Path\n[Task A] → [Task B] → [Task F] → [Release]\nEstimated duration: [N days]\n\n## Organizational Dependencies\n\n### Teams\n- [Backend Team]: Must deliver [API endpoint] by [date]\n- [Mobile Team]: Must update app for [feature] in [version]\n\n### Approvals\n- [Security review]: Required before production deployment\n- [Legal review]: Terms of service update needed\n\n### External\n- [Vendor X]: Provides [service] - SLA: [Y days] - Contact: [email]\n\n## Data Dependencies\n\n### Migrations\n- [User table]: Add [column] - Estimated: [N records] affected\n- [Backfill script]: Populate [data] for existing users\n\n### Access Needed\n- [Production logs]: Debug [issue] - Requires: [Access request process]\n- [Analytics data]: Validate [metric] - Owner: [Data team]\n\n## Risks\n\n### High-Impact Blockers\n- [Dependency X]: If delayed >1 week, blocks release\n- [Team Y]: No backup if key person unavailable\n\n### Mitigation\n- [Start coordination early with Team Y]\n- [Create parallel path using Alternative Z]" \
  --source "dependency-mapping" \
  --tags "dependency,dependency-map,[feature-name]"
```

### Step 3: Create Critical Path Reasoning

Identify the longest dependency chain:

```bash
engram reasoning create \
  --title "Critical Path Analysis: [Feature Name]" \
  --task-id [TASK_ID] \
  --content "**Critical Path:**\n[Task A] → [Task B] → [Task C] → [Task D] → [Release]\n\n**Total Duration:** [N days/weeks]\n\n**Path Breakdown:**\n1. [Task A]: [Duration] - [Description]\n   - Dependencies: None (can start immediately)\n   - Owner: [Person/Team]\n\n2. [Task B]: [Duration] - [Description]\n   - Dependencies: Task A must complete\n   - Owner: [Person/Team]\n\n3. [Task C]: [Duration] - [Description]\n   - Dependencies: Task B must complete\n   - Owner: [Person/Team]\n\n4. [Task D]: [Duration] - [Description]\n   - Dependencies: Task C must complete\n   - Owner: [Person/Team]\n\n**Bottlenecks:**\n- [Task B]: Single-threaded, cannot parallelize\n- [External review]: Outside our control, may slip\n\n**Opportunities to Reduce:**\n- [Start Task X early]: Currently waiting for Task B, could start with mock data\n- [Parallelize Task C]: Split into C1 and C2, save [N days]\n\n**Confidence:** [0.0-1.0]\n\n**Assumptions:**\n- [Assumption 1]: [e.g., no unexpected bugs in Task A]\n- [Assumption 2]: [e.g., external team delivers on time]" \
  --confidence [0.0-1.0] \
  --tags "dependency,critical-path,[feature-name]"
```

### Step 4: Create Blocker Tracking

For each known blocker, create reasoning entity:

```bash
engram reasoning create \
  --title "Blocker: [Short Description]" \
  --task-id [TASK_ID] \
  --content "**Status:** [Active/Resolved/Watching]\n\n**Description:**\n[What is blocking progress]\n\n**Blocking:**\n- [Task A]\n- [Task B]\n\n**Owner:** [Who is responsible for unblocking]\n**Contact:** [How to reach owner]\n\n**Action Required:**\n1. [Specific action to unblock]\n2. [Another action]\n\n**ETA:** [When blocker expected to resolve]\n**Actual Resolution:** [When actually resolved - update later]\n\n**Impact if Delayed:**\n- Slips release by [N days]\n- Forces [alternative approach]\n\n**Mitigation:**\n[What we can do while blocked or if blocker delays]" \
  --confidence 1.0 \
  --tags "dependency,blocker,[status],[feature-name]"
```

### Step 5: Link Dependencies to Tasks

```bash
# Link dependency map to parent task
engram relationship create \
  --source-id [TASK_ID] --source-type task \
  --target-id [DEPENDENCY_CONTEXT_ID] --target-type context \
  --relationship-type references \
  --agent [AGENT]

# Link critical path analysis
engram relationship create \
  --source-id [TASK_ID] --source-type task \
  --target-id [CRITICAL_PATH_ID] --target-type reasoning \
  --relationship-type documents \
  --agent [AGENT]

# Link blockers
engram relationship create \
  --source-id [TASK_ID] --source-type task \
  --target-id [BLOCKER_ID] --target-type reasoning \
  --relationship-type documents \
  --agent [AGENT]
```

### Step 6: Update Dependencies During Work

As work progresses, update dependency status:

```bash
# When blocker resolves
engram reasoning create \
  --title "Blocker Resolved: [Short Description]" \
  --task-id [TASK_ID] \
  --content "**Original Blocker:** [Reference to original blocker entity]\n\n**Resolved:** [Date/Time]\n**Resolution:** [What unblocked it]\n\n**Impact:**\n- Unblocked: [Task A, Task B]\n- Timeline: [Still on track / Delayed by N days]\n\n**Lessons Learned:**\n[What we learned about coordination or dependencies]" \
  --confidence 1.0 \
  --tags "dependency,blocker,resolved,[feature-name]"

# When new dependency discovered
engram reasoning create \
  --title "New Dependency Discovered: [Description]" \
  --task-id [TASK_ID] \
  --content "**Discovered:** [Date]\n**During:** [What task revealed this]\n\n**Dependency:**\n[What we now depend on]\n\n**Impact:**\n- Blocks: [Task X]\n- Adds: [N days] to timeline\n\n**Action Plan:**\n1. [How to handle new dependency]\n\n**Why Missed Initially:**\n[Reflection on why this wasn't caught in original mapping]" \
  --confidence 1.0 \
  --tags "dependency,discovered,[feature-name]"
```

## Example

User wants to implement payment processing integration with Stripe.

### Step 1: Map Dependencies

```bash
DEPMAP=$(engram context create \
  --title "Dependency Map: Stripe Payment Integration" \
  --content "## Technical Dependencies\n\n### Required Before Starting\n- Stripe SDK v12.0+: Payment processing library - Status: Available (v12.3 in package.json)\n- Stripe test account: For development - Status: Available (keys in vault)\n\n### Required During Implementation\n- Stripe webhook endpoint: Receive payment events - Owner: Backend team\n- Database schema: Add payment_transactions table - Coordination: DBA review\n- TLS certificate: Webhook endpoint requires HTTPS - Owner: DevOps\n\n### Required Before Release\n- Stripe production account: Live payment processing - Owner: Finance team approval needed\n- PCI compliance review: Required for payment handling - Contact: Security team\n- Rate limiting: Protect webhook endpoint - Owner: Backend team\n\n## Work Dependencies\n\n### Blocking (must complete first)\n1. Database migration: Add payment_transactions table - Status: Todo\n   - Blocks: Backend API, webhook handler\n   - Owner: Backend team\n   - ETA: 2 days\n\n2. Stripe production account setup: Get live API keys - Status: Blocked\n   - Blocks: Production deployment\n   - Owner: Finance team\n   - ETA: 5-7 business days (approval process)\n\n### Parallel (can work simultaneously)\n- Frontend payment form UI (3 days)\n- Backend API endpoint (2 days after migration)\n- Webhook handler (2 days after migration)\n- Converge at: Integration testing\n\n### Critical Path\nDatabase migration (2d) → Backend API (2d) → Integration testing (1d) → PCI review (3d) → Stripe prod account (7d) → Production deployment\nEstimated duration: 15 days\n\n## Organizational Dependencies\n\n### Teams\n- Finance Team: Must approve Stripe production account - Lead: Jane Smith (jane@company.com)\n- Security Team: Must complete PCI compliance review - Lead: Bob Wilson (bob@company.com)\n- Legal Team: Review terms of service for payment processing - Lead: Alice Chen (alice@company.com)\n\n### Approvals\n- PCI compliance: Required before production deployment (3-5 days)\n- Legal review: Terms update needed (1 week)\n- Finance approval: Budget and account setup (5-7 days)\n\n### External\n- Stripe Support: Questions about webhook retry logic - SLA: 24 hours - support@stripe.com\n- Bank: Confirm settlement account details - Contact: Corporate banking rep\n\n## Data Dependencies\n\n### Migrations\n- payment_transactions table: Store payment records - Estimated: New table, no backfill\n- user_billing_info table: Add stripe_customer_id column - Estimated: 50K users, backfill on first payment\n\n### Access Needed\n- Stripe test mode: Available (keys in vault)\n- Stripe production: Blocked on finance approval\n- PCI compliance docs: Request from security team\n\n## Risks\n\n### High-Impact Blockers\n- Stripe production account: If finance approval delayed >1 week, slips release\n- PCI compliance: If issues found, could require architecture changes\n\n### Mitigation\n- Start finance approval process immediately (day 1)\n- Engage security team early for PCI review (day 3)\n- Build with test mode, swap keys for production (no code changes)\n- Document PCI compliance approach in design doc for security review" \
  --source "dependency-mapping" \
  --tags "dependency,dependency-map,stripe-payments" \
  --json | jq -r '.id')

echo "Dependency map created: $DEPMAP"
```

### Step 2: Analyze Critical Path

```bash
CRITICAL=$(engram reasoning create \
  --title "Critical Path Analysis: Stripe Payment Integration" \
  --task-id a3f8b2c1-1234-5678-90ab-cdef12345678 \
  --content "**Critical Path:**\nDatabase migration → Backend API → Integration testing → PCI review → Stripe prod account → Production deployment\n\n**Total Duration:** 15 days\n\n**Path Breakdown:**\n1. Database migration: 2 days - Add payment_transactions and update user_billing_info\n   - Dependencies: None (can start immediately)\n   - Owner: Backend team\n\n2. Backend API: 2 days - Implement payment processing endpoints\n   - Dependencies: Database migration must complete\n   - Owner: Backend team\n\n3. Integration testing: 1 day - End-to-end payment flow with test mode\n   - Dependencies: Backend API and Frontend UI must complete\n   - Owner: QA + Backend team\n\n4. PCI compliance review: 3 days - Security team audit\n   - Dependencies: Integration testing complete, docs ready\n   - Owner: Security team\n\n5. Stripe production account: 7 days - Finance approval and account setup\n   - Dependencies: None (can start day 1), but blocks production deployment\n   - Owner: Finance team\n\n6. Production deployment: 1 day - Swap API keys and deploy\n   - Dependencies: PCI review passed, prod account ready\n   - Owner: DevOps + Backend team\n\n**Bottlenecks:**\n- Stripe production account (7 days): External approval process, cannot accelerate\n- PCI compliance review (3 days): Security team capacity, may slip if issues found\n\n**Opportunities to Reduce:**\n- Start finance approval process on day 1 (parallel with development): Save 7 days from critical path\n- Frontend UI can start immediately (parallel): No impact on critical path\n- Pre-review with security team before formal PCI review: Catch issues early\n\n**Optimized Timeline:** 8 days (if finance approval starts day 1)\n\n**Confidence:** 0.70\n\n**Assumptions:**\n- Database migration has no unexpected issues\n- PCI review finds no major issues requiring rework\n- Finance approval completes within 7 days (could slip to 10)\n- No bugs found during integration testing that require redesign" \
  --confidence 0.70 \
  --tags "dependency,critical-path,stripe-payments" \
  --json | jq -r '.id')

echo "Critical path analysis: $CRITICAL"
```

### Step 3: Track Key Blocker

```bash
BLOCKER1=$(engram reasoning create \
  --title "Blocker: Stripe Production Account Approval Pending" \
  --task-id a3f8b2c1-1234-5678-90ab-cdef12345678 \
  --content "**Status:** Active\n\n**Description:**\nFinance team must approve Stripe production account before we can process real payments. Approval process requires budget review, bank account verification, and risk assessment.\n\n**Blocking:**\n- Production deployment\n- Live payment processing\n- Revenue generation from paid features\n\n**Owner:** Jane Smith (Finance Team)\n**Contact:** jane@company.com, ext 1234\n\n**Action Required:**\n1. Submit account application with business justification\n2. Provide estimated transaction volume and revenue projections\n3. Confirm bank settlement account details\n4. Wait for approval (typically 5-7 business days)\n\n**ETA:** 7 business days from submission (targeting Jan 31)\n**Actual Resolution:** [Update when approved]\n\n**Impact if Delayed:**\n- Each day delayed slips production launch by 1 day\n- Forces extended testing period (not necessarily bad)\n- Revenue target for Q1 may be at risk if delayed >2 weeks\n\n**Mitigation:**\n- Start approval process on day 1 (today)\n- Build entire feature in Stripe test mode (no code changes to swap keys)\n- Prepare detailed docs for finance review to accelerate approval\n- Identify alternative payment processor if approval blocked (Paddle, Braintree)" \
  --confidence 1.0 \
  --tags "dependency,blocker,active,stripe-payments" \
  --json | jq -r '.id')

echo "Blocker tracked: $BLOCKER1"

BLOCKER2=$(engram reasoning create \
  --title "Blocker: PCI Compliance Review Required" \
  --task-id a3f8b2c1-1234-5678-90ab-cdef12345678 \
  --content "**Status:** Watching (not blocking yet)\n\n**Description:**\nSecurity team must complete PCI compliance review before production deployment. Review checks that we handle payment data correctly (using Stripe Elements, no card data on our servers).\n\n**Blocking:**\n- Production deployment\n\n**Owner:** Bob Wilson (Security Team)\n**Contact:** bob@company.com, ext 5678\n\n**Action Required:**\n1. Submit PCI compliance questionnaire (SAQ-A)\n2. Demonstrate payment flow in staging environment\n3. Document data flow and storage\n4. Receive approval from security team\n\n**ETA:** 3 days (starting after integration testing complete)\n**Actual Resolution:** [Update when passed]\n\n**Impact if Delayed:**\n- Standard review takes 3 days\n- If issues found: +2-5 days for fixes and re-review\n- Major issues could require architecture changes (+1-2 weeks)\n\n**Mitigation:**\n- Pre-review with security team during design phase (today)\n- Use Stripe Elements (client-side tokenization) to minimize PCI scope\n- Document that we never store or process raw card numbers\n- Schedule review time with security team in advance\n- Have alternative implementation ready if current approach rejected" \
  --confidence 1.0 \
  --tags "dependency,blocker,watching,stripe-payments" \
  --json | jq -r '.id')

echo "Blocker tracked: $BLOCKER2"
```

### Step 4: Link Everything

```bash
# Link dependency map to task
engram relationship create \
  --source-id a3f8b2c1-1234-5678-90ab-cdef12345678 --source-type task \
  --target-id $DEPMAP --target-type context \
  --relationship-type references \
  --agent default

# Link critical path analysis
engram relationship create \
  --source-id a3f8b2c1-1234-5678-90ab-cdef12345678 --source-type task \
  --target-id $CRITICAL --target-type reasoning \
  --relationship-type documents \
  --agent default

# Link blockers
engram relationship create \
  --source-id a3f8b2c1-1234-5678-90ab-cdef12345678 --source-type task \
  --target-id $BLOCKER1 --target-type reasoning \
  --relationship-type documents \
  --agent default

engram relationship create \
  --source-id a3f8b2c1-1234-5678-90ab-cdef12345678 --source-type task \
  --target-id $BLOCKER2 --target-type reasoning \
  --relationship-type documents \
  --agent default
```

### Step 5: Communicate to User

Agent presents dependency analysis:

"I've mapped 12 dependencies for Stripe payment integration. **Critical path: 15 days**, but can optimize to **8 days** by starting finance approval today (parallel with development). Two key blockers: finance approval (7 days) and PCI review (3 days). 

**Recommendation:** Start finance approval process today. Build in test mode while waiting for prod account. Pre-review with security team to catch PCI issues early. All dependencies tracked in Engram. Ready to proceed?"

## Querying Dependency Map

After creating dependency map, agents can retrieve information:

```bash
# Get dependency map for a task
engram relationship connected --entity-id [TASK_ID] | grep "Dependency Map"

# Get critical path analysis
engram reasoning list | grep "Critical Path Analysis"

# Get active blockers across all features
engram reasoning list | grep "Blocker:"

# Get resolved blockers (lessons learned)
engram reasoning list | grep "Blocker Resolved"

# Get all dependencies for a feature
engram relationship connected --entity-id [TASK_ID] | grep -E "Blocker|Critical Path"
```

## Related Skills

This skill integrates with:
- `engram-risk-assessment` - Dependency risks feed into risk analysis
- `engram-spike-investigation` - Spike external dependencies to reduce uncertainty
- `engram-brainstorming` - Consider dependencies during design
- `engram-writing-plans` - Incorporate dependencies into task ordering
- `engram-assumption-validation` - Test assumptions about dependency availability
