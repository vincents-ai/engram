Feature: Reasoning Chains
  As a developer using Engram
  I want to capture decision-making processes
  So that I can track and review architectural choices

  Scenario: Create reasoning chain
    Given I have a workspace
    When I create reasoning with title "Database Choice"
    And the reasoning has description "Evaluating PostgreSQL vs MongoDB"
    And the reasoning has conclusion "Choose PostgreSQL for ACID compliance"
    Then the reasoning should be created successfully
    And the reasoning should be stored in Git

  Scenario: Create reasoning with conclusion from file
    Given I have a workspace
    And I have a file "decision.txt" with conclusion text
    When I create reasoning with title "Architecture Decision" and conclusion-file "decision.txt"
    Then the reasoning should be created successfully
    And the conclusion should match the file content

  Scenario: Create reasoning from JSON
    Given I have a workspace
    When I pipe '{"title":"API Versioning","conclusion":"Use URL-based versioning"}' to reasoning create --json
    Then the reasoning should be created successfully
    And the conclusion should be "Use URL-based versioning"

  Scenario: Link reasoning to task
    Given I have a workspace
    And alice has a task "Implement database"
    When I create reasoning with title "DB Decision" and link to task
    Then the reasoning should be created successfully
    And the reasoning should reference the task ID

  Scenario: List reasoning chains
    Given I have a workspace
    And I have 3 reasoning chains
    When I list reasoning chains
    Then I should see 3 reasoning items

  Scenario: Show reasoning details
    Given I have a workspace
    And I have reasoning "My Decision"
    When I show the reasoning details
    Then I should see the reasoning title
    And I should see the description
    And I should see the conclusion
