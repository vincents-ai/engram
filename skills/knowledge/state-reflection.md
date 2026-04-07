---
name: engram-state-reflection
description: "When observed behaviour contradicts your current theory, create a StateReflection and record dissonance. If score >= 0.7, update the theory before writing any fix."
---

# State Reflection

## Overview

Every unexpected failure is a signal. Not just that the code is wrong — but that your mental model of the code is wrong. The bug is evidence that your theory predicted one outcome and the system produced another. If you fix the code without correcting the theory, the next agent (or the next version of you) will walk into the same gap.

`engram reflect` gives that gap a name. A `StateReflection` entity records what you observed, how far it diverges from your current theory (the `dissonance_score`), what specific contradictions you found, and what theory updates would resolve them. When the dissonance is severe enough, the system enforces a theory update before any fix is permitted.

## The Rule

**Fix the theory. Then fix the code.**

A `StateReflection` with `dissonance_score >= 0.7` means your theory is fundamentally broken. Do not touch the code until you have updated the theory and resolved the reflection.

## Dissonance Thresholds

| Score range | Severity | Action required |
|---|---|---|
| `0.0` | None | No dissonance recorded yet |
| `> 0.0` and `< 0.3` | Low | Note it; theory adjustment likely minor |
| `>= 0.3` and `< 0.5` | Medium | Theory needs updating; do it before closing the task |
| `>= 0.5` and `< 0.7` | High | Significant gap; update theory before writing the fix |
| `>= 0.7` | Critical | **Mutation required.** `engram reflect requires-mutation` exits 0. Do not proceed until theory is updated and reflection resolved. |

## Trigger Types

| Trigger type | When to use |
|---|---|
| `test_failure` | A test produced an unexpected result |
| `runtime_error` | A panic, crash, or unhandled error at runtime |
| `unexpected_output` | Function returned a value your theory didn't predict |
| `type_mismatch` | A type relationship or constraint was violated |
| `behavioral_deviation` | System behaved correctly but not as your model expected |
| `manual_observation` | You noticed something surprising while reading code or logs |
| `performance_anomaly` | Timing or resource usage contradicts your model |
| `security_concern` | A trust boundary or permission model behaved unexpectedly |

## The 5-Step Workflow

### Step 1: Create the reflection

```bash
engram reflect create \
  --theory <THEORY_ID> \
  --observed "<what you actually saw>" \
  --trigger-type <TRIGGER_TYPE> \
  --agent "<your-name>" \
  --output json
# Returns: REFLECTION_ID
```

If you don't have a theory yet for this domain, create one first with `engram theory create`. See the `engram-theory-building` skill.

### Step 2: Record each dissonance item

Call this once per specific contradiction you identify. The final score is the maximum of all recorded scores.

```bash
engram reflect record-dissonance \
  --id <REFLECTION_ID> \
  --description "<specific contradiction: what the theory predicted vs what happened>" \
  --score <0.0-1.0>
```

Be specific. "Something was wrong" scores 0.2. "The theory states invariant X must hold, but the test demonstrates it does not when input is empty" scores 0.8.

### Step 3: Propose theory updates

For each dissonance item, record what change to the theory would resolve it:

```bash
engram reflect propose-update \
  --id <REFLECTION_ID> \
  --update "<what should change in the theory: which concept/mapping/rationale/invariant>"
```

You are not fixing the code yet. You are deciding how your mental model needs to change.

### Step 4: Check whether mutation is required

```bash
engram reflect requires-mutation --id <REFLECTION_ID>
# Exit 0 = mutation IS required (score >= 0.7). Stop. Update theory first.
# Exit 1 = mutation not required. You may proceed to fix.
```

If exit 0:
1. Update the theory: `engram theory update --id <THEORY_ID> --concept ... --rationale ... --invariant ...`
2. Then return to step 5.

### Step 5: Resolve the reflection

Once the theory has been updated (or if mutation was not required and you have applied your fix):

```bash
engram reflect resolve \
  --id <REFLECTION_ID> \
  --new-theory <UPDATED_THEORY_ID>
```

`--new-theory` accepts the same theory ID if you updated in place (most common), or a new theory ID if you created a replacement.

## Session Integration

If you are working within a session bound to a theory, the session can enter a `Reflecting` status that blocks code changes at the session level:

```bash
# Trigger reflection status on the session (blocks code changes)
engram session trigger-reflection --id <SESSION_ID>

# After theory is updated and reflection resolved:
engram session resolve-reflection --id <SESSION_ID>
```

Session reflection status is typically managed automatically when using a bound theory, but can be triggered manually when you encounter something surprising before a formal reflection is recorded.

## Command Reference

| Command | Description |
|---|---|
| `engram reflect create --theory <ID> --observed "<text>" --trigger-type <TYPE>` | Create a new state reflection |
| `engram reflect list` | List reflections; filter with `--theory`, `--trigger-type`, `--unresolved` |
| `engram reflect show --id <ID>` | Show full reflection detail including dissonance items and proposed updates |
| `engram reflect record-dissonance --id <ID> --description "<text>" --score <float>` | Record a specific theory-reality gap |
| `engram reflect propose-update --id <ID> --update "<text>"` | Record a proposed theory correction |
| `engram reflect requires-mutation --id <ID>` | Exit 0 if mutation required (score >= 0.7); exit 1 otherwise |
| `engram reflect requires-mutation --id <ID> --threshold 0.5` | Use a custom threshold |
| `engram reflect resolve --id <ID> --new-theory <THEORY_ID>` | Mark reflection resolved after theory update |
| `engram reflect delete --id <ID>` | Delete a reflection |

## Example

A test failure reveals that the `dissonance_score` on a `StateReflection` was not being clamped correctly. The current theory states `dissonance_score is always clamped to 0.0–1.0` as an invariant — but the test produced a score of `1.4`.

```bash
# Step 1 — create the reflection
engram reflect create \
  --theory a1b2c3d4 \
  --observed "test_record_dissonance_accumulates_to_max passed a score of 1.4 and the entity stored 1.4, not 1.0" \
  --trigger-type test_failure \
  --agent "orchestrator" \
  --output json
# REFLECTION_ID = r9s8t7u6-...

# Step 2 — record the specific contradiction
engram reflect record-dissonance \
  --id r9s8t7u6 \
  --description "Theory invariant states score is clamped to 0.0-1.0. Test shows scores above 1.0 are accepted and stored unclamped." \
  --score 0.85

# Step 3 — propose the theory update
engram reflect propose-update \
  --id r9s8t7u6 \
  --update "Invariant 'dissonance_score is always clamped to 0.0-1.0' should be changed to 'clamping is the caller's responsibility; the entity stores whatever score is provided'. OR: confirm clamping is implemented and the invariant stands — the test is the bug."

# Step 4 — check mutation requirement
engram reflect requires-mutation --id r9s8t7u6
# Exit 0: mutation required (score 0.85 >= 0.7)

# Score is 0.85 — stop. Investigate and update theory first.
# After investigation: clamping IS missing from record_dissonance(). The invariant is correct; the implementation is wrong.
# Update theory to add rationale:
engram theory update --id a1b2c3d4 \
  --rationale "dissonance_score clamping:record_dissonance() is responsible for clamping; callers must not bypass this"

# Step 5 — resolve
engram reflect resolve \
  --id r9s8t7u6 \
  --new-theory a1b2c3d4

# Now fix the code.
```

## Related Skills

- `engram-theory-building` — create and maintain the Theory entity this skill reflects against
- `engram-systematic-debugging` — structured root cause investigation; use alongside reflection for complex bugs
- `engram-use-engram-memory` — foundational memory discipline all skills build on
- `engram-audit-trail` — ensure your reflection and resolution are linked to the active task
