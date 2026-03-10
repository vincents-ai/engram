# Example Workflows

This section provides complete end-to-end examples of how to use Engram's features together.

## Example 1: Basic Development Workflow

A human developer working on a feature.

```bash
# 1. Start a session
SESSION_ID=$(engram session start --agent developer --json | jq -r '.id')
echo "Started session: $SESSION_ID"

# 2. Create a task
TASK_ID=$(engram task create \
  --title "Implement user login endpoint" \
  --priority high \
  --json | jq -r '.id')
echo "Created task: $TASK_ID"

# 3. Add context (research)
CONTEXT_ID=$(engram context create \
  --title "OAuth2 Password Grant Flow" \
  --source "https://oauth.net/2/grant-types/password/" \
  --json | jq -r '.id')
echo "Created context: $CONTEXT_ID"

# 4. Link context to task
engram relationship create \
  --source-id $TASK_ID \
  --target-id $CONTEXT_ID \
  --type references

# 5. Implement and make a decision
REASONING_ID=$(engram reasoning create \
  --title "Use bcrypt for password hashing" \
  --description "bcrypt has built-in salting and adjustable cost factor. Scrypt or Argon2 would also work but bcrypt is more widely supported." \
  --task-id $TASK_ID \
  --json | jq -r '.id')
echo "Recorded reasoning: $REASONING_ID"

# 6. Complete the task
engram task update --id $TASK_ID --status completed
```

## Example 2: Theory Building for Codebase

An AI agent extracting mental models from a new codebase.

```bash
# 1. Start session as theorist agent
SESSION_ID=$(engram session start --agent the-theorist --json | jq -r '.id')

# 2. Create theories for major domains
THEORY_ID=$(engram theory create "Authentication System" --agent the-theorist --json | jq -r '.id')

# 3. Add conceptual model
engram theory update --id $THEORY_ID \
  --concept "User: An entity that can authenticate to the system with credentials"
engram theory update --id $THEORY_ID \
  --concept "Credential: Proof of identity (password, API key, OAuth token)"
engram theory update --id $THEORY_ID \
  --concept "Session: An authenticated context with a timeout"

# 4. Add system mappings
engram theory update --id $THEORY_ID \
  --mapping "User: src/entities/user.rs:42 (struct User)"
engram theory update --id $THEORY_ID \
  --mapping "Credential: src/entities/credential.rs:15 (enum CredentialType)"
engram theory update --id $THEORY_ID \
  --mapping "Session: src/entities/session.rs:28 (struct Session)"

# 5. Add design rationale
engram theory update --id $THEORY_ID \
  --rationale "Password hashing: Use bcrypt with cost factor 12 - balances security vs performance"
engram theory update --id $THEORY_ID \
  --rationale "Token expiry: 24 hours for access, 7 days for refresh - standard security practice"

# 6. Add invariants
engram theory update --id $THEORY_ID \
  --invariant "User.password_hash must never be NULL for users with PASSWORD credential type"
engram theory update --id $THEORY_ID \
  --invariant "Session.expires_at must be in the future for active sessions"
engram theory update --id $THEORY_ID \
  --invariant "A User may have multiple Credentials but at most one verified email"

# 7. Bind theory to session for future work
engram session bind-theory $SESSION_ID --theory $THEORY_ID

# 8. View the theory
engram theory show --id $THEORY_ID --show-metrics
```

## Example 3: State Reflection Workflow

When code behavior conflicts with theory, use reflection.

```bash
# Assume we have a theory about authentication
THEORY_ID="abc123"

# 1. Agent encounters test failure
# Test claims: "User without password should not be able to login"
# Code behavior: Users without password CAN authenticate with empty string

# 2. Create a reflection
REFLECTION_ID=$(engram reflect create \
  --theory $THEORY_ID \
  --observed "Test failed: test_login_without_password() - expected auth failure but succeeded" \
  --trigger-type test_failure \
  --agent the-architect \
  --json | jq -r '.id')

# 3. Record the dissonance
engram reflect record-dissonance \
  --id $REFLECTION_ID \
  --description "Theory invariant states 'User.password_hash must never be NULL' but code allows empty string which passes validation"

# 4. Propose theory updates
engram reflect propose-update \
  --id $REFLECTION_ID \
  --update "Either: (1) Enforce password_hash NOT NULL at DB level, OR (2) Update invariant to 'Empty password_hash rejects authentication'"

# 5. Check if mutation required
engram reflect requires-mutation --id $REFLECTION_ID
# If exit code 0, theory mutation is REQUIRED

# 6. After fixing (either update code or theory), resolve
engram reflect resolve --id $REFLECTION_ID --new-theory-id $UPDATED_THEORY_ID
```

## Example 4: Full AI Agent Workflow

Complete workflow from task creation to theory building to code implementation.

```bash
#!/bin/bash
# Full AI Agent Workflow Example

set -e

AGENT="assistant-001"
echo "=== Starting AI Agent Workflow ==="

# PHASE 1: Task Analysis
echo "Phase 1: Analyzing task requirements..."

TASK_ID=$(engram task create \
  --title "Add password reset feature" \
  --description "Implement password reset via email with time-limited tokens" \
  --priority high \
  --agent $AGENT \
  --json | jq -r '.id')

# PHASE 2: Research & Context
echo "Phase 2: Capturing research context..."

CONTEXT_IDS=()

# Store relevant docs
DOC_ID=$(engram context create \
  --title "JWT RFC 7519" \
  --source "https://datatracker.ietf.org/doc/html/rfc7519" \
  --agent $AGENT \
  --json | jq -r '.id')
CONTEXT_IDS+=($DOC_ID)

DOC_ID=$(engram context create \
  --title "Email Security Best Practices" \
  --source "https://example.com/email-security" \
  --agent $AGENT \
  --json | jq -r '.id')
CONTEXT_IDS+=($DOC_ID)

# Link contexts to task
for ctx_id in "${CONTEXT_IDS[@]}"; do
  engram relationship create \
    --source-id $TASK_ID \
    --target-id $ctx_id \
    --type references
done

# PHASE 3: Theory Building
echo "Phase 3: Building mental model..."

# Check if theory exists for relevant domain
THEORY_ID=$(engram theory list --agent $AGENT --json | jq -r '.[] | select(.domain_name == "Authentication") | .id' 2>/dev/null || echo "")

if [ -z "$THEORY_ID" ]; then
  THEORY_ID=$(engram theory create "Authentication" --agent $AGENT --json | jq -r '.id')
  
  engram theory update --id $THEORY_ID \
    --concept "User: Entity with authentication credentials"
  engram theory update --id $THEORY_ID \
    --concept "PasswordResetToken: Time-limited token for password reset"
  engram theory update --id $THEORY_ID \
    --concept "EmailNotification: Outbound email with reset link"
  engram theory update --id $THEORY_ID \
    --invariant "PasswordResetToken expires after 1 hour"
  engram theory update --id $THEORY_ID \
    --invariant "PasswordResetToken can only be used once"
fi

# Bind theory to session
SESSION_ID=$(engram session start --agent $AGENT --json | jq -r '.id')
engram session bind-theory $SESSION_ID --theory $THEORY_ID

# PHASE 4: Implementation
echo "Phase 4: Implementing feature..."

# Create subtasks
SUBTASK_1=$(engram task create \
  --title "Create PasswordResetToken entity" \
  --parent-id $TASK_ID \
  --agent $AGENT \
  --json | jq -r '.id')

SUBTASK_2=$(engram task create \
  --title "Implement token generation and validation" \
  --parent-id $TASK_ID \
  --agent $AGENT \
  --json | jq -r '.id')

SUBTASK_3=$(engram task create \
  --title "Add reset email template and sending" \
  --parent-id $TASK_ID \
  --agent $AGENT \
  --json | jq -r '.id')

# Record design decisions
engram reasoning create \
  --title "Use cryptographically secure random tokens" \
  --description "Tokens must be 32 bytes, URL-safe base64 encoded. Cannot use UUID as it's predictable." \
  --task-id $SUBTASK_2 \
  --agent $AGENT

engram reasoning create \
  --title "Store tokens as hash, not plain text" \
  --description "Prevents token leakage if database is compromised. Use same hashing as passwords." \
  --task-id $SUBTASK_2 \
  --agent $AGENT

# Update theory with new invariant discovered during implementation
engram theory update --id $THEORY_ID \
  --invariant "PasswordResetToken must be hashed before storage"

# PHASE 5: Verification
echo "Phase 5: Verifying implementation..."

# Run tests - if they fail, use reflection
TEST_RESULT=$(cargo test password_reset 2>&1 || true)

if echo "$TEST_RESULT" | grep -q "FAILED"; then
  echo "Tests failed - creating state reflection..."
  
  REFLECTION_ID=$(engram reflect create \
    --theory $THEORY_ID \
    --observed "$TEST_RESULT" \
    --trigger-type test_failure \
    --agent $AGENT \
    --json | jq -r '.id')
  
  engram reflect record-dissonance \
    --id $REFLECTION_ID \
    --description "Theory claims tokens expire in 1 hour but test expects 30 minutes"
  
  # Check if major theory update needed
  if engram reflect requires-mutation --id $REFLECTION_ID; then
    echo "Theory must be updated before fixing code"
    # Update theory...
  fi
fi

# PHASE 6: Complete
echo "Phase 6: Completing task..."

engram task update --id $SUBTASK_1 --status completed
engram task update --id $SUBTASK_2 --status completed
engram task update --id $SUBTASK_3 --status completed
engram task update --id $TASK_ID --status completed

echo "=== Workflow Complete ==="
echo "Task: $TASK_ID"
echo "Theory: $THEORY_ID"
echo "Session: $SESSION_ID"
```

## Example 5: Multi-Agent Collaboration

Multiple agents working on the same codebase.

```bash
# Agent 1: The Architect - defines the theory
ARCHITECT_SESSION=$(engram session start --agent the-architect --json | jq -r '.id')

THEORY_ID=$(engram theory create "Payment Processing" --agent the-architect --json | jq -r '.id')

engram theory update --id $THEORY_ID \
  --concept "Payment: Money transfer request with amount, currency, status"
engram theory update --id $THEORY_ID \
  --invariant "Payment.amount must be positive"
engram theory update --id $THEORY_ID \
  --invariant "Payment.status must transition: pending -> processing -> completed|failed"

# Agent 2: The Developer - implements and may find dissonance
DEV_SESSION=$(engram session start --agent developer-001 --json | jq -r '.id')

# Developer binds to existing theory
engram session bind-theory $DEV_SESSION --theory $THEORY_ID

# Developer finds issue during implementation
REFLECTION_ID=$(engram reflect create \
  --theory $THEORY_ID \
  --observed "Business requirement: refunds must be possible within 30 days" \
  --trigger-type manual_observation \
  --agent developer-001 \
  --json | jq -r '.id')

engram reflect propose-update \
  --id $REFLECTION_ID \
  --update "Add invariant: Payment can only be refunded within 30 days of completion"

# Architect reviews and resolves
engram reflect resolve --id $REFLECTION_ID --new-theory-id $THEORY_ID

# Now both agents work with updated theory
```

These examples demonstrate how all Engram features integrate: Tasks, Context, Reasoning, Knowledge, Theory, StateReflection, Sessions, and Relationships.
