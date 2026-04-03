---
name: engram-test-driven-development
description: "Engram-integrated version. Use when implementing any feature or bugfix - integrates engram validation checkpoints at each TDD phase."
---

# Test-Driven Development

## Overview

Write the test first. Watch it fail. Write minimal code to pass. Store evidence at each phase in engram. Run `engram validate check` at each gate.

## The Iron Law

```
NO PRODUCTION CODE WITHOUT A FAILING TEST FIRST
```

Write code before the test? Delete it. Start over. No exceptions.

## The Pattern

### 0. Search First

Before starting any TDD cycle, check for existing tests or context:

```bash
engram ask query "<feature or function name> test"
engram task show <UUID>
```

### 1. Anchor Work

```bash
engram task create --title "TDD: <feature description>"
# TASK_UUID = ...
engram task update <TASK_UUID> --status in_progress
```

### Phase 1: RED — Write the Failing Test

Write one minimal test that shows what should happen. One behaviour per test.

Store evidence:

```bash
engram context create \
  --title "RED: <test_name>" \
  --content "Test written: <test_name>\nLocation: <test/file/path>\nTest code:\n<full test code>\nExpected behaviour: <what this test verifies>" \
  --source "<test-file-path>"
# RED_CTX_UUID = ...

engram relationship create \
  --source-id <TASK_UUID> --source-type task \
  --target-id <RED_CTX_UUID> --target-type context \
  --relationship-type relates_to --agent "<name>"
```

### Phase 2: Verify RED — Watch It Fail

**Mandatory. Never skip.**

Run the test directly in your shell:

```bash
<test runner command>
```

Store the output as evidence:

```bash
engram reasoning create \
  --title "RED verification: <test_name>" \
  --task-id <TASK_UUID> \
  --content "RED verification:\nCommand: <test command>\nOutput:\n<full test output showing failure>\nExit code: non-zero\nVerification:\n- Test fails (not errors): YES\n- Failure message is expected: YES\n- Fails because feature missing (not typos): YES\nStatus: READY FOR GREEN"
# RED_VERIFY_UUID = ...

engram relationship create \
  --source-id <TASK_UUID> --source-type task \
  --target-id <RED_VERIFY_UUID> --target-type reasoning \
  --relationship-type explains --agent "<name>"
```

Run the quality gate:

```bash
engram validate check
```

**Test passes?** You are testing existing behaviour. Fix the test.
**Test errors?** Fix the error. Re-run until it fails correctly.

### Phase 3: GREEN — Minimal Code

Write the simplest code that makes the test pass. No extra features (YAGNI).

Run the test directly in your shell:

```bash
<test runner command>
```

Store the implementation:

```bash
engram context create \
  --title "GREEN: <test_name>" \
  --content "Implementation:\n<minimal implementation code>\nFiles modified:\n- <file>: <what changed>\nJustification: implements only what the test requires, no extra logic." \
  --source "<implementation-file-path>"
# GREEN_CTX_UUID = ...

engram relationship create \
  --source-id <TASK_UUID> --source-type task \
  --target-id <GREEN_CTX_UUID> --target-type context \
  --relationship-type relates_to --agent "<name>"
```

Store the verification:

```bash
engram reasoning create \
  --title "GREEN verification: <test_name>" \
  --task-id <TASK_UUID> \
  --content "GREEN verification:\nCommand: <test command>\nOutput:\n<full test output showing pass>\nExit code: 0\nAll other tests pass: YES\nStatus: READY FOR REFACTOR"
# GREEN_VERIFY_UUID = ...

engram relationship create \
  --source-id <TASK_UUID> --source-type task \
  --target-id <GREEN_VERIFY_UUID> --target-type reasoning \
  --relationship-type explains --agent "<name>"
```

Run the quality gate:

```bash
engram validate check
```

**Test fails?** Fix the code, not the test.
**Other tests fail?** Fix them now before proceeding.

### Phase 4: REFACTOR — Clean Up

Only after green. Do not add behaviour.

Run the full test suite directly in your shell:

```bash
<test runner command>
```

```bash
engram reasoning create \
  --title "REFACTOR: <test_name>" \
  --task-id <TASK_UUID> \
  --content "REFACTOR:\nChanges made:\n- Removed duplication: <what>\n- Improved names: <what>\n- Extracted helpers: <what>\nCode after:\n<refactored code>\nVerification: tests still pass after refactor: YES\nNew behaviour added: NO"
# REFACTOR_UUID = ...

engram relationship create \
  --source-id <TASK_UUID> --source-type task \
  --target-id <REFACTOR_UUID> --target-type reasoning \
  --relationship-type explains --agent "<name>"
```

Run final gate:

```bash
engram validate check
```

### Phase 5: Validate and Next Step

```bash
engram task update <TASK_UUID> --status done
engram next
```

## Terminal Commands

Run terminal commands directly in your shell. Do not use `engram sandbox execute` — that command does not exist.

If you need elevated permissions or human approval:

```bash
engram escalation create \
  --agent "<name>" \
  --operation-type "<type>" \
  --operation "<what you need to do>" \
  --justification "<why this is needed>"
```

## Good Tests

| Quality | Good | Bad |
|---------|------|-----|
| **Minimal** | One behaviour. "and" in name? Split it. | `test_validates_email_and_domain_and_whitespace` |
| **Clear** | Name describes the behaviour | `test1` |
| **Shows intent** | Demonstrates the desired API | Obscures what code should do |

## Example

```
Feature: "User login rejects invalid password"

[Search first]
engram ask query "login password validation test"

[Anchor]
engram task create --title "TDD: Login rejects invalid password"
# TASK_UUID = task-001
engram task update task-001 --status in_progress

[RED: Write failing test]
engram context create \
  --title "RED: login_rejects_invalid_password" \
  --content "Test: login_rejects_invalid_password\nLocation: tests/auth/login_test.rs\nCode:\n#[tokio::test]\nasync fn login_rejects_invalid_password() {\n    let resp = post(\"/auth/login\", json!({\"password\":\"wrong\"})).await;\n    assert_eq!(resp.status(), 401);\n}\nExpected: returns 401 Unauthorized on wrong password" \
  --source "tests/auth/login_test.rs"
# RED_CTX_UUID = ctx-002

engram relationship create \
  --source-id task-001 --source-type task \
  --target-id ctx-002 --target-type context \
  --relationship-type relates_to --agent "developer"

[Verify RED — run directly in shell]
cargo test login_rejects_invalid_password -- --nocapture

engram reasoning create \
  --title "RED verification: login_rejects_invalid_password" \
  --task-id task-001 \
  --content "RED verification:\nCommand: cargo test login_rejects_invalid_password\nOutput: FAILED — cannot find function 'post'\nFails because feature missing: YES\nStatus: READY FOR GREEN"
# RED_VERIFY_UUID = rsn-003

engram relationship create \
  --source-id task-001 --source-type task \
  --target-id rsn-003 --target-type reasoning \
  --relationship-type explains --agent "developer"

engram validate check

[GREEN: Minimal implementation]
cargo test login_rejects_invalid_password -- --nocapture

engram context create \
  --title "GREEN: login_rejects_invalid_password" \
  --content "Implementation: added post() helper and login handler returning 401 on bcrypt mismatch.\nFiles: src/api/auth/login.rs, tests/auth/helpers.rs" \
  --source "src/api/auth/login.rs"
# GREEN_CTX_UUID = ctx-004

engram relationship create \
  --source-id task-001 --source-type task \
  --target-id ctx-004 --target-type context \
  --relationship-type relates_to --agent "developer"

engram reasoning create \
  --title "GREEN verification: login_rejects_invalid_password" \
  --task-id task-001 \
  --content "GREEN verification:\nOutput: PASSED — 1 test passed\nAll other tests: 15/15 pass\nStatus: READY FOR REFACTOR"
# GREEN_VERIFY_UUID = rsn-005

engram relationship create \
  --source-id task-001 --source-type task \
  --target-id rsn-005 --target-type reasoning \
  --relationship-type explains --agent "developer"

engram validate check

[REFACTOR]
cargo test

engram reasoning create \
  --title "REFACTOR: login_rejects_invalid_password" \
  --task-id task-001 \
  --content "REFACTOR: extracted validate_credentials() helper from handler. No new behaviour. Tests: 16/16 pass."

engram validate check

[Close]
engram task update task-001 --status done
engram next
```

## Related Skills

- `engram-systematic-debugging` — debug failing tests systematically
- `engram-audit-trail` — complete record of TDD cycles
- `engram-subagent-driven-development` — execute TDD plan with subagents
