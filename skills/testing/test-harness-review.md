---
name: engram-test-harness-review
description: "Audit the test harness of any codebase. Asks whether behavioral, integration, property, scenario, and unit tests exist, evaluates their adequacy, and stores findings as engram context for traceability."
---

# Test Harness Review

## Overview

Systematically audit a project's testing coverage across five dimensions: unit, behavioral, integration, property, and scenario tests. Stores findings in engram so they can be queried, referenced in ADRs, and tracked over time.

## When to Use

- Before starting a new feature, to understand how safe the existing harness is
- During code review, to assess whether a PR adds the right test types
- When onboarding to a codebase to understand its testing posture
- After a bug is found in production, to understand which test type failed to catch it
- As a recurring quality gate before a release

## The Pattern

### 0. Search for Prior Reviews

```bash
engram ask query "test harness review"
```

If a recent review exists, compare against it rather than starting from scratch.

### 1. Anchor in Engram

```bash
engram task create --title "Test harness review: <project or module name>"
# REVIEW_TASK_UUID = ...
engram task update <REVIEW_TASK_UUID> --status in_progress
```

### 2. Inventory the Test Suite

Before forming any opinions, count what actually exists. Run in the project root:

```bash
# Count test files by type/location
find . -name "*test*" -o -name "*spec*" | grep -v target | grep -v node_modules | sort

# Rust: count #[test] and #[cfg(test)] blocks
grep -r "#\[test\]" --include="*.rs" -l | grep -v target | sort
grep -r "#\[cfg(test)\]" --include="*.rs" -l | grep -v target | sort

# JS/TS: count describe/it/test blocks
grep -r "describe\|it(\|test(" --include="*.test.*" --include="*.spec.*" -l | grep -v node_modules | sort

# Python: count test_ functions and classes
grep -r "def test_\|class Test" --include="*.py" -l | grep -v __pycache__ | sort

# Count total tests (language-specific)
cargo test -- --list 2>/dev/null | grep ": test" | wc -l   # Rust
pytest --collect-only -q 2>/dev/null | tail -1             # Python
```

Store the inventory:

```bash
engram context create \
  --title "Test inventory: <project>" \
  --content "Total test files: <N>\nTest locations: <list paths>\nTest runner: <cargo test / pytest / jest / etc>\nRaw count: <N> tests" \
  --source "test-harness-review"
# INVENTORY_UUID = ...

engram relationship create \
  --source-id <REVIEW_TASK_UUID> --source-type task \
  --target-id <INVENTORY_UUID> --target-type context \
  --relationship-type relates_to --agent "<your-name>"
```

### 3. Five Diagnostic Questions

Work through each question. Record a verdict and evidence for each.

---

#### Q1: Do we have unit tests?

**What to look for:**
- Tests that cover a single function, method, or module in isolation
- Dependencies are mocked or stubbed
- Fast to run (milliseconds each)
- Named for what they verify, not implementation details

**Red flags:**
- No tests at all
- Tests only exist in integration test files
- Tests that require a database, network, or filesystem to run
- Single monolithic test file for the whole codebase

**Verdict template:**
```
Unit tests: PRESENT / PARTIAL / ABSENT
Evidence: <file paths and counts>
Gap: <what is not covered>
```

---

#### Q2: Do we have behavioral tests?

**What to look for:**
- Tests written from the perspective of observable behaviour, not implementation
- Given/When/Then structure (explicit or implicit)
- Tests that would still pass if the internal implementation changed completely
- Often phrased as "when X happens, the system does Y"

**Red flags:**
- Tests that assert on private/internal state (white-box only)
- Tests that break every time code is refactored even when behaviour is unchanged
- No tests at the public API boundary

**Verdict template:**
```
Behavioral tests: PRESENT / PARTIAL / ABSENT
Evidence: <file paths or test names>
Gap: <which behaviours have no test>
```

---

#### Q3: Do we have integration tests?

**What to look for:**
- Tests that exercise two or more real components together
- Real I/O (real DB, real filesystem, real HTTP, real subprocess)
- Separate test binary, test directory, or tagged test suite
- Slower than unit tests — seconds, not milliseconds

**Red flags:**
- All tests mock every dependency (no real I/O tested anywhere)
- No tests that verify components wire together correctly
- Integration tests that are actually unit tests with a misleading name

**Verdict template:**
```
Integration tests: PRESENT / PARTIAL / ABSENT
Evidence: <file paths, test suite names>
Gap: <which integration points are untested>
```

---

#### Q4: Do we have property-based tests?

**What to look for:**
- Tests that generate random inputs and verify invariants hold for all of them
- Use of frameworks: `proptest` / `quickcheck` (Rust), `hypothesis` (Python), `fast-check` (JS)
- Properties expressed as "for all X, P(X) must be true"
- Tests that shrink failing cases to minimal counterexamples

**Red flags:**
- Every test uses hardcoded inputs only — no generative testing
- Edge cases (empty string, zero, max int, unicode) only tested if someone thought of them manually

**Verdict template:**
```
Property tests: PRESENT / PARTIAL / ABSENT
Evidence: <framework detected, file paths>
Gap: <which functions/invariants have no property coverage>
```

---

#### Q5: Do we have scenario tests?

**What to look for:**
- Tests that exercise a complete end-to-end user workflow or business scenario
- Realistic data, not toy fixtures
- Multiple steps in sequence that mirror how a real user would use the system
- Often called "acceptance tests", "E2E tests", or "smoke tests"

**Red flags:**
- No tests that exercise the system from the entry point (CLI, HTTP endpoint, UI) through to persistence
- All tests stop at the service layer — nothing tests the full stack
- No tests for the unhappy path through a complete flow

**Verdict template:**
```
Scenario tests: PRESENT / PARTIAL / ABSENT
Evidence: <file paths, runner>
Gap: <which scenarios are missing>
```

---

### 4. Assess Overall Harness Adequacy

Answer: **Is the test harness good enough to develop safely?**

Use this rubric:

| Level | Criteria |
|-------|----------|
| **Strong** | All 5 types present, tests run in CI, failures block merge, coverage ≥ 80% |
| **Adequate** | Unit + integration present, at least 1 of the other 3, CI enforced |
| **Weak** | Unit only, or integration only, no CI enforcement |
| **Absent** | Fewer than 2 types present, or tests exist but do not run reliably |

Store the verdict:

```bash
engram context create \
  --title "Test harness verdict: <project> — <Strong|Adequate|Weak|Absent>" \
  --content "Overall: <level>\n\nUnit: <verdict>\nBehavioral: <verdict>\nIntegration: <verdict>\nProperty: <verdict>\nScenario: <verdict>\n\nCI enforced: yes/no\nCoverage: <% or unknown>\n\nTop gaps:\n1. <gap>\n2. <gap>\n3. <gap>\n\nRecommendation: <what to add first>" \
  --source "test-harness-review"
# VERDICT_UUID = ...

engram relationship create \
  --source-id <REVIEW_TASK_UUID> --source-type task \
  --target-id <VERDICT_UUID> --target-type context \
  --relationship-type relates_to --agent "<your-name>"
```

### 5. Record Recommendations as Reasoning

```bash
engram reasoning create \
  --title "Test harness gaps and priority fixes: <project>" \
  --task-id <REVIEW_TASK_UUID> \
  --content "Gaps identified: <list>\nHighest priority fix: <type> — because <reason>\nSecond priority: <type> — because <reason>\nBlocking risk: <what failure mode is most likely given current harness>"
# REASONING_UUID = ...

engram relationship create \
  --source-id <REVIEW_TASK_UUID> --source-type task \
  --target-id <REASONING_UUID> --target-type reasoning \
  --relationship-type explains --agent "<your-name>"
```

### 6. Close the Task

```bash
engram validate check
engram task update <REVIEW_TASK_UUID> --status done \
  --outcome "Test harness reviewed: <Strong|Adequate|Weak|Absent>. Top gap: <one line>"
```

## Example

```
[Search first]
engram ask query "test harness review engram"
# No prior review found

[Anchor]
engram task create --title "Test harness review: engram CLI"
# REVIEW_TASK_UUID = task-abc
engram task update task-abc --status in_progress

[Inventory]
grep -r "#\[test\]" --include="*.rs" -l | grep -v target | wc -l
# 23 files with tests
cargo test -- --list 2>/dev/null | grep ": test" | wc -l
# 187 tests

engram context create \
  --title "Test inventory: engram CLI" \
  --content "187 unit/integration tests across 23 files. Runner: cargo test. No proptest or hypothesis found." \
  --source "test-harness-review"
# INVENTORY_UUID = ctx-001

engram relationship create \
  --source-id task-abc --source-type task \
  --target-id ctx-001 --target-type context \
  --relationship-type relates_to --agent "reviewer"

[Five questions]
# Q1 Unit: PRESENT — 187 tests, mostly in #[cfg(test)] blocks per module
# Q2 Behavioral: PARTIAL — CLI output tests check observable output, but only happy path
# Q3 Integration: PARTIAL — some tests spin up real storage, but no HTTP or subprocess tests
# Q4 Property: ABSENT — no proptest or quickcheck usage found
# Q5 Scenario: ABSENT — no end-to-end workflow tests from CLI entry point

[Verdict]
engram context create \
  --title "Test harness verdict: engram CLI — Adequate" \
  --content "Overall: Adequate\n\nUnit: PRESENT\nBehavioral: PARTIAL\nIntegration: PARTIAL\nProperty: ABSENT\nScenario: ABSENT\n\nCI enforced: yes\nCoverage: ~65% estimated\n\nTop gaps:\n1. No property tests for entity ID parsing and validation\n2. No scenario tests exercising full CLI workflows\n3. Behavioral tests only cover happy path\n\nRecommendation: Add proptest for ID parsing first (highest ROI, catches edge cases in critical path)" \
  --source "test-harness-review"
# VERDICT_UUID = ctx-002

engram relationship create \
  --source-id task-abc --source-type task \
  --target-id ctx-002 --target-type context \
  --relationship-type relates_to --agent "reviewer"

[Reasoning]
engram reasoning create \
  --title "Priority: property tests for ID parsing" \
  --task-id task-abc \
  --content "ID parsing is on the critical path of every command. A malformed UUID silently corrupts relationships. Property tests would catch this class of bug cheaply. Scenario tests are second priority — they would have caught the workflow context injection regression."
# REASONING_UUID = rsn-003

engram relationship create \
  --source-id task-abc --source-type task \
  --target-id rsn-003 --target-type reasoning \
  --relationship-type explains --agent "reviewer"

[Close]
engram validate check
engram task update task-abc --status done \
  --outcome "Test harness: Adequate. Top gap: no property or scenario tests."
```

## Related Skills

- `engram-test-driven-development` — implement new tests using TDD once gaps are identified
- `engram-systematic-debugging` — when a test failure needs root-cause investigation
- `engram-audit-trail` — store the review as a timestamped audit record
- `engram-requesting-code-review` — review a PR's tests before merging
