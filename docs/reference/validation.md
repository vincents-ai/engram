# Validation Reference

Engram validation protects task traceability and repository quality gates.

## Commit Message Validation

Commits must reference a valid engram task UUID:

```text
<type>: <title> [<ENGRAM_TASK_UUID>]
```

Example:

```text
fix: resolve sync conflict handling [171af123-9b96-48b1-b071-deba76492dac]
```

## CLI Usage

```bash
# Install git hooks
engram validate hook install

# Run validation checks
engram validate check

# Inspect validation configuration and results
engram validation list
engram validation run
```

## Quality Gates

Validation can enforce:

- Commit/task linkage
- Required reasoning relationships
- Repository consistency
- Configured rule and quality-gate checks

See also [Validation](../validation.md).
