---
name: engram-backlog-refinement
description: "Break epics into stories, prioritize work, estimate complexity, and link tasks to strategic goals."
---

# Backlog Refinement (Engram-Integrated)

## Overview

Systematically decompose epics into well-defined user stories, estimate complexity, prioritize based on business value and effort, and maintain traceability to strategic objectives. Store refined backlog items, prioritization rationale, and strategic alignment in Engram for transparent planning.

## When to Use

Use this skill when:
- Starting sprint planning with poorly defined stories
- Product backlog has grown unmanageable (>50 items)
- Team unclear on priorities or "what to build next"
- Stakeholders ask "why are we building this?"
- Stories are too large to complete in one sprint
- Need to align engineering work with business OKRs

## The Pattern

### Step 1: Break Epics Into User Stories

Decompose large initiatives into implementable units:

```bash
engram context create \
  --title "Epic Breakdown: [Epic Name]" \
  --content "## Epic\n\n**Title:** [Epic name]\n**Description:** [High-level capability or outcome]\n**Business Goal:** [Why we're building this - link to OKR or strategy]\n**Success Criteria:** [How we measure success]\n\n## User Stories\n\nDecompose epic into stories that:\n- Deliver incremental value\n- Complete in 1 sprint (2 weeks)\n- Have clear acceptance criteria\n- Are independently deployable (when possible)\n\n### Story 1: [Story Title]\n\n**User Story:**\nAs a [user type]\nI want [capability]\nSo that [benefit]\n\n**Acceptance Criteria:**\n- [ ] [Testable criterion 1]\n- [ ] [Testable criterion 2]\n- [ ] [Testable criterion 3]\n\n**Technical Notes:**\n- [Implementation approach]\n- [Dependencies or prerequisites]\n- [Edge cases to consider]\n\n**Complexity Estimate:** [T-shirt size: XS/S/M/L/XL or Story points: 1/2/3/5/8/13]\n**Priority:** [Must-have/Should-have/Nice-to-have]\n**Value:** [High/Medium/Low - business impact]\n**Effort:** [High/Medium/Low - engineering work]\n\n### Story 2: [Story Title]\n\n[Similar format...]\n\n### Story 3: [Story Title]\n\n[Similar format...]\n\n## Story Sequence\n\n**Dependencies:**\nStory 1 → Story 2 → Story 5 (critical path)\nStory 3 and Story 4 can be parallel\n\n**Incremental Delivery:**\n- MVP (Stories 1, 2, 3): Delivers [core value]\n- V1 (+ Stories 4, 5): Adds [enhancement]\n- V2 (+ Stories 6, 7): Completes [full vision]\n\n## Definition of Ready\n\n**Before story enters sprint:**\n- [ ] Acceptance criteria defined\n- [ ] Complexity estimated\n- [ ] Dependencies identified\n- [ ] Design mockups available (if UI)\n- [ ] Technical approach agreed\n- [ ] Team can commit to completing in 1 sprint\n\n## Definition of Done\n\n**Before story marked complete:**\n- [ ] All acceptance criteria met\n- [ ] Code reviewed and merged\n- [ ] Tests written and passing (unit, integration)\n- [ ] Documentation updated\n- [ ] Deployed to staging and validated\n- [ ] Product owner accepted\n\n## Links to Strategic Goals\n\n**Company OKR:** [Q2 2026: Increase enterprise revenue 50%]\n**Key Result:** [Close 50 enterprise deals by June 30]\n**How this epic contributes:** [Enterprise features enable $199/month tier]" \
  --source "backlog-refinement" \
  --tags "backlog,epic-breakdown,[epic-name]"
```

### Step 2: Estimate Complexity

Assign complexity scores using relative sizing:

```bash
engram reasoning create \
  --title "Complexity Estimation: [Story Title]" \
  --task-id [TASK_ID] \
  --content "## Story Summary\n\n**Story:** [Title and description]\n**User:** [Target user type]\n**Value:** [What user gains]\n\n## Complexity Factors\n\n### Technical Complexity\n- **Unknown tech:** [e.g., First time using Elasticsearch - HIGH]\n- **External integrations:** [e.g., Stripe API - MEDIUM]\n- **Performance requirements:** [e.g., <100ms response - HIGH]\n- **Scale considerations:** [e.g., 1M+ users - MEDIUM]\n\n**Score:** [High/Medium/Low]\n\n### Code Complexity\n- **Lines of code estimate:** [~500 lines]\n- **Files touched:** [~8 files across 3 services]\n- **New vs changes:** [80% new code, 20% modifications]\n- **Refactoring needed:** [e.g., Extract auth logic first - adds 5 points]\n\n**Score:** [High/Medium/Low]\n\n### Testing Complexity\n- **Test scenarios:** [~15 unit tests, 5 integration tests, 2 e2e tests]\n- **Edge cases:** [Payment failures, network timeouts, concurrency]\n- **Mock complexity:** [Must mock Stripe API, Redis, PostgreSQL]\n\n**Score:** [High/Medium/Low]\n\n### Uncertainty\n- **Requirements clear:** [Yes/No - details]\n- **Design finalized:** [Yes/No - details]\n- **Dependencies known:** [2 dependencies identified, both ready]\n- **Past experience:** [Team built similar feature last quarter]\n\n**Score:** [High/Medium/Low]\n\n## Relative Sizing\n\n**Comparison to known stories:**\n- Similar to [Story X] which was [8 points]\n- Simpler than [Story Y] which was [13 points]\n- More complex than [Story Z] which was [5 points]\n\n**Adjustment factors:**\n- +[N] points: External integration adds uncertainty\n- -[M] points: Reusing existing patterns\n\n## Final Estimate\n\n**T-shirt size:** [S/M/L]\n**Story points:** [8 points]\n**Confidence:** [0.0-1.0]\n\n**Rationale:**\n[Why this estimate - based on comparison, complexity factors, team velocity]\n\n**Assumptions:**\n- [Assumption 1: e.g., Stripe API works as documented]\n- [Assumption 2: e.g., No major refactoring required]\n- [Assumption 3: e.g., Designer delivers mockups on time]\n\n**Risks:**\n- [Risk 1: API rate limits may require retry logic]\n- [Risk 2: Payment edge cases may be more complex than expected]\n\n**If estimate is wrong:**\n- If takes <5 points: Story was over-estimated, refine future estimates\n- If takes >13 points: Story was under-estimated, consider spike next time" \
  --confidence [0.0-1.0] \
  --tags "backlog,estimation,complexity,[story-name]"
```

### Step 3: Prioritize Using Value/Effort Matrix

Rank stories by business value and engineering effort:

```bash
engram reasoning create \
  --title "Backlog Prioritization: [Release/Sprint]" \
  --task-id [TASK_ID] \
  --content "## Prioritization Framework\n\n**Axes:**\n- **Value:** Business impact (Revenue, User satisfaction, Strategic alignment)\n- **Effort:** Engineering complexity (Story points, Risk, Dependencies)\n\n**Quadrants:**\n1. **High Value, Low Effort** → DO FIRST (Quick wins)\n2. **High Value, High Effort** → DO NEXT (Strategic investments)\n3. **Low Value, Low Effort** → DO LATER (Fill gaps)\n4. **Low Value, High Effort** → DON'T DO (Avoid)\n\n## Story Prioritization\n\n### Quadrant 1: High Value, Low Effort (Quick Wins) - DO FIRST\n\n**Story: Dark Mode UI**\n- **Value:** High (200+ user requests, competitive parity, minimal effort)\n- **Effort:** Low (3 points - CSS changes, feature flag)\n- **ROI:** Very High\n- **Priority:** P0\n- **Rationale:** Highly visible, low risk, fast delivery\n\n**Story: Payment Receipt Email**\n- **Value:** High (Required for payment processing launch, compliance)\n- **Effort:** Low (2 points - email template + trigger)\n- **ROI:** Very High\n- **Priority:** P0\n- **Rationale:** Blocks payment launch, easy to implement\n\n### Quadrant 2: High Value, High Effort (Strategic) - DO NEXT\n\n**Story: Elasticsearch Integration**\n- **Value:** High (Enterprise tier differentiator, $50K+ revenue potential)\n- **Effort:** High (13 points - new infrastructure, learning curve)\n- **ROI:** High (value justifies effort)\n- **Priority:** P0\n- **Rationale:** Core enterprise feature, worth the investment\n\n**Story: WebSocket Real-time Sync**\n- **Value:** High (Improves UX significantly, competitive advantage)\n- **Effort:** High (13 points - new architecture, scaling unknowns)\n- **ROI:** Medium-High\n- **Priority:** P1\n- **Rationale:** Important but not blocking, sequence after search\n\n### Quadrant 3: Low Value, Low Effort (Fill Gaps) - DO LATER\n\n**Story: Export Data to CSV**\n- **Value:** Low (Nice to have, 10 user requests)\n- **Effort:** Low (3 points - straightforward)\n- **ROI:** Low\n- **Priority:** P2\n- **Rationale:** Use to fill sprint capacity if available\n\n**Story: Keyboard Shortcuts**\n- **Value:** Low (Power user feature, minimal impact)\n- **Effort:** Low (5 points - event handlers)\n- **ROI:** Low\n- **Priority:** P3\n- **Rationale:** Defer until higher priorities complete\n\n### Quadrant 4: Low Value, High Effort (Avoid) - DON'T DO\n\n**Story: Custom Theme Engine**\n- **Value:** Low (Only 3 requests, niche feature)\n- **Effort:** High (21 points - complex UI, state management)\n- **ROI:** Negative\n- **Priority:** Rejected\n- **Rationale:** Not worth effort, dark mode is sufficient\n\n**Story: GraphQL API Rewrite**\n- **Value:** Low (Internal preference, no user benefit)\n- **Effort:** High (34+ points - major refactor, migration)\n- **ROI:** Negative\n- **Priority:** Rejected\n- **Rationale:** REST API works fine, no business justification\n\n## Priority Tiers\n\n**P0 (Must Have - Current Sprint):**\n1. Dark Mode UI (3 pts) - Quick win\n2. Payment Receipt Email (2 pts) - Blocks launch\n3. Elasticsearch Integration (13 pts) - Strategic\n\n**Rationale:** P0 items deliver high value and unblock revenue. Total: 18 points fits in sprint (team velocity: 46 pts/2 weeks = 23 pts/sprint)\n\n**P1 (Should Have - Next Sprint):**\n4. WebSocket Real-time Sync (13 pts)\n5. Payment Refund Flow (8 pts)\n6. Advanced Search Filters (8 pts)\n\n**Rationale:** P1 items complete the enterprise feature set. Sequence after P0.\n\n**P2 (Nice to Have - Backlog):**\n7. Export Data to CSV (3 pts)\n8. User Profile Customization (5 pts)\n\n**Rationale:** P2 items fill gaps when capacity available or higher priorities blocked.\n\n**P3 (Wishlist - Icebox):**\n9. Keyboard Shortcuts (5 pts)\n10. Notification Preferences (5 pts)\n\n**Rationale:** P3 items are low impact. Revisit quarterly.\n\n**Rejected (Don't Build):**\n- Custom Theme Engine (21 pts) - Not worth effort\n- GraphQL API Rewrite (34 pts) - No business value\n\n**Rationale:** Rejected items fail value/effort test. Decline politely.\n\n## Prioritization Criteria\n\n**Value Scoring (0-10):**\n- **Revenue impact:** +3 pts if generates revenue, +1 if enables upsell\n- **User satisfaction:** +3 pts if >100 requests, +1 if <10 requests\n- **Strategic alignment:** +3 pts if linked to OKR, +0 if nice-to-have\n- **Risk reduction:** +1 pt if reduces tech debt or security risk\n\n**Effort Scoring (Story Points):**\n- Use story point estimate from complexity analysis\n- <5 pts = Low effort\n- 5-8 pts = Medium effort\n- >8 pts = High effort\n\n**Priority Formula:**\nPriority Score = Value Score / Effort Score\n\nHigher score = Higher priority\n\n## Alignment to Strategic Goals\n\n**Company OKR: Increase enterprise revenue 50% in Q2**\n- ✓ Elasticsearch Integration: Enables enterprise tier ($50K+ ARR)\n- ✓ WebSocket Real-time: Enterprise UX differentiator\n- ✓ Payment Receipt: Required for payment processing launch\n- ✓ Dark Mode: Competitive parity with enterprise products\n- ✗ Keyboard Shortcuts: No revenue impact\n- ✗ Custom Theme Engine: No revenue impact, high effort\n\n**Alignment Score:** 4 of 6 P0-P1 stories align with OKR (67%)\n\n## Confidence\n\n**Prioritization Confidence:** [0.0-1.0]\n\n**Assumptions:**\n- [Revenue projections accurate]\n- [User request volume reflects actual demand]\n- [Effort estimates within 20% accuracy]\n\n**Risks:**\n- [Priority may shift if enterprise sales slower than expected]\n- [Competitor launch may change strategic importance]\n- [Technical blockers may require re-prioritization]" \
  --confidence [0.0-1.0] \
  --tags "backlog,prioritization,[release-name]"
```

### Step 4: Link Stories to Strategic Goals

Create traceability from stories to company objectives:

```bash
engram reasoning create \
  --title "Strategic Alignment: [Story Title]" \
  --task-id [TASK_ID] \
  --content "## Story Summary\n\n**Story:** [Title]\n**Epic:** [Parent epic]\n**Business Value:** [Expected outcome]\n\n## Strategic Linkage\n\n### Company Mission\n**Mission:** [e.g., Empower teams to collaborate effectively]\n**How this story supports:** [e.g., Real-time collaboration enables distributed teams]\n\n### Company OKRs (Q2 2026)\n\n**Objective:** Increase enterprise revenue 50%\n\n**Key Result 1:** Close 50 enterprise deals by June 30\n- **Link:** This story enables enterprise tier pricing ($199/month)\n- **Impact:** Without this feature, can't sell enterprise tier (blocks $120K ARR)\n\n**Key Result 2:** Achieve 90% enterprise customer satisfaction\n- **Link:** This story improves UX for power users\n- **Impact:** Differentiation from competitors, reduces churn\n\n**Key Result 3:** Launch in 3 new verticals (Healthcare, Finance, Legal)\n- **Link:** This story adds compliance features required for regulated industries\n- **Impact:** Unblocks sales in healthcare vertical ($200K+ pipeline)\n\n### Product Roadmap\n\n**Theme:** Enterprise-ready platform\n**Milestone:** Q2 Enterprise Launch (March 31)\n**Feature:** Advanced collaboration suite\n\n**How this story fits:**\n- Part of \"Advanced Collaboration\" feature set\n- Blocks enterprise launch if not delivered\n- Critical path: Must complete before March 15 for release buffer\n\n### User Impact\n\n**Target Users:** Enterprise team leads (500+ employee companies)\n**Pain Point:** [e.g., Can't collaborate in real-time, resort to email]\n**This story solves:** [e.g., Real-time sync enables live collaboration]\n**Expected outcome:** [e.g., 30% reduction in email, 20% faster decision-making]\n\n### Competitive Positioning\n\n**Competitor:** [Competitor A]\n**Their capability:** [They have real-time collaboration]\n**Our gap:** [We only have async, losing deals]\n**This story closes gap:** [Yes - brings us to parity + our unique value]\n\n### Revenue Impact\n\n**Direct revenue:**\n- Enterprise tier pricing: $199/month vs $49/month (4x increase)\n- Estimated adoption: 50 customers in Q2\n- Revenue impact: $150K ARR (50 × $199 × 12 - 50 × $49 × 12)\n\n**Indirect revenue:**\n- Reduces churn: Enterprise customers churn at 5% vs 15% for standard\n- Upsell rate: 80% of standard customers express interest in enterprise\n- Long-term value: Enterprise LTV $14K vs standard LTV $2K\n\n### Risk If Not Built\n\n**Revenue risk:** Lose $150K ARR opportunity in Q2\n**Competitive risk:** Fall further behind competitors\n**Strategic risk:** Can't enter enterprise market, limits growth\n**Customer risk:** Current power users may churn to competitors\n\n### Success Metrics\n\n**How we measure success:**\n- 50+ enterprise customers using feature within 30 days of launch\n- 90%+ feature adoption among enterprise tier\n- >4/5 user satisfaction rating\n- <5% churn in enterprise tier\n- $150K+ ARR attributed to enterprise features\n\n**Tracking:**\n- Weekly: Active users, feature usage %\n- Monthly: Revenue, churn rate, NPS\n- Quarterly: OKR progress, strategic goal achievement\n\n## Decision Rationale\n\n**Why prioritize this story:**\n1. Direct link to Q2 OKR (enterprise revenue)\n2. High revenue impact ($150K+ ARR)\n3. Blocks enterprise launch (critical path)\n4. Competitive necessity (table stakes feature)\n5. Feasible to deliver in timeframe (13 points, 2 sprints)\n\n**Why not defer:**\n- Enterprise sales pipeline worth $500K waiting on this feature\n- Competitor launched similar feature last month\n- Window of opportunity closing (Q2 ends June 30)\n\n**Confidence:** [0.0-1.0]" \
  --confidence [0.0-1.0] \
  --tags "backlog,strategic-alignment,[story-name]"
```

### Step 5: Create Refined Backlog Context

Document the prioritized, estimated, aligned backlog:

```bash
engram context create \
  --title "Refined Backlog: [Sprint/Release]" \
  --content "## Backlog Status\n\n**Total Stories:** [N]\n**Total Points:** [M]\n**Ready for Sprint:** [P stories]\n**Needs Refinement:** [Q stories]\n**Blocked:** [R stories]\n\n## Sprint Candidates (Definition of Ready Met)\n\n### P0: Must Have\n\n1. **Story: Dark Mode UI**\n   - **Points:** 3\n   - **Value:** High (200+ requests)\n   - **Owner:** Carol (Frontend)\n   - **Links:** Epic: UI Improvements, OKR: User satisfaction\n   - **Status:** Ready ✓\n\n2. **Story: Payment Receipt Email**\n   - **Points:** 2\n   - **Value:** High (Compliance, blocks launch)\n   - **Owner:** Bob (Backend)\n   - **Links:** Epic: Payment Processing, OKR: Revenue\n   - **Status:** Ready ✓\n\n3. **Story: Elasticsearch Integration**\n   - **Points:** 13\n   - **Value:** High ($50K+ ARR)\n   - **Owner:** Alice (Backend)\n   - **Links:** Epic: Advanced Search, OKR: Revenue\n   - **Status:** Ready ✓ (Spike completed)\n\n**Subtotal P0:** 18 points\n\n### P1: Should Have\n\n4. **Story: WebSocket Real-time Sync**\n   - **Points:** 13\n   - **Value:** High (Competitive parity)\n   - **Owner:** Alice (Backend)\n   - **Links:** Epic: Real-time Collaboration, OKR: User satisfaction\n   - **Status:** Ready ✓ (Architecture reviewed)\n\n5. **Story: Payment Refund Flow**\n   - **Points:** 8\n   - **Value:** Medium (Required feature, low usage expected)\n   - **Owner:** Bob (Backend)\n   - **Links:** Epic: Payment Processing, OKR: Revenue\n   - **Status:** Ready ✓\n\n6. **Story: Advanced Search Filters**\n   - **Points:** 8\n   - **Value:** Medium (Enterprise nice-to-have)\n   - **Owner:** Carol (Frontend) + Bob (Backend)\n   - **Links:** Epic: Advanced Search, OKR: Revenue\n   - **Status:** Ready ✓\n\n**Subtotal P1:** 29 points\n\n### P2: Nice to Have\n\n7. **Story: Export Data to CSV**\n   - **Points:** 3\n   - **Value:** Low (10 requests)\n   - **Owner:** Bob (Backend)\n   - **Links:** Epic: Data Export\n   - **Status:** Ready ✓\n\n8. **Story: User Profile Customization**\n   - **Points:** 5\n   - **Value:** Low (Quality of life)\n   - **Owner:** Carol (Frontend)\n   - **Links:** Epic: User Experience\n   - **Status:** Needs design mockups ⚠️\n\n**Subtotal P2:** 8 points\n\n## Backlog Needing Refinement\n\n### Stories Without Acceptance Criteria\n\n- **Story: Mobile App Redesign** - Needs product review\n- **Story: Admin Dashboard** - Scope too large, split needed\n\n### Stories Without Estimate\n\n- **Story: API Rate Limiting** - Needs technical spike (complexity unknown)\n- **Story: SSO Integration** - Needs vendor evaluation (dependencies unknown)\n\n### Stories Blocked\n\n- **Story: Desktop App** - Blocked: Waiting on Electron upgrade (Security CVE)\n- **Story: Analytics Dashboard** - Blocked: Waiting on data team schema (Due March 20)\n\n## Sprint Planning Recommendations\n\n**For Next Sprint (March 15-28):**\n\n**Team Velocity:** 23 points/sprint (46 points per 2 sprints)\n\n**Recommended Commitment:**\n- Story 1: Dark Mode UI (3 pts)\n- Story 2: Payment Receipt Email (2 pts)\n- Story 3: Elasticsearch Integration (13 pts)\n- Story 4: Payment Refund Flow (8 pts) - if capacity\n\n**Total:** 18-26 points (fits velocity)\n\n**Rationale:**\n- All P0 stories fit (18 pts < 23 pts velocity)\n- Story 4 (P1) can be added if team has capacity\n- Mix of quick wins (Stories 1, 2) and strategic work (Story 3)\n- Aligns with Q2 OKR (enterprise revenue)\n\n**Risks:**\n- Elasticsearch integration is complex (13 pts) - may expand\n- If Story 3 grows, drop Story 4 from sprint\n\n## Strategic Alignment Summary\n\n**Stories linked to OKRs:** 6 of 8 (75%)\n**Revenue-generating stories:** 4 (Payments, Search, Refunds, Filters)\n**User satisfaction stories:** 3 (Dark mode, Real-time, Customization)\n**Technical debt stories:** 0 (Need to allocate capacity in future)\n\n**Alignment Score:** Strong (75% tied to strategic goals)\n\n## Backlog Health Metrics\n\n**Backlog size:** 15 stories (~120 points)\n- Target: <20 stories ready for sprint planning\n- Status: ✓ Healthy\n\n**Refinement rate:** 8 stories refined this week\n- Target: >5 stories/week to stay ahead\n- Status: ✓ On track\n\n**Age of stories:** Oldest story is 6 weeks old\n- Target: <8 weeks before review or delete\n- Status: ✓ Fresh backlog\n\n**Blocked stories:** 2 blocked (13% of backlog)\n- Target: <20% blocked\n- Status: ✓ Acceptable\n\n## Next Refinement Session\n\n**Date:** [Next refinement meeting date]\n**Focus:**\n- Refine P2 stories for future sprints\n- Break down \"Admin Dashboard\" epic\n- Run spike on \"API Rate Limiting\"\n- Review blocked stories for unblocking" \
  --source "backlog-refinement" \
  --tags "backlog,refined-backlog,[sprint-name]"
```

### Step 6: Link Backlog Entities

```bash
# Link epic breakdown
engram relationship create \
  --source-id [TASK_ID] --source-type task \
  --target-id [EPIC_BREAKDOWN_ID] --target-type context \
  --relationship-type references \
  --agent [AGENT]

# Link complexity estimates
engram relationship create \
  --source-id [TASK_ID] --source-type task \
  --target-id [COMPLEXITY_ESTIMATE_ID] --target-type reasoning \
  --relationship-type documents \
  --agent [AGENT]

# Link prioritization analysis
engram relationship create \
  --source-id [TASK_ID] --source-type task \
  --target-id [PRIORITIZATION_ID] --target-type reasoning \
  --relationship-type documents \
  --agent [AGENT]

# Link strategic alignment
engram relationship create \
  --source-id [TASK_ID] --source-type task \
  --target-id [STRATEGIC_ALIGNMENT_ID] --target-type reasoning \
  --relationship-type documents \
  --agent [AGENT]

# Link refined backlog
engram relationship create \
  --source-id [TASK_ID] --source-type task \
  --target-id [REFINED_BACKLOG_ID] --target-type context \
  --relationship-type references \
  --agent [AGENT]
```

## Example

User has a large backlog (30 stories) needing refinement for Q2 planning.

### Step 1: Break Down Epic

```bash
EPIC=$(engram context create \
  --title "Epic Breakdown: Enterprise Feature Set" \
  --content "## Epic\n\n**Title:** Enterprise Feature Set\n**Description:** Enable enterprise tier ($199/month) with advanced features for teams of 500+ employees\n**Business Goal:** Generate $150K ARR in Q2 from 50 enterprise customers\n**Success Criteria:** 50 customers adopt enterprise tier, 90% satisfaction, <5% churn\n\n## User Stories\n\n### Story 1: Payment Processing (Stripe Integration)\n\n**User Story:**\nAs a team admin\nI want to upgrade to enterprise tier with credit card\nSo that I can access advanced features immediately\n\n**Acceptance Criteria:**\n- [ ] User can enter credit card info (Stripe Elements)\n- [ ] Payment processes successfully (test and live mode)\n- [ ] User receives email receipt\n- [ ] Subscription status updates in database\n- [ ] Failed payments show clear error messages\n\n**Technical Notes:**\n- Use Stripe Checkout for PCI compliance\n- Webhook for async payment confirmation\n- Handle retries for failed payments\n- Edge cases: Expired cards, insufficient funds, disputes\n\n**Complexity Estimate:** 8 points\n**Priority:** Must-have (P0)\n**Value:** High (Blocks monetization)\n**Effort:** Medium\n\n### Story 2: Advanced Search (Elasticsearch)\n\n**User Story:**\nAs an enterprise user\nI want to search all documents instantly\nSo that I can find information across thousands of documents\n\n**Acceptance Criteria:**\n- [ ] Search returns results in <200ms (p95)\n- [ ] Search includes all document content and metadata\n- [ ] Search handles typos and partial matches\n- [ ] Search filters by date, author, tags\n- [ ] Search highlights matching text\n\n**Technical Notes:**\n- Elasticsearch cluster (3 nodes for redundancy)\n- Index backfill for existing documents\n- Sync strategy: Real-time via events\n- Edge cases: Index failures, cluster down (fallback to SQL)\n\n**Complexity Estimate:** 13 points\n**Priority:** Must-have (P0)\n**Value:** High (Enterprise differentiator)\n**Effort:** High\n\n### Story 3: Real-time Collaboration (WebSocket)\n\n**User Story:**\nAs an enterprise user\nI want to see teammate edits in real-time\nSo that we can collaborate live without conflicts\n\n**Acceptance Criteria:**\n- [ ] Changes from other users appear within 300ms\n- [ ] See cursor positions of other active users\n- [ ] Offline edits sync when reconnected\n- [ ] No lost edits due to network issues\n- [ ] Scales to 10 concurrent users per document\n\n**Technical Notes:**\n- WebSocket server with sticky sessions\n- CRDT (Yjs) for conflict-free merging\n- IndexedDB for offline persistence\n- Edge cases: Connection drops, concurrent edits\n\n**Complexity Estimate:** 13 points\n**Priority:** Should-have (P1)\n**Value:** High (Competitive advantage)\n**Effort:** High\n\n### Story 4: Payment Receipt Email\n\n**User Story:**\nAs a paying customer\nI want to receive email receipt for payments\nSo that I have records for accounting\n\n**Acceptance Criteria:**\n- [ ] Email sent within 1 minute of successful payment\n- [ ] Email includes: Amount, date, payment method, invoice ID\n- [ ] Email includes PDF receipt attachment\n- [ ] Email sent to billing contact (configurable)\n- [ ] Retry sending if email fails initially\n\n**Technical Notes:**\n- Email service: SendGrid\n- Template: Use Stripe invoice template\n- Trigger: Stripe webhook payment_intent.succeeded\n- Edge cases: Email bounce, spam filter\n\n**Complexity Estimate:** 2 points\n**Priority:** Must-have (P0)\n**Value:** High (Compliance requirement)\n**Effort:** Low\n\n### Story 5: Payment Refund Flow\n\n**User Story:**\nAs a team admin\nI want to request refund for overpayment\nSo that I'm only charged for actual usage\n\n**Acceptance Criteria:**\n- [ ] Admin can request refund via UI\n- [ ] Refund requests reviewed by support team\n- [ ] Approved refunds processed to original payment method\n- [ ] User notified of refund status via email\n- [ ] Refund reflected in billing history\n\n**Technical Notes:**\n- Stripe refunds API\n- Manual approval workflow (internal tool)\n- Partial refunds supported\n- Edge cases: Refund after cancellation, disputed charges\n\n**Complexity Estimate:** 8 points\n**Priority:** Should-have (P1)\n**Value:** Medium (Low usage expected but required)\n**Effort:** Medium\n\n### Story 6: Advanced Search Filters\n\n**User Story:**\nAs an enterprise power user\nI want to filter search by author, date range, tags\nSo that I can narrow results to relevant documents\n\n**Acceptance Criteria:**\n- [ ] Filter by author (multi-select)\n- [ ] Filter by date range (picker)\n- [ ] Filter by tags (autocomplete)\n- [ ] Filter by document type\n- [ ] Filters apply instantly without full page reload\n\n**Technical Notes:**\n- Elasticsearch aggregations for facets\n- Frontend: React filter components\n- URL params for shareable filtered searches\n- Edge cases: No results, contradictory filters\n\n**Complexity Estimate:** 8 points\n**Priority:** Should-have (P1)\n**Value:** Medium (Nice-to-have for enterprise)\n**Effort:** Medium\n\n### Story 7: Dark Mode UI\n\n**User Story:**\nAs a user working late\nI want to enable dark mode\nSo that I reduce eye strain\n\n**Acceptance Criteria:**\n- [ ] Toggle dark mode in user settings\n- [ ] Preference persists across sessions\n- [ ] All UI elements support dark mode\n- [ ] Contrast meets WCAG AA accessibility standards\n- [ ] System preference respected (prefers-color-scheme)\n\n**Technical Notes:**\n- CSS variables for theming\n- Feature flag: enable_dark_mode\n- Edge cases: Print styles, embedded content\n\n**Complexity Estimate:** 3 points\n**Priority:** Nice-to-have (P0 for quick win)\n**Value:** High (200+ requests, low effort)\n**Effort:** Low\n\n## Story Sequence\n\n**Dependencies:**\nStory 1 (Payments) → Story 4 (Receipt Email)\nStory 2 (Search) → Story 6 (Filters)\nStory 3, 5, 7 independent (can parallelize)\n\n**Incremental Delivery:**\n- **MVP (Sprint 1):** Stories 1, 4, 7 (13 pts) - Can charge customers\n- **V1 (Sprint 2):** Stories 2, 5 (21 pts) - Full enterprise features\n- **V2 (Sprint 3):** Stories 3, 6 (21 pts) - Competitive parity\n\n## Definition of Ready\n\n- [✓] All stories have acceptance criteria\n- [✓] All stories have complexity estimates\n- [✓] Dependencies identified\n- [⚠️] Design mockups: Needed for Stories 3, 6, 7\n- [✓] Technical approach agreed for Stories 1, 2\n- [⚠️] Spike needed for Story 3 (WebSocket scaling)\n\n**Actions before sprint planning:**\n- Request mockups from designer for Stories 3, 6, 7\n- Run 1-day spike on Story 3 (WebSocket architecture)\n\n## Definition of Done\n\n- [ ] All acceptance criteria met\n- [ ] Code reviewed (2 approvals)\n- [ ] Tests: Unit (>80% coverage), Integration, E2E\n- [ ] Documentation: API docs, user guide updated\n- [ ] Deployed to staging and validated\n- [ ] Product owner accepted\n- [ ] Monitoring and alerts configured\n\n## Links to Strategic Goals\n\n**Company OKR:** Q2 2026: Increase enterprise revenue 50%\n**Key Result:** Close 50 enterprise deals by June 30\n**How this epic contributes:** These 7 stories enable $199/month enterprise tier. Without them, can't sell enterprise. Projected impact: $150K ARR." \
  --source "backlog-refinement" \
  --tags "backlog,epic-breakdown,enterprise-features" \
  --json | jq -r '.id')

echo "Epic breakdown created: $EPIC"
```

### Step 2: Prioritize Using Value/Effort

```bash
PRIORITIZATION=$(engram reasoning create \
  --title "Backlog Prioritization: Q2 2026 Sprint Planning" \
  --task-id a3f8b2c1-1234-5678-90ab-cdef12345678 \
  --content "## Prioritization Framework\n\n**Value/Effort Matrix:**\n- High Value + Low Effort = DO FIRST (Quick wins)\n- High Value + High Effort = DO NEXT (Strategic)\n- Low Value + Low Effort = DO LATER (Fill gaps)\n- Low Value + High Effort = DON'T DO (Avoid)\n\n## Prioritized Stories\n\n### Quadrant 1: High Value, Low Effort - DO FIRST\n\n**1. Dark Mode UI**\n- Value: High (200+ requests, 10 min to implement)\n- Effort: Low (3 points)\n- ROI: 67 value points / 3 effort = 22.3 (Highest ROI)\n- Priority: P0\n- **Rationale:** Huge user demand, trivial effort, instant win\n\n**2. Payment Receipt Email**\n- Value: High (Compliance, blocks payment launch)\n- Effort: Low (2 points)\n- ROI: 10 / 2 = 5.0\n- Priority: P0\n- **Rationale:** Required for payment processing, easy to implement\n\n### Quadrant 2: High Value, High Effort - DO NEXT\n\n**3. Payment Processing (Stripe)**\n- Value: High (Blocks $150K ARR)\n- Effort: Medium (8 points)\n- ROI: 10 / 8 = 1.25\n- Priority: P0\n- **Rationale:** Must have for monetization, medium complexity\n\n**4. Advanced Search (Elasticsearch)**\n- Value: High (Enterprise differentiator, $50K+ ARR)\n- Effort: High (13 points)\n- ROI: 9 / 13 = 0.69\n- Priority: P0\n- **Rationale:** Core enterprise feature, worth investment\n\n**5. Real-time Collaboration (WebSocket)**\n- Value: High (Competitive parity)\n- Effort: High (13 points)\n- ROI: 8 / 13 = 0.62\n- Priority: P1\n- **Rationale:** Important but not blocking, sequence after search\n\n**6. Payment Refund Flow**\n- Value: Medium (Required but low usage)\n- Effort: Medium (8 points)\n- ROI: 5 / 8 = 0.63\n- Priority: P1\n- **Rationale:** Nice-to-have for launch, not blocking\n\n**7. Advanced Search Filters**\n- Value: Medium (Enterprise enhancement)\n- Effort: Medium (8 points)\n- ROI: 5 / 8 = 0.63\n- Priority: P1\n- **Rationale:** Completes search feature, sequence after basic search\n\n## Priority Tiers for Sprint Planning\n\n**P0: Must Have (Sprint 1 - March 15-28)**\n1. Dark Mode UI (3 pts) ← Quick win\n2. Payment Receipt Email (2 pts) ← Blocks payments\n3. Payment Processing (8 pts) ← Revenue unlock\n4. Advanced Search (13 pts) ← Enterprise core\n\n**Total P0:** 26 points\n**Team Velocity:** 23 points/sprint\n**Risk:** Slightly over velocity - may need to defer Story 4 if Story 3 expands\n\n**P1: Should Have (Sprint 2 - March 29-April 11)**\n5. Real-time Collaboration (13 pts)\n6. Payment Refund Flow (8 pts)\n7. Advanced Search Filters (8 pts)\n\n**Total P1:** 29 points (fits in 2 sprints)\n\n## Alignment to Q2 OKR\n\n**OKR: Increase enterprise revenue 50%**\n\n**Revenue-generating stories:**\n- Payment Processing: Unlocks all revenue ($150K ARR)\n- Advanced Search: Enterprise differentiator ($50K ARR)\n- Real-time Collaboration: Competitive advantage (reduces churn)\n- Refund Flow: Reduces customer friction\n\n**Alignment:** 4 of 7 stories directly tied to revenue (57%)\n**Recommendation:** Strong alignment, prioritization supports OKR\n\n## Risks and Mitigation\n\n**Risk 1: P0 Stories exceed velocity (26 > 23 pts)**\n- Mitigation: If Search (13 pts) expands, defer to Sprint 2\n- Fallback: Stories 1+2+3 (13 pts) still deliver payment capability\n\n**Risk 2: Search complexity unknown (Elasticsearch learning curve)**\n- Mitigation: Run spike in advance (1 day), validate 13 pt estimate\n- Fallback: SQL search with pagination if Elasticsearch too complex\n\n**Risk 3: Prioritizing features over tech debt (0% allocated)**\n- Mitigation: Reserve 10% capacity in future sprints for tech debt\n- Monitor: If velocity drops, increase tech debt allocation\n\n**Confidence:** 0.85" \
  --confidence 0.85 \
  --tags "backlog,prioritization,q2-2026" \
  --json | jq -r '.id')

echo "Prioritization created: $PRIORITIZATION"
```

### Step 3: Link to Strategic Goals

```bash
ALIGNMENT=$(engram reasoning create \
  --title "Strategic Alignment: Advanced Search (Elasticsearch)" \
  --task-id a3f8b2c1-1234-5678-90ab-cdef12345678 \
  --content "## Story Summary\n\n**Story:** Advanced Search (Elasticsearch Integration)\n**Epic:** Enterprise Feature Set\n**Business Value:** Enable enterprise customers to search across thousands of documents instantly\n\n## Strategic Linkage\n\n### Company Mission\n**Mission:** Empower teams to collaborate effectively\n**How this story supports:** Fast search enables teams to find information quickly, reducing time wasted searching, improving collaboration efficiency\n\n### Company OKRs (Q2 2026)\n\n**Objective:** Increase enterprise revenue 50%\n\n**Key Result 1:** Close 50 enterprise deals by June 30\n- **Link:** Advanced search is core enterprise tier feature ($199/month)\n- **Impact:** Without search, enterprise tier has no differentiation from standard ($49/month). Blocks entire enterprise strategy.\n- **Revenue:** $150K ARR at risk if not delivered\n\n**Key Result 2:** Achieve 90% enterprise customer satisfaction\n- **Link:** Search quality directly impacts user satisfaction\n- **Impact:** Slow or poor search quality is #1 complaint in competitor reviews\n- **Target:** <200ms p95 search latency, >95% relevant results\n\n**Key Result 3:** Launch in 3 new verticals (Healthcare, Finance, Legal)\n- **Link:** Regulated industries require advanced search for compliance (audit trail, eDiscovery)\n- **Impact:** Healthcare vertical requires search with HIPAA-compliant audit logging. This story enables healthcare sales ($200K+ pipeline).\n\n### Product Roadmap\n\n**Theme:** Enterprise-ready platform\n**Milestone:** Q2 Enterprise Launch (March 31, 2026)\n**Feature:** Advanced Collaboration Suite\n\n**How this story fits:**\n- Core component of enterprise tier\n- Critical path: Must complete by March 20 for integration testing\n- Dependency: Elasticsearch cluster must be provisioned by March 10\n- Risk: New technology for team, 13 pt complexity\n\n### User Impact\n\n**Target Users:** Enterprise team leads, knowledge workers at 500+ employee companies\n**Pain Point:** Standard SQL search limited to 100 results, slow (>2 seconds), no relevance ranking\n**This story solves:** Elasticsearch provides instant search (<200ms), relevance ranking, handles 100K+ documents\n**Expected outcome:** Users find information 10x faster, reduced frustration, increased productivity\n\n**User Research:**\n- 15 enterprise prospects cited \"better search\" as reason they'd switch from current solution\n- 80% of beta users tried search within first session\n- Current search has 35% abandonment rate (users give up)\n\n### Competitive Positioning\n\n**Competitors with advanced search:**\n- Competitor A: Elasticsearch-powered, <100ms latency\n- Competitor B: Algolia integration, typo-tolerance\n- Competitor C: Basic SQL search (similar to our current)\n\n**Our gap:** We're at parity with Competitor C (basic), losing deals to A and B\n**This story closes gap:** Brings us to parity with A, exceeds B (we add filters)\n**Differentiation:** After this story, we'll have search + our unique collaboration features\n\n### Revenue Impact\n\n**Direct revenue:**\n- Enterprise tier: $199/month vs standard $49/month\n- Search is core differentiator for enterprise tier\n- Estimated: 50 enterprise customers in Q2\n- Revenue impact: $150K ARR from enterprise tier\n\n**Breakdown:**\n- Without search: 0 enterprise customers (no value prop)\n- With search: 50 enterprise customers (proven demand)\n- Revenue delta: $150K ARR\n\n**Indirect revenue:**\n- Reduces churn: Power users no longer frustrated by slow search\n- Estimated churn reduction: 5% → 3% for standard tier\n- Churn value: 2% of 1000 users × $49/mo × 12 = $11.8K ARR saved\n\n**Total revenue impact: $161.8K ARR**\n\n### Risk If Not Built\n\n**Revenue risk:** Lose $150K+ ARR opportunity in Q2 (entire enterprise tier blocked)\n**Competitive risk:** Fall further behind competitors A and B, lose market position\n**Strategic risk:** Can't enter enterprise market, stuck in SMB segment (lower LTV)\n**Customer risk:** Power users churn to competitors with better search (current churn 5% → may increase to 8%)\n\n**Critical path:** This story blocks enterprise launch. If delayed, entire Q2 OKR at risk.\n\n### Success Metrics\n\n**How we measure success:**\n- **Performance:** p95 search latency <200ms (currently >2 seconds)\n- **Usage:** >80% of enterprise users use search weekly\n- **Satisfaction:** >4/5 rating for search quality\n- **Business:** 50+ enterprise customers within 60 days of launch\n- **Revenue:** $150K ARR attributed to enterprise tier by June 30\n\n**Tracking:**\n- Weekly: Search query volume, latency percentiles, error rate\n- Monthly: Enterprise customer adoption, feature usage %, NPS\n- Quarterly: OKR progress (enterprise revenue), strategic goal (50 deals)\n\n**Dashboard:** https://metabase.example.com/dashboard/enterprise-metrics\n\n## Decision Rationale\n\n**Why prioritize this story (P0):**\n1. ✓ Direct link to Q2 OKR (enterprise revenue 50% increase)\n2. ✓ High revenue impact ($150K+ ARR, blocks entire enterprise tier)\n3. ✓ Critical path item (blocks March 31 launch)\n4. ✓ Competitive necessity (table stakes for enterprise segment)\n5. ✓ Proven demand (15 prospects waiting on this feature)\n6. ✓ Feasible to deliver (13 points, fits in 2-sprint timeline)\n\n**Why not defer to Q3:**\n- Enterprise sales pipeline worth $500K blocked on this feature\n- Competitors launched similar features Q1 2026 (we're losing deals)\n- Q2 ends June 30 - window closing to hit OKR\n- Customer commitment: Promised feature to 5 beta customers by March 31\n\n**Trade-offs accepted:**\n- Technical debt: Adding Elasticsearch increases operational complexity\n- Cost: Elasticsearch cluster adds ~$500/month infrastructure cost\n- Learning curve: Team has no Elasticsearch experience (spike needed)\n- Risk: If estimate wrong (13 pts → 21 pts), delays entire release\n\n**Mitigation for trade-offs:**\n- Run 1-day spike to validate complexity estimate\n- DevOps team prepared to support Elasticsearch operations\n- Fallback: SQL search remains available if Elasticsearch fails\n- Buffer: 3 weeks between completion (March 20) and launch (March 31)\n\n**Confidence:** 0.90\n\n**Assumptions:**\n- Enterprise prospects will convert when feature delivered (15 committed)\n- Search quality meets user expectations (validated in spike)\n- Elasticsearch scales to 100K documents (load tested)\n- Team can deliver in 13 points (spike validates estimate)" \
  --confidence 0.90 \
  --tags "backlog,strategic-alignment,advanced-search" \
  --json | jq -r '.id')

echo "Strategic alignment created: $ALIGNMENT"
```

### Step 4: Create Refined Backlog

```bash
BACKLOG=$(engram context create \
  --title "Refined Backlog: Q2 2026 Sprint Planning" \
  --content "## Backlog Status\n\n**Total Stories:** 7 (from Enterprise Feature Set epic)\n**Total Points:** 55\n**Ready for Sprint:** 5 stories (35 points)\n**Needs Refinement:** 2 stories (20 points - Stories 3, 6 need design)\n**Blocked:** 0\n\n## Sprint 1 Candidates (March 15-28) - Team Velocity: 23 points\n\n### P0: Must Have\n\n1. **Story: Dark Mode UI**\n   - **Points:** 3\n   - **Value:** High (200+ user requests, quick win)\n   - **Owner:** Carol (Frontend)\n   - **Links:** Epic: Enterprise Features, OKR: User satisfaction\n   - **Status:** Ready ✓\n   - **Rationale:** High ROI (22.3), delivers instant value\n\n2. **Story: Payment Receipt Email**\n   - **Points:** 2\n   - **Value:** High (Compliance, required for payment processing)\n   - **Owner:** Bob (Backend)\n   - **Links:** Epic: Enterprise Features, OKR: Revenue\n   - **Status:** Ready ✓\n   - **Rationale:** Blocks payment launch, low effort\n\n3. **Story: Payment Processing (Stripe Integration)**\n   - **Points:** 8\n   - **Value:** High (Unlocks $150K ARR)\n   - **Owner:** Bob (Backend)\n   - **Links:** Epic: Enterprise Features, OKR: Revenue (KR1)\n   - **Status:** Ready ✓ (Stripe account approved, PCI review passed)\n   - **Rationale:** Core monetization feature, blocks enterprise tier\n\n4. **Story: Advanced Search (Elasticsearch Integration)**\n   - **Points:** 13\n   - **Value:** High (Enterprise differentiator, $50K+ ARR)\n   - **Owner:** Alice (Backend + DevOps)\n   - **Links:** Epic: Enterprise Features, OKR: Revenue (KR1), Verticals (KR3)\n   - **Status:** Ready ✓ (Spike completed March 10, cluster provisioned)\n   - **Rationale:** Critical path for enterprise launch, competitive necessity\n   - **Risk:** Large story (13 pts), may expand to 16-18 pts\n\n**Total Sprint 1:** 26 points (slightly over velocity)\n\n**Recommendation:**\n- Commit to Stories 1+2+3 (13 pts) as guaranteed\n- Add Story 4 (13 pts) as stretch goal\n- If Story 4 slips, move to Sprint 2 (still on track for March 31 launch)\n\n## Sprint 2 Candidates (March 29-April 11) - Team Velocity: 23 points\n\n### P1: Should Have\n\n5. **Story: Real-time Collaboration (WebSocket)**\n   - **Points:** 13\n   - **Value:** High (Competitive parity, reduces churn)\n   - **Owner:** Alice (Backend)\n   - **Links:** Epic: Enterprise Features, OKR: User satisfaction (KR2)\n   - **Status:** Needs design mockups ⚠️ (Due March 20)\n   - **Rationale:** Strategic feature, sequence after search\n\n6. **Story: Payment Refund Flow**\n   - **Points:** 8\n   - **Value:** Medium (Required but low usage expected)\n   - **Owner:** Bob (Backend)\n   - **Links:** Epic: Enterprise Features\n   - **Status:** Ready ✓\n   - **Rationale:** Completes payment feature set\n\n**Total Sprint 2:** 21 points (fits velocity comfortably)\n\n## Sprint 3 Candidates (April 12-25) - Team Velocity: 23 points\n\n### P1: Should Have\n\n7. **Story: Advanced Search Filters**\n   - **Points:** 8\n   - **Value:** Medium (Enterprise enhancement)\n   - **Owner:** Carol (Frontend) + Bob (Backend)\n   - **Links:** Epic: Enterprise Features, OKR: User satisfaction\n   - **Status:** Needs design mockups ⚠️ (Due March 20)\n   - **Rationale:** Completes search feature, nice-to-have\n\n**Total Sprint 3:** 8 points + capacity for other work (15 points available)\n\n## Backlog Health\n\n**Refined:** 7 stories from Enterprise epic (100% refined)\n**Estimated:** 7 stories (100% have complexity estimates)\n**Prioritized:** 7 stories ranked by value/effort\n**Aligned:** 6 stories linked to OKRs (86% strategic alignment)\n**Blocked:** 0 stories\n**Ready:** 5 stories ready for sprint (71%)\n\n**Actions needed before Sprint 1:**\n- [✓] All P0 stories have acceptance criteria\n- [✓] All P0 stories estimated\n- [✓] Elasticsearch spike complete\n- [✓] Stripe production account approved\n- [ ] Request design mockups for Stories 5, 7 from designer (Due March 20)\n\n## Strategic Alignment Summary\n\n**Revenue-generating stories:** 4 (Payments, Receipt, Search, Refunds) = $150K+ ARR impact\n**User satisfaction stories:** 3 (Dark mode, Real-time, Search) = Reduce churn 2-3%\n**Compliance stories:** 1 (Receipt Email) = Required for payment processing\n**Technical debt:** 0 (Need to allocate 10% capacity in future sprints)\n\n**OKR Alignment:**\n- KR1 (50 enterprise deals): Stories 1, 2, 3, 4 directly enable enterprise tier\n- KR2 (90% satisfaction): Stories 1, 4, 5 improve UX\n- KR3 (3 verticals): Story 4 (Search) enables healthcare vertical\n\n**Alignment Score:** 86% of stories tied to Q2 OKRs (Strong)\n\n## Risk Assessment\n\n**High Risk:**\n- Story 4 (Search 13 pts): May expand if complexity underestimated\n  - Mitigation: Spike validated estimate, 3-week buffer before launch\n\n**Medium Risk:**\n- Sprint 1 total (26 pts) exceeds velocity (23 pts)\n  - Mitigation: Story 4 can slip to Sprint 2 without impact\n\n**Low Risk:**\n- Stories 5, 7 need design mockups\n  - Mitigation: Scheduled with designer, due March 20 (before Sprint 2)\n\n## Next Refinement Session\n\n**Date:** March 22, 2026 (During Sprint 1)\n**Focus:**\n- Finalize Stories 5, 7 with design mockups\n- Refine backlog for Sprint 3 (identify 15+ points of work)\n- Break down next epic (Admin Dashboard)\n- Review Sprint 1 velocity and adjust Sprint 2 commitment" \
  --source "backlog-refinement" \
  --tags "backlog,refined-backlog,q2-2026" \
  --json | jq -r '.id')

echo "Refined backlog created: $BACKLOG"
```

### Step 5: Link All Entities

```bash
engram relationship create \
  --source-id a3f8b2c1-1234-5678-90ab-cdef12345678 --source-type task \
  --target-id $EPIC --target-type context \
  --relationship-type references \
  --agent default

engram relationship create \
  --source-id a3f8b2c1-1234-5678-90ab-cdef12345678 --source-type task \
  --target-id $PRIORITIZATION --target-type reasoning \
  --relationship-type documents \
  --agent default

engram relationship create \
  --source-id a3f8b2c1-1234-5678-90ab-cdef12345678 --source-type task \
  --target-id $ALIGNMENT --target-type reasoning \
  --relationship-type documents \
  --agent default

engram relationship create \
  --source-id a3f8b2c1-1234-5678-90ab-cdef12345678 --source-type task \
  --target-id $BACKLOG --target-type context \
  --relationship-type references \
  --agent default
```

### Step 6: Communicate to Team

Agent presents refined backlog:

"Refined backlog for Q2 ready. Epic broken into 7 stories (55 points). Sprint 1: 4 stories (26 pts) - dark mode, payments, search. Sprint 2-3: 3 stories (29 pts) - real-time, refunds, filters. 86% aligned to Q2 OKR (enterprise revenue). Risk: Search story (13 pts) may expand - have 3-week buffer. All P0 stories ready. Need design mockups for P1 by March 20. Questions?"

## Querying Refined Backlog

After backlog refinement, retrieve information:

```bash
# Get epic breakdowns
engram context list | grep "Epic Breakdown:"

# Get complexity estimates
engram reasoning list | grep "Complexity Estimation:"

# Get prioritization analysis
engram reasoning list | grep "Backlog Prioritization:"

# Get strategic alignment
engram reasoning list | grep "Strategic Alignment:"

# Get refined backlogs
engram context list | grep "Refined Backlog:"

# Get all backlog refinement for a sprint
engram relationship connected --entity-id [TASK_ID] | grep -E "Epic|Complexity|Prioritization|Alignment|Backlog"
```

## Related Skills

This skill integrates with:
- `engram-capacity-planning` - Validate stories fit team capacity
- `engram-roadmap-planning` - Ensure stories align with roadmap milestones
- `engram-risk-assessment` - Assess implementation risks for complex stories
- `engram-spike-investigation` - Run spikes to reduce estimation uncertainty
- `engram-dependency-mapping` - Identify story dependencies and sequencing
