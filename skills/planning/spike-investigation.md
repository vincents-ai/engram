---
name: engram-spike-investigation
description: "Time-boxed research to reduce uncertainty through rapid prototyping, storing findings and recommendations in Engram."
---

# Spike Investigation (Engram-Integrated)

## Overview

Conduct time-boxed technical investigations to reduce uncertainty before committing to an approach. Create throwaway prototypes, gather data, and store findings as Engram context and reasoning entities to inform decision-making without expanding scope.

## When to Use

Use this skill when:
- You have a high-risk technical decision with unknown feasibility
- Multiple approaches exist but trade-offs are unclear
- External API or library behavior is undocumented or uncertain
- Performance characteristics need measurement before committing
- You need to answer "can we even do this?" before planning
- A risk assessment identified high-probability unknowns

## The Pattern

### Step 1: Define Spike Boundaries

Create a spike task with clear time-box and questions:

```bash
engram task create \
  --title "Spike: [Investigation Topic]" \
  --description "**Time-box:** [N hours/days - HARD LIMIT]\n\n**Questions to Answer:**\n1. [Specific question]\n2. [Another specific question]\n\n**Success Criteria:**\n- [What defines a successful spike]\n- [Quantitative threshold if applicable]\n\n**Out of Scope:**\n- [What we're NOT investigating]\n- [Remind: this is throwaway code]\n\n**Deliverables:**\n- Engram context with findings\n- Engram reasoning with recommendation\n- Throwaway prototype (not for production)" \
  --priority high \
  --agent [AGENT] \
  --json | jq -r '.id'
```

### Step 2: Conduct Investigation

**Rules for spikes:**
- Set timer for time-box - stop when time expires
- Create throwaway code in `/tmp/spike-[topic]` or similar
- Focus on answering questions, not writing production code
- Document as you go - capture observations immediately
- Measure quantitatively where possible (performance, error rates, etc.)

**Investigation activities:**
- Write minimal proof-of-concept code
- Test edge cases and failure modes
- Measure performance or resource usage
- Read library source code if documentation is unclear
- Test integration points with existing systems

### Step 3: Document Findings as Context

Create context entity with detailed observations:

```bash
engram context create \
  --title "Spike Findings: [Investigation Topic]" \
  --content "**Duration:** [Actual time spent]\n**Time-box:** [Original limit]\n\n## Questions Investigated\n\n### 1. [Question 1]\n\n**Findings:**\n[What you learned]\n\n**Evidence:**\n- [Code snippet or test result]\n- [Measurement or observation]\n\n**Confidence:** [High/Medium/Low]\n\n### 2. [Question 2]\n\n**Findings:**\n[What you learned]\n\n**Evidence:**\n- [Code snippet or test result]\n\n**Confidence:** [High/Medium/Low]\n\n## Unexpected Discoveries\n\n- [Something you didn't expect to find]\n- [Another surprise]\n\n## Code Artifacts\n\n**Location:** /tmp/spike-[topic]\n**Status:** Throwaway - do not use in production\n**Key files:**\n- [prototype.py] - Demonstrates [concept]\n- [test_edge_case.py] - Shows [behavior]\n\n## References\n\n- [Link to documentation]\n- [Stack Overflow answer]\n- [Library source code file:line]" \
  --source "spike-investigation" \
  --tags "spike,investigation,[topic],[feature-name]"
```

### Step 4: Create Recommendation Reasoning

Based on findings, create recommendation with confidence:

```bash
engram reasoning create \
  --title "Spike Recommendation: [Investigation Topic]" \
  --task-id [SPIKE_TASK_ID] \
  --content "**Recommendation:** [Proceed/Use Alternative/Need More Investigation]\n\n**Rationale:**\n[Why this recommendation based on findings]\n\n**Confidence:** [0.0-1.0]\n\n**Supporting Evidence:**\n1. [Finding that supports recommendation]\n2. [Another supporting finding]\n\n**Risks Remaining:**\n- [Known unknown still present]\n- [Assumption that needs validation]\n\n**Next Steps:**\n1. [Immediate action if proceeding]\n2. [Another next step]\n\n**If Proceeding, Consider:**\n- [Trade-off to keep in mind]\n- [Implementation gotcha discovered]\n- [Performance consideration]\n\n**Alternative Considered:**\n[Brief note on other approaches evaluated and why not recommended]" \
  --confidence [0.0-1.0] \
  --tags "spike,recommendation,[topic],[feature-name]"
```

### Step 5: Handle Time-Box Expiry

If time-box expires before answering all questions:

```bash
engram reasoning create \
  --title "Spike Incomplete: [Investigation Topic]" \
  --task-id [SPIKE_TASK_ID] \
  --content "**Status:** Time-box expired\n**Time spent:** [N hours]\n\n**Questions Answered:**\n- [Question 1]: Yes\n- [Question 2]: Partially\n\n**Questions Remaining:**\n- [Question 3]: Not investigated\n\n**Recommendation:** [Extend spike/Use alternative approach/Proceed with risk]\n\n**Rationale:**\n[Why we couldn't answer everything and what to do next]\n\n**Extension Request:**\n[If recommending extension, justify with specific time needed and expected value]" \
  --confidence 0.5 \
  --tags "spike,incomplete,[topic]"
```

### Step 6: Link to Parent Task

```bash
# Link spike findings to parent task
engram relationship create \
  --source-id [PARENT_TASK_ID] --source-type task \
  --target-id [SPIKE_CONTEXT_ID] --target-type context \
  --relationship-type references \
  --agent [AGENT]

# Link recommendation to spike task
engram relationship create \
  --source-id [SPIKE_TASK_ID] --source-type task \
  --target-id [RECOMMENDATION_ID] --target-type reasoning \
  --relationship-type documents \
  --agent [AGENT]
```

### Step 7: Clean Up Throwaway Code

```bash
# Mark spike task as complete
engram task update --status done [SPIKE_TASK_ID]

# Remove throwaway code
rm -rf /tmp/spike-[topic]

# Note: Spike findings are preserved in Engram context [CONTEXT_ID]
```

## Example

User wants to use Operational Transform for collaborative editing but unsure if library works with existing data model.

### Step 1: Create Spike Task

```bash
SPIKE_TASK=$(engram task create \
  --title "Spike: OT.js Integration with Current Data Model" \
  --description "**Time-box:** 4 hours - HARD LIMIT\n\n**Questions to Answer:**\n1. Can OT.js operations serialize to our existing edit format?\n2. Does OT.js handle our custom inline annotations (mentions, links)?\n3. What's the performance overhead for documents >100KB?\n\n**Success Criteria:**\n- All 3 questions answered with code evidence\n- Performance within 50ms for 100KB document\n\n**Out of Scope:**\n- Full integration implementation\n- Production-ready error handling\n- UI components\n\n**Deliverables:**\n- Engram context with findings\n- Engram reasoning with recommendation\n- Throwaway prototype in /tmp/spike-ot" \
  --priority high \
  --agent default \
  --json | jq -r '.id')

echo "Spike task created: $SPIKE_TASK"
```

### Step 2: Conduct Investigation

Agent sets 4-hour timer and starts coding in `/tmp/spike-ot`:

```bash
# Agent creates throwaway prototype
mkdir -p /tmp/spike-ot
cd /tmp/spike-ot

# Install OT.js and test integration
npm init -y
npm install ot.js

# Write minimal test code (not shown for brevity)
# Agent spends ~3.5 hours testing, measuring, reading source
```

### Step 3: Document Findings

```bash
FINDINGS=$(engram context create \
  --title "Spike Findings: OT.js Integration with Current Data Model" \
  --content "**Duration:** 3 hours 45 minutes\n**Time-box:** 4 hours\n\n## Questions Investigated\n\n### 1. Can OT.js operations serialize to our existing edit format?\n\n**Findings:**\nYes, with adapter layer. OT.js uses {position, delete, insert} format. Our format uses {offset, length, text}. Created 50-line adapter that converts bidirectionally.\n\n**Evidence:**\n\`\`\`javascript\n// Adapter converts OT.js -> Our format\nfunction otToEdit(op) {\n  return {\n    offset: op.position,\n    length: op.delete || 0,\n    text: op.insert || ''\n  };\n}\n// Test: 1000 ops converted in 12ms\n\`\`\`\n\n**Confidence:** High\n\n### 2. Does OT.js handle our custom inline annotations (mentions, links)?\n\n**Findings:**\nNo, OT.js only handles plain text. Annotations are lost during transformation. Workaround: treat annotations as character metadata and transform separately using OT.js position transforms.\n\n**Evidence:**\n- Tested mention insertion during concurrent edit\n- Mention position shifted correctly when using position transform\n- Annotation preserved in separate data structure\n\n**Confidence:** Medium - needs more edge case testing\n\n### 3. What's the performance overhead for documents >100KB?\n\n**Findings:**\nAcceptable. Tested with 150KB document (50 pages). Applying 100 sequential operations takes 45ms. Composition of 100 ops takes 120ms but happens once on server.\n\n**Evidence:**\n\`\`\`\nDocument size: 153KB (51,234 chars)\nOperation application: 45ms (average of 10 runs)\nOperation composition: 118ms (average of 10 runs)\nMemory overhead: +2.3MB for operation history (100 ops)\n\`\`\`\n\n**Confidence:** High\n\n## Unexpected Discoveries\n\n- OT.js has built-in position transform utility perfect for annotations\n- Library is well-maintained but docs are sparse - had to read source\n- Composition is expensive but only happens server-side\n- No TypeScript types - would need to write our own\n\n## Code Artifacts\n\n**Location:** /tmp/spike-ot\n**Status:** Throwaway - do not use in production\n**Key files:**\n- adapter.js - Bidirectional format conversion\n- test-annotations.js - Annotation preservation demo\n- perf-test.js - Performance measurements\n\n## References\n\n- OT.js GitHub: https://github.com/Operational-Transformation/ot.js\n- Position transform source: lib/text-operation.js:234-267\n- Google Wave OT paper (implementation basis)" \
  --source "spike-investigation" \
  --tags "spike,investigation,otjs,collaboration" \
  --json | jq -r '.id')

echo "Findings documented: $FINDINGS"
```

### Step 4: Create Recommendation

```bash
RECOMMENDATION=$(engram reasoning create \
  --title "Spike Recommendation: Use OT.js with Adapter Layer" \
  --task-id $SPIKE_TASK \
  --content "**Recommendation:** Proceed with OT.js using adapter layer\n\n**Rationale:**\nOT.js meets performance requirements (45ms for 100KB docs) and handles our data format with 50-line adapter. Annotation handling requires separate position transform but pattern is proven. Library is mature and well-tested.\n\n**Confidence:** 0.80\n\n**Supporting Evidence:**\n1. Performance well within 50ms threshold (45ms measured)\n2. Format conversion is straightforward and fast (12ms for 1000 ops)\n3. Annotation workaround successfully tested with mentions\n\n**Risks Remaining:**\n- Annotation edge cases need more testing (medium confidence)\n- No TypeScript types - will need custom declarations\n- Library docs are sparse - team needs to read source code\n\n**Next Steps:**\n1. Create spike task for annotation edge cases (2 hour time-box)\n2. Write TypeScript declarations for OT.js\n3. Begin implementation plan with adapter-based approach\n\n**If Proceeding, Consider:**\n- Cache composed operations server-side (expensive at 120ms)\n- Separate annotation transform into utility module\n- Budget time for TypeScript declaration maintenance\n- Document gotchas for team (position vs offset semantics)\n\n**Alternative Considered:**\nCRDT (Yjs): Would handle annotations natively but requires full data model rewrite. OT.js integrates with existing format, lower risk." \
  --confidence 0.80 \
  --tags "spike,recommendation,otjs,collaboration" \
  --json | jq -r '.id')

echo "Recommendation created: $RECOMMENDATION"
```

### Step 5: Link to Parent Task

```bash
# Link findings to parent task (assume parent is collaboration feature)
engram relationship create \
  --source-id a3f8b2c1-1234-5678-90ab-cdef12345678 --source-type task \
  --target-id $FINDINGS --target-type context \
  --relationship-type references \
  --agent default

# Link recommendation to spike task
engram relationship create \
  --source-id $SPIKE_TASK --source-type task \
  --target-id $RECOMMENDATION --target-type reasoning \
  --relationship-type documents \
  --agent default
```

### Step 6: Clean Up

```bash
# Update spike task status
engram task update --status done $SPIKE_TASK

# Remove throwaway code
rm -rf /tmp/spike-ot

echo "Spike complete. Artifacts cleaned up. Findings in Engram."
```

### Step 7: Communicate to User

Agent presents recommendation:

"Spike complete (3h 45m / 4h). OT.js works with our data model using a 50-line adapter. Performance is good (45ms for 100KB docs). Annotations need separate position transform but pattern is proven. **Recommendation: Proceed with OT.js (confidence: 0.80)**. One medium-confidence risk: annotation edge cases need 2 more hours of testing. Ready to plan implementation?"

## Querying Spike Findings

After spike investigation, agents can retrieve findings:

```bash
# Get all spike findings for a feature task
engram relationship connected --entity-id [FEATURE_TASK_ID] | grep "Spike Findings"

# Get spike recommendations for a specific spike task
engram reasoning list --task-id [SPIKE_TASK_ID]

# Get all contexts (spikes and other)
engram context list | grep "Spike Findings"

# Get all reasoning entities for a spike
engram relationship connected --entity-id [SPIKE_TASK_ID] | grep "Spike"
```

## Related Skills

This skill integrates with:
- `engram-risk-assessment` - Use spikes to reduce high-probability risks
- `engram-assumption-validation` - Test specific assumptions through spikes
- `engram-dependency-mapping` - Investigate external dependency behavior
- `engram-brainstorming` - Use spikes to evaluate design alternatives
- `engram-writing-plans` - Incorporate spike findings into implementation plans
