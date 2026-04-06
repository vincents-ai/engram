---
name: engram-rules-standards
description: "Define and enforce automated rules and human-readable standards in engram. Use when establishing coding policies, process compliance checks, or automated data validation across entities."
---

# Rules and Standards

## Overview

Engram provides two complementary compliance primitives:

- **Rules** — machine-executable enforcement policies that run against entities automatically
- **Standards** — human-readable policy documents with structured requirements that agents and humans follow

Rules check and enforce. Standards define what is expected. Use them together: a standard declares the requirement; a rule enforces it.

## When to Use

Use this skill when:
- Defining coding, security, or process policies that should be enforceable
- Creating automated checks that run against tasks, contexts, or other entities
- Documenting standards that teams or agents must follow
- Linking enforcement rules to the standards they implement
- Auditing entity compliance with `rule execute`

## Command Reference

### Rules

```bash
# Create a rule
# --title is required
# --rule-type: validation | transformation | enforcement | notification (default: validation)
# --priority: low | medium | high | critical (default: medium)
# --entity-types: comma-separated list of entity types this rule applies to
# --condition: JSON object (default: {})
# --action: JSON object (default: {})
engram rule create --title "<title>" \
  --description "<what this rule checks or enforces>" \
  --rule-type <validation|transformation|enforcement|notification> \
  --priority <low|medium|high|critical> \
  --entity-types <task,context,reasoning> \
  --condition '{"field": "title", "operator": "contains", "value": "UUID"}' \
  --action '{"message": "Task title must reference a UUID"}' \
  --agent "<your-name>"

# List all rules
engram rule list

# Get a specific rule
engram rule get <RULE_UUID>

# Update a rule
engram rule update <RULE_UUID> --title "<new title>" --priority high

# Delete a rule
engram rule delete <RULE_UUID>

# Execute a rule against a specific entity — --entity-id and --entity-type are REQUIRED
engram rule execute <RULE_UUID> --entity-id <ENTITY_UUID> --entity-type <task|context|reasoning|adr>
```

### Standards

```bash
# Create a standard
# --title is required
# --category: coding | testing | documentation | security | performance | process | architecture (default: process)
# --version: default 1.0
# --effective-date: ISO 8601 format
engram standard create --title "<title>" \
  --description "<what this standard covers>" \
  --category <coding|testing|documentation|security|performance|process|architecture> \
  --version "1.0" \
  --effective-date "2026-01-01T00:00:00Z" \
  --agent "<your-name>"

# List all standards
engram standard list

# Get a specific standard
engram standard get <STANDARD_UUID>

# Update a standard
engram standard update <STANDARD_UUID> --version "1.1"

# Delete a standard
engram standard delete <STANDARD_UUID>

# Add a requirement to a standard — --title and --description are REQUIRED
# --mandatory: flag (presence means required, absence means optional)
# --priority: low | medium | high | critical
# --evidence-required: flag (presence means evidence must be attached)
engram standard add-requirement <STANDARD_UUID> \
  --title "<requirement short title>" \
  --description "<what must be true to satisfy this requirement>" \
  --mandatory \
  --priority high \
  --evidence-required
```

## Rule Types Explained

| Type | Purpose |
|---|---|
| `validation` | Checks whether an entity satisfies a condition. Use for data quality gates. |
| `enforcement` | Blocks or rejects an action if the condition fails. Use for hard policy gates. |
| `transformation` | Modifies entity data when a condition matches. Use for normalization. |
| `notification` | Emits an alert when a condition matches. Use for monitoring and observability. |

## Standard Categories Explained

| Category | Use for |
|---|---|
| `coding` | Style guides, naming conventions, complexity limits |
| `testing` | Coverage requirements, test type mandates |
| `documentation` | Docstring rules, README requirements |
| `security` | Auth patterns, secret handling, audit logging |
| `performance` | Latency budgets, memory limits |
| `process` | Commit message format, PR review requirements |
| `architecture` | ADR requirements, dependency rules |

## Rule vs Standard

| | Rule | Standard |
|---|---|---|
| Machine-executable | Yes | No |
| Human-readable | Partially | Yes |
| Has requirements | No | Yes |
| Can be run against entities | Yes (`rule execute`) | No |
| Defines policy intent | Partially | Yes |

**Pattern:** Define the standard first (intent), then create a rule that enforces it (mechanism). Link both to the task that required the policy.

## The Pattern

### 1. Create a Standard

```bash
engram standard create \
  --title "Commit Message Standard" \
  --description "All git commits must reference a valid engram task UUID to ensure traceability." \
  --category process \
  --version "1.0" \
  --effective-date "2026-01-01T00:00:00Z" \
  --agent "orchestrator"
# Returns: STANDARD_UUID
```

### 2. Add Requirements to the Standard

```bash
engram standard add-requirement <STANDARD_UUID> \
  --title "Include task UUID" \
  --description "Commit message body or footer must contain a valid engram task UUID in the format xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx" \
  --mandatory \
  --priority high \
  --evidence-required

engram standard add-requirement <STANDARD_UUID> \
  --title "Subject line under 72 characters" \
  --description "The first line of the commit message must not exceed 72 characters." \
  --mandatory \
  --priority medium
```

### 3. Create an Enforcement Rule

```bash
engram rule create \
  --title "Enforce task UUID in commit messages" \
  --description "Validates that a commit context entity references a task UUID" \
  --rule-type validation \
  --priority high \
  --entity-types context \
  --condition '{"field": "content", "operator": "matches", "value": "[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}"}' \
  --action '{"on_fail": "reject", "message": "Commit must reference a valid engram task UUID"}' \
  --agent "orchestrator"
# Returns: RULE_UUID
```

### 4. Execute the Rule Against an Entity

```bash
# --entity-id and --entity-type are REQUIRED
engram rule execute <RULE_UUID> \
  --entity-id <CONTEXT_UUID> \
  --entity-type context
```

### 5. Link Rule and Standard to the Task

```bash
engram relationship create \
  --source-id <TASK_UUID> --source-type task \
  --target-id <STANDARD_UUID> --target-type standard \
  --relationship-type relates_to --agent "orchestrator"

engram relationship create \
  --source-id <STANDARD_UUID> --source-type standard \
  --target-id <RULE_UUID> --target-type rule \
  --relationship-type relates_to --agent "orchestrator"
```

## Example: Security Standard with Validation Rule

```bash
# Standard
engram standard create \
  --title "Secret Handling Standard" \
  --category security \
  --description "No secrets, tokens, or credentials may appear in context or task content." \
  --version "1.0" \
  --agent "security-agent"
# STANDARD_UUID = sec-std-001

engram standard add-requirement sec-std-001 \
  --title "No plaintext secrets in context" \
  --description "Context entities must not contain API keys, passwords, or tokens in plaintext." \
  --mandatory \
  --priority critical

# Rule
engram rule create \
  --title "Detect plaintext secrets in context" \
  --rule-type enforcement \
  --priority critical \
  --entity-types context \
  --condition '{"field": "content", "operator": "not_matches", "value": "(sk-|ghp_|AKIA|-----BEGIN)"}' \
  --action '{"on_fail": "reject", "message": "Context contains potential secret material"}' \
  --agent "security-agent"
# RULE_UUID = sec-rule-001

# Execute against a context entity
engram rule execute sec-rule-001 --entity-id <CTX_UUID> --entity-type context
```

## Key Rules

1. `rule execute` always requires both `--entity-id` and `--entity-type` — neither is optional
2. `standard add-requirement` always requires both `--title` and `--description`
3. Rules run against entities — standards define what entities should satisfy
4. Link standards to rules with `engram relationship create` so future agents can traverse the policy graph
5. Use `--mandatory` on requirements that block compliance; omit it for advisory requirements

## Related Skills

- `engram-use-engram-memory` — storing context and linking records
- `engram-audit-trail` — full traceability of compliance decisions
- `engram-check-compliance` — audit against external frameworks
- `engram-security-architecture` — designing security controls
