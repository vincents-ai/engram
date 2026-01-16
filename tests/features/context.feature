Feature: Context Management
  As a developer using Engram
  I want to store and retrieve contextual information
  So that I can maintain relevant information across sessions

  Scenario: Create context with basic fields
    Given I have a workspace
    When I create a context with title "API Requirements"
    And the context has content "RESTful API with OAuth2"
    And the context has relevance "high"
    Then the context should be created successfully
    And the context should be stored in Git

  Scenario: Create context with content from stdin
    Given I have a workspace
    When I pipe "Long documentation content" to context create --title "Docs" --content-stdin
    Then the context should be created successfully
    And the context content should be "Long documentation content"

  Scenario: Create context with content from file
    Given I have a workspace
    And I have a file "readme.md" with content "# Project README"
    When I create a context with title "README" and content-file "readme.md"
    Then the context should be created successfully
    And the context content should be "# Project README"

  Scenario: List contexts
    Given I have a workspace
    And I have 3 contexts
    When I list contexts
    Then I should see 3 contexts

  Scenario: Show context details
    Given I have a workspace
    And I have a context "My Context"
    When I show the context details
    Then I should see the context title
    And I should see the context content
    And I should see the relevance level
