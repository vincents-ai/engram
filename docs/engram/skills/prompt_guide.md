# Engram Prompt Engineering Guide

This guide explains how to construct effective prompts for agents using the Engram CLI. The goal is to create deterministic, context-rich workflows where agents rely exclusively on Engram as their source of truth.

## Philosophy
- **Single Source of Truth**: Agents should look up task context via `engram task show` and `engram relationship`, not rely on prompt injection.
- **Atomic Operations**: Workflows should be broken down into discrete steps tracked by Engram tasks.
- **Validation**: Every step must be verifiable via `engram validate`.

## Standard Prompt Template

Use this template to instruct agents. Replace `{{TASK_ID}}` with the actual UUID.

```markdown
You are an autonomous agent using Engram for state management.
**Task ID**: `{{TASK_ID}}`

### Protocol
1. **Initialize**:
   - Run `engram task update {{TASK_ID}} --status inprogress`
   - Run `engram relationship connected --entity-id {{TASK_ID}}` to find related context/reasoning.

2. **Execute**:
   - Read the task description via `engram task show {{TASK_ID}}`.
   - Access linked contexts via `engram context show [CONTEXT_ID]` (found in step 1).
   - Perform the work (coding, writing, etc.).

3. **Validate**:
   - Run `engram validate check` to ensure workflow compliance.
   - Run tests/linters as required by the task.

4. **Complete**:
   - Run `engram task update {{TASK_ID}} --status done`.
   - Output a summary of work completed.
```

## Workflow Examples

### 1. Research & Planning
For tasks requiring investigation before implementation.

```markdown
**Phase**: Research
**Goal**: Analyze requirements and create a plan.

1. `engram task update {{TASK_ID}} --status inprogress`
2. **Gather Info**:
   - Use `engram knowledge list` to find relevant patterns.
   - Use `engram adr list` to check architectural constraints.
3. **Document Plan**:
   - Create a reasoning entity: `engram reasoning create --task-id {{TASK_ID}} --title "Implementation Plan"`
   - Create context entities for found info: `engram context create ...`
4. **Link**:
   - Ensure all new entities are linked to {{TASK_ID}} via `engram relationship create`.
5. `engram task update {{TASK_ID}} --status done`
```

### 2. Implementation
For coding tasks.

```markdown
**Phase**: Implementation
**Goal**: Write code and pass tests.

1. `engram task update {{TASK_ID}} --status inprogress`
2. **Context**:
   - `engram relationship connected --entity-id {{TASK_ID}} --relationship-type references` to get specs.
3. **Code**:
   - Implement changes.
   - **CRITICAL**: Commit messages MUST format as: `type: description [{{TASK_ID}}]`
4. **Verify**:
   - `cargo test` (or relevant test command)
   - `engram validation commit --message "feat: ... [{{TASK_ID}}]"` (dry run)
5. `engram task update {{TASK_ID}} --status done`
```

### 3. Documentation
For writing docs.

```markdown
**Phase**: Documentation
**Goal**: Update documentation to match code/specs.

1. `engram task update {{TASK_ID}} --status inprogress`
2. **Source**:
   - `engram relationship connected --entity-id {{TASK_ID}}` to find the code/spec entity.
3. **Write**:
   - Update markdown files.
4. **Commit**:
   - Message: `docs: update guide [{{TASK_ID}}]`
5. `engram task update {{TASK_ID}} --status done`
```

## Best Practices
- **No Hallucinations**: If a UUID isn't found, stop and error out. Don't invent IDs.
- **Link Everything**: If you create a new artifact (file, module), create a corresponding Engram entity and link it to the task.
- **Status Hygiene**: Always move tasks to `inprogress` before starting and `done` only after validation.
