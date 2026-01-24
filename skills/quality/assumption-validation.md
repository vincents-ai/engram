---
name: engram-assumption-validation
description: "Surface implicit assumptions, test them systematically, and create reasoning entities to document validation outcomes."
---

# Assumption Validation (Engram-Integrated)

## Overview

Systematically identify, surface, and test implicit assumptions that underlie technical decisions, designs, and implementations. Store assumptions and validation results as Engram reasoning entities to prevent costly mistakes from unverified beliefs.

## When to Use

Use this skill when:
- Starting implementation based on a design or plan
- Evaluating a technical approach recommended by someone else
- Debugging unexpected behavior ("it should work but doesn't")
- Reviewing code or architecture decisions
- Before committing to a costly technical decision
- A spike or prototype surfaced surprising results
- You catch yourself saying "I assume..." or "probably..."

## The Pattern

### Step 1: Surface Implicit Assumptions

Systematically examine these categories:

**Technical Assumptions:**
- Library or API behavior ("this function is thread-safe")
- Performance characteristics ("this will be fast enough")
- Platform capabilities ("browsers support this feature")
- Data format or schema ("IDs are always numeric")

**Integration Assumptions:**
- External service behavior ("API returns within 100ms")
- Error handling ("service returns 404 for missing resources")
- Rate limits ("we won't hit the 1000 req/sec limit")
- Authentication ("JWT tokens expire after 1 hour")

**Business Logic Assumptions:**
- User behavior ("users won't create >1000 items")
- Data distribution ("most users have <10 documents")
- Edge cases ("negative values are impossible here")
- Temporal assumptions ("events arrive in order")

**Operational Assumptions:**
- Deployment environment ("we have 16GB RAM available")
- Configuration ("environment variables are always set")
- Dependencies ("Redis is always available")
- Team knowledge ("everyone knows how to deploy this")

### Step 2: Create Assumption Entities

For each assumption, create a reasoning entity:

```bash
engram reasoning create \
  --title "Assumption: [Short Description]" \
  --task-id [TASK_ID] \
  --content "**Category:** [Technical/Integration/Business/Operational]\n\n**Assumption:**\n[What we believe to be true]\n\n**Where Used:**\n- [Code location, design decision, or implementation]\n- [Another place this assumption is relied upon]\n\n**Impact if Wrong:**\n- [What breaks if assumption is false]\n- [Severity: Critical/High/Medium/Low]\n\n**Validation Status:** [Untested/Validated/Invalidated]\n\n**Validation Method:**\n[How we plan to test this assumption]\n\n**Validation Result:**\n[To be filled after testing]\n\n**Confidence Before:** [0.0-1.0] - [How sure we are before testing]\n**Confidence After:** [To be filled after testing]" \
  --confidence [initial_confidence] \
  --tags "assumption,untested,[category],[feature-name]"
```

### Step 3: Prioritize Assumptions for Testing

Identify high-risk assumptions:

```bash
engram reasoning create \
  --title "Assumption Risk Analysis: [Feature Name]" \
  --task-id [TASK_ID] \
  --content "## High-Risk Assumptions (Must Validate)\n\n**Priority 1: [Assumption]**\n- Impact if wrong: Critical\n- Confidence: Low (0.3)\n- Risk Score: 0.7 × 1.0 = 0.70\n- Reason: [Why this is high risk]\n\n**Priority 2: [Assumption]**\n- Impact if wrong: High\n- Confidence: Medium (0.5)\n- Risk Score: 0.5 × 0.8 = 0.40\n- Reason: [Why this is high risk]\n\n## Medium-Risk Assumptions (Should Validate)\n\n**[Assumption]**\n- Impact if wrong: Medium\n- Confidence: Medium (0.6)\n- Risk Score: 0.4 × 0.5 = 0.20\n\n## Low-Risk Assumptions (Accept)\n\n**[Assumption]**\n- Impact if wrong: Low\n- Confidence: High (0.9)\n- Risk Score: 0.1 × 0.2 = 0.02\n- Reason: [Why we accept this risk]\n\n## Validation Plan\n\n1. Test [High-Risk Assumption 1] - [Method] - [ETA: N hours]\n2. Test [High-Risk Assumption 2] - [Method] - [ETA: N hours]\n3. Accept [Low-Risk Assumptions] - Document only" \
  --confidence 0.8 \
  --tags "assumption,risk-analysis,[feature-name]"
```

### Step 4: Validate Assumptions

For each high-risk assumption, run validation test:

**Validation Methods:**
- **Code test:** Write unit test that fails if assumption is false
- **Integration test:** Call external API/service and verify behavior
- **Performance test:** Measure actual performance vs assumed
- **Manual verification:** Read documentation, source code, or ask expert
- **Experiment:** Create minimal reproduction and observe behavior

### Step 5: Document Validation Results

Update assumption entity with results:

```bash
engram reasoning create \
  --title "Assumption Validated: [Short Description]" \
  --task-id [TASK_ID] \
  --content "**Original Assumption:**\n[What we believed to be true]\n\n**Validation Method:**\n[How we tested it]\n\n**Result:** [Validated/Invalidated/Partially True]\n\n**Evidence:**\n\`\`\`\n[Code snippet, test output, or measurement]\n\`\`\`\n\n**Findings:**\n[What we learned]\n\n**Impact:**\n- [If validated: proceed as planned]\n- [If invalidated: what needs to change]\n\n**Confidence Before:** [0.0-1.0]\n**Confidence After:** [0.0-1.0]\n\n**Recommendation:**\n[What to do based on validation result]" \
  --confidence [new_confidence] \
  --tags "assumption,validated,[category],[feature-name]"
```

### Step 6: Handle Invalidated Assumptions

When assumption proves false, create mitigation plan:

```bash
engram reasoning create \
  --title "Assumption Invalidated: [Short Description] - Mitigation Plan" \
  --task-id [TASK_ID] \
  --content "**Invalidated Assumption:**\n[What we believed but is false]\n\n**Actual Behavior:**\n[What is actually true]\n\n**Code/Design Impact:**\n[What parts of design or code relied on this]\n\n**Options:**\n\n### Option 1: [Approach]\n- **Change Required:** [What to modify]\n- **Effort:** [N hours/days]\n- **Pros:** [Benefits]\n- **Cons:** [Downsides]\n\n### Option 2: [Alternative Approach]\n- **Change Required:** [What to modify]\n- **Effort:** [N hours/days]\n- **Pros:** [Benefits]\n- **Cons:** [Downsides]\n\n**Recommendation:** [Which option to pursue]\n\n**Rationale:**\n[Why this option is best]\n\n**Updated Plan:**\n1. [Step to implement mitigation]\n2. [Another step]\n\n**Lessons Learned:**\n[What to validate earlier next time]" \
  --confidence 0.9 \
  --tags "assumption,invalidated,mitigation,[feature-name]"
```

### Step 7: Link Assumptions to Tasks

```bash
# Link assumptions to task
engram relationship create \
  --source-id [TASK_ID] --source-type task \
  --target-id [ASSUMPTION_ID] --target-type reasoning \
  --relationship-type documents \
  --agent [AGENT]

# Link validation results
engram relationship create \
  --source-id [TASK_ID] --source-type task \
  --target-id [VALIDATION_ID] --target-type reasoning \
  --relationship-type documents \
  --agent [AGENT]
```

## Example

User is implementing rate limiting and assumes Redis INCR is atomic.

### Step 1: Surface Assumption

Agent identifies assumption while reviewing implementation plan:

"I notice the rate limiting design assumes Redis INCR is atomic across concurrent requests. Let me validate this before implementation."

### Step 2: Create Assumption Entity

```bash
ASSUMPTION=$(engram reasoning create \
  --title "Assumption: Redis INCR is Atomic" \
  --task-id a3f8b2c1-1234-5678-90ab-cdef12345678 \
  --content "**Category:** Technical\n\n**Assumption:**\nRedis INCR command is atomic and thread-safe when called concurrently from multiple clients. Two simultaneous INCR calls will result in counter increasing by 2, never by 1.\n\n**Where Used:**\n- Rate limiting implementation in src/middleware/rate_limit.rs:45\n- Design assumes no race conditions when checking rate limits\n\n**Impact if Wrong:**\n- Rate limiting could allow more requests than intended\n- Severity: High - could lead to resource exhaustion or abuse\n\n**Validation Status:** Untested\n\n**Validation Method:**\nWrite test that spawns 100 concurrent threads, each calling INCR on same key 100 times. Verify final count is exactly 10,000.\n\n**Validation Result:**\n[To be filled after testing]\n\n**Confidence Before:** 0.70 - Documentation suggests atomicity but not explicitly stated\n**Confidence After:** [To be filled after testing]" \
  --confidence 0.70 \
  --tags "assumption,untested,technical,rate-limiting" \
  --json | jq -r '.id')

echo "Assumption created: $ASSUMPTION"
```

### Step 3: Validate Assumption

Agent writes test to validate:

```bash
# Create test file
cat > /tmp/test_redis_atomic.rs <<'EOF'
use redis::{Client, Commands};
use std::sync::Arc;
use std::thread;

#[test]
fn test_redis_incr_is_atomic() {
    let client = Client::open("redis://localhost:6379").unwrap();
    let conn = Arc::new(client.get_connection().unwrap());
    
    // Reset counter
    let _: () = conn.set("test_counter", 0).unwrap();
    
    // Spawn 100 threads, each increments 100 times
    let handles: Vec<_> = (0..100).map(|_| {
        let conn_clone = Arc::clone(&conn);
        thread::spawn(move || {
            for _ in 0..100 {
                let _: i32 = conn_clone.incr("test_counter", 1).unwrap();
            }
        })
    }).collect();
    
    // Wait for all threads
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Verify final count
    let final_count: i32 = conn.get("test_counter").unwrap();
    assert_eq!(final_count, 10_000, "INCR is not atomic!");
}
EOF

# Run test
cargo test test_redis_incr_is_atomic -- --nocapture
```

Test output:
```
running 1 test
test test_redis_incr_is_atomic ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

### Step 4: Document Validation Result

```bash
VALIDATION=$(engram reasoning create \
  --title "Assumption Validated: Redis INCR is Atomic" \
  --task-id a3f8b2c1-1234-5678-90ab-cdef12345678 \
  --content "**Original Assumption:**\nRedis INCR command is atomic and thread-safe when called concurrently from multiple clients.\n\n**Validation Method:**\nWrote test with 100 concurrent threads, each calling INCR 100 times (total 10,000 increments). Verified final counter value.\n\n**Result:** Validated\n\n**Evidence:**\n\`\`\`\ntest test_redis_incr_is_atomic ... ok\nFinal counter value: 10,000 (expected: 10,000)\n\`\`\`\n\n**Findings:**\nRedis INCR is indeed atomic. Redis is single-threaded and executes commands serially, guaranteeing atomicity for all operations including INCR. This is explicitly documented in Redis command reference.\n\n**Additional Discovery:**\nRedis also guarantees atomicity for INCRBY, DECR, DECRBY, and other numeric operations. This extends to INCRBYFLOAT as well.\n\n**Impact:**\n- Rate limiting implementation is safe as designed\n- Can proceed with implementation\n- No race conditions expected\n\n**Confidence Before:** 0.70 (documentation review only)\n**Confidence After:** 0.95 (validated with test)\n\n**Recommendation:**\nProceed with rate limiting implementation as planned. Add integration test to CI that validates concurrent rate limit checks." \
  --confidence 0.95 \
  --tags "assumption,validated,technical,rate-limiting" \
  --json | jq -r '.id')

echo "Validation complete: $VALIDATION"
```

### Step 5: Link to Task

```bash
engram relationship create \
  --source-id a3f8b2c1-1234-5678-90ab-cdef12345678 --source-type task \
  --target-id $ASSUMPTION --target-type reasoning \
  --relationship-type documents --agent default

engram relationship create \
  --source-id a3f8b2c1-1234-5678-90ab-cdef12345678 --source-type task \
  --target-id $VALIDATION --target-type reasoning \
  --relationship-type documents --agent default
```

### Step 6: Communicate to User

Agent reports validation result:

"Validated assumption: Redis INCR is atomic. Ran test with 100 concurrent threads (10K increments) - no race conditions. Confidence increased from 0.70 to 0.95. Rate limiting design is safe. Proceeding with implementation."

## Example: Invalidated Assumption

User assumes Stripe webhook events arrive in order.

### Step 1: Create Assumption

```bash
ASSUMPTION2=$(engram reasoning create \
  --title "Assumption: Stripe Webhook Events Arrive in Order" \
  --task-id a3f8b2c1-1234-5678-90ab-cdef12345678 \
  --content "**Category:** Integration\n\n**Assumption:**\nStripe sends webhook events in chronological order. If payment succeeds then fails, we receive payment_intent.succeeded before payment_intent.payment_failed.\n\n**Where Used:**\n- Webhook handler processes events sequentially\n- State machine assumes ordered state transitions\n\n**Impact if Wrong:**\n- Could process events out of order, causing incorrect state\n- Severity: Critical - could show failed payment as succeeded\n\n**Validation Status:** Untested\n\n**Validation Method:**\nRead Stripe documentation on webhook delivery guarantees. Check if Stripe provides ordering guarantees or event timestamps.\n\n**Confidence Before:** 0.50 - Uncertain, seems logical but unverified" \
  --confidence 0.50 \
  --tags "assumption,untested,integration,stripe-payments" \
  --json | jq -r '.id')
```

### Step 2: Validate

Agent reads Stripe documentation:

"Stripe documentation states: 'Webhook events are not guaranteed to arrive in order. Your endpoint should handle events in any order.' Events include created_at timestamp for ordering."

### Step 3: Document Invalidation

```bash
INVALIDATION=$(engram reasoning create \
  --title "Assumption Invalidated: Stripe Webhooks Are NOT Ordered" \
  --task-id a3f8b2c1-1234-5678-90ab-cdef12345678 \
  --content "**Invalidated Assumption:**\nStripe sends webhook events in chronological order.\n\n**Actual Behavior:**\nStripe explicitly does NOT guarantee webhook ordering. Events can arrive out of order due to retries, network delays, or concurrent delivery.\n\n**Evidence:**\n> \"Webhook events are not guaranteed to arrive in order. Your endpoint should handle events in any order.\"\n> - Stripe Webhooks Documentation\n\nEach event includes 'created' timestamp (Unix time) for ordering.\n\n**Code/Design Impact:**\n- Current webhook handler assumes sequential processing\n- State machine in src/payments/webhook.rs:78 will break with out-of-order events\n- Could mark succeeded payment as failed if failure event arrives first\n\n**Confidence Before:** 0.50\n**Confidence After:** 0.95 (explicitly documented)\n\n**Must Fix Before Implementation**" \
  --confidence 0.95 \
  --tags "assumption,invalidated,integration,stripe-payments" \
  --json | jq -r '.id')
```

### Step 4: Create Mitigation Plan

```bash
MITIGATION=$(engram reasoning create \
  --title "Assumption Invalidated: Stripe Webhooks - Mitigation Plan" \
  --task-id a3f8b2c1-1234-5678-90ab-cdef12345678 \
  --content "**Invalidated Assumption:**\nStripe webhook events arrive in order.\n\n**Actual Behavior:**\nEvents can arrive in any order. Must use 'created' timestamp to determine ordering.\n\n**Code/Design Impact:**\n- Webhook handler state machine in src/payments/webhook.rs:78\n- Payment status updates in database\n\n**Options:**\n\n### Option 1: Event Timestamp Ordering\n- **Change Required:** \n  1. Store latest event timestamp per payment_intent in database\n  2. Compare incoming event timestamp with stored timestamp\n  3. Only process event if timestamp is newer\n  4. Handle idempotency for duplicate events\n- **Effort:** 3 hours\n- **Pros:** \n  - Handles out-of-order delivery correctly\n  - Simple to implement\n  - Recommended by Stripe\n- **Cons:** \n  - Requires database schema change\n  - Must handle edge case of same-timestamp events\n\n### Option 2: Event Sequence Number\n- **Change Required:**\n  1. Fetch full payment_intent state from Stripe API on each event\n  2. Use authoritative state instead of event sequence\n- **Effort:** 2 hours\n- **Pros:** \n  - Always uses correct state\n  - No ordering concerns\n- **Cons:** \n  - Additional API call per webhook (latency + cost)\n  - Rate limit risk\n\n**Recommendation:** Option 1 (Event Timestamp Ordering)\n\n**Rationale:**\nOption 1 is the Stripe-recommended approach. It's efficient (no extra API calls) and handles out-of-order delivery correctly. Schema change is minor (add last_event_timestamp column).\n\n**Updated Plan:**\n1. Add migration: payment_intents.last_event_timestamp column\n2. Update webhook handler to check timestamp before processing\n3. Add test: send events out of order, verify correct final state\n4. Update state machine to be idempotent\n\n**Code Changes:**\n\`\`\`rust\n// Before processing event\nlet event_timestamp = event.created;\nlet last_timestamp = get_last_event_timestamp(payment_intent_id);\n\nif event_timestamp <= last_timestamp {\n    // Older event, ignore\n    return Ok(StatusCode::OK);\n}\n\n// Process event\nupdate_payment_state(payment_intent_id, event);\nupdate_last_event_timestamp(payment_intent_id, event_timestamp);\n\`\`\`\n\n**Lessons Learned:**\nAlways validate assumptions about external service behavior, especially ordering and timing guarantees. Read official documentation before designing integration." \
  --confidence 0.90 \
  --tags "assumption,invalidated,mitigation,stripe-payments" \
  --json | jq -r '.id')

echo "Mitigation plan created: $MITIGATION"
```

### Step 5: Communicate to User

Agent reports invalidated assumption and mitigation:

"Critical finding: Stripe webhooks are NOT ordered (docs explicitly state this). Current design would break with out-of-order events. **Mitigation:** Store event timestamps, only process newer events. Requires 3 hours: add database column, update handler logic, add tests. Must fix before implementation. Ready to proceed with mitigation?"

## Querying Assumptions

After assumption validation, agents can retrieve:

```bash
# Get all assumptions for a task
engram reasoning list --task-id [TASK_ID] | grep "Assumption"

# Get validated assumptions
engram reasoning list | grep "Assumption Validated"

# Get invalidated assumptions (need mitigation)
engram reasoning list | grep "Assumption Invalidated"

# Get all assumptions using relationships
engram relationship connected --entity-id [TASK_ID] | grep "Assumption"

# Get risk analyses
engram reasoning list | grep "Assumption Risk Analysis"
```

## Related Skills

This skill integrates with:
- `engram-spike-investigation` - Use spikes to test high-risk assumptions
- `engram-risk-assessment` - Assumptions with low confidence are risks
- `engram-brainstorming` - Surface assumptions during design discussions
- `engram-dependency-mapping` - Assumptions about dependency behavior
- `engram-system-design` - Validate assumptions about performance, consistency, availability
