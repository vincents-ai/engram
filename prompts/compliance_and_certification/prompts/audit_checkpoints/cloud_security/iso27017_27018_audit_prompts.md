# ISO 27017 and ISO 27018 Cloud Security Audit Prompts


  
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


This collection provides comprehensive audit checkpoint prompts for ISO/IEC 27017:2015 and ISO/IEC 27018:2019 cloud security implementation and assessment.

## Context and Background

### ISO/IEC 27017:2015 Information Technology Security Techniques - Code of Practice for Information Security Controls Based on ISO/IEC 27002 for Cloud Services
**Effective Date**: December 15, 2015  
**Scope**: Cloud service providers and cloud service customers  
**Key Focus**: Additional security controls specific to cloud environments  
**Structure**: 37 controls covering cloud-specific security aspects  
**Benefits**: Enhanced cloud security, standardized cloud controls, improved cloud service assurance

### ISO/IEC 27018:2019 Information Technology Security Techniques - Code of Practice for Protection of Personally Identifiable Information (PII) in Public Clouds Acting as PII Processors
**Effective Date**: September 12, 2019  
**Scope**: Public cloud service providers processing PII  
**Key Focus**: Protection of personally identifiable information in cloud environments  
**Structure**: 82 controls covering PII protection in cloud services  
**Benefits**: Enhanced PII protection, regulatory compliance, customer trust in cloud services

## Regulatory Context
- **ISO 27017**: Code of practice for cloud security controls
- **ISO 27018**: Specific guidance for PII protection in public clouds
- **Cloud Service Models**: Applicable to IaaS, PaaS, SaaS
- **Integration**: Complements ISO 27001 for cloud environments

## ISO 27017 Comprehensive Cloud Security Audit Prompt

Conduct comprehensive ISO 27017 cloud security audit:

**Cloud Service Model:** [IaaS_PaaS_SaaS]
**Deployment Model:** [PUBLIC_PRIVATE_HYBRID_COMMUNITY]
**Cloud Service Provider:** [PROVIDER_NAME_CERTIFICATION_STATUS]
**Customer Responsibility:** [SHARED_RESPONSIBILITY_MODEL]

**ISO 27017 Compliance Assessment:**

1. **Cloud Service Customer Controls**
   - Evaluate asset management in cloud
   - Assess information classification procedures
   - Review access control policies
   - Validate secure use of cloud services

2. **Cloud Service Provider Controls**
   - Assess virtual machine configuration
   - Review clock synchronization
   - Validate segregation in virtual environments
   - Confirm audit logging for cloud management

3. **Shared Controls**
   - Evaluate shared responsibility documentation
   - Assess incident management procedures
   - Review service level agreements
   - Validate monitoring and logging

4. **Risk Management in Cloud**
   - Assess cloud-specific risk assessment
   - Review supplier relationship management
   - Validate legal and regulatory compliance
   - Confirm business continuity planning

5. **Data Security in Cloud**
   - Evaluate data encryption mechanisms
   - Assess data disposal procedures
   - Review data location controls
   - Validate data portability requirements

6. **Identity and Access Management**
   - Assess identity management in cloud
   - Review authentication mechanisms
   - Validate authorization controls
   - Confirm access review procedures

7. **Virtualization Security**
   - Evaluate hypervisor security
   - Assess virtual network security
   - Review virtual machine isolation
   - Validate resource management

**ISO 27017 Compliance Scoring:**
- **Customer Controls**: [SCORE]/100
- **Provider Controls**: [SCORE]/100
- **Shared Controls**: [SCORE]/100
- **Risk Management**: [SCORE]/100
- **Data Security**: [SCORE]/100
- **Identity Management**: [SCORE]/100
- **Virtualization Security**: [SCORE]/100

## ISO 27018 PII Protection Audit Prompt

Conduct comprehensive ISO 27018 PII protection audit:

**PII Processing Scope:** [VOLUME_TYPES_SENSITIVITY]
**Data Residency Requirements:** [COUNTRIES_REGIONS_COMPLIANCE]
**Data Subject Rights:** [ACCESS_RECTIFICATION_ERASURE_PORTABILITY]
**Breach Notification:** [TIMELINES_PROCEDURES_COMPLIANCE]

**ISO 27018 Compliance Assessment:**

1. **PII Processing Consent**
   - Evaluate consent management procedures
   - Assess consent withdrawal mechanisms
   - Review consent record keeping
   - Validate consent verification processes

2. **PII Data Security**
   - Assess encryption of PII in transit
   - Review encryption of PII at rest
   - Validate PII data minimization
   - Confirm secure data disposal

3. **PII Data Subject Rights**
   - Evaluate access request procedures
   - Assess rectification mechanisms
   - Review erasure processes
   - Validate portability requirements

4. **PII Breach Management**
   - Assess breach detection procedures
   - Review breach notification timelines
   - Validate breach investigation processes
   - Confirm breach remediation

5. **PII Data Transfers**
   - Evaluate cross-border transfer controls
   - Assess adequacy determinations
   - Review transfer agreements
   - Validate transfer documentation

6. **PII Sub-processor Management**
   - Assess sub-processor selection criteria
   - Review sub-processor agreements
   - Validate sub-processor oversight
   - Confirm sub-processor compliance

7. **PII Audit and Accountability**
   - Evaluate audit logging procedures
   - Assess audit review processes
   - Review accountability mechanisms
   - Validate compliance monitoring

**ISO 27018 Compliance Scoring:**
- **Consent Management**: [SCORE]/100
- **Data Security**: [SCORE]/100
- **Data Subject Rights**: [SCORE]/100
- **Breach Management**: [SCORE]/100
- **Data Transfers**: [SCORE]/100
- **Sub-processor Management**: [SCORE]/100
- **Audit & Accountability**: [SCORE]/100

## Cloud Security Integration Assessment

**Multi-Framework Compliance:**
- ISO 27017 + ISO 27001: [ALIGNMENT_SCORE]/100
- ISO 27018 + GDPR: [ALIGNMENT_SCORE]/100
- ISO 27017 + CSA STAR: [ALIGNMENT_SCORE]/100

**Cloud Security Framework:**
1. **Cloud Service Provider Assessment**
2. **Shared Responsibility Model Implementation**
3. **PII Protection in Cloud Environments**
4. **Cloud-Specific Risk Management**
5. **Continuous Monitoring and Compliance**