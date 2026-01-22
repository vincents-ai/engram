---
name: engram-audit-trail
description: "Create complete audit trail of all work. Store decisions, actions, and outcomes in engram for traceability."
---

# Creating Audit Trails

## Overview

Every significant action should leave a trace in engram. This enables traceability, knowledge transfer, and compliance.

## When to Use

Use this skill when:
- Completing tasks
- Making decisions
- Deploying changes
- Responding to incidents
- Any work that needs to be remembered

## The Pattern

### 1. Record Actions
Store each action as it happens:

```bash
# Record work started
engram reasoning create \
  --title "[Work] Started" \
  --task-id [TASK_ID] \
  --content "**Started:** [timestamp]\n**Initial state:** [What was found]" \
  --confidence 1.0

# Record progress
engram reasoning create \
  --title "[Work] Progress: [Step]" \
  --task-id [TASK_ID] \
  --content "**Step:** [What was done]\n**Result:** [What happened]\n**Next:** [What comes next]" \
  --confidence 1.0

# Record completion
engram reasoning create \
  --title "[Work] Completed" \
  --task-id [TASK_ID] \
  --content "**Completed:** [timestamp]\n**Outcome:** [Result]\n**Artifacts:** [Files changed, if any]\n**Duration:** [Time taken]" \
  --confidence 1.0
```

### 2. Link Artifacts
Connect outputs to the work:

```bash
# Link created files
engram context create \
  --title "Artifact: [File Name]" \
  --content "[File content or summary]" \
  --source "artifact"

engram relationship create \
  --source-id [TASK_ID] \
  --target-id [ARTIFACT_ID] \
  --produces

# Link test results
engram context create \
  --title "Test Results: [Test Name]" \
  --content "**Passed:** [N]/[N]\n**Failed:** [List if any]" \
  --source "test-results"

engram relationship create \
  --source-id [TASK_ID] \
  --target-id [TEST_RESULTS_ID] \
  --validates
```

### 3. Create Summary
After completion, create a summary:

```bash
engram reasoning create \
  --title "[Work] Audit Summary" \
  --task-id [TASK_ID] \
  --content "## Summary\n**What:** [Work description]\n**Why:** [Purpose]\n**How:** [Approach taken]\n\n## Timeline\n- Started: [time]\n- Completed: [time]\n- Duration: [time]\n\n## Outcomes\n- ✅ [Outcome 1]\n- ✅ [Outcome 2]\n\n## Learnings\n- [What was learned]\n- [What would be done differently]\n\n## Related\n- Context: [IDs]\n- Reasoning: [IDs]\n- Artifacts: [IDs]" \
  --confidence 1.0
```

## Example

```
Task: "Fix authentication bug"

[Step 1: Record start]
engram reasoning create \
  --title "Auth Bug Fix Started" \
  --task-id bug-auth-123 \
  --content "Issue: Users unable to login after password reset\nInitial state: 15 failed login attempts logged"

[Step 2: Record investigation]
engram reasoning create \
  --title "Auth Bug: Root Cause Found" \
  --task-id bug-auth-123 \
  --content "Root cause: Token validation fails for reset tokens\nEvidence: Logs show 'invalid signature' for reset tokens issued after password change"

[Step 3: Record fix]
engram reasoning create \
  --title "Auth Bug: Fix Applied" \
  --task-id bug-auth-123 \
  --content "Fix: Added token version check in validate_token()\nFiles: src/auth/token.rs\nTests: Added test_token_version_validation()"

[Step 4: Record verification]
engram context create \
  --title "Test Results: Auth Bug Fix" \
  --content "Tests: 47/47 passed\nRegression: None\nPerformance: No impact"

[Step 5: Create summary]
engram reasoning create \
  --title "Auth Bug Fix Audit Summary" \
  --task-id bug-auth-123 \
  --content "## Summary\nFixed token validation bug affecting password reset flow.\n\n## Timeline\n- Started: 14:00\n- Root cause: 14:15\n- Fix: 14:30\n- Verified: 14:45\n\n## Outcome\nBug fixed. Users can login after password reset.\n\n## Learnings\n- Token versioning should be documented\n- Add integration test for reset flow"
```

## Integration with Engram

Audit trail stored as:
- **Reasoning**: Timeline of actions
- **Context**: Artifacts and results
- **Relationships**: What links to what
- **Compliance**: Evidence of work

## Querying Audit Trails

```bash
# Get full timeline
engram reasoning list --task-id [TASK_ID] | sort

# Get artifacts
engram relationship connected --entity-id [TASK_ID] --relationship-type produces

# Get test results
engram relationship connected --entity-id [TASK_ID] --relationship-type validates

# Get summary
engram reasoning list --task-id [TASK_ID] | grep "Audit Summary"

# Search audit logs
engram reasoning list | grep "[search term]"
```

## Why Audit Trails Matter

1. **Traceability** - Know what was done and why
2. **Knowledge Transfer** - Others can understand the work
3. **Compliance** - Evidence of due diligence
4. **Debugging** - Understand past decisions
5. **Learning** - Capture insights for future work

## Key Principles

1. **Record as You Go** - Don't wait until the end
2. **Be Specific** - Include relevant details
3. **Link Everything** - Connect related entities
4. **Capture Learnings** - What would you do differently?
5. **Create Summaries** - High-level overviews help
