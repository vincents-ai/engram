Feature: Pre-commit Hook Validation

  Scenario: User installs pre-commit hook
    Given I am in a git repository
    When I run "engram validation hook install"
    Then the hook should be installed successfully
    And the hook should contain validation logic

  Scenario: User validates a commit with valid task
    Given I have a task with required relationships
    When I run "engram validation commit --message 'feat: implement authentication [TASK-123]'"
    Then the validation should pass
    And I should see success message

  Scenario: User validates a commit without task reference
    When I run "engram validation commit --message 'chore: update dependencies'"
    Then the validation should pass due to exemption
    And I should see exemption message

  Scenario: User validates a commit with non-existent task
    When I run "engram validation commit --message 'feat: implement [TASK-999]'"
    Then the validation should fail
    And I should see task not found error
    And I should see helpful suggestion

  Scenario: User validates a commit with task missing relationships
    Given I have a task without reasoning relationship
    When I run "engram validation commit --message 'feat: implement [TASK-123]'"
    Then the validation should fail
    And I should see missing relationship error
    And I should see suggestion to add reasoning

  Scenario: Hook status check shows healthy setup
    Given I have a properly installed hook
    When I run "engram validation check"
    Then I should see all systems healthy
    And I should see success message

  Scenario: Hook status check shows issues
    Given I don't have the validation hook installed
    When I run "engram validation check"
    Then I should see hook not installed error
    And I should see suggestion to install hook

  Scenario: User uninstalls pre-commit hook
    Given I have a hook installed
    When I run "engram validation hook uninstall"
    Then the hook should be removed
    And I should see uninstall success message

  Scenario: Dry run validation without git repository
    When I run "engram validation commit --message 'test [TASK-123]' --dry-run"
    Then the validation should work
    And it should not require git repository

  Scenario: Validation with custom configuration
    Given I have a custom validation config
    When I run "engram validation commit --message 'custom: [CUSTOM-123]'"
    Then the validation should use custom rules
    And it should validate according to my patterns

  Scenario: Performance caching works
    Given I validate the same task multiple times
    When I run validation commands for the same task ID
    Then the second validation should use cached results
    And it should be faster than the first validation