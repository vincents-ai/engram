# SaaS/IT Audit Checkpoint Prompts


  
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


## SOC 2 (Service Organization Control 2) Audit Prompts

### Type II SOC 2 Audit - Trust Services Criteria

#### Security Criteria Audit Prompt
```
Conduct SOC 2 Type II Security criteria audit for SaaS platform:

**Service Organization:** [COMPANY_NAME_AND_SERVICE_DESCRIPTION]
**Audit Period:** [12_MONTH_EXAMINATION_PERIOD]
**Trust Services Criteria:** Security (Common Criteria)
**Complementary User Entity Controls (CUECs):** [IDENTIFIED_CUSTOMER_RESPONSIBILITIES]

**Security Controls Assessment:**

1. **Logical and Physical Access Controls**
   ```
   Test Operating Effectiveness:
   - Multi-factor authentication implementation
   - Role-based access control (RBAC) systems
   - Privileged access management (PAM)
   - Access review and recertification processes
   
   Sample Testing:
   - 25 access provisioning requests
   - 25 access deprovisioning events
   - Quarterly access reviews for all privileged users
   - Annual access reviews for all system users
   ```

2. **System Operations and Availability**
   ```
   Validate Controls:
   - Change management procedures
   - System monitoring and alerting
   - Incident response and management
   - Business continuity and disaster recovery
   
   Evidence Collection:
   - Change control board meeting minutes
   - Monitoring system configuration screenshots
   - Incident tickets and resolution documentation
   - DR test results and recovery time measurements
   ```

3. **Network Security Controls**
   ```
   Examine:
   - Firewall configurations and rule reviews
   - Network segmentation and DMZ implementation
   - Intrusion detection/prevention systems
   - Vulnerability scanning and remediation
   
   Testing Procedures:
   - Quarterly firewall rule reviews
   - Monthly vulnerability scan results
   - Network segmentation effectiveness testing
   - IDS/IPS alert investigation procedures
   ```

4. **Data Protection and Encryption**
   ```
   Verify:
   - Data classification and handling procedures
   - Encryption at rest and in transit
   - Key management and rotation
   - Data retention and disposal
   
   Control Testing:
   - Encryption algorithm validation (AES-256)
   - Key rotation schedule compliance
   - Data disposal certificate collection
   - Backup encryption verification
   ```

**Risk Assessment and Monitoring:**
- [ ] Annual risk assessment completion
- [ ] Risk register maintenance and updates
- [ ] Control effectiveness monitoring
- [ ] Management review and approval processes

**Vendor and Third-Party Management:**
- [ ] Vendor risk assessment procedures
- [ ] Third-party access control management
- [ ] Service provider monitoring and oversight
- [ ] Contract security requirement verification

**Security Awareness and Training:**
- [ ] Annual security awareness training
- [ ] Role-specific security training programs
- [ ] Phishing simulation testing
- [ ] Security incident response training

**Exception Documentation:**
- Any control deficiencies identified
- Compensating controls implemented
- Management responses and remediation plans
- Testing limitations and scope restrictions
```

#### Availability Criteria Audit Prompt
```
Perform SOC 2 Type II Availability criteria examination:

**Service Commitment:** [SLA_UPTIME_TARGETS_AND_AVAILABILITY_METRICS]
**System Boundaries:** [IN_SCOPE_SYSTEMS_AND_INFRASTRUCTURE]
**Measurement Period:** [CONTINUOUS_12_MONTH_MONITORING]

**Availability Controls Testing:**

1. **System Performance Monitoring**
   ```
   Validate Controls:
   - Real-time performance monitoring systems
   - Capacity planning and management
   - Performance threshold alerting
   - SLA compliance measurement and reporting
   
   Testing Approach:
   - Review 12 months of uptime statistics
   - Analyze performance degradation incidents
   - Verify alerting threshold configurations
   - Test escalation procedures effectiveness
   ```

2. **Environmental Protection**
   ```
   Examine:
   - Data center environmental controls
   - Power supply redundancy and UPS systems
   - Climate control and monitoring
   - Fire suppression and detection systems
   
   Evidence Requirements:
   - Environmental monitoring logs
   - Power failure incident documentation
   - Data center audit reports
   - Environmental control testing records
   ```

3. **System Backup and Recovery**
   ```
   Test:
   - Backup schedule and completion monitoring
   - Backup integrity verification procedures
   - Recovery testing and documentation
   - Recovery time objective (RTO) compliance
   
   Sample Testing:
   - Monthly backup completion reports
   - Quarterly backup restoration tests
   - Annual disaster recovery exercise
   - Recovery time measurement documentation
   ```

4. **Change Management Impact Assessment**
   ```
   Assess:
   - Change impact assessment procedures
   - Change approval and authorization
   - Change implementation and rollback procedures
   - Post-implementation review processes
   
   Control Testing:
   - Review 25 production changes
   - Verify change impact assessments
   - Test rollback procedure execution
   - Validate change success criteria
   ```

**Service Level Agreement Compliance:**
- [ ] 99.9% uptime target achievement
- [ ] Mean Time to Resolution (MTTR) metrics
- [ ] Planned maintenance window adherence
- [ ] Customer notification procedures

**Incident Management:**
- [ ] Incident classification and prioritization
- [ ] Escalation procedures and timeframes
- [ ] Root cause analysis completion
- [ ] Preventive action implementation

**Capacity Management:**
- [ ] Resource utilization monitoring
- [ ] Growth trend analysis and forecasting
- [ ] Capacity planning documentation
- [ ] Infrastructure scaling procedures

**Availability Reporting:**
- Monthly availability reports to management
- Quarterly SLA compliance reporting
- Annual availability trend analysis
- Customer-facing service status dashboard
```

#### Processing Integrity Criteria Audit Prompt
```
Conduct SOC 2 Processing Integrity criteria audit:

**System Processing:** [CRITICAL_BUSINESS_PROCESSES_AND_DATA_FLOWS]
**Processing Objectives:** [COMPLETENESS_ACCURACY_TIMELINESS_AUTHORIZATION]
**Control Environment:** [PROCESSING_CONTROLS_AND_MONITORING]

**Processing Integrity Controls:**

1. **Input Controls and Validation**
   ```
   Test:
   - Data input validation and verification
   - Exception handling and error processing
   - Completeness and accuracy checks
   - Authorized source verification
   
   Testing Procedures:
   - Sample 40 data input transactions
   - Verify validation rule effectiveness
   - Test error handling procedures
   - Confirm authorization controls
   ```

2. **Processing Controls**
   ```
   Examine:
   - Automated processing controls
   - Manual review and approval procedures
   - Processing sequence and dependency management
   - Data transformation accuracy
   
   Evidence Collection:
   - Processing control documentation
   - System logic and business rule verification
   - Data transformation mapping validation
   - Processing exception reports
   ```

3. **Output Controls and Distribution**
   ```
   Validate:
   - Output completeness and accuracy verification
   - Output distribution controls
   - Report generation and delivery
   - Output retention and archival
   
   Control Testing:
   - Sample 25 output generation processes
   - Verify output distribution authorization
   - Test report accuracy and completeness
   - Confirm retention policy compliance
   ```

4. **Standing Data Management**
   ```
   Assess:
   - Master data maintenance procedures
   - Standing data accuracy verification
   - Unauthorized change prevention
   - Data synchronization between systems
   
   Testing Approach:
   - Review master data change controls
   - Test data accuracy verification procedures
   - Verify synchronization processes
   - Confirm change authorization requirements
   ```

**Processing Monitoring and Reconciliation:**
- [ ] Automated processing monitoring
- [ ] Exception identification and investigation
- [ ] Reconciliation procedures and controls
- [ ] Processing integrity reporting

**Data Quality Management:**
- [ ] Data quality metrics and monitoring
- [ ] Data cleansing and correction procedures
- [ ] Data governance framework
- [ ] Quality assurance testing procedures

**System Interface Controls:**
- [ ] Interface mapping and documentation
- [ ] Data transmission controls
- [ ] Interface monitoring and error handling
- [ ] System integration testing procedures

**Processing Integrity Reporting:**
- Regular processing accuracy reports
- Exception summary and trend analysis
- Data quality dashboard maintenance
- Management oversight and review documentation
```

## ISO 27001 Information Security Management System (ISMS) Audit Prompts

### ISO 27001 Stage 2 Certification Audit Prompt
```
Conduct ISO 27001 Stage 2 certification audit for ISMS implementation:

**Organization:** [COMPANY_NAME_AND_SCOPE_OF_ISMS]
**Certification Scope:** [DEPARTMENTS_PROCESSES_LOCATIONS_COVERED]
**Applicable Controls:** [ANNEX_A_CONTROLS_SELECTION_FROM_SOA]
**Audit Duration:** [3_YEAR_CERTIFICATION_CYCLE_ASSESSMENT]

**ISMS Framework Assessment:**

1. **Context of the Organization (Clause 4)**
   ```
   Verify:
   - Understanding of organization and context
   - Interested parties and requirements identification
   - ISMS scope determination and documentation
   - Information security management system establishment
   
   Evidence Requirements:
   - Context analysis documentation
   - Stakeholder register and requirements
   - ISMS scope statement
   - ISMS policy and objectives
   ```

2. **Leadership and Commitment (Clause 5)**
   ```
   Assess:
   - Top management leadership and commitment
   - Information security policy establishment
   - Organizational roles and responsibilities
   - Management commitment demonstration
   
   Audit Activities:
   - Interview senior management
   - Review policy statements and communications
   - Verify resource allocation decisions
   - Assess management review processes
   ```

3. **Planning (Clause 6)**
   ```
   Examine:
   - Risk and opportunity management
   - Information security risk assessment
   - Information security risk treatment
   - Statement of Applicability (SoA) development
   
   Testing Procedures:
   - Review risk assessment methodology
   - Sample 20 identified risks and treatments
   - Verify SoA completeness and justification
   - Assess risk treatment plan implementation
   ```

4. **Support and Operation (Clauses 7-8)**
   ```
   Validate:
   - Resource provision and competency
   - Awareness and communication programs
   - Documented information management
   - Operational planning and control
   
   Control Testing:
   - Training records and competency assessments
   - Awareness program effectiveness measurement
   - Document control and version management
   - Operational procedure implementation
   ```

**Annex A Controls Implementation Testing:**

**A.5 Information Security Policies**
- [ ] Information security policy framework
- [ ] Information security in project management
- [ ] Inventory of assets
- [ ] Acceptable use of assets

**A.6 Organization of Information Security**
- [ ] Information security responsibilities
- [ ] Mobile device policy
- [ ] Teleworking arrangements
- [ ] Segregation of duties

**A.8 Asset Management**
- [ ] Asset inventory and ownership
- [ ] Information classification
- [ ] Media handling procedures
- [ ] Asset disposal and sanitization

**A.9 Access Control**
- [ ] Access control policy and procedures
- [ ] User access provisioning
- [ ] User access rights management
- [ ] System and application access control

**A.12 Operations Security**
- [ ] Operational procedures and responsibilities
- [ ] Protection from malware
- [ ] Information backup
- [ ] Event logging and monitoring

**A.13 Communications Security**
- [ ] Network security management
- [ ] Information transfer procedures
- [ ] Electronic messaging security
- [ ] Network access control

**A.14 System Acquisition, Development and Maintenance**
- [ ] Security requirements analysis
- [ ] Security in development processes
- [ ] Test data protection
- [ ] System security testing

**Performance Evaluation (Clause 9):**
- [ ] Monitoring and measurement procedures
- [ ] Internal audit program implementation
- [ ] Management review processes
- [ ] Performance evaluation results

**Improvement (Clause 10):**
- [ ] Nonconformity and corrective action
- [ ] Continual improvement processes
- [ ] ISMS effectiveness enhancement
- [ ] Lessons learned implementation

**Certification Decision Criteria:**
- ISMS fully implemented and effective
- All selected Annex A controls operational
- Evidence of continual improvement
- Management commitment demonstrated
```

### ISO 27001 Surveillance Audit Prompt
```
Perform ISO 27001 annual surveillance audit:

**Previous Audit:** [FINDINGS_AND_CORRECTIVE_ACTIONS_FROM_LAST_AUDIT]
**Audit Focus:** [RISK_BASED_AREAS_AND_CHANGES_SINCE_LAST_AUDIT]
**Surveillance Period:** [12_MONTH_PERIOD_SINCE_LAST_ASSESSMENT]

**Surveillance Audit Scope:**

1. **Management System Changes**
   ```
   Review:
   - Organizational changes and restructuring
   - ISMS scope modifications
   - Policy and procedure updates
   - New risks and opportunities identification
   
   Assessment Activities:
   - Document change control review
   - Impact assessment validation
   - Stakeholder communication verification
   - Change implementation effectiveness
   ```

2. **Corrective Action Follow-up**
   ```
   Verify:
   - Previous nonconformity closure
   - Corrective action effectiveness
   - Root cause elimination verification
   - Preventive measure implementation
   
   Evidence Collection:
   - Corrective action reports
   - Implementation verification records
   - Effectiveness monitoring results
   - Follow-up audit findings
   ```

3. **Internal Audit Program Effectiveness**
   ```
   Assess:
   - Internal audit schedule compliance
   - Auditor competence and independence
   - Audit finding quality and significance
   - Audit program improvement initiatives
   
   Review Activities:
   - Internal audit reports and schedules
   - Auditor training and certification records
   - Finding trend analysis
   - Audit program effectiveness metrics
   ```

4. **Management Review Process**
   ```
   Examine:
   - Management review meeting frequency
   - Input comprehensiveness and quality
   - Decision-making and action planning
   - Resource allocation and commitment
   
   Documentation Review:
   - Management review meeting minutes
   - Input data compilation and analysis
   - Decision documentation and tracking
   - Resource allocation records
   ```

**Risk-Based Control Sampling:**
- High-risk areas from previous assessments
- Areas with significant changes
- Controls with historical issues
- New or modified processes

**Performance Monitoring:**
- [ ] Security incident trend analysis
- [ ] Control effectiveness metrics
- [ ] Risk treatment progress monitoring
- [ ] Customer and stakeholder feedback

**Continual Improvement Evidence:**
- [ ] ISMS enhancement initiatives
- [ ] Process optimization activities
- [ ] Technology and methodology improvements
- [ ] Best practice adoption

**Surveillance Audit Outcomes:**
- Certificate maintenance recommendation
- Minor nonconformity identification
- Opportunities for improvement
- Positive practice recognition
```

## PCI DSS (Payment Card Industry Data Security Standard) Audit Prompts

### PCI DSS Compliance Assessment Prompt
```
Conduct comprehensive PCI DSS compliance assessment:

**Merchant Level:** [LEVEL_1_2_3_4_BASED_ON_TRANSACTION_VOLUME]
**Card Data Environment:** [SYSTEMS_STORING_PROCESSING_TRANSMITTING_CHD]
**Assessment Type:** [SELF_ASSESSMENT_SAQ_OR_EXTERNAL_QSA_ASSESSMENT]
**PCI DSS Version:** 4.0.1 (Current Standard)

**PCI DSS 12 Requirements Assessment:**

**Requirement 1: Install and Maintain Network Security Controls**
```
Validate:
- Firewall and router configuration standards
- Network security controls implementation
- Firewall rule review and approval process
- Network diagram accuracy and completeness

Testing Procedures:
- Review firewall rulesets and justifications
- Test unauthorized access prevention
- Verify network segmentation effectiveness
- Validate configuration change controls

Evidence Collection:
- Network architecture diagrams
- Firewall configuration files
- Change management documentation
- Penetration testing results
```

**Requirement 2: Apply Secure Configurations to All System Components**
```
Assess:
- System hardening standards and implementation
- Default security parameter changes
- Encryption protocols and configurations
- Vendor-supplied default removal

Control Testing:
- Configuration baseline validation
- Security configuration review
- Encryption strength verification (AES-256, TLS 1.2+)
- Default account and password elimination
```

**Requirement 3: Protect Stored Account Data**
```
Examine:
- Cardholder Data (CHD) storage minimization
- Data retention and disposal policies
- Strong cryptography implementation
- Key management procedures

Validation Activities:
- Data discovery and classification
- Encryption implementation testing
- Key storage and rotation verification
- Data disposal process validation
```

**Requirement 4: Protect Cardholder Data with Strong Cryptography During Transmission**
```
Test:
- Encryption for CHD transmission
- Wireless network security controls
- Public network protection measures
- Certificate management procedures

Security Testing:
- Network traffic analysis
- Wireless security configuration review
- SSL/TLS implementation validation
- Certificate expiration monitoring
```

**Requirement 5: Protect All Systems and Networks from Malicious Software**
```
Verify:
- Anti-malware software deployment
- Signature update procedures
- Periodic scanning processes
- Malware incident response procedures

Testing Approach:
- Anti-malware coverage validation
- Update mechanism verification
- Scan log review and analysis
- Incident response testing
```

**Requirement 6: Develop and Maintain Secure Systems and Software**
```
Assess:
- Vulnerability management processes
- Secure development lifecycle (SDLC)
- Code review and testing procedures
- Change management controls

Evaluation Activities:
- Vulnerability scan result review
- Code review process validation
- Security testing verification
- Change control effectiveness
```

**Requirement 7: Restrict Access to System Components and Cardholder Data by Business Need-to-Know**
```
Validate:
- Access control implementation
- Role-based access controls (RBAC)
- Least privilege principle application
- Access review procedures

Access Control Testing:
- User access matrix validation
- Privilege escalation prevention
- Access review documentation
- Segregation of duties verification
```

**Requirement 8: Identify Users and Authenticate Access to System Components**
```
Test:
- User identification procedures
- Authentication control implementation
- Password policy compliance
- Multi-factor authentication (MFA) deployment

Authentication Testing:
- Identity verification processes
- Password strength validation
- MFA implementation verification
- Session management controls
```

**Requirement 9: Restrict Physical Access to Cardholder Data**
```
Examine:
- Physical access control systems
- Visitor management procedures
- Media storage and destruction
- Device inventory and tracking

Physical Security Validation:
- Access control system testing
- Visitor log review
- Media handling procedure verification
- Device tracking accuracy
```

**Requirement 10: Log and Monitor All Access to System Components and Cardholder Data**
```
Verify:
- Logging mechanism implementation
- Log review and analysis procedures
- Security monitoring controls
- Incident detection capabilities

Logging Assessment:
- Log completeness and accuracy
- Review procedure effectiveness
- Monitoring system configuration
- Incident response integration
```

**Requirement 11: Test Security of Systems and Networks Regularly**
```
Assess:
- Vulnerability scanning procedures
- Penetration testing program
- Intrusion detection systems
- File integrity monitoring

Security Testing Validation:
- Scan frequency and coverage
- Penetration test methodology
- IDS/IPS effectiveness
- Change detection accuracy
```

**Requirement 12: Support Information Security with Organizational Policies and Programs**
```
Review:
- Information security policy framework
- Security awareness training programs
- Incident response procedures
- Risk assessment processes

Policy and Program Assessment:
- Policy completeness and currency
- Training effectiveness measurement
- Incident response testing
- Risk management maturity
```

**Compliance Validation:**
- [ ] All applicable requirements validated as "In Place"
- [ ] Compensating controls properly documented
- [ ] Quarterly network scanning completed
- [ ] Annual penetration testing conducted

**Assessment Deliverables:**
- Report on Compliance (ROC) or Self-Assessment Questionnaire (SAQ)
- Attestation of Compliance (AOC)
- Network segmentation validation
- Compensating control documentation

**Ongoing Compliance Requirements:**
- Quarterly vulnerability scans
- Annual assessments
- Incident response and forensics
- Regular security testing and monitoring
```

## Integrated SaaS/IT Compliance Dashboard Prompt
```
Create comprehensive SaaS/IT compliance dashboard:

**Multi-Framework Assessment:** SOC 2 + ISO 27001 + PCI DSS
**Platform Coverage:** [COMPLETE_SAAS_INFRASTRUCTURE_AND_APPLICATIONS]
**Assessment Frequency:** Continuous monitoring with annual certifications

**Unified Compliance Scorecard:**

1. **SOC 2 Trust Services Criteria Status**
   ```
   Security: [SCORE]/100 - [STATUS: PASS/FAIL/QUALIFIED]
   Availability: [SCORE]/100 - [STATUS: PASS/FAIL/QUALIFIED]
   Processing Integrity: [SCORE]/100 - [STATUS: PASS/FAIL/QUALIFIED]
   Confidentiality: [SCORE]/100 - [STATUS: PASS/FAIL/QUALIFIED]
   Privacy: [SCORE]/100 - [STATUS: PASS/FAIL/QUALIFIED]
   ```

2. **ISO 27001 ISMS Maturity Assessment**
   ```
   Context and Leadership: [MATURITY_LEVEL_1_5]
   Planning and Risk Management: [MATURITY_LEVEL_1_5]
   Support and Operation: [MATURITY_LEVEL_1_5]
   Performance and Improvement: [MATURITY_LEVEL_1_5]
   Annex A Controls: [IMPLEMENTED_CONTROLS]/[APPLICABLE_CONTROLS]
   ```

3. **PCI DSS Compliance Status**
   ```
   Network Security: [COMPLIANT/NON_COMPLIANT]
   Vulnerability Management: [COMPLIANT/NON_COMPLIANT]
   Access Control: [COMPLIANT/NON_COMPLIANT]
   Monitoring and Testing: [COMPLIANT/NON_COMPLIANT]
   Policy and Procedures: [COMPLIANT/NON_COMPLIANT]
   ```

**Cross-Framework Control Mapping:**
- Access control alignment across all frameworks
- Encryption standard harmonization
- Incident response process integration
- Risk management methodology consistency

**Continuous Monitoring Integration:**
- Real-time security control effectiveness
- Automated compliance status reporting
- Exception and deviation tracking
- Remediation progress monitoring

**Stakeholder Communication:**
- Executive summary for board reporting
- Technical details for IT and security teams
- Audit findings for compliance officers
- Customer assurance for sales and marketing

**Certification Maintenance Schedule:**
- SOC 2 Type II: Annual examination
- ISO 27001: 3-year cycle with annual surveillance
- PCI DSS: Annual assessment with quarterly scans
- Integrated compliance review: Semi-annual assessment

**Risk-Based Prioritization:**
- Critical: Certificate suspension/revocation risk
- High: Customer contract compliance requirements
- Medium: Process improvement opportunities
- Low: Documentation and training enhancements
```