---
name: engram-sandbox-escalation
description: "Manage agent permission boundaries with sandbox and request human approval for blocked operations via escalation. Use before running risky operations, and when an operation is denied by sandbox policy."
---

# Sandbox and Escalation

## Overview

Engram provides two primitives for safe agent operation:

- **Sandbox** — permission boundary configuration for agents. Defines what operations an agent is allowed to perform. Validate operations before running them.
- **Escalation** — when a sandbox denies an operation, create an escalation request for human review. A reviewer approves or denies. Approved escalations can become permanent policies.

**Rule for agents:** Never work around a sandbox denial. Always escalate. Never guess whether an operation is allowed — always call `sandbox validate` first.

## When to Use

Use this skill when:
- Configuring a new agent's permission boundaries
- Running potentially risky operations (file system writes, network calls, package installs, credential access)
- An operation was denied by sandbox and you need human approval
- Reviewing pending escalation requests as a human operator
- Creating permanent policies from one-off approvals so future agents don't re-escalate

## Command Reference

### Sandbox

```bash
# Create a sandbox configuration for an agent
# All flags are optional but --agent is strongly recommended
engram sandbox create \
  --agent "<agent-name>" \
  --level standard \
  --created-by "<creator-name>"

# List all sandboxes
engram sandbox list

# Get a specific sandbox
engram sandbox get <SANDBOX_UUID>

# Update a sandbox configuration
engram sandbox update <SANDBOX_UUID> --level strict

# Delete a sandbox
engram sandbox delete <SANDBOX_UUID>

# Validate an operation against sandbox policy BEFORE running it
# Call this before any risky operation
engram sandbox validate \
  --agent-id "<agent-name>" \
  --operation "<what you want to do>" \
  --resource-type "<file|network|package|credential|database>"

# View sandbox statistics
engram sandbox stats

# Reset sandbox state (clears transient violations, keeps config)
engram sandbox reset
```

### Escalation

```bash
# Create an escalation request — --block-reason is REQUIRED (see critical note below)
engram escalation create \
  --agent "<agent-name>" \
  --operation-type "<type of operation>" \
  --operation "<full description of what you need to do>" \
  --block-reason "<exact reason sandbox denied the operation>" \
  --justification "<why this operation is necessary and safe>" \
  --priority normal \
  --impact "<what happens if this is denied>" \
  --reviewer "<reviewer-name-or-id>"

# List all pending escalations
engram escalation list

# Get a specific escalation
engram escalation get <ESCALATION_UUID>

# Review an escalation (human operator action)
# --status is REQUIRED: approved | denied
engram escalation review <ESCALATION_UUID> \
  --status approved \
  --reason "<why you approved or denied this>" \
  --reviewer-id "<reviewer-id>" \
  --reviewer-name "<reviewer-name>" \
  --duration "<how long approval is valid, e.g. 1h>" \
  --create-policy \
  --notes "<any additional notes>"

# Cancel an escalation (agent withdraws the request)
engram escalation cancel <ESCALATION_UUID>

# Clean up expired or resolved escalations
engram escalation cleanup

# View escalation statistics
engram escalation stats
```

## CRITICAL: `--block-reason` Is Always Required

`engram escalation create` will fail with **"Entity validation error"** if `--block-reason` is omitted, even though some documentation marks it as optional. **Always provide `--block-reason`.**

This field must contain the exact reason the sandbox denied the operation. Copy the denial message verbatim when possible.

## The Full Flow

```
Agent wants to run a risky operation
    ↓
sandbox validate --agent-id <id> --operation <op> --resource-type <type>
    ↓
Allowed? → YES → proceed with operation
    ↓ NO
escalation create \
  --agent <id> \
  --operation "<op>" \
  --block-reason "<sandbox denial message>" \   ← REQUIRED
  --justification "<why needed>"
    ↓
Human reviews: escalation review <UUID> --status approved|denied
    ↓
Denied? → accept the denial, abort operation, record outcome
    ↓ Approved
Proceed with operation
    ↓
Should this approval become a standing policy?
→ YES → escalation review used --create-policy flag (policy is now stored)
→ NO → one-off approval only
```

## When to Use `--create-policy`

Add `--create-policy` to `escalation review` when:
- The same operation will occur frequently from the same agent type
- The operation is clearly safe and re-escalation would be friction
- You want future agents to proceed without manual review

Do not use `--create-policy` for:
- One-time operations in unusual circumstances
- Operations with elevated risk that warrant per-instance review
- Any operation where context could change the safety assessment

## The Pattern

### Agent: Validate Before Acting

```bash
# Before writing to a config file
engram sandbox validate \
  --agent-id "deploy-agent" \
  --operation "write /etc/app/config.yaml" \
  --resource-type file
```

If denied:

### Agent: Create Escalation With All Required Fields

```bash
engram escalation create \
  --agent "deploy-agent" \
  --operation-type "file-write" \
  --operation "Write updated configuration to /etc/app/config.yaml for deployment" \
  --block-reason "Sandbox policy denies file writes outside /tmp for agent level: standard" \
  --justification "Deployment requires updating live config. File is owned by deploy-agent service account. Change is reversible." \
  --priority normal \
  --impact "Deployment cannot proceed without this config write" \
  --reviewer "ops-team"
# Returns: ESCALATION_UUID
```

### Human Operator: Review and Optionally Create Policy

```bash
# Approve one-off
engram escalation review <ESCALATION_UUID> \
  --status approved \
  --reason "Config write is safe for this deployment; file verified." \
  --reviewer-id "alice" \
  --reviewer-name "Alice"

# Approve and create permanent policy so future deploys don't escalate
engram escalation review <ESCALATION_UUID> \
  --status approved \
  --reason "Standard deployment config write pattern; safe to allow permanently." \
  --reviewer-id "alice" \
  --reviewer-name "Alice" \
  --create-policy \
  --notes "Policy applies to deploy-agent writing to /etc/app/*.yaml only"
```

### Denied: Record and Abort

```bash
# If denied, record the outcome in engram and stop
engram context create \
  --title "Config write escalation denied" \
  --content "Escalation <UUID> for /etc/app/config.yaml write was denied. Deployment aborted. Manual intervention required." \
  --source "escalation:<ESCALATION_UUID>"

engram task update <TASK_UUID> --status blocked \
  --outcome "Blocked: sandbox escalation denied for config file write"
```

## Example: Install a Package

```bash
# Step 1: validate
engram sandbox validate \
  --agent-id "setup-agent" \
  --operation "npm install lodash" \
  --resource-type package

# If denied, step 2: escalate
engram escalation create \
  --agent "setup-agent" \
  --operation-type "package-install" \
  --operation "npm install lodash@4.17.21 — required by src/utils/format.ts" \
  --block-reason "Sandbox policy: package installs require explicit approval for level:standard agents" \
  --justification "lodash is a well-known utility library with no known vulnerabilities at this version. Required to resolve import error in format.ts." \
  --priority normal \
  --impact "Build fails without this dependency" \
  --reviewer "lead-developer"

# Step 3: wait for review, check status
engram escalation get <ESCALATION_UUID>

# Step 4 (if approved): proceed with install
```

## Key Rules

1. **Always validate first** — call `sandbox validate` before any risky operation, not after
2. **`--block-reason` is mandatory** — `escalation create` fails without it, regardless of docs
3. **Never circumvent sandbox** — if denied, escalate or abort; never bypass
4. **Use `--create-policy`** to reduce future escalation friction for recurring safe operations
5. **Cancel stale escalations** — use `escalation cancel` if the operation is no longer needed
6. **Record denied outcomes** — store denials in engram context so the task history is complete

## Related Skills

- `engram-use-engram-memory` — storing investigation context
- `engram-orchestrator` — orchestration loop that includes escalation handling
- `engram-audit-trail` — full traceability of escalation decisions
- `engram-security-architecture` — designing permission boundaries
