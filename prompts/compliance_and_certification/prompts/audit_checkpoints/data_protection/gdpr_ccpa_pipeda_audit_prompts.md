# Data Protection Compliance Audit Checkpoint Prompts


  
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


## GDPR (General Data Protection Regulation) Audit Prompts

### GDPR Comprehensive Compliance Audit Prompt
```
Conduct comprehensive GDPR compliance audit for data processing operations:

**Data Controller/Processor:** [ORGANIZATION_ROLE_AND_SCOPE]
**Geographic Scope:** [EU_DATA_SUBJECTS_AND_PROCESSING_LOCATIONS]
**Processing Activities:** [PERSONAL_DATA_CATEGORIES_AND_PURPOSES]
**Legal Basis:** [ARTICLE_6_LAWFUL_BASIS_FOR_PROCESSING]

**Article 5 - Principles of Processing Assessment:**

1. **Lawfulness, Fairness, and Transparency**
   ```
   Verify:
   - Valid legal basis for each processing activity
   - Clear and understandable privacy notices
   - Transparent communication about data use
   - Fair processing without deception
   
   Evidence Requirements:
   - Privacy notice content and accessibility
   - Legal basis documentation and justification
   - Data subject communication records
   - Processing purpose alignment validation
   ```

2. **Purpose Limitation**
   ```
   Assess:
   - Specific, explicit, and legitimate purposes
   - Processing limitation to stated purposes
   - Compatible use determination procedures
   - Purpose creep prevention controls
   
   Testing Activities:
   - Purpose statement review and validation
   - Secondary use compatibility assessment
   - Processing activity boundary verification
   - Change management for new purposes
   ```

3. **Data Minimisation**
   ```
   Evaluate:
   - Adequate, relevant, and limited data collection
   - Regular review of data necessity
   - Data retention period justification
   - Excessive data collection prevention
   
   Audit Procedures:
   - Data inventory completeness verification
   - Necessity assessment documentation
   - Retention schedule implementation
   - Regular data review processes
   ```

4. **Accuracy**
   ```
   Test:
   - Data accuracy maintenance procedures
   - Inaccurate data rectification processes
   - Data quality monitoring systems
   - Subject access and correction mechanisms
   
   Control Validation:
   - Data quality metrics and monitoring
   - Correction request handling procedures
   - Accuracy verification processes
   - Source data validation controls
   ```

5. **Storage Limitation**
   ```
   Examine:
   - Data retention policy implementation
   - Automated deletion procedures
   - Long-term storage justification
   - Archival and pseudonymization practices
   
   Testing Approach:
   - Retention schedule compliance verification
   - Automated deletion system testing
   - Archival process documentation
   - Data lifecycle management validation
   ```

6. **Integrity and Confidentiality**
   ```
   Validate:
   - Appropriate security measures implementation
   - Unauthorized access prevention
   - Accidental loss protection
   - Technical and organizational measures
   
   Security Assessment:
   - Encryption implementation verification
   - Access control effectiveness testing
   - Data loss prevention system validation
   - Incident response procedure testing
   ```

**Chapter II - Rights of Data Subjects Audit:**

**Article 12-14 - Information and Access Rights**
```
Assess:
- Privacy notice completeness and clarity
- Information provision at collection
- Data subject rights communication
- Accessible format provision

Testing Framework:
- Privacy notice content analysis
- Collection point information verification
- Rights communication effectiveness
- Accessibility compliance validation
```

**Article 15 - Right of Access**
```
Verify:
- Subject access request (SAR) procedures
- Response timeframe compliance (1 month)
- Information provision completeness
- Copy provision and format options

SAR Testing:
- Request handling procedure validation
- Response accuracy and completeness
- Timeframe compliance measurement
- Complex request management
```

**Article 16-18 - Rights to Rectification, Erasure, and Restriction**
```
Test:
- Data correction procedures
- Deletion request handling
- Processing restriction implementation
- Third-party notification processes

Control Testing:
- Rectification accuracy and timeliness
- Erasure completeness verification
- Restriction mechanism effectiveness
- Downstream notification validation
```

**Article 20 - Right to Data Portability**
```
Evaluate:
- Data export functionality
- Structured format provision
- Direct transmission capabilities
- Portability scope determination

Portability Assessment:
- Export format compliance (JSON, CSV, XML)
- Data completeness verification
- Direct transmission testing
- Automated portability tools
```

**Article 21 - Right to Object**
```
Examine:
- Objection handling procedures
- Processing cessation mechanisms
- Legitimate interest balancing
- Marketing opt-out processes

Objection Testing:
- Request processing effectiveness
- Balancing test documentation
- Processing cessation verification
- Marketing communication controls
```

**Chapter IV - Controller and Processor Responsibilities:**

**Article 25 - Data Protection by Design and by Default**
```
Assess:
- Privacy by design implementation
- Default privacy settings
- Technical measure integration
- Organizational measure embedding

Design Assessment:
- System architecture privacy integration
- Default setting privacy optimization
- Technical control implementation
- Process design privacy consideration
```

**Article 28 - Processor Obligations**
```
Verify:
- Processing contract compliance
- Processor instruction adherence
- Sub-processor management
- Data protection impact assessments

Contract Validation:
- Article 28 requirement inclusion
- Instruction documentation and tracking
- Sub-processor approval processes
- Security measure implementation
```

**Article 30 - Records of Processing Activities**
```
Examine:
- Processing record completeness
- Regular update procedures
- Record availability for authorities
- Cross-border transfer documentation

Record Keeping Assessment:
- Article 30 requirement compliance
- Update frequency and accuracy
- Authority access procedures
- Transfer mechanism documentation
```

**Article 32 - Security of Processing**
```
Test:
- Technical security measures
- Organizational security procedures
- Regular security testing
- Staff training and awareness

Security Control Testing:
- Encryption implementation
- Access control effectiveness
- Vulnerability management
- Security awareness programs
```

**Chapter V - International Transfers:**

**Article 44-49 - Transfer Mechanisms**
```
Assess:
- Adequacy decision reliance
- Standard contractual clauses (SCCs)
- Binding corporate rules (BCRs)
- Derogation justification

Transfer Assessment:
- Transfer inventory and mapping
- Legal mechanism validation
- Recipient country evaluation
- Safeguard implementation verification
```

**GDPR Compliance Scoring:**
- **Critical (0-59%):** Immediate regulatory action risk
- **Moderate (60-79%):** Compliance gaps requiring attention
- **Good (80-89%):** Minor improvements needed
- **Excellent (90-100%):** Strong compliance posture

**Regulatory Risk Assessment:**
- Administrative fine exposure (up to €20M or 4% turnover)
- Supervisory authority investigation likelihood
- Data subject complaint potential
- Reputational and business impact
```

### GDPR Data Protection Impact Assessment (DPIA) Prompt
```
Conduct Article 35 Data Protection Impact Assessment:

**Processing Operation:** [HIGH_RISK_PROCESSING_DESCRIPTION]
**Threshold Criteria:** [SYSTEMATIC_MONITORING_PROFILING_SPECIAL_CATEGORIES]
**DPIA Necessity:** [ARTICLE_35_CRITERIA_MET]

**DPIA Framework Assessment:**

1. **Processing Description and Purpose**
   ```
   Document:
   - Nature, scope, context, and purposes of processing
   - Personal data categories and data subject types
   - Processing operations and data flows
   - Data retention periods and storage locations
   
   Analysis Requirements:
   - Processing operation mapping
   - Data subject impact identification
   - Technology and methodology description
   - Processing lifecycle documentation
   ```

2. **Necessity and Proportionality Assessment**
   ```
   Evaluate:
   - Processing necessity for specified purposes
   - Proportionality of processing to objectives
   - Alternative processing method consideration
   - Least intrusive option selection
   
   Assessment Criteria:
   - Purpose achievement effectiveness
   - Data minimization principle compliance
   - Alternative solution evaluation
   - Processing impact justification
   ```

3. **Risk Identification and Assessment**
   ```
   Identify:
   - Privacy risks to data subjects
   - Likelihood and severity evaluation
   - Risk source and impact analysis
   - Stakeholder consultation results
   
   Risk Analysis Framework:
   - Confidentiality breach risks
   - Availability and integrity threats
   - Discrimination and exclusion potential
   - Financial and reputational harm
   ```

4. **Mitigation Measures and Safeguards**
   ```
   Design:
   - Technical protection measures
   - Organizational safeguards
   - Data subject right facilitation
   - Ongoing monitoring procedures
   
   Safeguard Implementation:
   - Privacy by design integration
   - Security control deployment
   - Access and correction mechanisms
   - Regular review and update procedures
   ```

**Stakeholder Consultation:**
- [ ] Data subject representative consultation
- [ ] Data Protection Officer (DPO) involvement
- [ ] External expert consultation (if required)
- [ ] Supervisory authority consultation (if high residual risk)

**DPIA Review and Updates:**
- [ ] Regular DPIA review schedule
- [ ] Processing change impact assessment
- [ ] Risk reassessment triggers
- [ ] Mitigation measure effectiveness monitoring

**DPIA Approval and Documentation:**
- [ ] Senior management approval
- [ ] DPO opinion documentation
- [ ] Supervisory authority consultation (if applicable)
- [ ] Implementation monitoring plan

**High-Risk Processing Indicators:**
- Systematic and extensive evaluation
- Processing of special category data
- Systematic monitoring of public areas
- Vulnerable data subject processing
- Innovative technology use
- Cross-border transfers to non-adequate countries
```

## CCPA (California Consumer Privacy Act) Audit Prompts

### CCPA Compliance Assessment Prompt
```
Conduct California Consumer Privacy Act compliance audit:

**Business Profile:** [CCPA_BUSINESS_DEFINITION_CRITERIA_MET]
**Personal Information Scope:** [CCPA_PI_CATEGORIES_PROCESSED]
**Consumer Rights Implementation:** [RIGHTS_FRAMEWORK_ASSESSMENT]
**Sale/Sharing Disclosure:** [CCPA_SALE_DEFINITION_ACTIVITIES]

**CCPA Business Obligations Assessment:**

1. **Consumer Right to Know (Sections 1798.100, 1798.110, 1798.115)**
   ```
   Verify:
   - Privacy policy disclosure requirements
   - Right to know request handling
   - Personal information categories disclosure
   - Business purpose and third-party sharing disclosure
   
   Implementation Testing:
   - Privacy policy content completeness
   - Request response procedures
   - Information accuracy and timeliness
   - Disclosure format and accessibility
   ```

2. **Consumer Right to Delete (Section 1798.105)**
   ```
   Test:
   - Deletion request processing procedures
   - Deletion scope and limitations
   - Third-party deletion notification
   - Retention exception applications
   
   Deletion Testing:
   - Request verification procedures
   - Deletion completeness validation
   - Exception justification documentation
   - Service provider notification processes
   ```

3. **Consumer Right to Opt-Out (Section 1798.120)**
   ```
   Assess:
   - "Do Not Sell My Personal Information" implementation
   - Opt-out mechanism accessibility
   - Third-party sales cessation
   - Global Privacy Control (GPC) recognition
   
   Opt-Out Verification:
   - Opt-out link functionality
   - Request processing effectiveness
   - Sales cessation implementation
   - GPC signal processing
   ```

4. **Consumer Right to Non-Discrimination (Section 1798.125)**
   ```
   Evaluate:
   - Non-discrimination policy implementation
   - Service quality maintenance
   - Pricing structure consistency
   - Incentive program CCPA compliance
   
   Discrimination Testing:
   - Service delivery comparison
   - Pricing differential analysis
   - Incentive program structure review
   - Financial incentive disclosure adequacy
   ```

**Privacy Policy and Disclosure Requirements:**

**Section 1798.130 - Privacy Policy Requirements**
```
Verify:
- Personal information category disclosures
- Business purpose explanations
- Third-party sharing descriptions
- Consumer rights explanations
- Contact information provision

Policy Assessment:
- Content completeness verification
- Update frequency compliance
- Accessibility requirement adherence
- Plain language usage validation
```

**Section 1798.135 - Methods for Submitting Requests**
```
Test:
- Multiple request submission methods
- Toll-free telephone number provision
- Website submission form functionality
- Request verification procedures

Method Testing:
- Submission channel effectiveness
- Response time compliance
- Verification process security
- Accessibility accommodation
```

**Service Provider and Third-Party Relationships:**

**Section 1798.140(v) - Service Provider Definition**
```
Assess:
- Service provider contract compliance
- Processing limitation adherence
- Personal information retention restrictions
- Sub-contractor management

Contract Validation:
- CCPA-compliant contract terms
- Processing purpose limitations
- Retention and deletion requirements
- Sub-contractor approval processes
```

**Section 1798.140(t) - Sale Definition and Activities**
```
Examine:
- Sale activity identification
- Monetary consideration evaluation
- Valuable consideration assessment
- Disclosure vs. sale determination

Sale Assessment:
- Activity classification accuracy
- Consideration value determination
- Disclosure justification validation
- Opt-out requirement compliance
```

**Data Minimization and Security:**

**Section 1798.150 - Security Requirements**
```
Validate:
- Reasonable security procedure implementation
- Personal information protection measures
- Security incident response procedures
- Breach notification compliance

Security Testing:
- Security control effectiveness
- Incident response capability
- Data protection measure adequacy
- Compliance with security regulations
```

**CCPA Compliance Metrics:**
- Consumer request response rates
- Opt-out implementation effectiveness
- Privacy policy accessibility compliance
- Non-discrimination adherence measurement

**Risk Assessment Framework:**
- **High Risk:** Attorney General enforcement action
- **Medium Risk:** Consumer class action lawsuits
- **Low Risk:** Regulatory guidance and warnings
- **Ongoing:** Reputational and competitive impact
```

### CPRA (California Privacy Rights Act) Update Audit Prompt
```
Assess CPRA amendments and enhanced requirements:

**CPRA Effective Date:** January 1, 2023
**Enhanced Rights:** [CORRECTION_LIMITATION_SENSITIVE_PI_PROTECTIONS]
**Regulatory Enforcement:** [CALIFORNIA_PRIVACY_PROTECTION_AGENCY]

**CPRA Enhanced Requirements:**

1. **Sensitive Personal Information Protections**
   ```
   Assess:
   - Sensitive PI category identification
   - Processing limitation compliance
   - Right to limit usage implementation
   - Enhanced disclosure requirements
   
   Sensitive PI Categories:
   - Social Security numbers and driver's licenses
   - Financial account information
   - Precise geolocation data
   - Racial and ethnic origin
   - Religious and philosophical beliefs
   - Biometric and genetic data
   - Health information
   - Sexual orientation and behavior
   ```

2. **Consumer Right to Correction**
   ```
   Verify:
   - Correction request handling procedures
   - Inaccurate information identification
   - Correction implementation processes
   - Third-party correction notification
   
   Correction Framework:
   - Request verification procedures
   - Accuracy assessment processes
   - Correction scope determination
   - Service provider notification
   ```

3. **Risk Assessment and Data Minimization**
   ```
   Evaluate:
   - Regular risk assessment conduct
   - Data minimization implementation
   - Purpose limitation enforcement
   - Processing impact evaluation
   
   Assessment Requirements:
   - Annual risk assessment completion
   - Data inventory and mapping
   - Processing necessity evaluation
   - Retention period justification
   ```

4. **Automated Decision-Making Transparency**
   ```
   Test:
   - Automated decision-making disclosure
   - Logic and significance explanation
   - Consumer opt-out mechanisms
   - Appeal and review processes
   
   Transparency Implementation:
   - Decision-making process documentation
   - Consumer notification procedures
   - Opt-out mechanism functionality
   - Review process effectiveness
   ```

**California Privacy Protection Agency (CPPA) Readiness:**
- [ ] Regulatory investigation preparedness
- [ ] Enforcement action response procedures
- [ ] Compliance documentation organization
- [ ] Administrative penalty mitigation strategies

**Cross-Regulation Harmonization:**
- CCPA/CPRA baseline compliance
- GDPR alignment opportunities
- Federal privacy law preparation
- State privacy law coordination
```

## PIPEDA (Personal Information Protection and Electronic Documents Act) Audit Prompts

### PIPEDA Compliance Assessment Prompt
```
Conduct PIPEDA compliance audit for Canadian privacy obligations:

**Organization Scope:** [FEDERAL_JURISDICTION_COMMERCIAL_ACTIVITIES]
**Personal Information Processing:** [COLLECTION_USE_DISCLOSURE_ACTIVITIES]
**Provincial Law Interaction:** [SUBSTANTIALLY_SIMILAR_PROVINCIAL_LAWS]

**PIPEDA Fair Information Principles Assessment:**

**Principle 1 - Accountability**
```
Verify:
- Designated privacy officer appointment
- Privacy policy development and maintenance
- Staff training and awareness programs
- Compliance monitoring procedures

Accountability Framework:
- Privacy officer role definition
- Policy governance structure
- Training program effectiveness
- Monitoring and oversight mechanisms
```

**Principle 2 - Identifying Purposes**
```
Assess:
- Purpose identification at collection
- Clear and understandable communication
- Purpose limitation compliance
- Secondary use authorization

Purpose Assessment:
- Collection purpose documentation
- Communication clarity validation
- Use limitation enforcement
- Consent for new purposes
```

**Principle 3 - Consent**
```
Test:
- Meaningful consent obtaining
- Consent withdrawal mechanisms
- Implied vs. explicit consent appropriateness
- Ongoing consent validation

Consent Validation:
- Consent quality assessment
- Withdrawal process effectiveness
- Consent type appropriateness
- Ongoing consent management
```

**Principle 4 - Limiting Collection**
```
Evaluate:
- Collection limitation to identified purposes
- Fair and lawful collection methods
- Individual awareness of collection
- Collection minimization implementation

Collection Assessment:
- Purpose alignment verification
- Collection method evaluation
- Individual notification validation
- Minimization principle compliance
```

**Principle 5 - Limiting Use, Disclosure, and Retention**
```
Examine:
- Use limitation to identified purposes
- Disclosure restriction compliance
- Retention period appropriateness
- Secure disposal procedures

Use and Disclosure Testing:
- Purpose limitation adherence
- Disclosure authorization verification
- Retention schedule implementation
- Disposal security validation
```

**Principle 6 - Accuracy**
```
Verify:
- Information accuracy maintenance
- Correction mechanism provision
- Accuracy verification procedures
- Update process implementation

Accuracy Framework:
- Data quality monitoring
- Correction request handling
- Verification process validation
- Update mechanism effectiveness
```

**Principle 7 - Safeguards**
```
Test:
- Security safeguard implementation
- Protection level appropriateness
- Staff access controls
- Third-party protection requirements

Security Assessment:
- Safeguard adequacy evaluation
- Access control effectiveness
- Third-party security validation
- Ongoing protection monitoring
```

**Principle 8 - Openness**
```
Assess:
- Policy and practice accessibility
- Information availability to individuals
- Contact information provision
- Transparency in processing

Openness Evaluation:
- Policy accessibility validation
- Information availability verification
- Contact information accuracy
- Processing transparency assessment
```

**Principle 9 - Individual Access**
```
Verify:
- Access request handling procedures
- Information provision completeness
- Response timeframe compliance
- Cost limitation adherence

Access Testing:
- Request processing effectiveness
- Information accuracy verification
- Timeframe compliance measurement
- Cost reasonableness assessment
```

**Principle 10 - Challenging Compliance**
```
Examine:
- Complaint handling procedures
- Investigation process implementation
- Resolution mechanism effectiveness
- Privacy commissioner cooperation

Challenge Assessment:
- Complaint process accessibility
- Investigation thoroughness
- Resolution effectiveness
- Regulatory cooperation
```

**Privacy Breach Management:**
- [ ] Breach identification procedures
- [ ] Risk assessment methodology
- [ ] Notification requirements compliance
- [ ] Remediation and prevention measures

**Cross-Border Transfer Considerations:**
- [ ] International transfer assessment
- [ ] Comparable protection evaluation
- [ ] Transfer agreement implementation
- [ ] Ongoing protection monitoring

**PIPEDA Compliance Maturity:**
- **Level 1:** Basic compliance framework
- **Level 2:** Comprehensive program implementation
- **Level 3:** Proactive privacy management
- **Level 4:** Privacy leadership and innovation
```

## Integrated Data Protection Compliance Dashboard Prompt
```
Create unified data protection compliance dashboard:

**Multi-Jurisdiction Coverage:** GDPR + CCPA/CPRA + PIPEDA
**Global Data Processing:** [WORLDWIDE_OPERATIONS_AND_DATA_FLOWS]
**Compliance Harmonization:** [CROSS_REGULATION_ALIGNMENT_STRATEGY]

**Unified Compliance Assessment:**

1. **Rights Management Framework**
   ```
   GDPR Rights Compliance:
   - Access: [COMPLIANCE_SCORE]/100
   - Rectification: [COMPLIANCE_SCORE]/100
   - Erasure: [COMPLIANCE_SCORE]/100
   - Portability: [COMPLIANCE_SCORE]/100
   - Objection: [COMPLIANCE_SCORE]/100
   
   CCPA/CPRA Rights Compliance:
   - Know: [COMPLIANCE_SCORE]/100
   - Delete: [COMPLIANCE_SCORE]/100
   - Opt-Out: [COMPLIANCE_SCORE]/100
   - Correct: [COMPLIANCE_SCORE]/100
   - Non-Discrimination: [COMPLIANCE_SCORE]/100
   
   PIPEDA Principles Compliance:
   - Consent: [COMPLIANCE_SCORE]/100
   - Access: [COMPLIANCE_SCORE]/100
   - Accuracy: [COMPLIANCE_SCORE]/100
   - Safeguards: [COMPLIANCE_SCORE]/100
   - Accountability: [COMPLIANCE_SCORE]/100
   ```

2. **Cross-Border Transfer Compliance**
   ```
   Transfer Mechanism Validation:
   - GDPR Article 44-49 compliance
   - CCPA third-party sharing disclosure
   - PIPEDA comparable protection assessment
   - Global transfer policy harmonization
   ```

3. **Data Subject Communication Harmonization**
   ```
   Privacy Notice Consolidation:
   - Multi-jurisdiction privacy policy
   - Rights communication standardization
   - Contact information centralization
   - Language and accessibility compliance
   ```

4. **Security and Technical Measures**
   ```
   Unified Security Framework:
   - GDPR Article 32 technical measures
   - CCPA reasonable security procedures
   - PIPEDA safeguard requirements
   - Cross-regulation security harmonization
   ```

**Risk-Based Compliance Prioritization:**
- **Critical:** Regulatory enforcement and penalty risk
- **High:** Data subject complaint and litigation exposure
- **Medium:** Operational efficiency and cost optimization
- **Low:** Process improvement and best practice adoption

**Global Compliance Monitoring:**
- Real-time compliance status dashboard
- Automated regulatory change monitoring
- Multi-jurisdiction incident response
- Centralized compliance reporting

**Strategic Compliance Roadmap:**
- Phase 1: Critical gap remediation (0-90 days)
- Phase 2: Process standardization (90-180 days)
- Phase 3: Technology optimization (180-365 days)
- Phase 4: Continuous improvement (ongoing)

**Stakeholder Communication Framework:**
- Executive compliance scorecards
- Legal and compliance team dashboards
- IT and security implementation guides
- Customer and public transparency reports
```