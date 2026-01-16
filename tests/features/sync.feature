Feature: Multi-agent Synchronization
  As a team using Engram
  I want to synchronize memory across agents
  So that we can collaborate effectively

  Scenario: Sync with latest_wins strategy
    Given I have a workspace
    And alice has a task "Task 1" updated at 2026-01-13T10:00:00Z
    And bob has a task "Task 1" updated at 2026-01-13T10:05:00Z
    When I sync agents "alice,bob" with strategy "latest_wins"
    Then the sync should succeed
    And the task should have bob's version
    And no conflicts should be reported

  Scenario: Sync with intelligent_merge strategy
    Given I have a workspace
    And alice has a task with title "Task 1" and description "Alice's description"
    And bob has the same task with title "Task 1" and priority "high"
    When I sync agents "alice,bob" with strategy "intelligent_merge"
    Then the sync should succeed
    And the merged task should have description "Alice's description"
    And the merged task should have priority "high"
    And no conflicts should be reported

  Scenario: Sync with conflict detection
    Given I have a workspace
    And alice has a task "Task 1" with description "Alice's version" updated at 2026-01-13T10:00:00Z
    And bob has a task "Task 1" with description "Bob's version" updated at 2026-01-13T10:01:00Z
    When I sync agents "alice,bob" with strategy "merge_with_conflict_resolution"
    Then the sync should succeed
    And conflicts should be detected
    And I should see a conflict report

  Scenario: Sync multiple agents
    Given I have a workspace
    And alice has 5 tasks
    And bob has 3 tasks
    And charlie has 2 tasks
    When I sync agents "alice,bob,charlie" with strategy "latest_wins"
    Then the sync should succeed
    And all unique tasks should be accessible
    And duplicate tasks should be resolved

  Scenario: Sync with single agent shows helpful message
    Given I have a workspace
    When I sync agents "alice" with strategy "latest_wins"
    Then I should see a message about single agent
    And no sync operations should be performed

  Scenario: Sync with empty agents list fails
    Given I have a workspace
    When I sync agents "" with strategy "latest_wins"
    Then the sync should fail
    And I should see an error about empty agents list

  Scenario: Sync with invalid strategy fails
    Given I have a workspace
    When I sync agents "alice,bob" with strategy "invalid_strategy"
    Then the sync should fail
    And I should see valid strategy options
