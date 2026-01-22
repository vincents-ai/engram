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

### 1. Identify Relevant Frameworks
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

### 2. Run Compliance Checks
Use relevant audit prompts.

### 3. Store Results in Engram

```bash
# Create compliance entity
engram compliance create \
  --title "Compliance: [Feature] - [Framework]" \
  --category [security/privacy/quality] \
  --requirements "✅ [Requirement 1]\n✅ [Requirement 2]\n❌ [Requirement 3 - gap]" \
  --tags "compliance,[framework],[feature]"

# Store detailed findings
engram context create \
  --title "Compliance Audit: [Feature] - [Framework]" \
  --content "**Framework:** [Full framework name]\n**Findings:**\n- ✅ [Pass 1]\n- ❌ [Fail 1]\n- ⚠️ [Gap 1]\n\n**Remediation:**\n[What needs to be fixed]" \
  --source "compliance-audit"

# Create reasoning for evidence
engram reasoning create \
  --title "Compliance Evidence: [Feature]" \
  --task-id [TASK_ID] \
  --content "**Framework:** [Framework]\n**Check:** [What was verified]\n**Evidence:** [Test results, code snippets, docs]\n**Status:** PASS/FAIL/GAP" \
  --confidence [0.0-1.0]
```

### 4. Update Task Status
Link compliance to task:

```bash
engram task update [TASK_ID] --status pending_compliance
# After compliance passes:
engram task update [TASK_ID] --status compliant
```

## Example

```
Feature: "User authentication API"

[Step 1: Identify frameworks]
- SOC2 (security criteria)
- GDPR (data protection)
- OWASP (security testing)

[Step 2: Run checks]
Use prompts from:
- ~/code/prompts/compliance_and_certification/prompts/audit_checkpoints/saas_it/soc2.yaml
- ~/code/prompts/compliance_and_certification/prompts/audit_checkpoints/data_protection/gdpr.yaml
- ~/code/prompts/compliance_and_certification/prompts/audit_checkpoints/software_development/owasp.yaml

[Step 3: Store results]
engram compliance create \
  --title "Compliance: Auth API - SOC2" \
  --category security \
  --requirements "✅ Access controls\n✅ Network security\n✅ Data protection\n✅ Encryption at rest\n✅ Encryption in transit"

engram context create \
  --title "Compliance Audit: Auth API - SOC2" \
  --content "Findings:\n- ✅ All SOC2 security criteria met\n- ✅ Access controls implemented\n- ✅ Encryption verified\nStatus: COMPLIANT"

[Step 4: Link to task]
engram relationship create --source-id [AUTH_TASK] --target-id [COMPLIANCE_ID] --fulfills
```

## Integration with Engram

Compliance stored in engram:
- **Compliance**: Requirements and status
- **Context**: Detailed audit findings
- **Reasoning**: Evidence and confidence
- **Relationships**: Task → Compliance linkage

## Querying Compliance

```bash
# Get compliance status
engram compliance list | grep "[feature]"

# Get audit details
engram context list | grep "Compliance Audit"

# Get evidence
engram reasoning list | grep "Compliance Evidence"

# Check task compliance
engram relationship connected --entity-id [TASK] --relationship-type fulfills
```

## Common Compliance Frameworks

| Area | Framework | Location |
|------|-----------|----------|
| Security | SOC2 | saas_it/soc2.yaml |
| Security | ISO 27001 | saas_it/iso27001.yaml |
| Privacy | GDPR | data_protection/gdpr.yaml |
| Privacy | CCPA | data_protection/ccpa.yaml |
| Security | OWASP | software_development/owasp.yaml |
| Security | NIST CSF | cybersecurity_policies/nist-csf.yaml |
| EU Regulation | AI Act | eu_regulations/ai_act.yaml |
| EU Regulation | NIS2 | eu_regulations/nis2.yaml |
| EU Regulation | DORA | eu_regulations/dora.yaml |
| Gaming | GLI-19 | igaming/gli-19.yaml |
| Gaming | MGA | igaming/mga.yaml |

See full catalog: `ls ~/code/prompts/compliance_and_certification/prompts/audit_checkpoints/`
