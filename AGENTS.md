This is a NixOS environment.

You need to use a flake.nix with a devShell to access additional tooling.

All building and testing should happened via the flake.nix, either `nix build` or `nix checks` depending on if we're building or testing. Create additional targets as required.

When you want to run commands, check if we are in a Nix devShell already via looking for `IN_NIX_SHELL` defined in the environment variables.
If we're not in the devShell you'll need to issue commands with `nix develop -c -- <command>` to expose the build environment from the flake.nix, you should also warn the user, and suggest they start the devShell before `opencode`.

direnv is used, with the nix specific version to make things quicker. To ensure repositories you work on have a `.envrc` containing `use flake`.

Maintain the `.gitignore` file. No binaries are allowed to be committed. We also shouldn't commit random test scripts unless they are part of the testing framework of the repository.

If there are tools available to your session, make note of their features now and favor them over built-in tooling like `read`, `write`, `edit`, if there is git tooling available, you should use it, always perform testing, don't disable tests, don't cut corners. Your job is not to make decisions which deviate from the requested task, that is for the human operator.

Nothing is too difficult. It just needs to be broken down into smaller tasks.

Don't run `pkill -f`, if you need to start a service in the background ensure you can track its PID to stop it later on.

Documentation Style:
- Do not use emojis in documentation files
- Use clear, professional language without decorative symbols
- Maintain consistent formatting without emoji decorations

# Engram Workflow Integration

This project uses Engram for task-driven development with workflow-based quality gates. In each session, you MUST check engram status and follow established workflows.

## Session Startup Protocol

1. **Check Engram Availability**:
   ```bash
   # Verify engram is available (nix build result or cargo build)
   ./result/bin/engram --help 2>/dev/null || nix build
   ```

2. **Review Current Tasks**:
   ```bash
   # List all pending tasks
   ./result/bin/engram task list
   
   # Find your active/assigned tasks
   ./result/bin/engram task list | grep -E "(inprogress|assigned)"
   ```

3. **Check Workflow Status**:
   ```bash
   # List active workflows
   ./result/bin/engram workflow list
   
   # Check if any tasks are blocked by workflow states
   ./result/bin/engram validate check
   ```

4. **Review Validation System**:
   ```bash
   # Ensure commit validation is working
   ./result/bin/engram validate hook status
   
   # Install hook if missing
   ./result/bin/engram validate hook install
   ```

## Task Execution Workflow

### Before Starting Work on Any Task

1. **Get Task Details**:
   ```bash
   # Get task information and relationships
   ./result/bin/engram task list | grep [TASK_ID]
   ./result/bin/engram relationship connected --entity-id [TASK_ID]
   ```

2. **Check Task Dependencies**:
   ```bash
   # Find what this task depends on
   ./result/bin/engram relationship list | grep [TASK_ID]
   
   # Ensure dependencies are completed
   ```

3. **Verify Workflow State**:
   ```bash
   # Check if task is in correct workflow state for the work you plan to do
   # (This command will be available after workflow implementation)
   ./result/bin/engram workflow status --task-id [TASK_ID]
   ```

### During Development

1. **Update Task Status**:
   ```bash
   # Mark task as in progress
   ./result/bin/engram task update [TASK_ID] --status inprogress
   ```

2. **Follow Commit Validation**:
   - All commits MUST reference valid task IDs with proper relationships
   - Use format: `[UUID]` for task references
   - Ensure task has both context and reasoning relationships

3. **Respect Workflow Gates**:
   - Planning/Research stages: NO code commits allowed (engram entities only)
   - Development stages: Code commits allowed with test validation
   - Integration stages: Full test suite must pass

### Task Completion

1. **Verify All Requirements Met**:
   ```bash
   # Check task relationships are complete
   ./result/bin/engram relationship connected --entity-id [TASK_ID]
   
   # Ensure all subtasks completed (if any)
   ./result/bin/engram relationship list | grep "contains.*[TASK_ID]"
   ```

2. **Update Task Status**:
   ```bash
   ./result/bin/engram task update [TASK_ID] --status done
   ```

3. **Document Outcomes**:
   - Update reasoning entities with results
   - Create new context entities if knowledge was gained
   - Link any new relationships discovered

## Planning and Task Creation

When creating new tasks or breaking down work:

1. **Create Proper Entity Structure**:
   ```bash
   # Create main task
   ./result/bin/engram task create --title "Task description"
   
   # Create supporting context
   ./result/bin/engram context create --title "Context description"
   
   # Create reasoning
   ./result/bin/engram reasoning create --task-id [TASK_ID] --title "Reasoning description"
   
   # Link relationships
   ./result/bin/engram relationship create --source-id [TASK_ID] --source-type task --target-id [CONTEXT_ID] --target-type context --relationship-type references --agent default
   ./result/bin/engram relationship create --source-id [TASK_ID] --source-type task --target-id [REASONING_ID] --target-type reasoning --relationship-type references --agent default
   ```

2. **For Complex Tasks, Create Subtasks**:
   ```bash
   # Create subtasks
   ./result/bin/engram task create --title "Subtask 1"
   ./result/bin/engram task create --title "Subtask 2"
   
   # Link to parent task
   ./result/bin/engram relationship create --source-id [PARENT_TASK] --source-type task --target-id [SUBTASK] --target-type task --relationship-type contains --agent default
   
   # Create dependencies between subtasks
   ./result/bin/engram relationship create --source-id [SUBTASK2] --source-type task --target-id [SUBTASK1] --target-type task --relationship-type depends_on --agent default
   ```

3. **Create Workflows for Complex Projects**:
   ```bash
   # Create workflow to track execution
   ./result/bin/engram workflow create --title "Project workflow" --description "Description of the workflow process"
   ```

## Commit Validation Requirements

All commits MUST:

1. Reference a valid task ID using format: `[uuid]`
2. Task must have both context and reasoning relationships
3. Respect workflow stage commit policies:
   - Planning/Research: Only engram entity changes allowed
   - Development: Code + test validation required
   - Integration: Full test suite + build required

Example valid commit:
```
feat: implement user authentication [2aaa23b1-20e8-45c2-a64f-acdb6a148a49]

Added JWT token validation and user session management.
```

## Finding and Resuming Work

### When Returning to a Session:

1. **Check Recent Activity**:
   ```bash
   # See recently created/modified tasks
   ./result/bin/engram task list | head -10
   
   # Check recent relationships
   ./result/bin/engram relationship list | head -10
   ```

2. **Find In-Progress Work**:
   ```bash
   # Find your active tasks
   ./result/bin/engram task list | grep inprogress
   
   # Find tasks assigned to you
   ./result/bin/engram task list | grep "agent-$(whoami)\|default"
   ```

3. **Review Project Status**:
   ```bash
   # Check for any validation issues
   ./result/bin/engram validate check
   
   # Review recent workflows
   ./result/bin/engram workflow list | grep -v "complete"
   ```

## Storage Architecture Notes

- Engram entities are stored in `.engram/.git` (memory repository) - this is separate from main project Git
- Main project repository only contains source code, configs, and build artifacts
- `.engram/*` is gitignored in main repository to enforce separation
- All entity operations work across this architecture automatically

## Quality and Testing Integration

- All workflow stages that allow code commits will trigger quality gates
- Test results and build outputs are stored in engram for agent context
- Workflow progression depends on quality gate success
- Failed tests/builds block task progression until resolved

## Troubleshooting

**Hook Not Working**:
```bash
./result/bin/engram validate hook status
./result/bin/engram validate hook install
```

**Task Missing Relationships**:
```bash
# Check what relationships exist
./result/bin/engram relationship connected --entity-id [TASK_ID]

# Create missing relationships
./result/bin/engram relationship create --source-id [TASK_ID] --source-type task --target-id [CONTEXT_ID] --target-type context --relationship-type references --agent default
```

**Storage Issues**:
- Verify `.engram/.git` exists and is a valid Git repository
- Check that main repo has `.engram/*` in `.gitignore`
- Entity files should only exist in `.engram/.git`, never in main repository

Remember: Always use engram commands to discover current state rather than making assumptions about project status or requirements.