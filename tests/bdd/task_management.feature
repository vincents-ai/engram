Feature: Task Management

  Background:
    Given I have a workspace
    And I am logged in as agent "test-agent"

  Scenario: Create a new task
    When I create a new task "Implement login feature" with description "Add user authentication"
    Then the task should be created successfully
    And I should see the task "Implement login feature" in the list

  Scenario: Create task with high priority
    Given I have a workspace
    And I am logged in as agent "test-agent"
    When I create a new task "Critical bug fix" with description "Fix authentication issue"
    And I set the task priority to "high"
    Then the task should be created successfully

  Scenario: List all tasks
    Given I have a workspace
    And I am logged in as agent "test-agent"
    And I have created a task "Sample task"
    When I list all tasks
    Then I should see the task "Sample task" in the list

  Scenario: Handle invalid task creation
    Given I have a workspace
    And I am logged in as agent "test-agent"
    When I create a new task with empty title
    Then the operation should fail with error "title cannot be empty"

  Scenario: Task storage integrity
    Given I have a workspace
    And I am logged in as agent "test-agent"
    When I create a new task "Test task" with description "Test description"
    Then the operation should succeed
    And the task should be stored with correct content