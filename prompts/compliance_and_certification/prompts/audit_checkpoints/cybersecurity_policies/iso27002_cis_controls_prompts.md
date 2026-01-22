# ISO 27002 Security Controls and CIS Critical Security Controls Audit Checkpoint Prompts

## Overview
This collection provides comprehensive audit checkpoint prompts for ISO/IEC 27002:2022 security controls implementation and CIS Critical Security Controls v8.1 assessment and optimization.

---

## ISO 27002:2022 Security Controls Assessment

### Organizational Controls (5.1-5.37)

#### 5.1-5.8: Information Security Policies and Organization
```
PROMPT: Assess organizational information security policies and management structure.

Evaluate organizational security controls per ISO 27002:2022 requirements:

INFORMATION SECURITY POLICIES (5.1-5.2):
• Policy Framework: Do we have comprehensive information security policies covering all business activities?
• Policy Content: Are policies current, comprehensive, and aligned with business objectives and legal requirements?
• Policy Communication: Are policies effectively communicated to all relevant personnel and stakeholders?
• Policy Review: Are policies reviewed regularly and updated as needed?
• Policy Compliance: How effectively is policy compliance monitored and enforced?

INFORMATION SECURITY ROLES AND RESPONSIBILITIES (5.3-5.4):
• Role Definition: Are information security roles and responsibilities clearly defined and documented?
• Segregation of Duties: Are conflicting duties appropriately segregated to reduce risk of unauthorized activities?
• Management Responsibilities: Are management responsibilities for information security clearly established?
• Authority Assignment: Is appropriate authority assigned for information security decision-making?

CONTACT WITH AUTHORITIES AND SPECIAL INTEREST GROUPS (5.5-5.6):
• Authority Contacts: Do we maintain appropriate contacts with law enforcement and regulatory authorities?
• Professional Groups: Are we engaged with relevant information security professional groups and forums?
• Threat Intelligence: Do we receive and act on threat intelligence from appropriate sources?
• Incident Reporting: Are processes established for reporting incidents to relevant authorities?

PROJECT MANAGEMENT AND SUPPLIER RELATIONSHIPS (5.7-5.8):
• Project Security: Is information security integrated into project management methodologies?
• Supplier Security: Are information security requirements established for supplier relationships?
• Supply Chain Security: Do we assess and manage supply chain security risks?
• Contract Security: Are appropriate security requirements included in supplier contracts?

OUTPUT REQUIREMENTS:
1. Organizational control implementation assessment
2. Policy framework effectiveness evaluation
3. Role and responsibility clarity analysis
4. Authority and supplier relationship security assessment
5. Specific recommendations for organizational control improvement

Include policy gap analysis and recommended organizational security framework enhancements.
```

#### 5.9-5.16: Human Resource Security
```
PROMPT: Evaluate human resource security controls and personnel security management.

Assess human resource security per ISO 27002:2022 controls:

PERSONNEL SCREENING AND TERMS OF EMPLOYMENT (5.9-5.11):
• Background Verification: Are background checks conducted appropriately based on role sensitivity and legal requirements?
• Terms and Conditions: Are information security terms and conditions clearly established in employment agreements?
• Disciplinary Process: Is there a formal disciplinary process for information security violations?
• Security Awareness: Are personnel aware of their information security responsibilities?

INFORMATION SECURITY AWARENESS AND TRAINING (5.12-5.13):
• Awareness Program: Do we have a comprehensive information security awareness program?
• Training Effectiveness: Is security training effective and regularly updated?
• Specialized Training: Do personnel with specific security responsibilities receive appropriate specialized training?
• Training Records: Are training records maintained and training compliance monitored?

EMPLOYMENT TERMINATION (5.14-5.16):
• Termination Process: Is there a formal process for terminating access rights when employment ends?
• Asset Return: Are processes in place to ensure return of organizational assets?
• Access Removal: Are access rights removed promptly and completely upon termination?
• Ongoing Obligations: Are ongoing security obligations clearly communicated to former employees?

OUTPUT REQUIREMENTS:
1. Human resource security control maturity assessment
2. Personnel screening effectiveness evaluation
3. Security awareness program quality analysis
4. Employment termination process security assessment
5. Specific recommendations for human resource security improvement

Provide human resource security policy recommendations and implementation procedures.
```

#### 5.17-5.23: Physical and Environmental Security
```
PROMPT: Assess physical and environmental security controls implementation.

Evaluate physical security per ISO 27002:2022 requirements:

SECURE AREAS AND PHYSICAL ENTRY (5.17-5.20):
• Physical Security Perimeter: Are physical security perimeters established and maintained appropriately?
• Physical Entry Controls: Are entry controls implemented to restrict access to secure areas?
• Protection from Environmental Threats: Are facilities protected from environmental threats (fire, flood, earthquake)?
• Working in Secure Areas: Are appropriate procedures established for working in secure areas?

EQUIPMENT PROTECTION (5.21-5.23):
• Equipment Siting: Is equipment appropriately sited and protected from environmental and security threats?
• Equipment Maintenance: Are equipment maintenance procedures established to ensure continued security?
• Equipment Disposal: Are secure disposal procedures implemented for equipment containing sensitive information?
• Supporting Utilities: Are supporting utilities (power, telecommunications) appropriately protected?

CLEAR DESK AND CLEAR SCREEN POLICIES:
• Clear Desk Policy: Is a clear desk policy implemented and enforced?
• Clear Screen Policy: Are computer screens protected from unauthorized viewing?
• Sensitive Information Handling: Are procedures established for handling sensitive information in work areas?
• Visitor Management: Are visitor access and activities appropriately controlled and monitored?

OUTPUT REQUIREMENTS:
1. Physical security control implementation assessment
2. Environmental protection effectiveness evaluation
3. Equipment security management analysis
4. Physical access control security assessment
5. Specific recommendations for physical security enhancement

Include physical security risk assessment and recommended protection measures.
```

### Technological Controls (8.1-8.34)

#### 8.1-8.8: Access Management
```
PROMPT: Evaluate technological access management controls implementation.

Assess access management per ISO 27002:2022 technological controls:

USER ACCESS MANAGEMENT (8.1-8.4):
• Access Control Policy: Is there a comprehensive access control policy covering all systems and applications?
• User Registration: Are user registration and deregistration processes effectively managed?
• Privileged Access Management: How effectively are privileged access rights managed and monitored?
• User Access Review: Are user access rights reviewed regularly and systematically?

AUTHENTICATION AND ACCESS CONTROL (8.5-8.8):
• Authentication Methods: Are appropriate authentication methods implemented based on risk and context?
• Multi-Factor Authentication: Is MFA implemented appropriately for sensitive systems and privileged access?
• Password Management: Are strong password policies implemented and enforced?
• Access Control Systems: Are access control systems properly configured and maintained?

SYSTEM AND APPLICATION ACCESS CONTROL:
• System Access Controls: Are appropriate access controls implemented at the system level?
• Application Access Controls: Are application-level access controls properly implemented?
• Session Management: Are user sessions managed securely with appropriate timeouts and controls?
• Remote Access: Are remote access connections secured appropriately?

OUTPUT REQUIREMENTS:
1. Access management control implementation assessment
2. Authentication method effectiveness evaluation
3. Access control system security analysis
4. User access management process assessment
5. Specific recommendations for access management improvement

Provide access control policy recommendations and implementation guidelines.
```

#### 8.9-8.16: Cryptography and System Security
```
PROMPT: Assess cryptographic controls and system security implementation.

Evaluate cryptography and system security per ISO 27002:2022:

CRYPTOGRAPHIC CONTROLS (8.9-8.12):
• Cryptographic Policy: Is there a comprehensive policy for the use of cryptographic controls?
• Key Management: Are cryptographic keys managed securely throughout their lifecycle?
• Encryption Implementation: Is encryption appropriately implemented for data at rest and in transit?
• Digital Signatures: Are digital signatures used appropriately for authentication and non-repudiation?

SYSTEM SECURITY (8.13-8.16):
• System Documentation: Are systems documented with appropriate security configuration information?
• Secure Installation: Are systems installed and configured securely?
• Security Testing: Are systems security tested before implementation and regularly thereafter?
• Change Management: Are system changes managed securely with appropriate testing and approval?

SECURITY ARCHITECTURE AND CONFIGURATION:
• Security Architecture: Is there a defined security architecture for technological systems?
• Secure Configuration: Are systems configured according to security baselines and standards?
• Vulnerability Management: Are system vulnerabilities identified and remediated systematically?
• Security Monitoring: Are systems monitored continuously for security events and anomalies?

OUTPUT REQUIREMENTS:
1. Cryptographic control implementation assessment
2. System security configuration evaluation
3. Security architecture maturity analysis
4. Vulnerability management effectiveness assessment
5. Specific recommendations for cryptography and system security improvement

Include cryptographic policy recommendations and secure system configuration guidelines.
```

### Incident Management Controls (8.25-8.28)

#### Incident Response and Management
```
PROMPT: Evaluate incident management controls per ISO 27002:2022 requirements.

Assess incident management capabilities:

INCIDENT RESPONSE PLANNING (8.25-8.26):
• Incident Response Policy: Is there a comprehensive incident response policy and procedures?
• Incident Classification: Are incidents classified appropriately based on severity and impact?
• Response Team: Is an incident response team established with defined roles and responsibilities?
• Response Procedures: Are response procedures documented, tested, and regularly updated?

INCIDENT DETECTION AND REPORTING (8.27-8.28):
• Incident Detection: Are incidents detected promptly through monitoring and reporting mechanisms?
• Incident Reporting: Are incident reporting procedures established and communicated?
• Evidence Collection: Are evidence collection procedures established for incident investigation?
• Incident Analysis: Are incidents analyzed to identify root causes and prevent recurrence?

INCIDENT RECOVERY AND LESSONS LEARNED:
• Recovery Procedures: Are recovery procedures established to restore normal operations?
• Communication: Are communication procedures established for incident response activities?
• Lessons Learned: Are lessons learned captured and used to improve incident response capabilities?
• Legal Requirements: Are legal and regulatory reporting requirements addressed?

OUTPUT REQUIREMENTS:
1. Incident management control implementation assessment
2. Incident response capability maturity evaluation
3. Incident detection and reporting effectiveness analysis
4. Recovery and improvement process assessment
5. Specific recommendations for incident management enhancement

Provide incident response procedures and recommended incident management framework.
```

---

## CIS Critical Security Controls v8 Assessment

### Basic CIS Controls (CIS 1-6)

#### CIS Control 1: Inventory and Control of Enterprise Assets
```
PROMPT: Assess enterprise asset inventory and control implementation per CIS Control 1.

Evaluate asset management against CIS Control 1 requirements:

ASSET DISCOVERY AND INVENTORY (CIS 1.1-1.3):
• Automated Asset Discovery: Do we use automated tools to discover enterprise assets on our networks?
• Asset Inventory Maintenance: Is an up-to-date inventory of enterprise assets maintained?
• Asset Classification: Are assets classified based on criticality and business function?
• Unauthorized Asset Detection: Can we detect unauthorized assets connecting to our networks?

ASSET MANAGEMENT PROCESSES (CIS 1.4-1.5):
• Asset Management Software: Do we use dedicated asset management software for tracking?
• Asset Information Standards: Are asset information standards established (naming, tagging, documentation)?
• Asset Lifecycle Management: Are assets managed throughout their lifecycle from acquisition to disposal?
• Asset Owner Assignment: Is ownership assigned for all enterprise assets?

IMPLEMENTATION GROUPS ASSESSMENT:
• IG1 (Basic): Active inventory of physical devices and systems
• IG2 (Foundational): Automated asset discovery and inventory management
• IG3 (Advanced): Integration with configuration management and security tools

OUTPUT REQUIREMENTS:
1. Asset inventory completeness assessment (percentage of assets inventoried)
2. Asset discovery automation maturity evaluation
3. Asset management process effectiveness analysis
4. Implementation Group compliance assessment
5. Specific recommendations for CIS Control 1 improvement

Provide asset inventory gap analysis and recommended asset management procedures.
```

#### CIS Control 2: Inventory and Control of Software Assets
```
PROMPT: Evaluate software asset inventory and control per CIS Control 2 requirements.

Assess software inventory management against CIS Control 2:

SOFTWARE INVENTORY MANAGEMENT (CIS 2.1-2.3):
• Authorized Software Inventory: Do we maintain an inventory of authorized software and versions?
• Software Installation Control: Are software installations controlled and authorized?
• Unauthorized Software Detection: Can we detect unauthorized software on enterprise assets?
• Software Inventory Automation: Do we use automated tools for software inventory management?

SOFTWARE SECURITY MANAGEMENT (CIS 2.4-2.7):
• Application Allowlisting: Is application allowlisting implemented where appropriate?
• Software Restriction Policies: Are software restriction policies implemented and enforced?
• Software Update Management: Are software updates managed systematically across the enterprise?
• Third-Party Software Management: How do we manage third-party software security risks?

IMPLEMENTATION GROUPS ASSESSMENT:
• IG1 (Basic): Authorized and unauthorized software identification
• IG2 (Foundational): Automated software inventory and allowlisting
• IG3 (Advanced): Comprehensive software security management

OUTPUT REQUIREMENTS:
1. Software inventory completeness assessment
2. Software control implementation effectiveness evaluation
3. Unauthorized software detection capability analysis
4. Software security management maturity assessment
5. Specific recommendations for CIS Control 2 enhancement

Include software inventory procedures and recommended software control policies.
```

#### CIS Control 3: Data Protection
```
PROMPT: Assess data protection implementation per CIS Control 3 requirements.

Evaluate data protection against CIS Control 3 standards:

DATA IDENTIFICATION AND CLASSIFICATION (CIS 3.1-3.3):
• Data Classification Scheme: Is a data classification scheme established and implemented?
• Sensitive Data Inventory: Do we maintain an inventory of sensitive data and its locations?
• Data Flow Mapping: Are data flows documented and analyzed for security risks?
• Data Retention Policies: Are data retention and disposal policies implemented?

DATA PROTECTION CONTROLS (CIS 3.4-3.7):
• Data Encryption: Is sensitive data encrypted appropriately at rest and in transit?
• Data Loss Prevention: Are DLP controls implemented to prevent unauthorized data disclosure?
• Data Backup Security: Are data backups secured and tested regularly?
• Data Sharing Controls: Are controls implemented for secure data sharing with third parties?

IMPLEMENTATION GROUPS ASSESSMENT:
• IG1 (Basic): Basic data protection and retention
• IG2 (Foundational): Comprehensive data classification and protection
• IG3 (Advanced): Advanced data protection and monitoring

OUTPUT REQUIREMENTS:
1. Data classification implementation assessment
2. Data protection control effectiveness evaluation
3. Data loss prevention capability analysis
4. Data backup and recovery security assessment
5. Specific recommendations for CIS Control 3 improvement

Provide data classification procedures and recommended data protection controls.
```

#### CIS Control 4: Secure Configuration of Enterprise Assets and Software
```
PROMPT: Evaluate secure configuration management per CIS Control 4 requirements.

Assess configuration management against CIS Control 4:

CONFIGURATION STANDARDS (CIS 4.1-4.3):
• Security Configuration Standards: Are security configuration standards established for all asset types?
• Configuration Baselines: Are secure configuration baselines documented and maintained?
• Configuration Implementation: Are secure configurations implemented consistently across the enterprise?
• Configuration Documentation: Is configuration information properly documented and maintained?

CONFIGURATION MANAGEMENT PROCESSES (CIS 4.4-4.7):
• Configuration Change Control: Are configuration changes controlled and authorized?
• Configuration Monitoring: Are configuration changes monitored and unauthorized changes detected?
• Configuration Validation: Are configurations validated against security baselines regularly?
• Default Configuration Security: Are default configurations hardened before deployment?

IMPLEMENTATION GROUPS ASSESSMENT:
• IG1 (Basic): Basic secure configuration implementation
• IG2 (Foundational): Automated configuration management and monitoring
• IG3 (Advanced): Comprehensive configuration security management

OUTPUT REQUIREMENTS:
1. Configuration standards completeness assessment
2. Configuration management process maturity evaluation
3. Configuration compliance monitoring effectiveness analysis
4. Configuration change control assessment
5. Specific recommendations for CIS Control 4 enhancement

Include secure configuration baselines and recommended configuration management procedures.
```

#### CIS Control 5: Account Management
```
PROMPT: Assess account management implementation per CIS Control 5 requirements.

Evaluate account management against CIS Control 5 standards:

ACCOUNT LIFECYCLE MANAGEMENT (CIS 5.1-5.3):
• Account Inventory: Do we maintain an inventory of all accounts across enterprise systems?
• Account Provisioning: Are account provisioning processes standardized and controlled?
• Account Deprovisioning: Are accounts deprovisioned promptly when no longer needed?
• Shared Account Management: Are shared accounts minimized and properly managed?

PRIVILEGED ACCOUNT MANAGEMENT (CIS 5.4-5.6):
• Privileged Account Inventory: Are privileged accounts identified and inventoried?
• Privileged Access Control: Are privileged access rights controlled and monitored?
• Privileged Session Management: Are privileged sessions monitored and recorded?
• Multi-Factor Authentication: Is MFA required for privileged accounts?

IMPLEMENTATION GROUPS ASSESSMENT:
• IG1 (Basic): Basic account management and privileged account controls
• IG2 (Foundational): Automated account management and monitoring
• IG3 (Advanced): Comprehensive privileged access management

OUTPUT REQUIREMENTS:
1. Account management process maturity assessment
2. Privileged account security evaluation
3. Account lifecycle management effectiveness analysis
4. MFA implementation coverage assessment
5. Specific recommendations for CIS Control 5 improvement

Provide account management procedures and privileged access management recommendations.
```

#### CIS Control 6: Access Control Management
```
PROMPT: Evaluate access control management per CIS Control 6 requirements.

Assess access control implementation against CIS Control 6:

ACCESS CONTROL POLICIES (CIS 6.1-6.3):
• Access Control Standards: Are access control standards established and documented?
• Least Privilege Implementation: Is the principle of least privilege consistently implemented?
• Access Review Processes: Are access rights reviewed regularly and systematically?
• Role-Based Access Control: Is RBAC or similar access control models implemented appropriately?

ACCESS CONTROL IMPLEMENTATION (CIS 6.4-6.8):
• Network Access Control: Are network access controls implemented appropriately?
• Remote Access Security: Is remote access secured with appropriate controls?
• Access Control Monitoring: Are access events monitored and analyzed for anomalies?
• Access Control Automation: Are access control processes automated where appropriate?

IMPLEMENTATION GROUPS ASSESSMENT:
• IG1 (Basic): Basic access control implementation
• IG2 (Foundational): Systematic access control management
• IG3 (Advanced): Advanced access control analytics and automation

OUTPUT REQUIREMENTS:
1. Access control policy implementation assessment
2. Least privilege implementation effectiveness evaluation
3. Access review process maturity analysis
4. Access control monitoring capability assessment
5. Specific recommendations for CIS Control 6 enhancement

Include access control policy recommendations and implementation procedures.
```

### Foundational CIS Controls (CIS 7-16)

#### CIS Control 7: Continuous Vulnerability Management
```
PROMPT: Assess continuous vulnerability management per CIS Control 7 requirements.

Evaluate vulnerability management against CIS Control 7:

VULNERABILITY ASSESSMENT PROCESSES (CIS 7.1-7.4):
• Vulnerability Scanning: Are vulnerability scans conducted regularly and comprehensively?
• Scan Coverage: Do vulnerability scans cover all enterprise assets and applications?
• Automated Scanning: Are vulnerability scans automated and scheduled appropriately?
• Scan Results Management: Are vulnerability scan results managed and tracked systematically?

VULNERABILITY REMEDIATION (CIS 7.5-7.7):
• Remediation Prioritization: Are vulnerabilities prioritized based on risk and exploitability?
• Remediation Timelines: Are remediation timelines established and met based on vulnerability severity?
• Patch Management: Is patch management integrated with vulnerability management processes?
• Remediation Verification: Is vulnerability remediation verified through follow-up scanning?

IMPLEMENTATION GROUPS ASSESSMENT:
• IG1 (Basic): Basic vulnerability scanning and remediation
• IG2 (Foundational): Comprehensive vulnerability management program
• IG3 (Advanced): Advanced vulnerability analytics and threat-based prioritization

OUTPUT REQUIREMENTS:
1. Vulnerability management program maturity assessment
2. Vulnerability scanning coverage evaluation
3. Remediation effectiveness analysis
4. Patch management integration assessment
5. Specific recommendations for CIS Control 7 improvement

Provide vulnerability management procedures and recommended remediation workflows.
```

#### CIS Control 8: Audit Log Management
```
PROMPT: Evaluate audit log management implementation per CIS Control 8 requirements.

Assess logging and monitoring against CIS Control 8:

LOG COLLECTION AND MANAGEMENT (CIS 8.1-8.4):
• Log Collection Standards: Are logging standards established for all enterprise systems?
• Centralized Log Management: Are logs collected centrally for analysis and retention?
• Log Retention: Are log retention policies established and implemented appropriately?
• Log Protection: Are audit logs protected from unauthorized access and modification?

LOG ANALYSIS AND MONITORING (CIS 8.5-8.8):
• Log Monitoring: Are logs monitored continuously for security events?
• Automated Analysis: Are automated tools used for log analysis and correlation?
• Alerting: Are alerting mechanisms established for critical security events?
• Log Review: Are logs reviewed regularly for security incidents and anomalies?

IMPLEMENTATION GROUPS ASSESSMENT:
• IG1 (Basic): Basic audit logging and retention
• IG2 (Foundational): Centralized log management and monitoring
• IG3 (Advanced): Advanced log analytics and correlation

OUTPUT REQUIREMENTS:
1. Audit log management program assessment
2. Log collection completeness evaluation
3. Log analysis and monitoring effectiveness analysis
4. Log retention and protection assessment
5. Specific recommendations for CIS Control 8 enhancement

Include logging standards and recommended log management procedures.
```

### Organizational CIS Controls (CIS 17-18)

#### CIS Control 17: Incident Response Management
```
PROMPT: Assess incident response management per CIS Control 17 requirements.

Evaluate incident response against CIS Control 17:

INCIDENT RESPONSE PLANNING (CIS 17.1-17.4):
• Incident Response Plan: Is a comprehensive incident response plan established and maintained?
• Response Team: Is an incident response team established with defined roles and responsibilities?
• Response Procedures: Are incident response procedures documented and tested regularly?
• Communication Plans: Are communication plans established for incident response activities?

INCIDENT RESPONSE IMPLEMENTATION (CIS 17.5-17.9):
• Incident Detection: Are incidents detected promptly through monitoring and reporting?
• Incident Analysis: Are incidents analyzed systematically to determine scope and impact?
• Incident Containment: Are containment procedures implemented effectively?
• Incident Recovery: Are recovery procedures established and tested?
• Lessons Learned: Are lessons learned captured and used for improvement?

IMPLEMENTATION GROUPS ASSESSMENT:
• IG1 (Basic): Basic incident response capabilities
• IG2 (Foundational): Comprehensive incident response program
• IG3 (Advanced): Advanced incident response with threat intelligence integration

OUTPUT REQUIREMENTS:
1. Incident response program maturity assessment
2. Response capability effectiveness evaluation
3. Incident response testing and training analysis
4. Recovery and lessons learned process assessment
5. Specific recommendations for CIS Control 17 improvement

Provide incident response procedures and recommended response framework.
```

#### CIS Control 18: Penetration Testing
```
PROMPT: Evaluate penetration testing program per CIS Control 18 requirements.

Assess penetration testing against CIS Control 18:

PENETRATION TESTING PROGRAM (CIS 18.1-18.3):
• Testing Strategy: Is a penetration testing strategy established based on risk assessment?
• Testing Scope: Does penetration testing cover all critical systems and applications?
• Testing Frequency: Is penetration testing conducted with appropriate frequency?
• Testing Methodology: Are established penetration testing methodologies used?

TESTING IMPLEMENTATION AND RESULTS (CIS 18.4-18.5):
• Qualified Testers: Are penetration tests conducted by qualified internal or external testers?
• Results Management: Are penetration testing results managed and tracked systematically?
• Remediation Verification: Is remediation of identified vulnerabilities verified through retesting?
• Red Team Exercises: Are red team exercises conducted to test detection and response capabilities?

IMPLEMENTATION GROUPS ASSESSMENT:
• IG1 (Basic): Not applicable (IG1 does not include penetration testing)
• IG2 (Foundational): Basic penetration testing program
• IG3 (Advanced): Comprehensive penetration testing with red team exercises

OUTPUT REQUIREMENTS:
1. Penetration testing program maturity assessment
2. Testing coverage and methodology evaluation
3. Results management and remediation analysis
4. Red team exercise effectiveness assessment
5. Specific recommendations for CIS Control 18 improvement

Include penetration testing procedures and recommended testing framework.
```

---

## Implementation Groups Assessment

### Overall CIS Implementation Group Compliance
```
PROMPT: Assess overall CIS Controls implementation group compliance and maturity.

Evaluate CIS Implementation Group compliance across all controls:

IMPLEMENTATION GROUP 1 (IG1) - BASIC CYBER HYGIENE:
• Control Coverage: Are all IG1 controls implemented for basic cyber hygiene?
• Resource Requirements: Are appropriate resources allocated for IG1 implementation?
• Process Maturity: What is the maturity level of IG1 control implementation?
• Compliance Gaps: What gaps exist in IG1 compliance?

IMPLEMENTATION GROUP 2 (IG2) - FOUNDATIONAL CONTROLS:
• Advanced Controls: Are IG2 foundational controls implemented beyond basic hygiene?
• Automation Level: What level of automation is achieved in IG2 controls?
• Integration Effectiveness: How well are IG2 controls integrated across the enterprise?
• Risk Management: How effectively do IG2 controls support risk management objectives?

IMPLEMENTATION GROUP 3 (IG3) - ADVANCED CONTROLS:
• Sophisticated Threats: Are IG3 controls effectively addressing sophisticated threat scenarios?
• Advanced Analytics: Are advanced analytics and threat intelligence integrated into IG3 controls?
• Continuous Improvement: Is there systematic continuous improvement of IG3 control effectiveness?
• Industry Leadership: Does IG3 implementation represent industry leading practices?

OUTPUT REQUIREMENTS:
1. Overall CIS IG compliance assessment (IG1/IG2/IG3)
2. Control implementation maturity evaluation
3. Resource allocation and capability analysis
4. Implementation roadmap for advancing to higher IGs
5. Specific recommendations for CIS Implementation Group advancement

Provide detailed compliance matrix and recommended implementation priorities for each IG level.
```

---

## Integration Requirements
These prompts should be used in conjunction with:
- NIST CSF prompts for framework alignment
- Compliance framework prompts for regulatory integration
- Industry-specific security requirements
- Organizational risk management processes

---

## Usage Guidelines
1. **Risk-Based Implementation**: Prioritize controls based on organizational risk assessment
2. **Maturity-Based Approach**: Progress through Implementation Groups systematically
3. **Continuous Monitoring**: Implement ongoing monitoring for all assessed controls
4. **Documentation Standards**: Maintain comprehensive documentation per ISO and CIS standards
5. **Integration Planning**: Ensure integration with existing security and compliance programs

---

## Output Standards
All assessments should provide:
- Current implementation status for each control
- Maturity ratings and gap analysis
- Risk-prioritized recommendations
- Implementation roadmaps with resource requirements
- Success metrics and monitoring procedures
- Integration points with other frameworks and standards