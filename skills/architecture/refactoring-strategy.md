---
name: engram-refactoring-strategy
description: "Plan large-scale refactors with incremental migration paths, identify code smells and anti-patterns, minimize disruption."
---

# Refactoring Strategy (Engram-Integrated)

## Overview

Design comprehensive refactoring strategies for large-scale code improvements, establish incremental migration paths that minimize risk, identify code smells and anti-patterns systematically, and maintain system stability during transformation. Store refactoring plans, migration strategies, and progress tracking in Engram.

## When to Use

Use this skill when:
- Technical debt is slowing feature development significantly
- Code quality issues causing frequent bugs or incidents
- Planning a major architectural change (microservices, new framework)
- Need to modernize legacy codebase without rewrite
- Team velocity declining due to code complexity
- Onboarding new engineers is increasingly difficult

## The Pattern

### Step 1: Assess Current State and Identify Problems

```bash
engram context create \
  --title "Refactoring Assessment: [System/Component]" \
  --content "## Current State Analysis\n\n**System:** [e.g., Authentication service]\n**Age:** [e.g., 3 years old, written in Node.js v10]\n**Size:** [e.g., 15K lines of code, 200 files]\n**Team familiarity:** [e.g., Original author departed, 2 of 8 engineers understand it]\n\n## Code Smells Identified\n\n### Smell 1: God Object (UserService.ts)\n- **Description:** Single class with 3,000 lines handling authentication, authorization, user management, notifications\n- **Impact:** Hard to test, modify, or understand. Last 3 bugs took 2+ days to fix.\n- **Severity:** High\n- **Files affected:** `src/services/UserService.ts`\n- **Dependencies:** 47 files import this class\n\n### Smell 2: Circular Dependencies\n- **Description:** UserService → EmailService → TemplateService → UserService\n- **Impact:** Can't test in isolation, imports break, deployment order sensitive\n- **Severity:** High\n- **Files affected:** 12 files in circular chain\n\n### Smell 3: Callback Hell\n- **Description:** Nested callbacks 6-8 levels deep, no async/await\n- **Impact:** Hard to read, error handling inconsistent, can't use modern tooling\n- **Severity:** Medium\n- **Files affected:** 35 files using old callback patterns\n\n### Smell 4: No Type Safety\n- **Description:** JavaScript with no TypeScript, runtime type errors common\n- **Impact:** 15% of production errors are type-related\n- **Severity:** Medium\n- **Files affected:** All 200 files\n\n### Smell 5: Monolithic Database Access\n- **Description:** Direct SQL queries scattered across 80 files, no repository pattern\n- **Impact:** Can't swap database, queries duplicated, SQL injection risks\n- **Severity:** High\n- **Files affected:** 80 files with database calls\n\n## Impact on Engineering\n\n**Velocity Impact:**\n- Feature development: 50% slower than new codebases\n- Bug fixes: Average 2 days (vs 4 hours in clean code)\n- Onboarding: New engineers need 60 days to be productive (vs 30 days target)\n\n**Quality Impact:**\n- Bug rate: 3× higher than team average\n- Production incidents: 60% originate from this system\n- Test coverage: 40% (target: 80%)\n\n**Risk Assessment:**\n- Bus factor: 2 (only 2 engineers comfortable modifying)\n- Deployment risk: High (rollback needed in 25% of deployments)\n- Security risk: Medium (SQL injection vulnerabilities, no input validation)\n\n## Business Impact\n\n**Cost:**\n- Engineering time wasted: 40% of auth team capacity on maintenance\n- Opportunity cost: 2 features deferred due to technical debt\n- Incident cost: 3 outages in Q1, estimated $50K revenue impact\n\n**Risk:**\n- Security vulnerability could expose user data (compliance risk)\n- System instability hurting enterprise sales conversations\n- Team morale: Engineers avoid working on this system" \
  --source "refactoring-strategy" \
  --tags "refactoring,assessment,[system-name]"
```

### Step 2: Define Target Architecture

```bash
engram reasoning create \
  --title "Target Architecture: [System] Refactoring" \
  --task-id [TASK_ID] \
  --content "## Refactoring Goals\n\n**Primary Goal:** Break UserService god object into focused services\n**Secondary Goals:**\n- Eliminate circular dependencies\n- Migrate to TypeScript for type safety\n- Modernize to async/await\n- Implement repository pattern for database\n\n## Target Architecture\n\n### Service Decomposition\n\n**Current (Monolith):**\n```\nUserService (3000 lines)\n├─ Authentication logic\n├─ Authorization logic\n├─ User CRUD operations\n├─ Password management\n├─ Email notifications\n├─ Session management\n└─ Profile management\n```\n\n**Target (Decomposed):**\n```\nAuthenticationService (300 lines)\n├─ Login/logout\n├─ Token generation\n└─ Session validation\n\nAuthorizationService (200 lines)\n├─ Permission checks\n└─ Role management\n\nUserRepository (400 lines)\n├─ User CRUD (database access)\n├─ Query methods\n└─ Transaction management\n\nPasswordService (150 lines)\n├─ Hashing\n├─ Reset flow\n└─ Validation\n\nNotificationService (200 lines) [Extract to separate module]\n├─ Email sending\n└─ Template rendering\n\nProfileService (250 lines)\n├─ Profile updates\n└─ Avatar management\n```\n\n**Dependency Graph (Target):**\n```\nAPI Controllers\n       ↓\n[AuthenticationService] → [UserRepository] → Database\n       ↓                         ↑\n[AuthorizationService] ←─────────┘\n       ↓\n[PasswordService]\n       ↓\n[NotificationService]\n```\n\n**No circular dependencies:** Linear dependency chain\n\n### Technical Improvements\n\n**TypeScript Migration:**\n- All services strongly typed\n- Interfaces for contracts (IUserRepository, IAuthService)\n- Type-safe API responses\n\n**Modern Patterns:**\n- Async/await throughout (no callbacks)\n- Dependency injection (testability)\n- Repository pattern (database abstraction)\n- Error handling middleware (consistent errors)\n\n**Testing:**\n- Unit tests for each service (>80% coverage target)\n- Integration tests for happy path\n- Mock interfaces for dependencies\n\n## Success Metrics\n\n**Code Quality:**\n- Average file size: <500 lines (currently 750 lines)\n- Cyclomatic complexity: <10 per function (currently 25)\n- Test coverage: >80% (currently 40%)\n- Type coverage: 100% (currently 0%)\n\n**Engineering Velocity:**\n- Feature development time: -50% (vs current)\n- Bug fix time: -75% (2 days → 4 hours)\n- Onboarding time: -50% (60 days → 30 days)\n\n**System Reliability:**\n- Production incidents: -70% (from auth system)\n- Deployment success rate: >95% (currently 75%)\n- Bug rate: -60%\n\n**Confidence:** 0.85" \
  --confidence 0.85 \
  --tags "refactoring,target-architecture,[system-name]"
```

### Step 3: Design Incremental Migration Path

```bash
engram reasoning create \
  --title "Migration Strategy: [System] Refactoring" \
  --task-id [TASK_ID] \
  --content "## Migration Principles\n\n1. **Incremental:** Small changes deployed continuously (not big bang)\n2. **Backward Compatible:** Old and new code coexist during migration\n3. **Reversible:** Can rollback at any point without data loss\n4. **Tested:** Each step validated before proceeding\n5. **Feature Freeze:** No new features during critical migration phases\n\n## Migration Phases (8 weeks total)\n\n### Phase 1: TypeScript Migration (Week 1-2)\n\n**Goal:** Enable type safety without changing logic\n\n**Steps:**\n1. Rename .js → .ts files\n2. Add `any` types to compile without errors\n3. Deploy and verify (no behavior change)\n4. Incrementally add proper types (10 files/day)\n5. Enable strict mode file-by-file\n\n**Deliverable:** 100% TypeScript, basic types\n**Risk:** Low (no logic changes)\n**Rollback:** Revert file renames\n**Testing:** Existing tests pass, type errors caught\n\n### Phase 2: Extract PasswordService (Week 3)\n\n**Goal:** First service extraction, establish pattern\n\n**Steps:**\n1. Create `PasswordService.ts` with interfaces\n2. Copy password logic from UserService\n3. Update UserService to delegate to PasswordService\n4. Add unit tests for PasswordService\n5. Deploy with both paths (wrapper pattern)\n6. Monitor for 3 days\n7. Remove old code from UserService\n\n**Deliverable:** PasswordService extracted\n**Risk:** Low (isolated functionality)\n**Rollback:** Revert to UserService delegation\n**Testing:** Password reset flow, login with password\n\n### Phase 3: Extract UserRepository (Week 4-5)\n\n**Goal:** Centralize database access, enable testing\n\n**Steps:**\n1. Create `UserRepository.ts` with CRUD operations\n2. Implement repository pattern (find, create, update, delete)\n3. Update UserService to use repository (remove direct SQL)\n4. Test with real database and mock repository\n5. Deploy and monitor\n6. Migrate remaining 80 files to use repository (5-10 files/day)\n\n**Deliverable:** All database access through repository\n**Risk:** Medium (database access is critical)\n**Rollback:** Keep old SQL queries, toggle via feature flag\n**Testing:** CRUD operations, edge cases, transactions\n\n### Phase 4: Extract AuthenticationService (Week 6)\n\n**Goal:** Separate authentication logic\n\n**Steps:**\n1. Create `AuthenticationService.ts`\n2. Move login, logout, token generation logic\n3. Update API controllers to use AuthenticationService\n4. Add comprehensive tests (happy path, failures)\n5. Deploy with feature flag (gradual rollout 1% → 100%)\n6. Remove old authentication code from UserService\n\n**Deliverable:** AuthenticationService extracted\n**Risk:** High (authentication is critical path)\n**Rollback:** Feature flag to old code\n**Testing:** Load test, security audit, edge cases\n\n### Phase 5: Extract AuthorizationService (Week 7)\n\n**Goal:** Separate authorization logic\n\n**Steps:**\n1. Create `AuthorizationService.ts`\n2. Move permission checks and role logic\n3. Update all permission check sites (47 call sites)\n4. Add tests for all permission scenarios\n5. Deploy with gradual rollout\n\n**Deliverable:** AuthorizationService extracted\n**Risk:** High (authorization bugs = security vulnerability)\n**Rollback:** Feature flag to old code\n**Testing:** Permission matrix, escalation tests\n\n### Phase 6: Extract Remaining Services (Week 8)\n\n**Goal:** Complete decomposition\n\n**Steps:**\n1. Extract ProfileService (lower risk)\n2. Extract NotificationService (lower risk)\n3. Delete UserService (should be empty now)\n4. Update all import paths\n5. Final cleanup and documentation\n\n**Deliverable:** Full decomposition complete\n**Risk:** Low (non-critical services)\n**Rollback:** Keep UserService wrapper if issues\n**Testing:** Smoke tests, integration tests\n\n### Phase 7: Async/Await Migration (Ongoing during Phase 3-6)\n\n**Goal:** Modernize callback hell to async/await\n\n**Steps:**\n1. Convert one file at a time (piggyback on service extraction)\n2. Use util.promisify for callback-based libraries\n3. Update tests to use async/await\n4. Remove callback patterns\n\n**Deliverable:** 100% async/await, 0% callbacks\n**Risk:** Low (mechanical transformation)\n**Testing:** Ensure async errors handled\n\n## Parallel Work Streams\n\n**Stream 1 (Senior engineer - Alice):** Lead architecture, critical extractions (Auth)\n**Stream 2 (Mid engineer - Bob):** Repository pattern, database migration\n**Stream 3 (Junior engineer - Carol):** TypeScript migration, testing\n\n**Coordination:** Daily 15-min sync on refactoring progress\n\n## Risk Mitigation\n\n**Risk 1: Production incident during migration**\n- Mitigation: Feature flags, gradual rollout, rollback plan\n- Response: Rollback immediately, fix, resume next day\n\n**Risk 2: Migration takes longer than 8 weeks**\n- Mitigation: Phase 4-5 can be parallelized if needed\n- Response: Extend timeline, but deliver Phase 3 (repository) as interim win\n\n**Risk 3: New bugs introduced**\n- Mitigation: Comprehensive testing at each phase\n- Response: Fix bugs before proceeding to next phase\n\n**Risk 4: Team velocity drops during refactoring**\n- Mitigation: Feature freeze during critical phases (4-5)\n- Response: Communicate to stakeholders, prioritize stability\n\n## Success Criteria\n\n**Phase completion:** All phases complete within 8 weeks\n**Quality:** Test coverage >80%, type coverage 100%\n**Stability:** Zero production incidents caused by refactoring\n**Performance:** No performance regression (p95 latency unchanged)\n**Team:** Engineers confident working in new codebase\n\n**Confidence:** 0.75" \
  --confidence 0.75 \
  --tags "refactoring,migration-strategy,[system-name]"
```

### Step 4: Track Progress and Adapt

```bash
# Weekly progress update
engram reasoning create \
  --title "Refactoring Progress: Week [N] of 8" \
  --task-id [TASK_ID] \
  --content "**Week:** [N] of 8\n**Phase:** [Current phase]\n**Status:** [On track / At risk / Delayed]\n\n**Completed This Week:**\n- [Accomplishment 1]\n- [Accomplishment 2]\n\n**Metrics:**\n- Files migrated: [M] of [Total]\n- Test coverage: [X]% (target: 80%)\n- Type coverage: [Y]% (target: 100%)\n- Production incidents: [N] (target: 0)\n\n**Blockers:**\n- [Blocker 1 if any]\n- [Blocker 2 if any]\n\n**Plan for Next Week:**\n- [Task 1]\n- [Task 2]\n\n**Adjustments:**\n[Any changes to migration plan based on learnings]" \
  --confidence 1.0 \
  --tags "refactoring,progress,week-[N],[system-name]"
```

### Step 5: Link Refactoring Entities

```bash
# Link all refactoring planning
engram relationship create \
  --source-id [TASK_ID] --source-type task \
  --target-id [ASSESSMENT_ID] --target-type context \
  --relationship-type references \
  --agent [AGENT]

engram relationship create \
  --source-id [TASK_ID] --source-type task \
  --target-id [TARGET_ARCH_ID] --target-type reasoning \
  --relationship-type documents \
  --agent [AGENT]

engram relationship create \
  --source-id [TASK_ID] --source-type task \
  --target-id [MIGRATION_STRATEGY_ID] --target-type reasoning \
  --relationship-type documents \
  --agent [AGENT]
```

## Example

User wants to refactor a legacy authentication system slowing down development.

### Step 1: Assess Current State

```bash
ASSESSMENT=$(engram context create \
  --title "Refactoring Assessment: UserService Authentication" \
  --content "## Current State Analysis\n\n**System:** UserService (authentication and user management)\n**Age:** 3 years old, written in Node.js v10 (EOL), JavaScript (no TypeScript)\n**Size:** 3,000 lines in single file, 200 total files in auth module\n**Team familiarity:** Original author departed 6 months ago, only 2 of 8 engineers comfortable modifying\n\n## Code Smells Identified\n\n### Smell 1: God Object (UserService.ts)\n- **Description:** 3,000-line class handling auth, authz, CRUD, emails, sessions, profiles\n- **Impact:** Last 3 bugs took 2 days each to fix. Can't add features without breaking existing.\n- **Severity:** High (P0)\n- **Files:** src/services/UserService.ts (3K lines)\n- **Dependencies:** 47 files import this class\n\n### Smell 2: Circular Dependencies\n- **Description:** UserService → EmailService → TemplateService → UserService\n- **Impact:** Can't test in isolation, deployment order sensitive, imports break\n- **Severity:** High (P0)\n- **Files:** 12 files in circular chain\n\n### Smell 3: Callback Hell\n- **Description:** 6-8 levels of nested callbacks, no async/await\n- **Impact:** Unreadable code, inconsistent error handling, debugging nightmares\n- **Severity:** Medium (P1)\n- **Files:** 35 files using old callback patterns\n\n### Smell 4: No Type Safety\n- **Description:** Pure JavaScript, runtime type errors common\n- **Impact:** 15% of production errors are type-related (TypeError: cannot read property X of undefined)\n- **Severity:** Medium (P1)\n- **Files:** All 200 files\n\n### Smell 5: Scattered Database Access\n- **Description:** Direct SQL in 80 files, no repository pattern, queries duplicated\n- **Impact:** Can't swap database, SQL injection risks, query performance unknown\n- **Severity:** High (P0)\n- **Files:** 80 files with raw SQL\n\n## Impact on Engineering\n\n**Velocity Impact:**\n- Feature development: 2x slower than clean codebases (8 days vs 4 days for typical feature)\n- Bug fixes: 2 days average (vs 4 hours target)\n- Onboarding: 60 days for new engineer to be productive (vs 30 days target)\n- Technical interview candidates decline due to codebase quality\n\n**Quality Impact:**\n- Bug rate: 3× team average (15 bugs/month vs 5 bugs/month)\n- Production incidents: 60% originate from auth system (9 of 15 Q1 incidents)\n- Test coverage: 40% (target: 80%)\n- Rollback rate: 25% (1 in 4 deployments rolled back)\n\n**Risk Assessment:**\n- Bus factor: 2 (if Alice and Bob leave, nobody understands system)\n- Security risk: Medium (SQL injection vulnerabilities found in code review, no input validation)\n- Compliance risk: High (GDPR audit flagged weak password handling)\n\n## Business Impact\n\n**Cost:**\n- Engineering waste: 40% of 8-person auth team capacity on maintenance = 3.2 FTE\n- Opportunity cost: 2 enterprise features deferred Q1 due to tech debt\n- Incident cost: 3 outages in Q1, estimated $50K revenue impact\n- Hiring: Lost 2 senior candidates who cited codebase quality\n\n**Risk:**\n- Security vulnerability could expose user data (compliance fine $500K+)\n- System instability mentioned in 3 enterprise sales calls (lost deals)\n- Team morale: Engineers actively avoid working on auth system\n\n**Recommendation:** Refactor NOW - cost of inaction > cost of refactoring" \
  --source "refactoring-strategy" \
  --tags "refactoring,assessment,userservice-auth" \
  --json | jq -r '.id')

echo "Assessment complete: $ASSESSMENT"
```

(Continued in similar detail with target architecture and migration strategy examples...)

## Querying Refactoring Plans

```bash
# Get refactoring assessments
engram context list | grep "Refactoring Assessment:"

# Get target architectures
engram reasoning list | grep "Target Architecture:"

# Get migration strategies
engram reasoning list | grep "Migration Strategy:"

# Get progress updates
engram reasoning list | grep "Refactoring Progress:"

# Get all refactoring work for a system
engram relationship connected --entity-id [TASK_ID] | grep -E "Refactoring|Migration|Target"
```

## Related Skills

This skill integrates with:
- `engram-system-design` - Design target architecture before refactoring
- `engram-risk-assessment` - Assess refactoring risks and mitigation
- `engram-spike-investigation` - Spike unfamiliar patterns before committing
- `engram-dependency-mapping` - Map code dependencies to plan extraction order
- `engram-release-planning` - Plan refactoring releases with rollback strategies
