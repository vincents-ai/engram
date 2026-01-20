@regression
Feature: NLQ Query Reliability
  As a user of engram
  I want the natural language query interface to find recently created entities
  So that I can immediately search for items I just created

  Background:
    Given I have a workspace
    And I am logged in as agent "test-agent"

  Scenario: NLQ finds task immediately after creation
    When I create a task with title "Unique Searchable Task TestTimestamp123"
    And I verify the task exists in the task list
    Then the NLQ query "show me the task about Unique Searchable Task TestTimestamp123" should find the task
    And the query result should include the task title

  Scenario: NLQ finds context immediately after creation
    When I create a context with title "Unique Context RefID456"
    And I verify the context exists in the context list
    Then the NLQ query "find context about Unique Context RefID456" should find the context
    And the query result should include the context title

  Scenario: NLQ handles multiple similar entities
    When I create a task with title "Search Test Alpha"
    And I create a task with title "Search Test Beta"
    And I create a task with title "Search Test Gamma"
    Then the NLQ query "show tasks about Search Test" should find all three tasks
    And the query results should include "Alpha", "Beta", and "Gamma"
