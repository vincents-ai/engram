Feature: Session Management
  As a developer using Engram
  I want to track my work sessions
  So that I can analyze productivity and collaboration patterns

  Scenario: Start a new session with auto-detect
    Given I have a workspace
    When I run session start --name "alice" --auto-detect
    Then a session should be created
    And the session should have a unique ID
    And auto-detection should identify Engram project work
    And the session status should be "active"

  Scenario: Start a session without auto-detect
    Given I have a workspace
    When I run session start --name "bob"
    Then a session should be created
    And the session should not have auto-detection enabled

  Scenario: Show session status with metrics
    Given I have a workspace
    And alice has an active session
    When I run session status --id <session-id> --metrics
    Then I should see the session details
    And I should see SPACE framework metrics
    And I should see DORA metrics
    And I should see the session duration

  Scenario: End a session with summary
    Given I have a workspace
    And alice has an active session
    When I run session end --id <session-id> --generate-summary
    Then the session status should be "completed"
    And I should see a productivity summary
    And I should see the total duration
    And I should see activity counts

  Scenario: List sessions for an agent
    Given I have a workspace
    And alice has 3 completed sessions
    And bob has 2 completed sessions
    When I list sessions for agent "alice"
    Then I should see 3 sessions
    And all sessions should be for agent "alice"

  Scenario: List sessions with limit
    Given I have a workspace
    And alice has 10 completed sessions
    When I list sessions for agent "alice" with limit 5
    Then I should see 5 sessions
    And they should be the 5 most recent sessions
