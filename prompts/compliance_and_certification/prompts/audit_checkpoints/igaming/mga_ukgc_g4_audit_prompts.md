# MGA (Malta Gaming Authority) Audit Checkpoint Prompts


  
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


## MGA License Compliance Audit

### Class 1 Gaming License Audit Prompt
```
Conduct comprehensive MGA Class 1 gaming license compliance audit:

**License Type:** [CLASS_1_REMOTE_GAMING_LICENSE]
**Operator Details:** [COMPANY_NAME_AND_LICENSE_NUMBER]
**Audit Scope:** [SYSTEMS_AND_PROCESSES_COVERED]
**Regulatory Framework:** MGA Gaming Act (Chapter 583) and Gaming Regulations

**Core Compliance Areas:**

1. **Technical Systems Reliability**
   ```
   Verify:
   - 99.5% minimum system uptime requirement
   - Redundancy and failover mechanisms
   - Data backup and disaster recovery procedures
   - System monitoring and alerting capabilities
   ```

2. **Player Protection Framework**
   ```
   Assess:
   - Self-exclusion system (SENS) integration
   - Deposit limit controls and cooling-off periods
   - Reality check implementations
   - Vulnerable player identification procedures
   ```

3. **Anti-Money Laundering (AML) Controls**
   ```
   Evaluate:
   - Customer Due Diligence (CDD) procedures
   - Enhanced Due Diligence (EDD) triggers
   - Suspicious Transaction Reporting (STR)
   - Ongoing monitoring and risk assessment
   ```

4. **Advertising and Marketing Compliance**
   ```
   Review:
   - Marketing communications approval process
   - Responsible advertising guidelines adherence
   - Age verification in marketing materials
   - Bonus terms and conditions clarity
   ```

**Player Fund Protection Audit:**
- [ ] Segregated account requirements (minimum 5% of gross gaming revenue)
- [ ] Bank guarantee or insurance policy validation
- [ ] Monthly reconciliation procedures
- [ ] Independent auditor confirmation

**Data Protection and Privacy:**
- [ ] GDPR compliance implementation
- [ ] Data retention and deletion policies
- [ ] Cross-border data transfer mechanisms
- [ ] Player consent management systems

**Financial Controls:**
- [ ] Audited financial statements review
- [ ] Minimum share capital maintenance
- [ ] Key person requirements validation
- [ ] Corporate governance structure

**Regulatory Reporting:**
- [ ] Monthly technical compliance reports
- [ ] Annual compliance attestation
- [ ] Incident reporting procedures
- [ ] Regulatory correspondence tracking

**Risk Assessment Matrix:**
- **Critical:** License suspension/revocation risk
- **High:** Regulatory penalty exposure
- **Medium:** Warning notice potential
- **Low:** Administrative improvement areas
```

### Class 4 Gaming License Audit Prompt
```
Perform MGA Class 4 gaming services provider license audit:

**Service Provider Type:** [B2B_GAMING_SERVICES_CATEGORY]
**Client Portfolio:** [OPERATOR_CLIENTS_AND_JURISDICTIONS]
**Service Scope:** [SPECIFIC_SERVICES_PROVIDED]

**B2B Service Provider Compliance:**

1. **Platform Integration Security**
   ```
   Verify:
   - Secure API implementation and authentication
   - Data encryption standards (AES-256 minimum)
   - Access control and authorization mechanisms
   - Third-party integration security protocols
   ```

2. **Client Due Diligence**
   ```
   Assess:
   - Client licensing verification procedures
   - Ongoing compliance monitoring
   - Contract terms and SLA compliance
   - Termination procedures for non-compliant clients
   ```

3. **Technical Standards Compliance**
   ```
   Validate:
   - RNG certification and testing
   - Game mathematics verification
   - Return to Player (RTP) accuracy
   - Game integrity monitoring systems
   ```

4. **Service Level Agreements**
   ```
   Review:
   - Uptime guarantees and penalties
   - Support response time commitments
   - Data availability and backup procedures
   - Performance monitoring and reporting
   ```

**Software Development Lifecycle:**
- [ ] Secure coding practices implementation
- [ ] Code review and testing procedures
- [ ] Version control and change management
- [ ] Vulnerability assessment and remediation

**Intellectual Property Protection:**
- [ ] Software licensing compliance
- [ ] Source code protection measures
- [ ] Third-party component inventory
- [ ] Copyright and trademark compliance

**Business Continuity Planning:**
- [ ] Disaster recovery procedures
- [ ] Business impact analysis
- [ ] Recovery time objectives (RTO)
- [ ] Alternative service delivery options

**Client Relationship Management:**
- [ ] Contract management procedures
- [ ] Service performance monitoring
- [ ] Dispute resolution mechanisms
- [ ] Client communication protocols
```

## UKGC (UK Gambling Commission) Audit Checkpoint Prompts

### Remote Operating License Audit Prompt
```
Conduct UKGC Remote Operating License compliance audit:

**License Category:** [REMOTE_CASINO_BETTING_BINGO_LOTTERY]
**Operator Jurisdiction:** United Kingdom
**Regulatory Framework:** Gambling Act 2005 and LCCP (License Conditions and Codes of Practice)

**Social Responsibility Compliance:**

1. **Customer Interaction Framework**
   ```
   Evaluate:
   - Customer risk assessment procedures
   - Mandatory customer interaction triggers
   - Staff training on customer vulnerability
   - Interaction recording and documentation
   ```

2. **Safer Gambling Tools**
   ```
   Verify:
   - Deposit limit setting (mandatory)
   - Time-based session reminders
   - Loss limit notifications
   - Self-exclusion options (single and multi-operator)
   ```

3. **Marketing and Advertising Standards**
   ```
   Assess:
   - CAP Code compliance verification
   - Age-restricted audience targeting
   - Responsible advertising messages
   - Bonus and promotion clarity
   ```

4. **Age Verification Controls**
   ```
   Test:
   - Registration age verification (within 72 hours)
   - Enhanced verification for higher risk customers
   - Document authentication procedures
   - Ongoing monitoring for account sharing
   ```

**Financial Crime Prevention:**
- [ ] Proceeds of Crime Act compliance
- [ ] Money Laundering Regulations adherence
- [ ] Source of funds verification procedures
- [ ] Enhanced due diligence triggers and implementation

**Technical Standards:**
- [ ] RNG testing and certification (annual)
- [ ] Game outcome recording and storage
- [ ] Player transaction audit trails
- [ ] System security and data protection

**Consumer Protection:**
- [ ] Terms and conditions clarity and fairness
- [ ] Dispute resolution procedures
- [ ] Complaint handling and escalation
- [ ] Player fund segregation and protection

**Regulatory Reporting:**
- [ ] Quarterly returns submission
- [ ] Incident reporting (within 24 hours)
- [ ] Material change notifications
- [ ] Annual compliance assessment

**Penalty Risk Assessment:**
- **Severe:** License revocation or suspension
- **Substantial:** Significant financial penalties (up to £2M+)
- **Moderate:** Warning letters and conditions
- **Minor:** Informal guidance and improvement plans
```

### Personal License Holder Audit Prompt
```
Audit UKGC Personal License holder compliance:

**Personal License Category:** [PML_PERSONAL_MANAGEMENT_LICENSE]
**Role and Responsibilities:** [SPECIFIC_MANAGEMENT_FUNCTION]
**Operating License Association:** [LINKED_OPERATING_LICENSE]

**Key Person Suitability Assessment:**

1. **Fitness and Propriety**
   ```
   Verify:
   - Enhanced DBS check validity
   - Financial probity assessment
   - Competence and experience validation
   - Ongoing suitability monitoring
   ```

2. **Regulatory Compliance Knowledge**
   ```
   Test:
   - Understanding of gambling legislation
   - LCCP knowledge and application
   - Social responsibility awareness
   - Anti-money laundering procedures
   ```

3. **Operational Responsibilities**
   ```
   Assess:
   - Day-to-day compliance oversight
   - Staff training and development
   - Incident response and reporting
   - Risk management implementation
   ```

4. **Professional Development**
   ```
   Review:
   - Continuing professional development
   - Industry training participation
   - Regulatory update awareness
   - Best practice implementation
   ```

**Management System Controls:**
- [ ] Compliance monitoring procedures
- [ ] Staff training programs
- [ ] Performance measurement systems
- [ ] Regulatory change management

**Accountability Framework:**
- [ ] Clear role definitions and boundaries
- [ ] Reporting structure and escalation
- [ ] Decision-making authority limits
- [ ] Performance review processes

**Risk Management:**
- [ ] Risk assessment procedures
- [ ] Mitigation strategy implementation
- [ ] Regular review and updates
- [ ] Board/senior management reporting

**Documentation Requirements:**
- [ ] Role description and responsibilities
- [ ] Training records and certifications
- [ ] Performance evaluation reports
- [ ] Regulatory compliance attestations
```

## G4 (Global Gaming Guidance Group) Audit Checkpoint Prompts

### Responsible Gaming Standards Audit Prompt
```
Conduct G4 Responsible Gaming standards compliance audit:

**Operator Profile:** [MULTI_JURISDICTIONAL_GAMING_OPERATOR]
**G4 Framework:** [APPLICABLE_STANDARDS_AND_GUIDELINES]
**Assessment Scope:** [GLOBAL_OPERATIONS_COVERAGE]

**G4 Core Standards Assessment:**

1. **Research, Education and Awareness**
   ```
   Evaluate:
   - Problem gambling research participation
   - Public awareness campaign contributions
   - Educational material development and distribution
   - Community outreach program effectiveness
   ```

2. **Employee Training and Development**
   ```
   Verify:
   - Comprehensive staff training programs
   - Regular refresher training schedules
   - Problem gambling recognition skills
   - Intervention technique competency
   ```

3. **Game Design and Features**
   ```
   Assess:
   - Responsible design principles implementation
   - Player protection feature integration
   - Risk assessment for new game features
   - Behavioral impact consideration in development
   ```

4. **Responsible Marketing and Advertising**
   ```
   Review:
   - Marketing code compliance
   - Target audience appropriateness
   - Problem gambling sensitivity
   - Clear and transparent communications
   ```

**Player Protection Tools:**
- [ ] Comprehensive self-exclusion systems
- [ ] Flexible limit-setting options
- [ ] Reality check implementations
- [ ] Account activity monitoring

**Operational Excellence:**
- [ ] Management commitment demonstration
- [ ] Resource allocation adequacy
- [ ] Performance measurement systems
- [ ] Continuous improvement processes

**Industry Collaboration:**
- [ ] Information sharing participation
- [ ] Best practice development contribution
- [ ] Research collaboration engagement
- [ ] Standards development involvement

**Multi-Jurisdictional Compliance:**
- [ ] Local regulation alignment
- [ ] Cultural sensitivity adaptation
- [ ] Cross-border coordination
- [ ] Harmonized standard implementation
```

### Global Anti-Money Laundering Framework Audit Prompt
```
Audit G4 global AML framework implementation:

**Global Operations:** [JURISDICTIONAL_COVERAGE_AND_LICENSES]
**AML Framework:** [G4_AML_GUIDELINES_AND_FATF_RECOMMENDATIONS]
**Risk Assessment:** [COUNTRY_AND_PRODUCT_RISK_MATRIX]

**Global AML Compliance Assessment:**

1. **Risk-Based Approach Implementation**
   ```
   Verify:
   - Comprehensive risk assessment methodology
   - Country and customer risk categorization
   - Product and transaction risk evaluation
   - Regular risk assessment updates
   ```

2. **Customer Due Diligence (CDD) Standards**
   ```
   Assess:
   - Identity verification procedures
   - Source of wealth/funds verification
   - Enhanced due diligence triggers
   - Ongoing monitoring requirements
   ```

3. **Suspicious Activity Monitoring**
   ```
   Evaluate:
   - Transaction monitoring systems
   - Alert generation and investigation
   - Suspicious activity reporting (SAR)
   - Cross-jurisdictional information sharing
   ```

4. **Record Keeping and Reporting**
   ```
   Review:
   - Transaction record retention (minimum 5 years)
   - Customer identification documentation
   - Regulatory reporting compliance
   - Audit trail maintenance
   ```

**Technology and Systems:**
- [ ] AML monitoring system effectiveness
- [ ] False positive rate optimization
- [ ] Cross-border data integration
- [ ] Real-time screening capabilities

**Staff Training and Awareness:**
- [ ] AML training program comprehensiveness
- [ ] Role-specific training modules
- [ ] Regular update and refresher training
- [ ] Competency assessment procedures

**Regulatory Coordination:**
- [ ] Multi-jurisdictional reporting compliance
- [ ] Regulatory examination cooperation
- [ ] Information sharing protocols
- [ ] Cross-border investigation support

**Performance Metrics:**
- [ ] SAR filing rates and quality
- [ ] Investigation completion timeframes
- [ ] False positive reduction targets
- [ ] Regulatory feedback incorporation
```

## Cross-Platform iGaming Audit Summary Prompt
```
Compile comprehensive cross-platform iGaming compliance summary:

**Audit Coverage:** GLI Technical Standards + MGA Regulatory Requirements + UKGC Consumer Protection + G4 Responsible Gaming
**Assessment Period:** [COMPREHENSIVE_AUDIT_TIMEFRAME]
**Platform Scope:** [COMPLETE_GAMING_ECOSYSTEM]

**Integrated Compliance Dashboard:**

1. **Technical Compliance Matrix**
   ```
   GLI Standards Compliance:
   - GLI-11 Gaming Devices: [SCORE]/100
   - GLI-19 Interactive Gaming: [SCORE]/100
   - GLI-33 Event Wagering: [SCORE]/100
   
   MGA Regulatory Compliance:
   - License Conditions: [SCORE]/100
   - Player Protection: [SCORE]/100
   - AML/CFT: [SCORE]/100
   
   UKGC Consumer Protection:
   - Social Responsibility: [SCORE]/100
   - Safer Gambling: [SCORE]/100
   - Fair Trading: [SCORE]/100
   
   G4 Industry Standards:
   - Responsible Gaming: [SCORE]/100
   - Employee Training: [SCORE]/100
   - Research Participation: [SCORE]/100
   ```

2. **Risk Consolidation Framework**
   ```
   Critical Risk Areas:
   - Multi-jurisdictional compliance gaps
   - Cross-platform consistency issues
   - Regulatory conflict resolution needs
   - Technology standardization requirements
   ```

3. **Harmonized Remediation Plan**
   ```
   Phase 1: Critical compliance gaps (immediate)
   Phase 2: Platform standardization (30-60 days)
   Phase 3: Process optimization (60-120 days)
   Phase 4: Continuous improvement (ongoing)
   ```

4. **Global Certification Readiness**
   ```
   Certification Timeline:
   - GLI Standards: [READINESS_STATUS]
   - MGA License Renewal: [READINESS_STATUS]
   - UKGC Compliance Assessment: [READINESS_STATUS]
   - G4 Standards Verification: [READINESS_STATUS]
   ```

**Executive Reporting Package:**
- Multi-jurisdictional compliance dashboard
- Risk-adjusted investment priorities
- Regulatory relationship management plan
- Industry best practice benchmarking

**Ongoing Monitoring Protocol:**
- Monthly compliance scorecards
- Quarterly risk assessments
- Annual certification renewals
- Continuous regulatory monitoring
```