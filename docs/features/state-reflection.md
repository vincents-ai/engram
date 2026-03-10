# State Reflection

> Cognitive Dissonance Detection for AI Agents

## Overview

When an agent's theory conflicts with observed reality, this is "cognitive dissonance." State Reflection captures this gap and enforces that the theory is updated before code is modified.

**Key principle**: Bugs often indicate a flawed mental model, not just a typo. Fix the theory first, then fix the code.

## The Dissonance Threshold

| Score | Action Required |
|-------|-----------------|
| < 0.7 | Minor adjustments to theory may suffice |
| ≥ 0.7 | **Theory mutation required** before code fixes |

## CLI Usage

### Create a Reflection

```bash
./target/release/engram reflect create \
  --theory <THEORY_ID> \
  --observed "Test failed: expected User, got None" \
  --trigger-type test_failure \
  --agent "the-architect"
```

### Record Dissonance

```bash
./target/release/engram reflect record-dissonance \
  --id <REFLECTION_ID> \
  --description "Theory claims User is always created before Session, but code allows Session without User"
```

### Propose Theory Update

```bash
./target/release/engram reflect propose-update \
  --id <REFLECTION_ID> \
  --update "Add invariant: Session.user_id is optional; remove requirement that User must exist"
```

### Check if Mutation Required

```bash
./target/release/engram reflect requires-mutation --id <REFLECTION_ID>
# Exit code 0 = mutation required (dissonance >= 0.7)
# Exit code 1 = mutation not required
```

### Resolve Reflection

```bash
# After updating the theory
./target/release/engram reflect resolve \
  --id <REFLECTION_ID> \
  --new-theory-id <UPDATED_THEORY_ID>
```

## Session Integration

Sessions can be bound to theories, and will enter `Reflecting` state when dissonance is detected:

```bash
# Bind theory to session
./target/release/engram session bind-theory <SESSION_ID> --theory <THEORY_ID>

# Session automatically enters Reflecting state when reflection is triggered
./target/release/engram session trigger-reflection <SESSION_ID>
```

## Integration with Theory Building

State Reflection is the validation mechanism for [Theory Building](theory-building.md):

1. Agent builds a theory about a domain
2. Agent encounters test failure or unexpected behavior
3. Agent creates a StateReflection to analyze the dissonance
4. If dissonance ≥ 0.7, agent **must** update the theory
5. Only after theory is updated can code fixes be attempted

This enforces Naur's insight: the bug is often in the mental model, not the code.
