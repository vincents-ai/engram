Feature: Knowledge Management
  As a developer using Engram
  I want to capture and organize knowledge
  So that I can build a reusable knowledge base

  Scenario: Create knowledge with pattern type
    Given I have a workspace
    When I create knowledge with title "Factory Pattern"
    And the knowledge type is "pattern"
    And the confidence is 0.9
    Then the knowledge should be created successfully
    And the knowledge should be stored in Git

  Scenario: Create knowledge with invalid confidence fails
    Given I have a workspace
    When I create knowledge with confidence 1.5
    Then the creation should fail
    And I should see a validation error about confidence range

  Scenario: Create knowledge from JSON
    Given I have a workspace
    When I pipe '{"title":"API Design","knowledge_type":"pattern","confidence":0.85}' to knowledge create --json
    Then the knowledge should be created successfully
    And the knowledge confidence should be 0.85

  Scenario: List knowledge by type
    Given I have a workspace
    And I have 3 knowledge items of type "pattern"
    And I have 2 knowledge items of type "lesson"
    When I list knowledge with filter type "pattern"
    Then I should see 3 knowledge items

  Scenario: Show knowledge details
    Given I have a workspace
    And I have knowledge "OAuth2 Flow"
    When I show the knowledge details
    Then I should see the knowledge title
    And I should see the knowledge type
    And I should see the confidence score
