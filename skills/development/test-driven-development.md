---
name: test-driven-development-engram
description: "Engram-integrated version. Use when implementing any feature or bugfix - integrates engram validation checkpoints at each TDD phase."
---

# Test-Driven Development (Engram-Integrated)

## Overview

Write the test first. Watch it fail. Write minimal code to pass. Integrate engram validation at each phase for quality gates.

## Key Changes from Original

**Original:** Manual verification of RED/GREEN/REFACTOR phases
**Engram-integrated:** Call `engram validate check` at each phase to enforce quality gates and store evidence as reasoning entities.

## The Iron Law

```
NO PRODUCTION CODE WITHOUT A FAILING TEST FIRST
```

Write code before the test? Delete it. Start over. No exceptions.

## Red-Green-Refactor with Engram

### Phase 1: RED - Write Failing Test

Write one minimal test showing what should happen.

```bash
# Create reasoning entity for RED phase
engram reasoning create \
  --title "[Feature] TDD Phase: RED" \
  --task-id [TASK_ID] \
  --content "**Test written:** \`[test_name]\`\n\n**Location:** \`[test/file/path.rs]\`\n\n**Test code:**\n\`\`\`[language]\n[test code]\n\`\`\`\n\n**Expected behavior:**\n[What the test should verify]\n\n**Files created/modified:**\n- \`[test/file/path.rs]\`" \
  --confidence 1.0 \
  --tags "tdd,red,[feature-name]"
```

### Phase 2: Verify RED - Watch It Fail (Engram Validation)

**MANDATORY. Never skip.**

```bash
# Run test to verify it fails
pytest [test/path] -v

# Create reasoning entity with evidence
engram reasoning create \
  --title "[Feature] TDD Phase: RED Verification" \
  --task-id [TASK_ID] \
  --content "**Command run:** \`pytest [test/path] -v\`\n\n**Output:**\n\`\`\`\n[test output showing failure]\n\`\`\`\n\n**Exit code:** [non-zero]\n\n**Verification:**\n✅ Test fails (not errors)\n✅ Failure message is expected: \"[expected message]\"\n✅ Fails because feature missing (not typos)\n\n**Status:** READY FOR GREEN PHASE" \
  --confidence 1.0 \
  --tags "tdd,red-verification,[feature-name]"

# Enforce: validate check must pass before proceeding
engram validate check --dry-run
```

**Test passes?** You're testing existing behavior. Fix test.

**Test errors?** Fix error, re-run until it fails correctly.

### Phase 3: GREEN - Minimal Code

Write simplest code to pass the test.

```bash
# Create reasoning entity for GREEN phase
engram reasoning create \
  --title "[Feature] TDD Phase: GREEN" \
  --task-id [TASK_ID] \
  --content "**Implementation code:**\n\`\`\`[language]\n[minimal implementation code]\n\`\`\`\n\n**Files created/modified:**\n- \`[implementation/file.rs]\`\n\n**Justification:**\n- Only implements what test requires\n- No extra features (YAGNI)\n- No refactoring (done in REFACTOR phase)" \
  --confidence 1.0 \
  --tags "tdd,green,[feature-name]"
```

### Phase 4: Verify GREEN - Watch It Pass (Engram Validation)

**MANDATORY.**

```bash
# Run test to verify it passes
pytest [test/path] -v

# Create reasoning entity with evidence
engram reasoning create \
  --title "[Feature] TDD Phase: GREEN Verification" \
  --task-id [TASK_ID] \
  --content "**Command run:** \`pytest [test/path] -v\`\n\n**Output:**\n\`\`\`\n[test output showing pass]\n\`\`\`\n\n**Exit code:** [0]\n\n**Verification:**\n✅ Test passes\n✅ All other tests still pass\n✅ Output pristine (no errors, warnings)\n\n**Status:** READY FOR REFACTOR PHASE" \
  --confidence 1.0 \
  --tags "tdd,green-verification,[feature-name]"

# Enforce: validate check must pass before proceeding
engram validate check
```

**Test fails?** Fix code, not test.

**Other tests fail?** Fix now.

### Phase 5: REFACTOR - Clean Up

After green only:

```bash
# Create reasoning entity for REFACTOR phase
engram reasoning create \
  --title "[Feature] TDD Phase: REFACTOR" \
  --task-id [TASK_ID] \
  --content "**Refactoring changes:**\n- Removed duplication: [what was extracted]\n- Improved names: [what was renamed]\n- Extracted helpers: [what was created]\n\n**Code after refactor:**\n\`\`\`[language]\n[refactored code]\n\`\`\`\n\n**Verification:**\n✅ Tests still pass after refactor\n✅ No new behavior added\n✅ Code is cleaner" \
  --confidence 1.0 \
  --tags "tdd,refactor,[feature-name]"

# Final validation
engram validate check
```

## Good Tests

| Quality | Good | Bad |
|---------|------|-----|
| **Minimal** | One behavior. "and" in name? Split it. | `test('validates email and domain and whitespace')` |
| **Clear** | Name describes behavior | `test('test1')` |
| **Shows intent** | Demonstrates desired API | Obscures what code should do |

## Why Engram Integration Matters

1. **Audit Trail:** Every TDD phase is documented in engram
2. **Evidence-Based:** Verification outputs stored as reasoning
3. **Queryable:** `engram reasoning list --task-id [ID]` shows full TDD history
4. **Quality Gates:** `engram validate check` enforced at each phase
5. **Prevents Regression:** Full history prevents "just this once" shortcuts

## Example Workflow

```bash
# TDD Cycle 1: User login

# RED
engram reasoning create \
  --title "Auth TDD: RED - Login rejects invalid password" \
  --task-id $TASK \
  --content "Test: login_rejects_invalid_password\nCode: [test code]" \
  --confidence 1.0 \
  --tags "tdd,red,auth"

# Verify RED
pytest tests/auth/login_test.py::login_rejects_invalid_password -v

engram reasoning create \
  --title "Auth TDD: RED Verification" \
  --task-id $TASK \
  --content "Output: FAILED\nExpected: Password mismatch error\nStatus: CORRECT FAILURE" \
  --confidence 1.0 \
  --tags "tdd,red,auth"

engram validate check --dry-run

# GREEN
engram reasoning create \
  --title "Auth TDD: GREEN - Minimal password check" \
  --task-id $TASK \
  --content "Code: [minimal implementation]" \
  --confidence 1.0 \
  --tags "tdd,green,auth"

# Verify GREEN
pytest tests/auth/login_test.py::login_rejects_invalid_password -v

engram reasoning create \
  --title "Auth TDD: GREEN Verification" \
  --task-id $TASK \
  --content "Output: PASSED\nAll tests: 15/15 passed\nStatus: CORRECT" \
  --confidence 1.0 \
  --tags "tdd,green,auth"

engram validate check

# REFACTOR
engram reasoning create \
  --title "Auth TDD: REFACTOR - Extracted validation helper" \
  --task-id $TASK \
  --content "Extracted validate_password function\nTests still pass: YES" \
  --confidence 1.0 \
  --tags "tdd,refactor,auth"

engram validate check
```

## Querying TDD History

```bash
# Get all TDD phases for a task
engram reasoning list --task-id [TASK_ID] | grep tdd

# Get specific phase
engram reasoning show [REASONING_ID]

# Get all phases with evidence
engram relationship connected --entity-id [TASK_ID] --relationship-type documents | grep -E "tdd|TDD"
```

## Related Skills

This skill integrates with:
- `engram-testing` - Track test execution and coverage
- `engram-use-memory` - Store TDD cycles for learning
- `engram-audit-trail` - Complete record of red-green-refactor cycles
- `engram-check-compliance` - TDD provides test evidence for compliance
- `engram-systematic-debugging` - Debug failing tests systematically
