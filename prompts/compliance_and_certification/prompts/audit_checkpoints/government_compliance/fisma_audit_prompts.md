# FISMA (Federal Information Security Management Act) Audit Prompts


  
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


This collection provides comprehensive audit checkpoint prompts for FISMA compliance assessment in federal agencies and contractors.

## Context and Background

### FISMA (Federal Information Security Management Act)
**Effective Date**: 2002 (amended by various acts including FISMA 2014)  
**Scope**: All federal agencies and contractors handling federal information  
**Key Requirements**: Security controls, risk assessments, continuous monitoring, incident reporting  
**Penalties**: Funding reductions, management accountability, potential criminal penalties  
**Framework**: NIST SP 800-53 security controls, annual FISMA reporting

## Regulatory Context
- **FISMA**: Federal law requiring information security programs in federal agencies
- **OMB Oversight**: Office of Management and Budget provides guidance and oversight
- **NIST Standards**: SP 800-53 provides security control requirements
- **Annual Reporting**: Agencies submit FISMA reports to OMB and Congress

## FISMA Comprehensive Security Audit Prompt

Conduct comprehensive FISMA compliance audit for federal information systems:

**Agency Type:** [EXECUTIVE_AGENCY_INDEPENDENT_AGENCY]
**System Categorization:** [LOW_MODERATE_HIGH_IMPACT]
**ATO Status:** [AUTHORIZATION_TO_OPERATE_LEVEL]
**POA&M Items:** [NUMBER_OF_OPEN_PLAN_OF_ACTION_ITEMS]

**FISMA Compliance Assessment:**

1. **Security Control Implementation**
   - Evaluate NIST SP 800-53 control implementation
   - Assess control inheritance from higher systems
   - Review compensating control documentation
   - Validate control testing and evaluation

2. **Risk Assessment and Management**
   - Assess annual risk assessment completion
   - Review risk mitigation strategies
   - Validate risk acceptance documentation
   - Confirm ongoing risk monitoring

3. **System Security Plan (SSP)**
   - Evaluate SSP completeness and accuracy
   - Assess system boundary definition
   - Review control implementation descriptions
   - Validate SSP maintenance procedures

4. **Security Assessment and Authorization**
   - Assess security assessment report quality
   - Review authorization package completeness
   - Validate authorization decision documentation
   - Confirm authorization renewal procedures

5. **Continuous Monitoring**
   - Evaluate monitoring strategy implementation
   - Assess security control effectiveness
   - Review vulnerability scanning frequency
   - Validate incident detection capabilities

6. **Incident Response and Reporting**
   - Assess incident response plan effectiveness
   - Review incident reporting procedures
   - Validate breach notification compliance
   - Confirm lessons learned implementation

7. **Configuration Management**
   - Evaluate configuration management plan
   - Assess change control procedures
   - Review configuration baseline maintenance
   - Validate security impact assessments

8. **Plan of Action and Milestones (POA&M)**
   - Assess POA&M completeness and accuracy
   - Review milestone achievement tracking
   - Validate risk mitigation progress
   - Confirm POA&M update procedures

**FISMA Compliance Scoring:**
- **Security Controls**: [SCORE]/100
- **Risk Management**: [SCORE]/100
- **System Security Plan**: [SCORE]/100
- **Assessment & Authorization**: [SCORE]/100
- **Continuous Monitoring**: [SCORE]/100
- **Incident Response**: [SCORE]/100
- **Configuration Management**: [SCORE]/100
- **POA&M Management**: [SCORE]/100

## FISMA Integration with Other Standards

**Multi-Framework Compliance:**
- FISMA + FedRAMP: [ALIGNMENT_SCORE]/100
- FISMA + NIST CSF: [ALIGNMENT_SCORE]/100
- FISMA + ISO 27001: [ALIGNMENT_SCORE]/100

**Federal Security Framework:**
1. **Information System Categorization**
2. **Security Control Selection and Implementation**
3. **Security Control Assessment**
4. **Authorization and Continuous Monitoring**
5. **Incident Response and System Maintenance**