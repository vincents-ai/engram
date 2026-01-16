Feature: Task Management
  As a developer using Engram
  I want to create and manage tasks
  So that I can track work across sessions

  Scenario: Create a new task with basic fields
    Given I have a workspace
    When I create a task with title "Implement authentication"
    And the task has priority "high"
    And the task is assigned to agent "alice"
    Then the task should be created successfully
    And the task should be stored in Git
    And the task ID should be returned

  Scenario: List tasks for an agent
    Given I have a workspace
    And alice has a task "Task 1"
    And alice has a task "Task 2"
    And bob has a task "Task 3"
    When I list tasks for agent "alice"
    Then I should see 2 tasks
    And I should see "Task 1"
    And I should see "Task 2"
    And I should not see "Task 3"

  Scenario: Update task status
    Given I have a workspace
    And alice has a task "My Task" with status "pending"
    When I update the task status to "in_progress"
    Then the task status should be "in_progress"

  Scenario: Show task details
    Given I have a workspace
    And alice has a task "Detailed Task"
    When I show the task details
    Then I should see the task title
    And I should see the assigned agent
    And I should see the creation timestamp
