@regression
Feature: CLI Argument Error Messages
  As a user of engram
  I want clear and comprehensive error messages
  So that I can quickly fix command-line argument issues

  Background:
    Given I have a workspace
    And I am logged in as agent "test-agent"

  Scenario: Workflow add-state shows clear error for missing state-type
    When I run "engram workflow add-state" with missing state-type argument
    Then the error message should explicitly mention "--state-type"
    And the error message should list valid state-type values

  Scenario: Escalation create shows all missing arguments at once
    When I run "engram escalation create" with only the request-type argument
    Then the error message should list all missing required arguments
    And the error message should include "--operation-type"
    And the error message should include "--block-reason"

  Scenario: Task create with status argument handling
    When I run "engram task create --title 'Status Test' --status 'in_progress'"
    Then either the task should be created with the specified status
    Or a clear error should explain that status cannot be set at creation

  Scenario: Sandbox create distinguishes agent and agent-id
    When I run "engram sandbox create --help"
    Then the help text should clearly distinguish "--agent-id" from "--agent"
    And the help text should explain that "--agent-id" is the target agent
    And the help text should explain that "--agent" is the acting agent
