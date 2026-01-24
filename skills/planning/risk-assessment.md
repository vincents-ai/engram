---
name: engram-risk-assessment
description: "Identify technical risks, dependencies, and mitigation strategies with probability/impact scoring before starting implementation."
---

# Risk Assessment (Engram-Integrated)

## Overview

Systematically identify technical risks, dependencies, and constraints before beginning implementation. Store risk analysis in Engram as reasoning entities with probability/impact scores to support decision-making and track mitigation strategies over time.

## When to Use

Use this skill when:
- Starting a new feature or architectural change with unknown complexity
- Planning work that touches critical systems or has external dependencies
- Evaluating whether to proceed with an approach or pivot
- A stakeholder asks "what could go wrong?"
- You need to justify resource allocation or timeline estimates

## The Pattern

### Step 1: Identify Risk Categories

Systematically examine these risk categories:

**Technical Risks:**
- Unknown APIs or third-party integrations
- Performance bottlenecks or scalability limits
- Data migration or backward compatibility
- Security vulnerabilities or compliance issues

**Dependency Risks:**
- External services or APIs with unknown reliability
- Library versions or platform constraints
- Team dependencies or resource availability
- Upstream/downstream system changes

**Implementation Risks:**
- Unclear requirements or acceptance criteria
- Complex state management or edge cases
- Testing challenges or deployment complexity
- Rollback or recovery procedures

### Step 2: Create Risk Reasoning Entities

For each identified risk, create a reasoning entity with structured assessment:

```bash
engram reasoning create \
  --title "Risk: [Short Description]" \
  --task-id [TASK_ID] \
  --content "**Category:** [Technical/Dependency/Implementation]\n\n**Description:** [What could go wrong and why]\n\n**Probability:** [High/Medium/Low] ([0.0-1.0])\n**Impact:** [Critical/High/Medium/Low] ([0.0-1.0])\n**Risk Score:** [Probability × Impact = 0.0-1.0]\n\n**Indicators:**\n- [Observable signal that risk is materializing]\n- [Another early warning sign]\n\n**Mitigation Strategy:**\n1. [Preventive action to reduce probability]\n2. [Contingency plan if risk occurs]\n3. [Monitoring or checkpoints]\n\n**Owner:** [Who tracks this risk]\n**Review Date:** [When to reassess]" \
  --confidence [risk_score] \
  --tags "risk,risk-assessment,[category],[feature-name]"
```

### Step 3: Score Risks with Probability × Impact

**Probability Scale:**
- High (0.7-1.0): Very likely to occur
- Medium (0.4-0.6): Might occur
- Low (0.1-0.3): Unlikely but possible

**Impact Scale:**
- Critical (0.8-1.0): Blocks release or causes major incidents
- High (0.6-0.7): Significant rework or delays
- Medium (0.3-0.5): Minor delays or workarounds needed
- Low (0.1-0.2): Negligible impact

**Risk Score = Probability × Impact**
- 0.5-1.0: High priority - needs mitigation before proceeding
- 0.2-0.5: Medium priority - monitor and plan contingencies
- 0.0-0.2: Low priority - accept and document

### Step 4: Create Risk Summary Reasoning

After assessing all risks, create a summary:

```bash
engram reasoning create \
  --title "Risk Assessment Summary: [Feature Name]" \
  --task-id [TASK_ID] \
  --content "**Total Risks Identified:** [N]\n\n**High Priority (≥0.5):**\n- [Risk 1] (score: 0.72)\n- [Risk 2] (score: 0.56)\n\n**Medium Priority (0.2-0.5):**\n- [Risk 3] (score: 0.35)\n\n**Low Priority (<0.2):**\n- [Risk 4] (score: 0.15)\n\n**Recommendation:**\n[Proceed/Proceed with caution/Pivot to alternative approach]\n\n**Rationale:**\n[Why this recommendation based on risk analysis]\n\n**Critical Mitigations Required:**\n1. [Must-do action before proceeding]\n2. [Another must-do action]" \
  --confidence [overall_confidence] \
  --tags "risk,risk-assessment,summary,[feature-name]"
```

### Step 5: Link Risks to Task

```bash
# Link all risk entities to the task
engram relationship create \
  --source-id [TASK_ID] --source-type task \
  --target-id [RISK_REASONING_ID] --target-type reasoning \
  --relationship-type documents \
  --agent [AGENT]
```

### Step 6: Update Risks During Implementation

As work progresses, update risk status:

```bash
# When a risk materializes or is mitigated
engram reasoning create \
  --title "Risk Update: [Original Risk Name]" \
  --task-id [TASK_ID] \
  --content "**Original Risk:** [Reference to original assessment]\n\n**Status:** [Materialized/Mitigated/Monitoring]\n\n**What Happened:**\n[Description of event or mitigation action]\n\n**Impact:**\n[Actual impact vs predicted]\n\n**Lessons Learned:**\n[What we learned for future risk assessments]" \
  --confidence 1.0 \
  --tags "risk,risk-update,[feature-name]"
```

## Example

User wants to implement real-time collaboration features with WebSocket connections.

### Step 1: Identify Risks

Agent examines categories and identifies 5 risks:

1. WebSocket server scalability under load
2. Message ordering guarantees with multiple clients
3. Connection recovery and state synchronization
4. Browser compatibility for WebSocket API
5. Deployment complexity with persistent connections

### Step 2: Create Risk Entities

```bash
# Risk 1: WebSocket Scalability
RISK1=$(engram reasoning create \
  --title "Risk: WebSocket Server Cannot Scale Under High Load" \
  --task-id a3f8b2c1-1234-5678-90ab-cdef12345678 \
  --content "**Category:** Technical\n\n**Description:** Current server architecture uses single-threaded event loop. Under high load (>1000 concurrent connections), server may become unresponsive, dropping connections or delaying message delivery.\n\n**Probability:** Medium (0.5)\n**Impact:** High (0.7)\n**Risk Score:** 0.35\n\n**Indicators:**\n- Load tests show degradation above 500 connections\n- CPU usage approaches 100% on single core\n- Message latency increases beyond 100ms\n\n**Mitigation Strategy:**\n1. Run load tests before implementation (preventive)\n2. Design horizontal scaling with Redis pub/sub (preventive)\n3. Implement connection limits and queuing (contingency)\n4. Monitor connection count and latency in production\n\n**Owner:** Backend team\n**Review Date:** Before starting implementation" \
  --confidence 0.35 \
  --tags "risk,risk-assessment,technical,websocket-collab" \
  --json | jq -r '.id')

# Risk 2: Message Ordering
RISK2=$(engram reasoning create \
  --title "Risk: Message Ordering Breaks with Concurrent Edits" \
  --task-id a3f8b2c1-1234-5678-90ab-cdef12345678 \
  --content "**Category:** Implementation\n\n**Description:** Multiple clients editing same document simultaneously may send operations out of order. Without operational transformation or CRDT, document state could diverge between clients.\n\n**Probability:** High (0.8)\n**Impact:** Critical (0.9)\n**Risk Score:** 0.72\n\n**Indicators:**\n- Concurrent edit tests show state divergence\n- Users report \"lost edits\" or incorrect content\n- Conflict resolution logic triggers frequently\n\n**Mitigation Strategy:**\n1. Research CRDT libraries (Yjs, Automerge) before implementation\n2. Spike: Build proof-of-concept with 2 clients (1 day time-box)\n3. Use established OT/CRDT library instead of custom implementation\n4. Add integration tests for concurrent edits\n5. Implement conflict detection and user warnings\n\n**Owner:** Frontend team\n**Review Date:** After spike completes" \
  --confidence 0.72 \
  --tags "risk,risk-assessment,implementation,websocket-collab" \
  --json | jq -r '.id')

# Risk 3: Connection Recovery
RISK3=$(engram reasoning create \
  --title "Risk: Connection Loss Causes Data Loss" \
  --task-id a3f8b2c1-1234-5678-90ab-cdef12345678 \
  --content "**Category:** Technical\n\n**Description:** WebSocket connections drop due to network issues, mobile switching networks, or laptop sleep. Unsent changes may be lost if not persisted locally.\n\n**Probability:** High (0.7)\n**Impact:** Medium (0.5)\n**Risk Score:** 0.35\n\n**Indicators:**\n- Connection drop rate >5% in testing\n- Users report lost changes after network issues\n- Reconnection attempts fail or hang\n\n**Mitigation Strategy:**\n1. Implement exponential backoff reconnection (preventive)\n2. Queue unsent operations in IndexedDB (preventive)\n3. Add \"Connection lost - changes saved locally\" UI indicator\n4. Test with browser dev tools network throttling\n5. Support offline mode with sync on reconnect\n\n**Owner:** Frontend team\n**Review Date:** Before beta release" \
  --confidence 0.35 \
  --tags "risk,risk-assessment,technical,websocket-collab" \
  --json | jq -r '.id')

# Risk 4: Browser Compatibility
RISK4=$(engram reasoning create \
  --title "Risk: WebSocket API Not Supported in Target Browsers" \
  --task-id a3f8b2c1-1234-5678-90ab-cdef12345678 \
  --content "**Category:** Dependency\n\n**Description:** Older browsers or corporate proxies may not support WebSocket protocol. Users unable to connect would have degraded experience.\n\n**Probability:** Low (0.2)\n**Impact:** Medium (0.4)\n**Risk Score:** 0.08\n\n**Indicators:**\n- Browser analytics show users on IE11 or old Android\n- Customer support tickets about connection failures\n- WebSocket connection fails in testing\n\n**Mitigation Strategy:**\n1. Check analytics for browser versions before starting\n2. Document minimum browser requirements\n3. Implement graceful fallback to polling for unsupported browsers\n4. Test on actual target browsers, not just modern ones\n\n**Owner:** Frontend team\n**Review Date:** During requirements phase" \
  --confidence 0.08 \
  --tags "risk,risk-assessment,dependency,websocket-collab" \
  --json | jq -r '.id')

# Risk 5: Deployment Complexity
RISK5=$(engram reasoning create \
  --title "Risk: Sticky Sessions Required for WebSocket Routing" \
  --task-id a3f8b2c1-1234-5678-90ab-cdef12345678 \
  --content "**Category:** Dependency\n\n**Description:** Current load balancer does not support sticky sessions. WebSocket connections may route to different servers on reconnect, causing state loss.\n\n**Probability:** Medium (0.6)\n**Impact:** Medium (0.5)\n**Risk Score:** 0.30\n\n**Indicators:**\n- Load balancer config does not show session affinity\n- Deployment tests show reconnection to different servers\n- State is lost on server restart\n\n**Mitigation Strategy:**\n1. Audit current load balancer capabilities (preventive)\n2. Configure sticky sessions or upgrade LB if needed\n3. Alternative: Use Redis for shared state across servers\n4. Test failover scenarios in staging\n5. Document deployment requirements for ops team\n\n**Owner:** DevOps team\n**Review Date:** Before production deployment" \
  --confidence 0.30 \
  --tags "risk,risk-assessment,dependency,websocket-collab" \
  --json | jq -r '.id')
```

### Step 3: Create Summary

```bash
# Risk assessment summary
SUMMARY=$(engram reasoning create \
  --title "Risk Assessment Summary: Real-Time WebSocket Collaboration" \
  --task-id a3f8b2c1-1234-5678-90ab-cdef12345678 \
  --content "**Total Risks Identified:** 5\n\n**High Priority (≥0.5):**\n- Message Ordering Breaks with Concurrent Edits (score: 0.72)\n\n**Medium Priority (0.2-0.5):**\n- WebSocket Server Cannot Scale Under High Load (score: 0.35)\n- Connection Loss Causes Data Loss (score: 0.35)\n- Sticky Sessions Required for WebSocket Routing (score: 0.30)\n\n**Low Priority (<0.2):**\n- WebSocket API Not Supported in Target Browsers (score: 0.08)\n\n**Recommendation:** Proceed with caution after spike\n\n**Rationale:**\nMessage ordering (score: 0.72) is a critical risk that could cause data corruption. The other risks are manageable with known mitigation strategies. Recommend 1-day spike to validate CRDT approach before committing to full implementation.\n\n**Critical Mitigations Required:**\n1. Complete CRDT spike investigation within 1 day\n2. Run load tests to validate scalability assumptions\n3. Design connection recovery with local persistence\n4. Audit and configure load balancer for sticky sessions" \
  --confidence 0.65 \
  --tags "risk,risk-assessment,summary,websocket-collab" \
  --json | jq -r '.id')
```

### Step 4: Link to Task

```bash
# Link all risk entities to task
engram relationship create \
  --source-id a3f8b2c1-1234-5678-90ab-cdef12345678 --source-type task \
  --target-id $RISK1 --target-type reasoning \
  --relationship-type documents --agent default

engram relationship create \
  --source-id a3f8b2c1-1234-5678-90ab-cdef12345678 --source-type task \
  --target-id $RISK2 --target-type reasoning \
  --relationship-type documents --agent default

engram relationship create \
  --source-id a3f8b2c1-1234-5678-90ab-cdef12345678 --source-type task \
  --target-id $RISK3 --target-type reasoning \
  --relationship-type documents --agent default

engram relationship create \
  --source-id a3f8b2c1-1234-5678-90ab-cdef12345678 --source-type task \
  --target-id $RISK4 --target-type reasoning \
  --relationship-type documents --agent default

engram relationship create \
  --source-id a3f8b2c1-1234-5678-90ab-cdef12345678 --source-type task \
  --target-id $RISK5 --target-type reasoning \
  --relationship-type documents --agent default

engram relationship create \
  --source-id a3f8b2c1-1234-5678-90ab-cdef12345678 --source-type task \
  --target-id $SUMMARY --target-type reasoning \
  --relationship-type documents --agent default
```

### Step 5: Communicate Decision

Agent presents summary to user:

"I've assessed 5 risks for the WebSocket collaboration feature. One critical risk emerged: message ordering with concurrent edits (score: 0.72). I recommend a 1-day spike to validate CRDT approach before proceeding. The other 4 risks are manageable with known mitigations. All risks stored in Engram. Should we proceed with the spike?"

## Querying Risk Assessment

After completing risk assessment, agents can retrieve analysis:

```bash
# Get all risks for a task
engram relationship connected --entity-id [TASK_ID] | grep "Risk:"

# Get risk summary
engram reasoning list | grep "Risk Assessment Summary"

# Get reasoning for a specific task (includes high-priority risks)
engram reasoning list --task-id [TASK_ID]

# Get risk updates during implementation
engram relationship connected --entity-id [TASK_ID] | grep "Risk Update"
```

## Related Skills

This skill integrates with:
- `engram-spike-investigation` - Time-box research to address high-probability risks
- `engram-dependency-mapping` - Identify dependency risks and critical paths
- `engram-assumption-validation` - Test implicit assumptions that may introduce risk
- `engram-brainstorming` - Use during design phase to surface risks early
- `engram-writing-plans` - Incorporate mitigation strategies into implementation plans
