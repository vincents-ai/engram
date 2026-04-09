---
name: engram-test-harness-review
description: "Use when auditing a project's testing framework; asks whether unit behavioral integration property scenario BDD flakiness feedback and validation tests exist; evaluates adequacy for each technique across 10 categories and stores findings as context for traceability purposes"
---

# Test Harness Review

## Overview

Systematically audit codebase test framework; classify into project type then evaluate all applicable techniques across 10 categories + store results as context for querying with ADRs tracking over time.

## When to Use

- Before starting a new feature, to understand how safe the existing harness is
- During code review, to assess whether a PR adds the right test types
- When onboarding to a codebase to understand its testing posture
- After a bug is found in production, to understand which test type failed to catch it
- As a recurring quality gate before a release
- After enabling flakiness tracking, to establish a baseline

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

### 2. Project Classification

Before any diagnostic questions, classify project type(s):

| Type | Indicators |
|-----|------------|
| cli-tool | Binary with no HTTP server, no UI |
| web-api | HTTP endpoints REST GraphQL gRPC |
| web-frontend | React Vue HTML CSS browser-rendered UI |
| mobile | iOS Android, React Native Flutter |
| desktop | Electron native OS app |
| infrastructure | Terraform, Pulumi, Nix, IaC definitions |
| ai-ml | Model training, inference, embeddings |
| embedded | Firmware, microcontrollers, hardware interfaces |
| library | Crate/package/module consumed by other code |

Store classification:

```bash
engram context create \
  --title "Project classification: <project>" \
  --content "Types: [cli-tool, ...]" \
  --source "test-harness-review"
# CLASSIFICATION_UUID = ...

engram relationship create \
  --source-id <REVIEW_TASK_UUID> --source-type task \
  --target-id <CLASSIFICATION_UUID> --target-type context \
  --relationship-type relates_to --agent "<your-name>"
```

### 3. Inventory Existing Tests

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

# BDD: check for Gherkin feature files and step definitions
find . -name "*.feature" | grep -v node_modules | sort
grep -r "#\[given\|#\[when\|#\[then" --include="*.rs" -l | grep -v target | sort

# Property-based: check for proptest/quickcheck/hypothesis
grep -r "proptest!\|quickcheck!\|hypothesis" --include="*.rs" --include="*.py" --include="*.ts" -l | grep -v target | sort

# Flakiness tracking: check for FlakinessTracker usage
grep -r "FlakinessTracker\|flakiness_tracker" --include="*.rs" -l | grep -v target | sort

# StructuredFeedback: check for trait implementations
grep -r "StructuredFeedback\|impl.*StructuredFeedback" --include="*.rs" -l | grep -v target | sort

# Count total tests (language-specific)
cargo test -- --list 2>/dev/null | grep ": test" | wc -l   # Rust
pytest --collect-only -q 2>/dev/null | tail -1             # Python
```

Store the inventory:

```bash
engram context create \
  --title "Test inventory: <project>" \
  --content "Total test files: <N>\nTest locations: <list paths>\nTest runner: <cargo test / pytest / jest / etc>\nRaw count: <N> tests\nBDD feature files: <N>\nProperty test files: <N>\nFlakiness tracking: <yes/no>\nStructured feedback: <yes/no>" \
  --source "test-harness-review"
# INVENTORY_UUID = ...

engram relationship create \
  --source-id <REVIEW_TASK_UUID> --source-type task \
  --target-id <INVENTORY_UUID> --target-type context \
  --relationship-type relates_to --agent "<your-name>"
```

### 4. Ten Diagnostic Questions

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
- Tests that verify serialization roundtrips, validation invariants, state machine properties

**Red flags:**
- Every test uses hardcoded inputs only — no generative testing
- Edge cases (empty string, zero, max int, unicode) only tested if someone thought of them manually
- No `from_generic`/`to_generic` roundtrip property tests for entity types

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

#### Q6: Do we have BDD (Behavior-Driven Development) tests?

**What to look for:**
- Gherkin `.feature` files with Given/When/Then scenarios
- Step definition bindings (e.g., `cucumber` crate in Rust, `cucumber-js`, `behave` in Python)
- Feature files written in domain language readable by non-developers
- A shared test world providing state between steps

**Red flags:**
- No `.feature` files exist
- Step definitions exist but no feature files consume them
- All scenarios only cover the happy path
- No scenario for error handling, edge cases, or constraint violations

**Verdict template:**
```
BDD tests: PRESENT / PARTIAL / ABSENT
Evidence: <framework, feature files count, step definition files>
Gap: <which workflows have no BDD coverage>
```

---

#### Q7: Do we have flakiness tracking?

**What to look for:**
- A flakiness tracker that records pass/fail history per test gate
- Configurable window size and failure rate threshold
- Auto-blacklisting of flaky gates (e.g., 30% failure rate over 10 runs)
- Ability to unblacklist gates after fixes are applied
- History frozen once a gate is blacklisted (no mutation of flaky results)

**Red flags:**
- Intermittent test failures are manually investigated each time
- Flaky tests are left in CI without quarantine
- No mechanism to distinguish "always fails" from "sometimes fails"

**Verdict template:**
```
Flakiness tracking: PRESENT / PARTIAL / ABSENT
Evidence: <tracker module, config defaults, blacklist mechanism>
Gap: <which gates lack flakiness monitoring>
```

---

#### Q8: Do we have structured test feedback?

**What to look for:**
- A `StructuredFeedback` trait or equivalent for machine-readable test results
- JSON output for all test/validation/gate results
- ANSI-stripped output for programmatic consumers
- Unified status codes (success/failed/warning/skipped) across result types
- Implementations for `ActionResult`, `ValidationResult`, `GateResult`, `WorkflowExecutionResult`

**Red flags:**
- Test results are only human-readable (coloured terminal output)
- No JSON output mode for CI integration
- Different result types have inconsistent status representations

**Verdict template:**
```
Structured feedback: PRESENT / PARTIAL / ABSENT
Evidence: <trait/module, implemented result types>
Gap: <which result types lack structured output>
```

---

#### Q9: Do we have validation and commit-gate tests?

**What to look for:**
- Pre-commit hooks that validate commit messages reference task UUIDs
- `engram validate check` or equivalent validating entity integrity
- Gate-based validation (quality gates with pass/fail scores)
- Relationship validation (orphan detection, cycle prevention)
- Entity validation (required fields, format constraints)

**Red flags:**
- Commits can be made without task references
- No validation that linked entities still exist
- No cycle detection in relationship graphs

**Verdict template:**
```
Validation/commit-gate tests: PRESENT / PARTIAL / ABSENT
Evidence: <hook mechanism, gate types, validation rules>
Gap: <which validations are missing>
```

---

#### Q10: Do we have observability for test results?

**What to look for:**
- Test execution time tracking
- Coverage reporting (line, branch, function)
- Historical trend data for test pass rates
- Integration with CI dashboards

**Red flags:**
- No coverage data collected
- Test times not tracked — no way to detect slow tests
- No visibility into which tests fail most frequently

**Verdict template:**
```
Test observability: PRESENT / PARTIAL / ABSENT
Evidence: <coverage tool, CI integration, metrics>
Gap: <what is not measured>
```

---

### 5. Assess Overall Harness Adequacy

Answer: **Is the test harness good enough to develop safely?**

Use this rubric:

| Level | Criteria |
|-------|----------|
| **Strong** | 8+ of 10 types present, tests run in CI, failures block merge, flakiness tracked, coverage >= 80% |
| **Adequate** | Unit + integration present, at least 3 of the other 8, CI enforced |
| **Weak** | Unit only, or integration only, no CI enforcement, no flakiness tracking |
| **Absent** | Fewer than 3 types present, or tests exist but do not run reliably |

Store the verdict:

```bash
engram context create \
  --title "Test harness verdict: <project> — <Strong|Adequate|Weak|Absent>" \
  --content "Overall: <level>\n\nUnit: <verdict>\nBehavioral: <verdict>\nIntegration: <verdict>\nProperty: <verdict>\nScenario: <verdict>\nBDD: <verdict>\nFlakiness tracking: <verdict>\nStructured feedback: <verdict>\nValidation/commit-gate: <verdict>\nTest observability: <verdict>\n\nCI enforced: yes/no\nCoverage: <% or unknown>\n\nTop gaps:\n1. <gap>\n2. <gap>\n3. <gap>\n\nRecommendation: <what to add first>" \
  --source "test-harness-review"
# VERDICT_UUID = ...

engram relationship create \
  --source-id <REVIEW_TASK_UUID> --source-type task \
  --target-id <VERDICT_UUID> --target-type context \
  --relationship-type relates_to --agent "<your-name>"
```

### 6. Record Recommendations as Reasoning

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

### 7. Close the Task

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
find . -name "*.feature" | wc -l
# 1 BDD feature file
grep -r "proptest!" --include="*.rs" -l | grep -v target | wc -l
# 1 property test file

engram context create \
  --title "Test inventory: engram CLI" \
  --content "187 unit/integration tests across 23 files. Runner: cargo test. 1 BDD feature file (tests/bdd/). 1 property test file (tests/property_tests.rs) using proptest. FlakinessTracker in src/validation/flakiness_tracker.rs. StructuredFeedback trait in src/feedback/mod.rs." \
  --source "test-harness-review"
# INVENTORY_UUID = ctx-001

engram relationship create \
  --source-id task-abc --source-type task \
  --target-id ctx-001 --target-type context \
  --relationship-type relates_to --agent "reviewer"

[Ten questions]
# Q1 Unit: PRESENT — 187 tests, mostly in #[cfg(test)] blocks per module
# Q2 Behavioral: PARTIAL — CLI output tests check observable output, but only happy path
# Q3 Integration: PARTIAL — some tests spin up real storage, but no HTTP or subprocess tests
# Q4 Property: PRESENT — proptest for entity roundtrips, validation invariants, workflow state machine, NLQ classify-never-panics
# Q5 Scenario: ABSENT — no end-to-end workflow tests from CLI entry point
# Q6 BDD: PRESENT — cucumber BDD with 60+ step definitions in tests/bdd/, 1 feature file, covers tasks/contexts/knowledge/reasoning/sessions/relationships/sync/workflows
# Q7 Flakiness tracking: PRESENT — FlakinessTracker with configurable window (default 10), 30% failure threshold, auto-blacklist, history freeze on flaky gates
# Q8 Structured feedback: PRESENT — StructuredFeedback trait with JSON, ANSI-stripped output, unified FeedbackStatus for ActionResult/ValidationResult/GateResult/WorkflowExecutionResult
# Q9 Validation/commit-gate: PRESENT — engram validate check, pre-commit hook enforcing task UUID in commits, quality gates with scores
# Q10 Test observability: PARTIAL — execution time tracked per gate, no coverage reporting

[Verdict]
engram context create \
  --title "Test harness verdict: engram CLI — Strong" \
  --content "Overall: Strong\n\nUnit: PRESENT\nBehavioral: PARTIAL\nIntegration: PARTIAL\nProperty: PRESENT\nScenario: ABSENT\nBDD: PRESENT\nFlakiness tracking: PRESENT\nStructured feedback: PRESENT\nValidation/commit-gate: PRESENT\nTest observability: PARTIAL\n\nCI enforced: yes\nCoverage: unknown\n\nTop gaps:\n1. No scenario tests exercising full CLI workflows\n2. Behavioral tests only cover happy path\n3. No coverage reporting\n\nRecommendation: Add CLI scenario tests for critical workflows first (highest ROI for regression prevention)" \
  --source "test-harness-review"
# VERDICT_UUID = ctx-002

engram relationship create \
  --source-id task-abc --source-type task \
  --target-id ctx-002 --target-type context \
  --relationship-type relates_to --agent "reviewer"

[Reasoning]
engram reasoning create \
  --title "Priority: CLI scenario tests" \
  --task-id task-abc \
  --content "Property tests and BDD now cover entity integrity and domain workflows. The remaining gap is full CLI integration — exercising commands from shell entry point through to git refs persistence. This would have caught the workflow context injection regression."
# REASONING_UUID = rsn-003

engram relationship create \
  --source-id task-abc --source-type task \
  --target-id rsn-003 --target-type reasoning \
  --relationship-type explains --agent "reviewer"

[Close]
engram validate check
engram task update task-abc --status done \
  --outcome "Test harness: Strong. Top gap: no CLI scenario tests."
```

## Related Skills

- `engram-test-driven-development` — implement new tests using TDD once gaps are identified
- `engram-systematic-debugging` — when a test failure needs root-cause investigation
- `engram-audit-trail` — store the review as a timestamped audit record
- `engram-requesting-code-review` — review a PR's tests before merging
- `engram-edge-cases` — systematic edge case identification using property-based and fuzzing techniques
- `engram-testing` — comprehensive testing strategy tracking with engram
