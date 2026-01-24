# Workflow Management

## Implementation Plan: BDD Workflow in Engram

### Overview
We'll create a fully automated BDD workflow with strict commit policies and a single workflow instance associated with the main task that automatically transitions through states.

Issues itentified trying to impliment the following:

❌ NOT SUPPORTED (Yet)
1. Workflow-Based Commit Policies
   - ❌ No commit policy enforcement based on workflow state
   - ❌ States don't have configurable commit restrictions
   - ❌ No "block code commits in planning phase" feature
   - ❌ Validation doesn't check current workflow state
2. Automatic Transition Triggers
   - ❌ Transitions marked "automatic" but don't auto-execute
   - ❌ No task completion triggers for workflow transitions
   - ❌ No test result-based transitions
   - ❌ Manual transition execution required via command
3. Workflow-Task Integration
   - ❌ No automatic task state updates based on workflow state
   - ❌ Tasks don't know which workflow state they map to
   - ❌ No query like "what tasks are in this workflow state"
4. External Commands
   - ❌ No way to define
   - ❌ No way to trigger from cli
   - ❌ No query like "what commands/checks/guards are avaialbe in this workflow"


---
## Phase 1: Define Workflow States
Create 7 states in the workflow:
1. specification (start state)
      engram workflow add-state b36f3fb9-b2d9-49fd-94d8-85a3a29f6ddf \
     --name "specification" \
     --state-type start \
     --description "Writing Gherkin feature specifications - only engram entity changes allowed"
      - Commit policy: Block all code/config commits, only engram changes
   - Maps to task: 75aef678

2. framework_setup (in_progress)
      engram workflow add-state b36f3fb9-b2d9-49fd-94d8-85a3a29f6ddf \
     --name "framework_setup" \
     --state-type in_progress \
     --description "Setting up BDD framework - config and tooling changes only"
      - Commit policy: Allow config/build files, block feature code
      - External Checks: code linting, formatting, tests
   - Maps to task: c3c4d6c8

3. red_phase (in_progress)
      engram workflow add-state b36f3fb9-b2d9-49fd-94d8-85a3a29f6ddf \
     --name "red_phase" \
     --state-type in_progress \
     --description "RED: Writing step definitions - tests must compile but fail"
      - Commit policy: Test code allowed, must compile, must have failing tests
      - External Checks: code linting, formatting, tests, build
   - Maps to task: 0390c634

4. green_phase (in_progress)
      engram workflow add-state b36f3fb9-b2d9-49fd-94d8-85a3a29f6ddf \
     --name "green_phase" \
     --state-type in_progress \
     --description "GREEN: Implementing feature - all tests must pass"
      - Commit policy: Code + tests required, all tests must pass
      - External Checks: code linting, formatting, tests, build
   - Maps to task: c2aa22fd

5. refactor_phase (in_progress)
      engram workflow add-state b36f3fb9-b2d9-49fd-94d8-85a3a29f6ddf \
     --name "refactor_phase" \
     --state-type in_progress \
     --description "REFACTOR: Cleaning up implementation - tests stay green"
      - Commit policy: Code changes allowed, tests must remain passing
      - External Checks: code linting, formatting, tests, build
   - Maps to task: ce02255a

6. integration_testing (review)
      engram workflow add-state b36f3fb9-b2d9-49fd-94d8-85a3a29f6ddf \
     --name "integration_testing" \
     --state-type review \
     --description "Integration testing in NixOS VM - full test suite required"
      - Commit policy: Full build + test suite + VM tests must pass
      - External Checks: code linting, formatting, tests, build
   - Maps to task: 920724a3

7. completed (done, final)
      engram workflow add-state b36f3fb9-b2d9-49fd-94d8-85a3a29f6ddf \
     --name "completed" \
     --state-type done \
     --is-final \
     --description "Feature fully implemented and validated"
      - No commit restrictions (feature complete)
---
## Phase 2: Define Workflow Transitions
Create 6 automatic transitions between states:
1. specification → framework_setup
      engram workflow add-transition b36f3fb9-b2d9-49fd-94d8-85a3a29f6ddf \
     --name "specs_approved" \
     --from-state <specification-state-id> \
     --to-state <framework-state-id> \
     --transition-type automatic \
     --description "Gherkin specs committed - transition to framework setup"
      - Trigger: Task 75aef678 marked as done

2. framework_setup → red_phase
      engram workflow add-transition b36f3fb9-b2d9-49fd-94d8-85a3a29f6ddf \
     --name "framework_ready" \
     --from-state <framework-state-id> \
     --to-state <red-phase-state-id> \
     --transition-type automatic \
     --description "BDD framework configured - transition to RED phase"
      - Trigger: Task c3c4d6c8 marked as done

3. red_phase → green_phase
      engram workflow add-transition b36f3fb9-b2d9-49fd-94d8-85a3a29f6ddf \
     --name "tests_failing" \
     --from-state <red-phase-state-id> \
     --to-state <green-phase-state-id> \
     --transition-type automatic \
     --description "Step definitions complete with failing tests - transition to GREEN phase"
      - Trigger: Task 0390c634 marked as done + tests fail

4. green_phase → refactor_phase
      engram workflow add-transition b36f3fb9-b2d9-49fd-94d8-85a3a29f6ddf \
     --name "tests_passing" \
     --from-state <green-phase-state-id> \
     --to-state <refactor-phase-state-id> \
     --transition-type automatic \
     --description "All tests passing - transition to REFACTOR phase"
      - Trigger: Task c2aa22fd marked as done + tests pass

5. refactor_phase → integration_testing
      engram workflow add-transition b36f3fb9-b2d9-49fd-94d8-85a3a29f6ddf \
     --name "refactoring_complete" \
     --from-state <refactor-phase-state-id> \
     --to-state <integration-state-id> \
     --transition-type automatic \
     --description "Refactoring complete, tests still green - transition to integration"
      - Trigger: Task ce02255a marked as done + tests pass

6. integration_testing → completed
      engram workflow add-transition b36f3fb9-b2d9-49fd-94d8-85a3a29f6ddf \
     --name "validation_complete" \
     --from-state <integration-state-id> \
     --to-state <completed-state-id> \
     --transition-type automatic \
     --description "VM tests pass - feature complete"
      - Trigger: Task 920724a3 marked as done + VM tests pass
---
## Phase 3: Activate Workflow
Activate the workflow definition:
engram workflow activate b36f3fb9-b2d9-49fd-94d8-85a3a29f6ddf
This marks the workflow as ready for use.
---
## Phase 4: Start Workflow Instance
Create workflow instance for the main task:
engram workflow start b36f3fb9-b2d9-49fd-94d8-85a3a29f6ddf \
  --entity-id e642def8-7b2d-4828-bc42-c333d8d589d4 \
  --entity-type task \
  --agent default
This:
- Associates workflow with main task e642def8
- Sets initial state to specification
- Enables automatic transitions based on task completion
- Enforces commit policies based on current state
---
## Phase 5: Update Task Descriptions with State Mapping
Optionally update task descriptions to reference workflow states:
1. Task 75aef678: Add note "Workflow state: specification"
2. Task c3c4d6c8: Add note "Workflow state: framework_setup"
3. Task 0390c634: Add note "Workflow state: red_phase"
4. Task c2aa22fd: Add note "Workflow state: green_phase"
5. Task ce02255a: Add note "Workflow state: refactor_phase"
6. Task 920724a3: Add note "Workflow state: integration_testing"
---
## Phase 6: Verification
Verify the workflow is properly configured:
# Check workflow structure
engram workflow get b36f3fb9-b2d9-49fd-94d8-85a3a29f6ddf
# Check workflow instance
engram workflow instances
# Check current status
engram workflow status --task-id e642def8-7b2d-4828-bc42-c333d8d589d4
---
Expected Behavior After Implementation
1. Current state: specification (start)
2. Active task: 75aef678 (Write Gherkin specs)
3. Commit restrictions: Only engram entity changes allowed
When you mark task 75aef678 as done:
- Workflow automatically transitions to framework_setup
- Commit policy changes to allow config/build files
- Next task c3c4d6c8 becomes active
And so on through the workflow...
---
Commit Policy Details by State
| State | Engram | Config/Build | Test Code | Feature Code | Test Pass Required |
|-------|--------|--------------|-----------|--------------|-------------------|
| specification | ✅ | ❌ | ❌ | ❌ | N/A |
| framework_setup | ✅ | ✅ | ❌ | ❌ | N/A |
| red_phase | ✅ | ✅ | ✅ | ❌ | Must compile, must fail |
| green_phase | ✅ | ✅ | ✅ | ✅ | Must pass |
| refactor_phase | ✅ | ✅ | ✅ | ✅ | Must pass |
| integration_testing | ✅ | ✅ | ✅ | ✅ | Must pass + VM tests |
| completed | ✅ | ✅ | ✅ | ✅ | N/A (final) |
---
Benefits of This Approach
1. Enforced discipline: Can't commit code during planning
2. Automatic progression: Transitions happen when tasks complete
3. Clear state tracking: Always know which BDD phase you're in
4. Validation built-in: Commit hooks check workflow state
5. Reusable pattern: Can use this workflow for future features
---
Notes
- State IDs will be generated when creating states - we'll need to capture them for creating transitions
- The workflow instance ID will be generated when starting the workflow
- Commit validation hook should check engram workflow status before allowing commits
- Task completion triggers should be manual (engram task update <id> --status done)
---
