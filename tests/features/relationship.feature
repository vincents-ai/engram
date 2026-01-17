Feature: Entity Relationship Management
  As a developer using Engram
  I want to create and manage relationships between entities
  So that I can model dependencies and associations across my work

  Background:
    Given I have a workspace
    And I am logged in as agent "test-agent"

  Scenario: Create a basic dependency relationship
    Given I have an entity "task1" of type "task"
    And I have an entity "task2" of type "task"
    When I create a relationship from "task1" to "task2" of type "depends-on"
    Then the relationship should be created successfully
    And the relationship should be stored in Git
    And the relationship ID should be returned

  Scenario: Create bidirectional relationship
    Given I have an entity "component1" of type "component"
    And I have an entity "component2" of type "component"
    When I create a bidirectional relationship from "component1" to "component2" of type "associated-with"
    Then the relationship should be created successfully
    And the relationship direction should be "bidirectional"

  Scenario: Create relationship with custom strength
    Given I have an entity "module1" of type "module"
    And I have an entity "module2" of type "module"
    When I create a relationship from "module1" to "module2" of type "depends-on" with strength "critical"
    Then the relationship should be created successfully
    And the relationship strength should be "critical"

  Scenario: List relationships for an entity
    Given I have an entity "task-main" of type "task"
    And I have an entity "task-a" of type "task"
    And I have an entity "task-b" of type "task"
    And "task-main" depends on "task-a"
    And "task-main" depends on "task-b"
    When I list relationships for entity "task-main"
    Then I should see 2 relationships
    And I should see a relationship to "task-a"
    And I should see a relationship to "task-b"

  Scenario: Filter relationships by type
    Given I have an entity "project" of type "project"
    And I have an entity "task1" of type "task"
    And I have an entity "doc1" of type "document"
    And "project" contains "task1"
    And "project" references "doc1"
    When I list relationships for "project" filtered by type "contains"
    Then I should see 1 relationship
    And I should see a relationship to "task1"
    And I should not see a relationship to "doc1"

  Scenario: Show relationship details
    Given I have an entity "parent" of type "task"
    And I have an entity "child" of type "task"
    And I create a relationship from "parent" to "child" of type "contains" with description "Parent task containing subtask"
    When I show the relationship details
    Then I should see the source entity "parent"
    And I should see the target entity "child"
    And I should see the relationship type "contains"
    And I should see the description "Parent task containing subtask"

  Scenario: Delete a relationship
    Given I have an entity "source" of type "task"
    And I have an entity "target" of type "task"
    And "source" depends on "target"
    When I delete the relationship between "source" and "target"
    Then the relationship should be deleted successfully
    And the relationship should not exist in storage

  Scenario: Find path between entities
    Given I have an entity "start" of type "task"
    And I have an entity "middle" of type "task"
    And I have an entity "end" of type "task"
    And "start" depends on "middle"
    And "middle" depends on "end"
    When I find a path from "start" to "end"
    Then I should find a path
    And the path should include "start", "middle", "end" in order

  Scenario: Find path with no connection
    Given I have an entity "isolated-1" of type "task"
    And I have an entity "isolated-2" of type "task"
    When I find a path from "isolated-1" to "isolated-2"
    Then I should find no path

  Scenario: Get connected entities
    Given I have an entity "hub" of type "project"
    And I have an entity "task1" of type "task"
    And I have an entity "task2" of type "task"
    And I have an entity "doc1" of type "document"
    And "hub" contains "task1"
    And "hub" contains "task2"
    And "hub" references "doc1"
    When I get entities connected to "hub"
    Then I should see 3 connected entities
    And I should see "task1"
    And I should see "task2"
    And I should see "doc1"

  Scenario: Generate relationship statistics
    Given I have multiple entities with various relationships
    When I generate relationship statistics
    Then I should see the total number of relationships
    And I should see relationships broken down by type
    And I should see the most connected entity
    And I should see relationship density

  Scenario: Prevent cyclic relationships when not allowed
    Given I have an entity "task-a" of type "task"
    And I have an entity "task-b" of type "task"
    And "task-a" depends on "task-b"
    When I try to create a relationship where "task-b" depends on "task-a"
    And the relationship constraints do not allow cycles
    Then the relationship creation should fail
    And I should see a cycle prevention error

  Scenario: Allow cyclic relationships when permitted
    Given I have an entity "concept-a" of type "concept"
    And I have an entity "concept-b" of type "concept"
    And "concept-a" is associated with "concept-b"
    When I try to create a relationship where "concept-b" is associated with "concept-a"
    And the relationship constraints allow cycles
    Then the relationship should be created successfully

  Scenario: Respect maximum outbound relationship limits
    Given I have an entity "limited-entity" of type "component"
    And the relationship constraints limit outbound relationships to 2
    And "limited-entity" already has 2 outbound relationships
    When I try to create a third outbound relationship from "limited-entity"
    Then the relationship creation should fail
    And I should see a relationship limit error

  Scenario: Update relationship strength
    Given I have an entity "source" of type "task"
    And I have an entity "target" of type "task"
    And "source" depends on "target" with strength "medium"
    When I update the relationship strength to "critical"
    Then the relationship strength should be "critical"

  Scenario: Relationship persistence across sessions
    Given I create a relationship between "persistent-1" and "persistent-2"
    When I restart the system
    And I list relationships for "persistent-1"
    Then I should still see the relationship to "persistent-2"