@regression
Feature: Workflow Instance Persistence
  As a user of engram
  I want workflow instances to persist after creation
  So that I can query their status immediately after starting

  Background:
    Given I have a workspace
    And I am logged in as agent "test-agent"

  Scenario: Workflow instance can be queried immediately after creation
    Given I have created a task with title "Test Task"
    And I have created a workflow with title "Test Workflow"
    When I start the workflow for the task
    Then the workflow instance should be created successfully
    And I should be able to query the workflow instance status immediately
    And the workflow instance status should show the current state

  Scenario: Multiple workflow instances persist independently
    Given I have created a task with title "Task 1"
    And I have created another task with title "Task 2"
    And I have created a workflow with title "Multi-Instance Workflow"
    When I start the workflow for "Task 1"
    And I start the workflow for "Task 2"
    Then both workflow instances should exist
    And each instance should have its own state
    And querying one instance should not affect the other
