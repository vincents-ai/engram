---
name: engram-tech-debt
description: "Identify, quantify, prioritize technical debt, track debt ratio, and plan incremental paydown strategies."
---

# Technical Debt Management (Engram-Integrated)

## Overview

Systematically identify, quantify, and prioritize technical debt. Track debt ratio over time to prevent accumulation. Plan incremental paydown strategies that balance new features with code health. Store debt inventory, assessments, and paydown plans in Engram for visibility and accountability.

## When to Use

Use this skill when:
- Codebase feels increasingly difficult to change
- Bug rate is increasing
- Velocity is slowing down
- Planning quarterly or annual roadmaps
- After rapid prototyping or MVP development
- Before major refactoring efforts
- Onboarding new team members (document known issues)
- Evaluating whether to rewrite vs refactor
- Need to justify refactoring time to stakeholders

## The Pattern

### Step 1: Identify Technical Debt

Create inventory of debt:

```bash
engram context create \
  --title "Technical Debt Inventory: [System/Codebase]" \
  --content "## System Overview\n\n**Codebase:** [Name]\n**Size:** [Lines of code, number of services]\n**Age:** [Years in development]\n**Team Size:** [Number of developers]\n\n## Debt Categories\n\n### Code Debt\n\n**Definition:** Code that works but is hard to maintain\n\n**Examples:**\n1. **Complex function**: `processOrder()` in orders.js:234 (500 lines, cyclomatic complexity: 45)\n   - Issue: Too many responsibilities, hard to test\n   - Impact: Takes 2 hours to add new payment method\n   - Effort to fix: 2 days (break into smaller functions)\n\n2. **Duplicated code**: User validation duplicated in 5 places\n   - Files: users.js:12, orders.js:45, admin.js:78, api.js:123, webhooks.js:234\n   - Issue: Changes require updating 5 places, risk of inconsistency\n   - Impact: Bug fix took 3 hours instead of 30 minutes\n   - Effort to fix: 4 hours (extract to shared validator module)\n\n3. **God object**: `Application` class has 50 methods, 3000 lines\n   - Issue: Violates single responsibility, everything depends on it\n   - Impact: Hard to test, changes risk breaking everything\n   - Effort to fix: 2 weeks (split into multiple classes)\n\n### Design Debt\n\n**Definition:** Architecture doesn't support current or future needs\n\n**Examples:**\n1. **Monolith**: All features in single service\n   - Issue: Can't scale components independently\n   - Impact: Memory-intensive report generation slows API responses\n   - Effort to fix: 6 weeks (extract report service)\n\n2. **Tight coupling**: Payment module directly calls 10 other modules\n   - Issue: Changes to payment affect many modules\n   - Impact: Can't replace payment provider without rewriting integrations\n   - Effort to fix: 3 weeks (introduce payment abstraction layer)\n\n3. **No caching**: Database hit on every request\n   - Issue: High latency, expensive database queries\n   - Impact: p95 latency is 800ms, database is bottleneck\n   - Effort to fix: 1 week (add Redis cache layer)\n\n### Test Debt\n\n**Definition:** Inadequate or missing tests\n\n**Examples:**\n1. **Low coverage**: Only 40% test coverage\n   - Issue: Fear of breaking things when changing code\n   - Impact: Bugs slip to production, slow development\n   - Effort to fix: Ongoing (add tests with each feature)\n\n2. **Flaky tests**: 5 tests fail randomly\n   - Issue: Can't trust CI/CD, must re-run multiple times\n   - Impact: Deployment delayed by test failures\n   - Effort to fix: 2 days (fix race conditions, remove timeouts)\n\n3. **No integration tests**: Only unit tests exist\n   - Issue: Services work individually but fail when integrated\n   - Impact: Integration bugs found in production\n   - Effort to fix: 1 week (add integration test suite)\n\n### Documentation Debt\n\n**Definition:** Missing or outdated documentation\n\n**Examples:**\n1. **No API docs**: Endpoints not documented\n   - Issue: Users struggle to integrate\n   - Impact: 30% of support tickets are API questions\n   - Effort to fix: 3 days (generate from OpenAPI spec)\n\n2. **Outdated architecture diagram**: Diagram from 2 years ago\n   - Issue: New team members confused about system design\n   - Impact: Onboarding takes 2 weeks instead of 3 days\n   - Effort to fix: 1 day (update diagram)\n\n3. **No runbooks**: No incident response procedures\n   - Issue: Incidents take longer to resolve\n   - Impact: Mean time to recovery (MTTR) is 4 hours\n   - Effort to fix: 1 week (write runbooks for common incidents)\n\n### Infrastructure Debt\n\n**Definition:** Outdated or suboptimal infrastructure\n\n**Examples:**\n1. **Manual deployments**: Deployment requires 15 manual steps\n   - Issue: Error-prone, slow, requires specific person\n   - Impact: Can only deploy during business hours\n   - Effort to fix: 2 weeks (automate deployment pipeline)\n\n2. **Old dependencies**: Using Node.js 12 (EOL)\n   - Issue: Security vulnerabilities, missing features\n   - Impact: Can't use latest libraries\n   - Effort to fix: 1 week (upgrade to Node.js 20)\n\n3. **No monitoring**: Basic metrics only, no traces\n   - Issue: Hard to debug production issues\n   - Impact: Takes hours to find root cause\n   - Effort to fix: 2 weeks (add OpenTelemetry tracing)\n\n### Security Debt\n\n**Definition:** Known security issues or gaps\n\n**Examples:**\n1. **Unpatched CVEs**: 12 high-severity CVE alerts from Dependabot\n   - Issue: Known vulnerabilities in dependencies\n   - Impact: Risk of exploit\n   - Effort to fix: 3 days (update dependencies, test)\n\n2. **No audit logs**: Admin actions not logged\n   - Issue: Can't track who did what\n   - Impact: Compliance violation, can't investigate incidents\n   - Effort to fix: 1 week (add audit logging)\n\n3. **Secrets in code**: API keys hardcoded in 3 files\n   - Issue: Keys visible in git history\n   - Impact: Security breach if repo leaked\n   - Effort to fix: 2 days (move to environment variables, rotate keys)\n\n## Summary Statistics\n\n**Total Debt Items:** 18\n**By Category:**\n- Code Debt: 3 items\n- Design Debt: 3 items\n- Test Debt: 3 items\n- Documentation Debt: 3 items\n- Infrastructure Debt: 3 items\n- Security Debt: 3 items\n\n**Total Estimated Effort:** 26 weeks\n**Critical Items:** 4 (security, flaky tests, manual deploys, no caching)" \
  --source "tech-debt" \
  --tags "tech-debt,inventory,[system]"
```

### Step 2: Quantify Technical Debt

Measure debt metrics:

```bash
engram reasoning create \
  --title "Technical Debt Quantification: [System]" \
  --task-id [TASK_ID] \
  --content "## Quantification Approach\n\n**Why quantify?**\n- Make debt visible to stakeholders\n- Prioritize paydown efforts\n- Track progress over time\n- Justify refactoring investment\n\n## Metrics\n\n### 1. Debt Ratio\n\n**Formula:** `Debt Ratio = Cost to Fix / Total Codebase Value`\n\n**Calculation:**\n- Cost to fix all debt: 26 weeks\n- Current development capacity: 4 developers × 2 weeks/sprint = 8 dev-weeks/sprint\n- Debt as percentage of capacity: 26 / 8 = 3.25 sprints worth of debt\n\n**Interpretation:**\n- < 5%: Healthy (< 1 sprint of debt)\n- 5-10%: Manageable (1-2 sprints)\n- 10-20%: High (2-4 sprints)\n- > 20%: Critical (> 4 sprints)\n\n**Our Debt Ratio:** 16% (High - need to address)\n\n### 2. Interest Rate\n\n**Definition:** Extra time spent due to debt\n\n**Measurement:**\n- Feature A without debt: 2 days\n- Feature A with current debt: 4 days\n- Interest rate: (4-2)/2 = 100% (debt doubles development time)\n\n**Examples:**\n- Complex function: Adds 2 hours to every payment feature\n- Duplicated code: Adds 30 min to every validation change\n- No caching: Requires performance optimization for every feature\n\n**Total Interest per Sprint:**\n- 5 features/sprint × 2 days extra/feature = 10 days wasted\n- Capacity: 8 dev-weeks = 40 days\n- Interest rate: 10/40 = 25% of time wasted on debt\n\n### 3. Code Quality Metrics\n\n**Cyclomatic Complexity:**\n```bash\n# Measure complexity\nnpx complexity-report src/\n\n# Results:\n# Average complexity: 8 (good: < 10)\n# Max complexity: 45 (processOrder function)\n# Functions > 15: 12 (these are debt)\n```\n\n**Code Duplication:**\n```bash\n# Measure duplication\njscpd src/\n\n# Results:\n# Duplication: 15% of codebase\n# Target: < 5%\n# Debt: 10% excess duplication\n```\n\n**Test Coverage:**\n```bash\n# Measure coverage\nnpm test -- --coverage\n\n# Results:\n# Coverage: 40%\n# Target: > 80%\n# Debt: 40% of code untested\n```\n\n**Dependency Freshness:**\n```bash\n# Check outdated dependencies\nnpm outdated\n\n# Results:\n# Outdated: 32 packages\n# Major versions behind: 8 packages\n# Security vulnerabilities: 12 (5 high, 7 medium)\n```\n\n### 4. Velocity Impact\n\n**Historical Velocity:**\n```\nQ1 2025: 20 story points/sprint\nQ2 2025: 18 story points/sprint (-10%)\nQ3 2025: 15 story points/sprint (-25%)\nQ4 2025: 12 story points/sprint (-40%)\n```\n\n**Trend:** Velocity declining 10% per quarter due to debt\n\n**Projection:** At current rate, velocity will be 8 points/sprint by Q2 2026 (-60%)\n\n### 5. Bug Rate\n\n**Historical Bug Rate:**\n```\nQ1 2025: 5 bugs/sprint\nQ2 2025: 8 bugs/sprint (+60%)\nQ3 2025: 12 bugs/sprint (+140%)\nQ4 2025: 15 bugs/sprint (+200%)\n```\n\n**Correlation:** As debt increases, bug rate increases\n\n**Root Cause Analysis:**\n- 40% of bugs from complex function (processOrder)\n- 30% of bugs from duplicated validation\n- 20% of bugs from tight coupling\n- 10% of bugs from other causes\n\n### 6. MTTR (Mean Time to Repair)\n\n**Historical MTTR:**\n```\nQ1 2025: 1 hour\nQ2 2025: 2 hours\nQ3 2025: 3 hours\nQ4 2025: 4 hours\n```\n\n**Cause:** Lack of monitoring and documentation makes debugging slow\n\n## Debt Score\n\n**Weighted score for each debt item:**\n\n```\nScore = (Impact × Frequency × Uncertainty) / Effort\n\nWhere:\n- Impact: 1-5 (1=low, 5=critical)\n- Frequency: 1-5 (1=rare, 5=every day)\n- Uncertainty: 1-3 (1=well understood, 3=unknown unknowns)\n- Effort: days to fix\n```\n\n**Example Scores:**\n\n1. **Complex processOrder function**\n   - Impact: 4 (slows development significantly)\n   - Frequency: 5 (touched every sprint)\n   - Uncertainty: 1 (well understood)\n   - Effort: 10 days\n   - Score: (4 × 5 × 1) / 10 = 2.0\n\n2. **Security vulnerabilities**\n   - Impact: 5 (critical security risk)\n   - Frequency: 1 (not actively causing problems yet)\n   - Uncertainty: 2 (unknown if exploited)\n   - Effort: 3 days\n   - Score: (5 × 1 × 2) / 3 = 3.3 (HIGHEST PRIORITY)\n\n3. **No API documentation**\n   - Impact: 3 (slows integrations)\n   - Frequency: 2 (occasional)\n   - Uncertainty: 1 (straightforward)\n   - Effort: 3 days\n   - Score: (3 × 2 × 1) / 3 = 2.0\n\n4. **Manual deployments**\n   - Impact: 4 (blocks frequent deployments)\n   - Frequency: 3 (weekly)\n   - Uncertainty: 1 (well understood)\n   - Effort: 10 days\n   - Score: (4 × 3 × 1) / 10 = 1.2\n\n**Sorted by Score (Highest Priority First):**\n1. Security vulnerabilities: 3.3\n2. Complex processOrder: 2.0\n3. No API docs: 2.0\n4. Manual deployments: 1.2\n5. ...\n\n## Summary\n\n**Current State:**\n- Debt Ratio: 16% (High)\n- Interest Rate: 25% of time wasted\n- Velocity: Declining 10% per quarter\n- Bug Rate: Increasing 60% per quarter\n- MTTR: 4 hours (2x slower than Q1)\n\n**Recommendation:** Allocate 30% of capacity to debt paydown for next 3 sprints\n\n**Expected Outcome:**\n- Reduce debt ratio to 10%\n- Stop velocity decline\n- Reduce bug rate by 50%\n- Reduce MTTR to 2 hours\n\n**Confidence:** [0.0-1.0]" \
  --confidence [0.0-1.0] \
  --tags "tech-debt,quantification,[system]"
```

### Step 3: Prioritize Debt Paydown

Create paydown strategy:

```bash
engram reasoning create \
  --title "Debt Paydown Strategy: [System]" \
  --task-id [TASK_ID] \
  --content "## Prioritization Framework\n\n### High Priority (Do First)\n\n**Criteria:**\n- High impact on velocity or quality\n- Frequently encountered (daily/weekly)\n- Relatively quick to fix (< 1 week)\n- Blocks other improvements\n- Security risk\n\n**Items:**\n\n1. **Security vulnerabilities** (Score: 3.3)\n   - Why: Critical security risk, quick fix\n   - Effort: 3 days\n   - Impact: Eliminate security risk\n   - When: Sprint 1, Week 1\n\n2. **Fix flaky tests** (Score: 2.8)\n   - Why: Blocks reliable CI/CD, wastes time re-running tests\n   - Effort: 2 days\n   - Impact: Reliable deployments, faster feedback\n   - When: Sprint 1, Week 1\n\n3. **Add caching layer** (Score: 2.5)\n   - Why: Every request hits database, affects all users\n   - Effort: 5 days\n   - Impact: Reduce p95 latency from 800ms to 200ms\n   - When: Sprint 1, Week 2\n\n### Medium Priority (Do Soon)\n\n**Criteria:**\n- Moderate impact\n- Occasional (monthly)\n- Medium effort (1-2 weeks)\n- Compounds over time\n\n**Items:**\n\n4. **Refactor processOrder function** (Score: 2.0)\n   - Why: Touched every sprint, hard to test\n   - Effort: 10 days\n   - Impact: Faster development of payment features\n   - When: Sprint 2\n\n5. **Extract duplicated validation** (Score: 1.8)\n   - Why: Changes require updating 5 places\n   - Effort: 4 days\n   - Impact: Reduce bug rate, faster changes\n   - When: Sprint 2\n\n6. **Automate deployments** (Score: 1.2)\n   - Why: Enables frequent deployments, reduces errors\n   - Effort: 10 days\n   - Impact: Deploy anytime, not just business hours\n   - When: Sprint 3\n\n### Low Priority (Do Later)\n\n**Criteria:**\n- Low impact\n- Rare (quarterly or less)\n- Large effort (> 2 weeks)\n- Nice to have but not blocking\n\n**Items:**\n\n7. **Update architecture diagram** (Score: 0.8)\n   - Why: Helps onboarding but not urgent\n   - Effort: 1 day\n   - Impact: Faster onboarding\n   - When: Sprint 4\n\n8. **Split monolith into services** (Score: 0.5)\n   - Why: Large effort, not immediately blocking\n   - Effort: 30 days\n   - Impact: Independent scaling, better organization\n   - When: Q2 2026 (after higher priority items)\n\n### Don't Fix (Accept as Debt)\n\n**Criteria:**\n- Very low impact\n- Code rarely touched\n- Effort >> benefit\n- Scheduled for deprecation\n\n**Items:**\n\n9. **Legacy admin UI** (Score: 0.2)\n   - Why: Used by 2 people, being replaced next quarter\n   - Effort: 15 days\n   - Impact: Minimal\n   - Decision: Don't fix, replace instead\n\n## Paydown Allocation\n\n**Available Capacity:**\n- 4 developers × 2 weeks/sprint = 8 dev-weeks/sprint = 40 days\n\n**Allocation Strategy:**\n\n**Option 1: Big Bang (Not Recommended)**\n- Sprint 1-3: 100% debt paydown, 0% features\n- Pros: Debt cleared quickly\n- Cons: No new value delivered, stakeholders unhappy\n\n**Option 2: Balanced (Recommended)**\n- 30% capacity to debt: 12 days/sprint\n- 70% capacity to features: 28 days/sprint\n- Pros: Continuous delivery, manageable debt reduction\n- Cons: Takes longer to clear debt\n\n**Option 3: Opportunistic**\n- Fix debt when working in that area\n- \"Boy Scout Rule\": Leave code better than you found it\n- Pros: No dedicated time needed\n- Cons: Slow progress, may not address worst debt\n\n**Selected: Option 2 (Balanced) with 30% allocation**\n\n## 3-Sprint Paydown Plan\n\n### Sprint 1 (12 days for debt)\n\n**Week 1:**\n- Security vulnerabilities: 3 days\n- Flaky tests: 2 days\n\n**Week 2:**\n- Caching layer: 5 days\n- Buffer: 2 days\n\n**Outcome:**\n- ✓ Security risk eliminated\n- ✓ Reliable CI/CD\n- ✓ 75% latency reduction\n\n### Sprint 2 (12 days for debt)\n\n**Week 1:**\n- Refactor processOrder: 5 days\n\n**Week 2:**\n- Refactor processOrder: 5 days\n- Extract validation: 2 days\n\n**Outcome:**\n- ✓ Payment features 2x faster to develop\n- ✓ Validation consistent across codebase\n\n### Sprint 3 (12 days for debt)\n\n**Week 1-2:**\n- Automate deployments: 10 days\n- Buffer: 2 days\n\n**Outcome:**\n- ✓ Deploy anytime, not just business hours\n- ✓ Deployment time: 30 min → 5 min\n\n## Expected Results (After 3 Sprints)\n\n**Debt Metrics:**\n- Debt ratio: 16% → 10% (reduced by 38%)\n- Interest rate: 25% → 15% (reduced by 40%)\n- Debt items: 18 → 12 (reduced by 33%)\n\n**Velocity:**\n- Current: 12 points/sprint\n- Expected: 16 points/sprint (+33%)\n- Rationale: Caching + refactoring + validation = faster development\n\n**Bug Rate:**\n- Current: 15 bugs/sprint\n- Expected: 9 bugs/sprint (-40%)\n- Rationale: Better code structure + tests\n\n**MTTR:**\n- Current: 4 hours\n- Expected: 2 hours (-50%)\n- Rationale: Caching reduces database issues, better monitoring\n\n## Tracking Progress\n\n**Weekly:**\n- Review debt items completed\n- Measure velocity and bug rate\n- Adjust allocation if needed\n\n**Monthly:**\n- Recalculate debt ratio\n- Update debt inventory\n- Report to stakeholders\n\n**Quarterly:**\n- Reassess priorities\n- Plan next quarter's debt work\n- Celebrate improvements\n\n**Dashboards:**\n- Debt ratio over time\n- Velocity trend\n- Bug rate trend\n- Code quality metrics\n\n**Confidence:** [0.0-1.0]" \
  --confidence [0.0-1.0] \
  --tags "tech-debt,prioritization,paydown,[system]"
```

### Step 4: Implement Debt Paydown

Track paydown execution:

```bash
engram task create \
  --title "Debt Paydown Sprint 1: Security + Performance" \
  --description "**Goal:** Fix critical security vulnerabilities and add caching\n\n**Items:**\n1. Update 12 vulnerable dependencies (3 days)\n2. Fix flaky tests (2 days)\n3. Add Redis caching layer (5 days)\n\n**Success Criteria:**\n- Zero high-severity CVEs\n- All tests pass consistently (3 runs in a row)\n- p95 latency < 200ms\n\n**Timeline:** Sprint 1 (Jan 24 - Feb 7)\n**Assigned:** @team" \
  --priority high \
  --tags tech-debt,sprint-1
```

### Step 5: Link Debt Analysis to System

```bash
# Link all debt docs to system task
engram relationship create \
  --source-id [SYSTEM_TASK_ID] --source-type task \
  --target-id [INVENTORY_ID] --target-type context \
  --relationship-type documents --agent default

engram relationship create \
  --source-id [SYSTEM_TASK_ID] --source-type task \
  --target-id [QUANTIFICATION_ID] --target-type reasoning \
  --relationship-type documents --agent default

engram relationship create \
  --source-id [SYSTEM_TASK_ID] --source-type task \
  --target-id [STRATEGY_ID] --target-type reasoning \
  --relationship-type documents --agent default

engram relationship create \
  --source-id [SYSTEM_TASK_ID] --source-type task \
  --target-id [PAYDOWN_TASK_ID] --target-type task \
  --relationship-type depends_on --agent default
```

## Example

Manage technical debt for payment service.

### Step 1: Identify

```bash
INVENTORY=$(engram context create \
  --title "Technical Debt Inventory: Payment Service" \
  --content "Code Debt:\n1. PaymentProcessor class: 800 lines, complexity 35\n2. Retry logic duplicated in 4 places\n\nDesign Debt:\n1. Tight coupling to Stripe (can't switch providers)\n2. Synchronous processing blocks requests\n\nTest Debt:\n1. No integration tests with Stripe API\n2. Test coverage: 55%\n\nSecurity Debt:\n1. stripe-node package: 2 major versions behind\n2. API keys logged in error messages\n\nTotal: 8 items, 18 days effort" \
  --source "tech-debt" \
  --tags "tech-debt,inventory,payment-service" \
  --json | jq -r '.id')
```

### Step 2: Quantify

```bash
QUANT=$(engram reasoning create \
  --title "Technical Debt Quantification: Payment Service" \
  --task-id payment-service-123 \
  --content "Debt Ratio: 18 days / 40 days capacity = 45% (CRITICAL)\n\nInterest Rate:\n- Adding new payment method: 5 days (should be 2 days)\n- Interest: 3 days wasted per payment feature\n\nVelocity Impact:\n- Payment features: 2 per sprint → 1 per sprint (-50%)\n\nScores:\n1. Security (API keys): 5.0 (URGENT)\n2. Tight coupling: 3.2\n3. PaymentProcessor refactor: 2.8\n\nRecommendation: 50% allocation to debt for 2 sprints" \
  --confidence 0.85 \
  --tags "tech-debt,quantification,payment-service" \
  --json | jq -r '.id')
```

### Step 3: Prioritize

```bash
STRATEGY=$(engram reasoning create \
  --title "Debt Paydown Strategy: Payment Service" \
  --task-id payment-service-123 \
  --content "Sprint 1 (20 days for debt):\n- Fix API key logging: 1 day\n- Update stripe-node: 2 days\n- Add payment abstraction layer: 12 days\n- Add integration tests: 5 days\n\nSprint 2 (20 days for debt):\n- Refactor PaymentProcessor: 10 days\n- Extract retry logic: 3 days\n- Improve test coverage to 80%: 7 days\n\nExpected: Debt ratio 45% → 15%, velocity +50%" \
  --confidence 0.85 \
  --tags "tech-debt,strategy,payment-service" \
  --json | jq -r '.id')
```

### Step 4: Track

```bash
# Create tasks for debt paydown
engram task create \
  --title "Payment Service: Debt Paydown Sprint 1" \
  --description "Fix security issues and decouple from Stripe" \
  --priority high \
  --tags tech-debt,payment-service,sprint-1

# Link to debt docs
engram relationship create \
  --source-id payment-service-123 \
  --target-id $INVENTORY \
  --relationship-type documents
```

## Querying Technical Debt

```bash
# Get all debt inventories
engram context list | grep "Technical Debt Inventory"

# Get debt quantifications
engram reasoning list | grep "Technical Debt Quantification"

# Get paydown strategies
engram reasoning list | grep "Debt Paydown Strategy"

# Find high-priority debt items
engram context list | grep -i "critical"

# Track debt tasks
engram task list --tags tech-debt --status in_progress
```

## Related Skills

This skill integrates with:
- `engram-requesting-code-review` - Identify debt during code review
- `engram-refactoring-strategy` - Plan debt paydown refactoring
- `engram-risk-assessment` - Assess risks of accumulated debt
- `engram-system-design` - Design to minimize future debt
- `engram-capacity-planning` - Allocate capacity for debt paydown
