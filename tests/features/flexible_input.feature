Feature: Flexible Input System
  As a developer using Engram
  I want to create entities using various input methods
  So that I can integrate Engram into different workflows

  Scenario: Create task from stdin for description
    Given I have a workspace
    When I pipe "This is a detailed description" to task create --title "My Task" --description-stdin
    Then the task should be created successfully
    And the task description should be "This is a detailed description"

  Scenario: Create task from JSON
    Given I have a workspace
    When I pipe '{"title":"JSON Task","priority":"high","agent":"alice"}' to task create --json
    Then the task should be created successfully
    And the task title should be "JSON Task"
    And the task priority should be "high"
    And the task agent should be "alice"

  Scenario: Create context from file
    Given I have a workspace
    And I have a file "docs.md" with content "API documentation content"
    When I create a context with title "API Docs" and content-file "docs.md"
    Then the context should be created successfully
    And the context content should be "API documentation content"

  Scenario: Create knowledge from JSON file
    Given I have a workspace
    And I have a JSON file "knowledge.json" with knowledge items
    When I create knowledge items from JSON file "knowledge.json"
    Then all knowledge items should be created successfully

  Scenario: Create reasoning with conclusion from file
    Given I have a workspace
    And I have a file "conclusion.txt" with reasoning conclusion
    When I create reasoning with title "Architecture Decision" and conclusion-file "conclusion.txt"
    Then the reasoning should be created successfully
    And the conclusion should match the file content
