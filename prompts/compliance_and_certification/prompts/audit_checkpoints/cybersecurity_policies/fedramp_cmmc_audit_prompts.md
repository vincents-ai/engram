# FedRAMP and CMMC Audit Prompts

This collection provides comprehensive audit checkpoint prompts for FedRAMP (Federal Risk and Authorization Management Program) and CMMC (Cybersecurity Maturity Model Certification) assessments.

## Context and Background

### FedRAMP (Federal Risk and Authorization Management Program)
**Effective Date**: 2011 (program inception), continuously updated  
**Scope**: Cloud service providers (CSPs) offering services to U.S. federal agencies  
**Key Requirements**: NIST SP 800-53 security controls, continuous monitoring, third-party assessment  
**Authorization Types**: Low, Moderate, High impact levels  
**Benefits**: Standardized security assessment, reduced duplication, improved security posture

### CMMC (Cybersecurity Maturity Model Certification)
**Effective Date**: 2020 (program launch), Level 2/3 mandatory for DoD contracts  
**Scope**: Defense Industrial Base (DIB) contractors and subcontractors  
**Key Requirements**: 17 capability domains, 5 maturity levels, annual self-assessments  
**Assessment**: CMMC Registered Practitioner Organizations (RPOs) or Certified Third-Party Assessment Organizations (C3PAOs)  
**Benefits**: Standardized cybersecurity requirements, improved supply chain security, better protection of sensitive information

## Regulatory Context
- **FedRAMP**: U.S. government program for secure cloud adoption
- **CMMC**: DoD requirement for DIB cybersecurity maturity
- **Integration**: CMMC incorporates many FedRAMP controls
- **Compliance**: Mandatory for federal cloud services and DoD contracts

## FedRAMP Authorization Audit Prompts

### FedRAMP Comprehensive Security Authorization Audit Prompt

Conduct comprehensive FedRAMP security authorization audit for cloud service providers:

**Authorization Type:** [LOW_MODERATE_HIGH_IMPACT]
**Service Model:** [IaaS_PaaS_SaaS]
**Deployment Model:** [PUBLIC_PRIVATE_HYBRID_COMMUNITY]
**Assessment Approach:** [FEDRAMP_AUTHORIZED_3PAO]

**FedRAMP Compliance Assessment:**

1. **Access Control (AC)**
   - Evaluate account management
   - Assess access enforcement
   - Review information flow control
   - Validate separation of duties

2. **Awareness and Training (AT)**
   - Assess security awareness training
   - Review training records
   - Validate training frequency
   - Confirm training effectiveness

3. **Audit and Accountability (AU)**
   - Evaluate audit logging
   - Assess audit review and analysis
   - Review audit retention
   - Validate audit reduction and reporting

4. **Security Assessment and Authorization (CA)**
   - Assess security assessment plans
   - Review security control assessments
   - Validate system interconnection
   - Confirm plan of action and milestones

5. **Configuration Management (CM)**
   - Evaluate configuration management plans
   - Assess baseline configurations
   - Review configuration changes
   - Validate security impact analysis

6. **Contingency Planning (CP)**
   - Assess contingency planning policies
   - Review contingency plan development
   - Validate contingency testing
   - Confirm plan maintenance

7. **Identification and Authentication (IA)**
   - Evaluate identification and authentication policies
   - Assess device identification and authentication
   - Review identifier management
   - Validate authenticator management

8. **Incident Response (IR)**
   - Assess incident response policies
   - Review incident response training
   - Validate incident handling
   - Confirm incident monitoring

9. **Maintenance (MA)**
   - Evaluate maintenance policies
   - Assess maintenance personnel
   - Review maintenance tools
   - Validate maintenance records

10. **Media Protection (MP)**
    - Assess media protection policies
    - Review media access control
    - Validate media sanitization
    - Confirm media disposal

11. **Physical and Environmental Protection (PE)**
    - Evaluate physical access control
    - Assess physical access monitoring
    - Review visitor control
    - Validate environmental controls

12. **Planning (PL)**
    - Assess security planning policies
    - Review rules of behavior
    - Validate privacy impact assessment
    - Confirm security planning updates

13. **Personnel Security (PS)**
    - Evaluate personnel security policies
    - Assess position risk designation
    - Review personnel screening
    - Validate personnel termination

14. **Risk Assessment (RA)**
    - Assess risk assessment policies
    - Review vulnerability scanning
    - Validate risk assessment updates
    - Confirm technical vulnerability management

15. **System and Services Acquisition (SA)**
    - Evaluate acquisition policies
    - Assess security requirements
    - Review supplier agreements
    - Validate developer security testing

16. **System and Communications Protection (SC)**
    - Assess system communications protection
    - Review boundary protection
    - Validate transmission integrity
    - Confirm cryptographic protection

17. **System and Information Integrity (SI)**
    - Evaluate system and information integrity policies
    - Assess flaw remediation
    - Review malicious code protection
    - Validate information system monitoring

**FedRAMP Compliance Scoring:**
- **Access Control**: [SCORE]/100
- **Awareness & Training**: [SCORE]/100
- **Audit & Accountability**: [SCORE]/100
- **Security Assessment**: [SCORE]/100
- **Configuration Management**: [SCORE]/100
- **Contingency Planning**: [SCORE]/100
- **Identification & Auth**: [SCORE]/100
- **Incident Response**: [SCORE]/100
- **Maintenance**: [SCORE]/100
- **Media Protection**: [SCORE]/100
- **Physical Protection**: [SCORE]/100
- **Planning**: [SCORE]/100
- **Personnel Security**: [SCORE]/100
- **Risk Assessment**: [SCORE]/100
- **System Acquisition**: [SCORE]/100
- **Communications Protection**: [SCORE]/100
- **System Integrity**: [SCORE]/100

## CMMC (Cybersecurity Maturity Model Certification) Audit Prompts

### CMMC Comprehensive Maturity Assessment Audit Prompt

Conduct comprehensive CMMC assessment for Defense Industrial Base contractors:

**CMMC Level:** [LEVEL_1_2_3]
**Domain Focus:** [PROTECTION_RECOVERY_DETECTION_RESPONSE]
**Assessment Method:** [SELF_ASSESSMENT_CMMC_REGISTERED_PRACTITIONER_ORG]

**CMMC Compliance Assessment:**

1. **Access Control (AC) - Level 1**
   - Evaluate account management
   - Assess access control policies
   - Review remote access restrictions
   - Validate access removal procedures

2. **Identification and Authentication (IA) - Level 1**
   - Assess identifier management
   - Review authenticator management
   - Validate device identification
   - Confirm authentication mechanisms

3. **Media Protection (MP) - Level 1**
   - Evaluate media protection policies
   - Assess media sanitization
   - Review physical media protection
   - Validate electronic media protection

4. **Physical Protection (PE) - Level 1**
   - Assess physical access control
   - Review monitoring physical access
   - Validate visitor access control
   - Confirm access control for transmission

5. **Recovery (RE) - Level 2**
   - Evaluate recovery planning
   - Assess backup procedures
   - Review recovery testing
   - Validate system recovery

6. **Risk Assessment (RM) - Level 2**
   - Assess risk management strategy
   - Review risk assessment procedures
   - Validate security categorization
   - Confirm vulnerability management

7. **System and Information Integrity (SI) - Level 2**
   - Evaluate flaw remediation
   - Assess malicious code protection
   - Review security alerts and advisories
   - Validate error handling

8. **Security Assessment (SA) - Level 2**
   - Assess security control assessments
   - Review system interconnections
   - Validate security certification
   - Confirm system security plan

9. **Situational Awareness (SA) - Level 2**
   - Evaluate security awareness training
   - Assess training records
   - Review contact with security groups
   - Validate training effectiveness

10. **Audit and Accountability (AU) - Level 3**
    - Assess audit policies and procedures
    - Review audit event logging
    - Validate audit review and analysis
    - Confirm audit retention

11. **Configuration Management (CM) - Level 3**
    - Evaluate configuration management plans
    - Assess baseline configurations
    - Review configuration change control
    - Validate security impact analysis

12. **Incident Response (IR) - Level 3**
    - Assess incident response training
    - Review incident handling
    - Validate incident monitoring
    - Confirm incident response testing

13. **Asset Management (AM) - Level 3**
    - Evaluate asset management policies
    - Assess hardware asset management
    - Review software asset management
    - Validate data asset management

**CMMC Compliance Scoring:**
- **Level 1 Controls**: [SCORE]/100
- **Level 2 Controls**: [SCORE]/100
- **Level 3 Controls**: [SCORE]/100
- **Overall Maturity**: [SCORE]/100

## FedRAMP + CMMC Integration Assessment

**Government Cloud Security Framework:**
- FedRAMP + CMMC Alignment: [INTEGRATION_SCORE]/100
- NIST 800-53 + CMMC Mapping: [MAPPING_SCORE]/100

**Federal Contractor Security Requirements:**
1. **Authorization Boundaries**
2. **Security Control Implementation**
3. **Assessment and Authorization**
4. **Continuous Monitoring**
5. **Incident Response and Reporting**