# Regression Test Plan (Based on v0.1.1 Improvements)

This plan details the regression tests required to verify fixes for issues identified in `IMPROVEMENTS.md`.

## 1. Critical Priority (Bugs)

### 1.1. Workflow Instance Persistence Failure

*   **Test Scenario**:
    1.  Select an existing Task ID and Workflow ID.
    2.  Run `engram workflow start --agent default --entity-id <TASK_ID> --entity-type task <WORKFLOW_ID>`.
    3.  Capture the returned Instance UUID.
    4.  Immediately run `engram workflow status <INSTANCE_UUID>`.
*   **Expected Result**: The `status` command returns the details of the workflow instance (status, current state, etc.).
*   **Actual Result (from report)**: The `status` command returns "Not found" despite the `start` command reporting success.
*   **Automated Test Candidate**: Yes (Integration Test).

### 1.2. `engram ask` Query Reliability

*   **Test Scenario**:
    1.  Create a unique task: `engram task create --title "Unique Searchable Task <TIMESTAMP>"`.
    2.  Verify it exists with `engram task list`.
    3.  Immediately run `engram ask query "show me the task about Unique Searchable Task <TIMESTAMP>"`.
*   **Expected Result**: The NLQ interface identifies and returns the newly created task.
*   **Actual Result (from report)**: Returns "No tasks found" immediately after creation (likely indexing latency).
*   **Automated Test Candidate**: Yes (Integration Test with configurable latency check).

## 2. UX & CLI Consistency

### 2.1. Argument Naming - Workflow Add State

*   **Test Scenario**:
    1.  Run `engram workflow add-state` with missing arguments (specifically omitting `--state-type`).
*   **Expected Result**: Error message explicitly states that `--state-type` is missing or invalid.
*   **Actual Result (from report)**: Error message is ambiguous and does not clearly point to the missing `--state-type` argument.
*   **Automated Test Candidate**: Yes (CLI Output Assertion).

### 2.2. Argument Naming - Escalation Create (Missing Args)

*   **Test Scenario**:
    1.  Run `engram escalation create` providing only one of the required arguments (e.g., only `--request-type`).
*   **Expected Result**: Error message lists **all** missing required arguments (e.g., "Missing required arguments: --operation-type, --block-reason").
*   **Actual Result (from report)**: Error messages appear iteratively (one by one) as the user fixes them, specifically hiding the need for `--block-reason` until previous errors are fixed.
*   **Automated Test Candidate**: Yes (CLI Output Assertion).

### 2.3. Argument Naming - Task Create Status

*   **Test Scenario**:
    1.  Run `engram task create --title "Status Test" --status "in_progress"`.
*   **Expected Result**:
    *   *Option A*: Task is created with the specified status.
    *   *Option B*: Clear error message stating that status cannot be set at creation and must be managed via workflow transitions.
*   **Actual Result (from report)**: Command fails (likely "unknown argument" or similar unhandled clap error).
*   **Automated Test Candidate**: Yes.

### 2.4. Argument Naming - Sandbox Agent ID

*   **Test Scenario**:
    1.  Run `engram sandbox create --help`.
    2.  Run `engram sandbox create --agent <NAME>` (attempting to use `--agent` instead of `--agent-id`).
*   **Expected Result**:
    1.  Help text clearly distinguishes between the target Agent ID (`--agent-id`) and the acting agent (`--agent`).
    2.  Command provides a helpful error ("Did you mean --agent-id?") if the wrong flag is used.
*   **Actual Result (from report)**: Confusion between `--agent` and `--agent-id` in help text and usage.
*   **Automated Test Candidate**: Yes (Help Text Verification).
