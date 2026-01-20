# Regression Test Framework

## Overview

The regression test framework ensures that previously reported bugs and issues do not reoccur in future releases. It integrates with the existing BDD (Behavior-Driven Development) framework using Cucumber and Gherkin syntax.

## Structure

```
tests/
├── features/
│   ├── regression/                      # Regression-specific features
│   │   ├── workflow_instance_persistence.feature
│   │   ├── cli_error_messages.feature
│   │   ├── nlq_query_reliability.feature
│   │   └── ...
│   ├── task.feature                     # Regular feature tests
│   ├── context.feature
│   └── ...
├── bdd/                                 # Step implementations
│   ├── mod.rs
│   ├── steps.rs
│   └── workflow_steps.rs
├── regression_plan.md                   # Planning document
└── bdd_runner.rs                        # Test runner
```

## Naming Convention

Regression test files follow this pattern:
- **File name**: Descriptive name of the issue/bug (e.g., `workflow_instance_persistence.feature`)
- **Tag**: All regression tests use `@regression` tag
- **Feature title**: Clear description of what regression is being prevented

## Creating New Regression Tests

### 1. Document the Issue

When a bug is reported:
1. Add it to `tests/regression_plan.md` with:
   - Test scenario description
   - Expected vs actual behavior
   - Steps to reproduce
   - Priority level

### 2. Create a Feature File

Create a new `.feature` file in `tests/features/regression/`:

```gherkin
@regression
Feature: [Issue Description]
  As a [user/developer/agent]
  I want [functionality]
  So that [benefit]

  Background:
    Given I have a workspace
    And I am logged in as agent "test-agent"

  Scenario: [Specific test case]
    Given [preconditions]
    When [action]
    Then [expected outcome]
```

### 3. Implement Step Definitions (if needed)

If your regression test requires new steps not already in `tests/bdd/steps.rs`:

1. Add new step functions to `steps.rs`
2. Use the `#[given]`, `#[when]`, `#[then]` attributes
3. Update the `EngramWorld` struct if additional state is needed

### 4. Run the Tests

```bash
# Run all regression tests
cargo test --test bdd -- --tags @regression

# Run specific regression test
cargo test --test bdd -- workflow_instance_persistence

# Run all BDD tests (including regression)
cargo test --test bdd
```

## Existing Regression Tests

### 1. Workflow Instance Persistence
**File**: `workflow_instance_persistence.feature`  
**Issue**: Workflow instances were not persisting correctly, causing "Not found" errors immediately after creation.  
**Tests**:
- Immediate query after workflow start
- Multiple independent workflow instances

### 2. CLI Error Messages
**File**: `cli_error_messages.feature`  
**Issues**: CLI error messages were unclear or incomplete  
**Tests**:
- Missing `--state-type` argument clarity
- Comprehensive error reporting for missing arguments
- Task status argument handling
- Agent vs agent-id distinction

### 3. NLQ Query Reliability
**File**: `nlq_query_reliability.feature`  
**Issue**: Natural language queries failed to find recently created entities (indexing latency)  
**Tests**:
- Immediate task search after creation
- Immediate context search after creation
- Multiple similar entities handling

## Best Practices

### DO:
- Tag all regression tests with `@regression`
- Use descriptive file and scenario names
- Include Background section for common setup
- Test both success and failure cases
- Document the original issue in comments
- Link to issue tracking (GitHub issue, user report ID, etc.)

### DON'T:
- Duplicate existing feature tests (use regression only for previously broken functionality)
- Create overly generic tests (be specific to the regression)
- Skip documentation in `regression_plan.md`
- Mix regression tests with regular feature tests in the same file

## Continuous Integration

Regression tests run automatically:
- On every commit (via CI pipeline)
- Before releases (as part of release checklist)
- On demand via `cargo test --test bdd`

## Maintenance

### When a Regression Test Fails:
1. Determine if it's a new bug or test issue
2. If new bug: Fix the code, verify test passes
3. If test issue: Update the test to reflect correct behavior
4. Never delete regression tests unless the feature is intentionally removed

### Reviewing Regression Tests:
- Quarterly review of all `@regression` tests
- Remove tests for deprecated features
- Update tests for changed behavior (with documentation)
- Ensure tests still provide value

## Integration with BDD Runner

The BDD runner automatically discovers and runs regression tests:

```rust
// tests/bdd_runner.rs
bdd::EngramWorld::cucumber()
    .run_and_exit("tests/features")  // Includes regression/ subdirectory
    .await;
```

All `.feature` files under `tests/features/` (including subdirectories) are automatically discovered and executed.

## Reporting

After running regression tests:

```bash
# Generate test report
cargo test --test bdd -- --format junit > regression_report.xml

# View test results
cargo test --test bdd -- --format pretty
```

## Adding Regression Tests to Release Checklist

Before each release:
1. Review `tests/regression_plan.md` for new issues
2. Ensure all documented issues have corresponding tests
3. Run full regression suite: `cargo test --test bdd -- --tags @regression`
4. Update CHANGELOG.md with any bugs fixed
5. Mark regression test coverage in release notes

## Related Documentation

- `tests/regression_plan.md` - Planning and issue tracking
- `tests/bdd/` - Step implementations
- `tests/features/` - All feature test files
- Main README.md - Overall project testing strategy
