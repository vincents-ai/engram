---
name: engram-check-compliance
description: "Check compliance against frameworks using prompts from ~/code/prompts/compliance_and_certification/ and store audit results in engram."
---

# Checking Compliance

## Overview

Validate work against compliance frameworks. Use audit checkpoint prompts and store results in engram.

## When to Use

Use this skill when:
- Implementing security-sensitive features
- Preparing for audits
- Following regulatory requirements
- Ensuring code quality

## The Pattern

### 1. Search First

Check for existing compliance work before starting:

```bash
engram ask query "<feature> compliance <framework>"
engram task show <UUID>
```

### 2. Identify Relevant Frameworks

Choose from available compliance areas:
```
~/code/prompts/compliance_and_certification/prompts/
├── audit_checkpoints/
│   ├── igaming/           # GLI, MGA, UKGC, G4
│   ├── saas_it/           # SOC2, ISO27001, PCI DSS
│   ├── data_protection/   # GDPR, CCPA, PIPEDA
│   ├── eu_regulations/    # DSA, DMA, AI Act, NIS2, DORA
│   ├── gaming_certification/ # RNG, RTP, Fairness
│   ├── software_development/ # OWASP, SDL, ISO 12207
│   ├── german_compliance/ # GoBD, DSGVO, BSI
│   ├── medical_device/    # IEC 62304
│   ├── cross_compliance/  # Multi-framework
│   └── cybersecurity_policies/ # NIST CSF, RMF, ISO 27002
```

### 3. Anchor Work

```bash
engram task create --title "Compliance: <feature> — <framework>"
# TASK_UUID = ...
engram task update <TASK_UUID> --status in_progress
```

### 4. Run Compliance Checks

Use relevant audit prompts from the directory above.

### 5. Store Results in Engram

```bash
# Create compliance entity
engram compliance create \
  --title "Compliance: <Feature> - <Framework>" \
  --category "<security|privacy|quality>" \
  --description "✅ <Requirement 1>\n✅ <Requirement 2>\n❌ <Requirement 3 - gap>" \
  --agent "<name>"
# COMPLIANCE_UUID = ...

# Store detailed findings as context
engram context create \
  --title "Compliance Audit: <Feature> - <Framework>" \
  --content "**Framework:** <Full framework name>\n**Findings:**\n- ✅ <Pass 1>\n- ❌ <Fail 1>\n- ⚠️ <Gap 1>\n\n**Remediation:**\n<What needs to be fixed>" \
  --source "compliance-audit"
# AUDIT_CTX_UUID = ...

# Create reasoning for evidence
engram reasoning create \
  --title "Compliance Evidence: <Feature>" \
  --task-id <TASK_UUID> \
  --content "**Framework:** <Framework>\n**Check:** <What was verified>\n**Evidence:** <Test results, code snippets, docs>\n**Status:** PASS/FAIL/GAP" \
  --confidence 0.9
# EVIDENCE_UUID = ...
```

### 6. Link Everything

```bash
engram relationship create \
  --source-id <TASK_UUID> --source-type task \
  --target-id <COMPLIANCE_UUID> --target-type compliance \
  --relationship-type relates_to --agent "<name>"

engram relationship create \
  --source-id <TASK_UUID> --source-type task \
  --target-id <AUDIT_CTX_UUID> --target-type context \
  --relationship-type relates_to --agent "<name>"

engram relationship create \
  --source-id <COMPLIANCE_UUID> --source-type compliance \
  --target-id <EVIDENCE_UUID> --target-type reasoning \
  --relationship-type relates_to --agent "<name>"
```

### 7. Update Task Status

Valid statuses are: `todo`, `in_progress`, `done`, `blocked`, `cancelled`

```bash
# After all compliance checks are complete:
engram task update <TASK_UUID> --status done --outcome "All <framework> requirements met"

# If blocked by gaps requiring remediation:
engram task update <TASK_UUID> --status blocked --reason "Gaps found: <list>"
```

## Example

```
Feature: "User authentication API"

[Search first]
engram ask query "authentication API SOC2 GDPR compliance"

[Anchor]
engram task create --title "Compliance: Auth API — SOC2 + GDPR"
# TASK_UUID = task-001
engram task update task-001 --status in_progress

[Step 1: Identify frameworks]
- SOC2 (security criteria)
- GDPR (data protection)
- OWASP (security testing)

[Step 2: Run checks]
Use prompts from:
- ~/code/prompts/compliance_and_certification/prompts/audit_checkpoints/saas_it/soc2_iso27001_pci_audit_prompts.md
- ~/code/prompts/compliance_and_certification/prompts/audit_checkpoints/data_protection/gdpr_ccpa_pipeda_audit_prompts.md
- ~/code/prompts/compliance_and_certification/prompts/audit_checkpoints/software_development/owasp_sdl_iso12207_prompts.md

[Step 3: Store results]
engram compliance create \
  --title "Compliance: Auth API - SOC2" \
  --category security \
  --description "✅ Access controls implemented\n✅ Network security configured\n✅ Data protection in place\n✅ Encryption at rest\n✅ Encryption in transit" \
  --agent "compliance-agent"
# COMPLIANCE_UUID = cmp-001

engram context create \
  --title "Compliance Audit: Auth API - SOC2" \
  --content "Framework: SOC2 Security Criteria\nFindings:\n- ✅ All SOC2 security criteria met\n- ✅ Access controls implemented\n- ✅ Encryption verified\nStatus: COMPLIANT" \
  --source "compliance-audit"
# AUDIT_CTX_UUID = ctx-002

engram reasoning create \
  --title "Compliance Evidence: Auth API SOC2" \
  --task-id task-001 \
  --content "Framework: SOC2\nCheck: Access controls and encryption\nEvidence: JWT tokens with RS256, TLS 1.3 enforced, bcrypt password hashing\nStatus: PASS" \
  --confidence 0.95
# EVIDENCE_UUID = rsn-003

[Step 4: Link compliance to task]
engram relationship create \
  --source-id task-001 --source-type task \
  --target-id cmp-001 --target-type compliance \
  --relationship-type relates_to --agent "compliance-agent"

engram relationship create \
  --source-id task-001 --source-type task \
  --target-id ctx-002 --target-type context \
  --relationship-type relates_to --agent "compliance-agent"

engram relationship create \
  --source-id cmp-001 --source-type compliance \
  --target-id rsn-003 --target-type reasoning \
  --relationship-type relates_to --agent "compliance-agent"

[Close]
engram validate check
engram task update task-001 --status done --outcome "SOC2, GDPR, OWASP compliance verified"
```

## Integration with Engram

Compliance stored in engram:
- **Compliance**: Requirements and status (`engram compliance create`)
- **Context**: Detailed audit findings (`engram context create`)
- **Reasoning**: Evidence and confidence (`engram reasoning create`)
- **Relationships**: Task → Compliance linkage

## Querying Compliance

```bash
# Get compliance status
engram compliance list

# Get audit details
engram context list

# Get all entities connected to a task
engram relationship connected --entity-id <TASK_UUID> --max-depth 2

# Search for compliance records
engram ask query "<feature> compliance audit"
```

**Note:** `engram compliance list` may return empty results even when records exist — this is a known CLI issue. Use `engram ask query` and `engram relationship connected` to retrieve compliance records reliably.

## Common Compliance Frameworks

| Area | Framework | Location |
|------|-----------|----------|
| Security | SOC2 | saas_it/soc2_iso27001_pci_audit_prompts.md |
| Security | ISO 27001 | saas_it/soc2_iso27001_pci_audit_prompts.md |
| Privacy | GDPR | data_protection/gdpr_ccpa_pipeda_audit_prompts.md |
| Privacy | CCPA | data_protection/gdpr_ccpa_pipeda_audit_prompts.md |
| Security | OWASP | software_development/owasp_sdl_iso12207_prompts.md |
| Security | NIST CSF | cybersecurity_policies/nist_csf_risk_management_prompts.md |
| EU Regulation | AI Act | eu_regulations/dsa_dma_ai_act_data_act_prompts.md |
| EU Regulation | NIS2 | eu_regulations/nis2_dora_cybersecurity_prompts.md |
| EU Regulation | DORA | eu_regulations/nis2_dora_cybersecurity_prompts.md |
| Gaming | GLI-19 | igaming/gli_audit_prompts.md |
| Gaming | MGA | igaming/mga_ukgc_g4_audit_prompts.md |

See full catalog: `ls ~/code/prompts/compliance_and_certification/prompts/audit_checkpoints/`

## Related Skills

- `engram-use-engram-memory` — store compliance findings and evidence
- `engram-audit-trail` — complete audit trail for compliance work
- `engram-test-driven-development` — tests provide compliance evidence
- `engram-requesting-code-review` — code review for security compliance
- `engram-plan-feature` — ensure compliance from planning stage
