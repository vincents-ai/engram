---
name: engram-testing
description: "Comprehensive testing strategy using engram to track test execution, results, and coverage."
---

# Testing with Engram

## Overview

Use engram to track test planning, execution, results, and coverage. Store test outcomes as evidence for compliance and quality assurance.

## When to Use

Use this skill when:
- Planning test strategy
- Writing tests
- Running test suites
- Tracking test coverage
- Debugging test failures
- Providing compliance evidence

## The Pattern

### 1. Plan Test Strategy

Create test plan in engram:

```bash
engram context create \
  --title "Test Plan: [Feature Name]" \
  --content "## Test Strategy\n**Unit Tests:** [Coverage goals]\n**Integration Tests:** [Key scenarios]\n**E2E Tests:** [User workflows]\n\n## Test Cases\n1. [Test case 1]\n2. [Test case 2]\n\n## Coverage Target\n[Percentage or criteria]" \
  --source "test-planning"
```

### 2. Record Test Execution

Store test runs:

```bash
engram context create \
  --title "Test Results: [Feature] - [Date]" \
  --content "**Passed:** [N]/[Total]\n**Failed:** [List]\n**Duration:** [Time]\n**Coverage:** [Percentage]\n\n## Failures\n- [Test 1]: [Reason]\n- [Test 2]: [Reason]" \
  --source "test-results"
```

### 3. Link to Tasks

Connect tests to work:

```bash
engram relationship create \
  --source-id [TASK_ID] \
  --target-id [TEST_RESULTS_ID] \
  --validates
```

### 4. Track Coverage

Store coverage reports:

```bash
engram context create \
  --title "Coverage Report: [Feature]" \
  --content "**Line Coverage:** [Percentage]\n**Branch Coverage:** [Percentage]\n**Uncovered:**\n- [File:line]\n- [File:line]" \
  --source "coverage"
```

## Example

```
Feature: "User authentication API"

[Step 1: Create test plan]
engram context create \
  --title "Test Plan: Auth API" \
  --content "Unit: 100% of auth logic\nIntegration: Login, logout, token refresh\nE2E: Full auth flow"

[Step 2: Run tests]
cargo test

[Step 3: Store results]
engram context create \
  --title "Test Results: Auth API - 2026-01-23" \
  --content "Passed: 47/47\nCoverage: 95%\nDuration: 2.3s"

[Step 4: Link to task]
engram relationship create \
  --source-id [AUTH_TASK] \
  --target-id [TEST_RESULTS_ID] \
  --validates
```

## Integration with Engram

Test data stored as:
- **Context**: Test plans, results, coverage
- **Relationships**: Task validation linkage
- **Compliance**: Evidence for audits

## Querying Test Results

```bash
# Get test results for a task
engram relationship connected --entity-id [TASK] --relationship-type validates

# Find test failures
engram context list | grep "Test Results" | grep "Failed"

# Get coverage reports
engram context list | grep "Coverage Report"

# Search test history
engram context list --source test-results
```

## Test Types

| Type | Purpose | Coverage Goal |
|------|---------|---------------|
| Unit | Function/method logic | 90%+ |
| Integration | Component interaction | 80%+ |
| E2E | User workflows | Critical paths |
| Regression | Prevent bugs | All fixed bugs |
| Performance | Speed/resource usage | Key operations |
| Security | Vulnerabilities | OWASP Top 10 |

## Related Skills

This skill integrates with:
- `engram-test-driven-development` - TDD workflow with red-green-refactor
- `engram-audit-trail` - Track testing progress over time
- `engram-check-compliance` - Tests provide compliance evidence
- `engram-systematic-debugging` - Debug test failures systematically
- `engram-use-memory` - Store test insights for future reference
