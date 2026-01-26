# HIPAA and HITECH Audit Prompts


  
  EVIDENCE-BASED VALIDATION REQUIREMENTS:
  - provide evidence-based validation for all final claims instead of unsubstantiated assertions
  - Every assertion must be supported by concrete evidence from code, tests, logs, or documentation
  - Include specific examples, file paths, line numbers, and verifiable measurements
  - Provide quantifiable metrics and test results when making claims about improvements
  - Reference official documentation or standards when applicable

  EVIDENCE COLLECTION INSTRUCTIONS:
  - Always provide evidence in this format:
    ## Claim: [Your specific assertion]
    ### Evidence:
    - **Code Reference**: `src/file.rs:123` - Function demonstrates...
    - **Test Results**: [Command output showing results]
    - **Execution Log**: [Timestamped log entries]
    - **Documentation**: [Citations from official docs]
  - Never make unsubstantiated claims like "the code is better", "this improves security", or "the refactoring is better", "this improves security", or "the refactoring is better"
  - Instead, provide quantifiable evidence: "Reduced processing time from 500ms to 120ms"



  
  EVIDENCE-BASED VALIDATION REQUIREMENTS:
  - provide evidence-based validation for all final claims instead of unsubstantiated assertions
  - Every assertion must be supported by concrete evidence from code, tests, logs, or documentation
  - Include specific examples, file paths, line numbers, and verifiable measurements
  - Provide quantifiable metrics and test results when making claims about improvements
  - Reference official documentation or standards when applicable


This collection provides comprehensive audit checkpoint prompts for HIPAA (Health Insurance Portability and Accountability Act) and HITECH (Health Information Technology for Economic and Clinical Health) Act compliance assessments.

## Context and Background

### HIPAA (Health Insurance Portability and Accountability Act)
**Effective Date**: 1996 (Privacy Rule 2003, Security Rule 2005, Breach Notification 2009)  
**Scope**: Covered entities (healthcare providers, health plans, healthcare clearinghouses) and business associates  
**Key Rules**: Privacy Rule, Security Rule, Breach Notification Rule, Enforcement Rule  
**Penalties**: Civil penalties up to $50,000 per violation, criminal penalties up to 10 years imprisonment  
**Protected Health Information (PHI)**: Individually identifiable health information

### HITECH Act (Health Information Technology for Economic and Clinical Health)
**Effective Date**: 2009 (part of ARRA - American Recovery and Reinvestment Act)  
**Scope**: Strengthens HIPAA enforcement, extends to business associates, promotes EHR adoption  
**Key Provisions**: Increased penalties, business associate liability, breach notification requirements, EHR meaningful use incentives  
**Penalties**: Civil penalties up to $1.5 million per year for repeated violations

## Regulatory Context
- **HIPAA**: Federal law protecting patient health information privacy and security
- **HITECH**: Enhances HIPAA with stronger enforcement and breach reporting
- **OCR Enforcement**: HHS Office for Civil Rights oversees compliance
- **Business Associates**: Vendors processing PHI must comply with HIPAA

## HIPAA Comprehensive Compliance Audit Prompt

Conduct comprehensive HIPAA compliance audit for healthcare organizations:

**Covered Entity Type:** [HEALTHCARE_PROVIDER_HEALTH_PLAN_CLEARINGHOUSE]
**Business Associates:** [VENDORS_PROCESSING_PHI_COUNT]
**Risk Assessment:** [LOW_MODERATE_HIGH_RISK_LEVEL]
**Breach History:** [NUMBER_OF_REPORTED_BREACHES]

**HIPAA Compliance Assessment:**

1. **Privacy Rule Compliance**
   - Evaluate Notice of Privacy Practices distribution
   - Assess patient authorization mechanisms
   - Review minimum necessary standard implementation
   - Validate accounting of disclosures procedures

2. **Security Rule Compliance**
   - Evaluate administrative safeguards (policies, procedures, risk analysis)
   - Assess physical safeguards (facility access, workstation security)
   - Review technical safeguards (access control, audit controls, encryption)
   - Validate security incident procedures

3. **Breach Notification Rule Compliance**
   - Assess breach identification procedures
   - Review notification timeframes (60 days for media, immediate for individuals)
   - Validate notification content requirements
   - Confirm HHS breach reporting mechanisms

4. **Business Associate Agreements**
   - Evaluate BAA template and execution
   - Assess subcontractor BAAs
   - Review BAA amendment procedures
   - Validate breach notification clauses

5. **Risk Analysis and Management**
   - Assess annual risk analysis completion
   - Review risk mitigation strategies
   - Validate risk analysis documentation
   - Confirm ongoing risk monitoring

**HIPAA Compliance Scoring:**
- **Privacy Rule**: [SCORE]/100
- **Security Rule**: [SCORE]/100
- **Breach Notification**: [SCORE]/100
- **Business Associates**: [SCORE]/100
- **Risk Management**: [SCORE]/100

## HITECH Enhanced Compliance Assessment

**HITECH-Specific Requirements:**
- **Increased Penalties**: Tiered penalty structure based on culpability
- **Business Associate Direct Liability**: BAs subject to direct OCR enforcement
- **State Attorney General Enforcement**: State AGs can enforce HIPAA standards
- **Breach Notification Thresholds**: Lower threshold for reporting (500+ individuals)

**HITECH Compliance Scoring:**
- **Penalty Tier Assessment**: [TIER_1_2_3_4]
- **Business Associate Compliance**: [SCORE]/100
- **State Law Alignment**: [SCORE]/100
- **EHR Security**: [SCORE]/100

## HIPAA + HITECH Integration Assessment

**Multi-Framework Compliance:**
- HIPAA + ISO 27001: [ALIGNMENT_SCORE]/100
- HIPAA + NIST CSF: [ALIGNMENT_SCORE]/100
- HIPAA + GDPR: [ALIGNMENT_SCORE]/100

**Healthcare Compliance Framework:**
1. **Privacy and Security Policies**
2. **Access Controls and Authentication**
3. **Audit Logging and Monitoring**
4. **Incident Response and Breach Notification**
5. **Business Associate Management**
6. **Risk Assessment and Mitigation**