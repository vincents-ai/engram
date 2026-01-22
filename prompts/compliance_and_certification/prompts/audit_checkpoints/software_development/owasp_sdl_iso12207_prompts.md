# Software Development Framework Audit Checkpoint Prompts

## OWASP (Open Web Application Security Project) Audit Prompts

### OWASP Top 10 Security Assessment Prompt
```
Conduct comprehensive OWASP Top 10 web application security audit:

**Application Portfolio:** [WEB_APPLICATIONS_APIS_MOBILE_APPS_SERVICES]
**Technology Stack:** [LANGUAGES_FRAMEWORKS_DATABASES_INFRASTRUCTURE]
**Environment Scope:** [DEVELOPMENT_STAGING_PRODUCTION_ENVIRONMENTS]
**Assessment Methodology:** [AUTOMATED_MANUAL_PENETRATION_TESTING]

**OWASP Top 10 2021 Security Risk Assessment:**

1. **A01:2021 - Broken Access Control**
   ```
   Assess Access Control Implementation:
   Vulnerability Categories:
   - Violation of least privilege principle
   - Bypassing access control checks
   - Privilege escalation attacks
   - Metadata manipulation and tampering
   - CORS misconfiguration
   - Force browsing to authenticated pages
   
   Testing Procedures:
   - Role-based access control (RBAC) validation
   - Horizontal privilege escalation testing
   - Vertical privilege escalation assessment
   - Path traversal and directory listing attempts
   - API endpoint authorization testing
   - Session management and token validation
   
   Prevention Validation:
   - Default deny access control implementation
   - Centralized access control mechanism
   - Secure coding practice verification
   - Access control unit test coverage
   - Logging and monitoring of access failures
   ```

2. **A02:2021 - Cryptographic Failures**
   ```
   Evaluate Cryptographic Implementation:
   Data Protection Assessment:
   - Data classification and sensitivity analysis
   - Encryption at rest implementation
   - Encryption in transit validation
   - Key management and rotation procedures
   
   Cryptographic Standards:
   - Strong encryption algorithms (AES-256, RSA-2048+)
   - Secure hashing functions (SHA-256+, bcrypt)
   - Proper salt and IV generation
   - Certificate validation and pinning
   
   Common Vulnerabilities:
   - Weak or deprecated algorithms
   - Hardcoded cryptographic keys
   - Insufficient entropy in random generation
   - Missing or weak certificate validation
   - Plaintext storage of sensitive data
   
   Testing Framework:
   - Static code analysis for crypto issues
   - Dynamic testing of encryption endpoints
   - Certificate chain validation testing
   - Key storage and rotation verification
   ```

3. **A03:2021 - Injection**
   ```
   Test Injection Vulnerability Prevention:
   Injection Types Assessment:
   - SQL injection (SQLi)
   - NoSQL injection
   - OS command injection
   - LDAP injection
   - XPath injection
   - Expression language injection
   
   Input Validation Testing:
   - Parameterized query usage verification
   - Input sanitization and encoding validation
   - Whitelist input validation implementation
   - Output encoding and escaping procedures
   
   Prevention Mechanism Validation:
   - Prepared statement implementation
   - Stored procedure security assessment
   - ORM framework security configuration
   - Input validation library usage
   - SQL query construction review
   
   Automated Testing:
   - SAST tool integration and results
   - DAST scanner injection testing
   - Manual penetration testing validation
   - Fuzzing and boundary value testing
   ```

4. **A04:2021 - Insecure Design**
   ```
   Assess Secure Design Implementation:
   Design Security Evaluation:
   - Threat modeling process implementation
   - Security architecture review
   - Design pattern security assessment
   - Business logic vulnerability identification
   
   Secure Development Lifecycle:
   - Security requirements definition
   - Security design review process
   - Secure coding guidelines implementation
   - Security testing integration
   
   Common Design Flaws:
   - Missing or inadequate threat modeling
   - Insecure reference architecture
   - Lack of security controls in design
   - Insufficient attack surface analysis
   
   Prevention Validation:
   - Security champion program implementation
   - Design review checklist usage
   - Security training and awareness
   - Secure development methodology adoption
   ```

5. **A05:2021 - Security Misconfiguration**
   ```
   Verify Security Configuration Management:
   Configuration Assessment:
   - Default credential usage elimination
   - Unnecessary feature and service disabling
   - Security header implementation
   - Error handling and information disclosure
   
   Infrastructure Security:
   - Server and framework hardening
   - Database security configuration
   - Cloud service security settings
   - Network security configuration
   
   Deployment Security:
   - Secure deployment pipeline
   - Configuration management automation
   - Environment-specific security settings
   - Regular security update procedures
   
   Monitoring and Maintenance:
   - Configuration drift detection
   - Security baseline compliance
   - Automated security scanning
   - Vulnerability management process
   ```

6. **A06:2021 - Vulnerable and Outdated Components**
   ```
   Assess Component Security Management:
   Inventory Management:
   - Complete component inventory maintenance
   - Dependency mapping and tracking
   - License compliance and security assessment
   - End-of-life component identification
   
   Vulnerability Management:
   - Regular vulnerability scanning
   - Security advisory monitoring
   - Patch management process
   - Risk assessment and prioritization
   
   Supply Chain Security:
   - Vendor security assessment
   - Source code repository security
   - Component integrity verification
   - Third-party library evaluation
   
   Automated Tools Integration:
   - Software composition analysis (SCA)
   - Dependency checking automation
   - Continuous monitoring implementation
   - Alert and notification systems
   ```

7. **A07:2021 - Identification and Authentication Failures**
   ```
   Test Authentication and Session Management:
   Authentication Security:
   - Strong password policy enforcement
   - Multi-factor authentication implementation
   - Account lockout and rate limiting
   - Credential recovery security
   
   Session Management:
   - Secure session token generation
   - Session timeout implementation
   - Session invalidation procedures
   - Cross-site request forgery protection
   
   Common Vulnerabilities:
   - Weak password requirements
   - Session fixation attacks
   - Credential stuffing vulnerabilities
   - Authentication bypass attempts
   
   Testing Procedures:
   - Brute force attack testing
   - Session management validation
   - Password policy compliance
   - Authentication mechanism assessment
   ```

8. **A08:2021 - Software and Data Integrity Failures**
   ```
   Verify Software and Data Integrity:
   CI/CD Pipeline Security:
   - Build process integrity verification
   - Code signing and verification
   - Artifact repository security
   - Deployment pipeline protection
   
   Update Mechanism Security:
   - Secure update delivery
   - Update verification procedures
   - Rollback capability validation
   - Update notification security
   
   Data Integrity Protection:
   - Data validation and verification
   - Integrity checking mechanisms
   - Tamper detection implementation
   - Audit trail maintenance
   
   Supply Chain Integrity:
   - Source code integrity verification
   - Third-party component validation
   - Build environment security
   - Release process controls
   ```

9. **A09:2021 - Security Logging and Monitoring Failures**
   ```
   Assess Logging and Monitoring Implementation:
   Logging Framework:
   - Comprehensive event logging
   - Log data protection and integrity
   - Centralized log management
   - Log retention and archival
   
   Monitoring and Alerting:
   - Real-time security monitoring
   - Anomaly detection implementation
   - Incident response integration
   - Performance and availability monitoring
   
   Log Analysis:
   - Automated log analysis tools
   - Pattern recognition and correlation
   - Threat intelligence integration
   - Forensic analysis capability
   
   Compliance and Audit:
   - Regulatory logging requirements
   - Audit trail completeness
   - Log review and analysis procedures
   - Evidence preservation protocols
   ```

10. **A10:2021 - Server-Side Request Forgery (SSRF)**
    ```
    Test SSRF Prevention Mechanisms:
    SSRF Vulnerability Assessment:
    - Internal service access attempts
    - Metadata service exploitation
    - Port scanning and service discovery
    - File system access validation
    
    Input Validation:
    - URL validation and sanitization
    - Whitelist-based filtering
    - Network-level access controls
    - Response validation procedures
    
    Network Security:
    - Network segmentation implementation
    - Firewall rule configuration
    - Internal service protection
    - Zero-trust architecture adoption
    
    Prevention Testing:
    - URL parsing and validation testing
    - Network access control verification
    - Response handling security assessment
    - Error message information disclosure
    ```

**OWASP Top 10 Compliance Scoring:**
- **Critical (0-3 vulnerabilities):** Immediate remediation required
- **High (4-6 vulnerabilities):** Significant security risk
- **Medium (7-8 vulnerabilities):** Moderate security posture
- **Good (9-10 vulnerabilities):** Strong security implementation

**Automated Tool Integration:**
- Static Application Security Testing (SAST)
- Dynamic Application Security Testing (DAST)
- Interactive Application Security Testing (IAST)
- Software Composition Analysis (SCA)
- Container and Infrastructure Security Scanning
```

### OWASP ASVS (Application Security Verification Standard) Audit Prompt
```
Conduct comprehensive OWASP ASVS security verification assessment:

**ASVS Level Selection:** [LEVEL_1_OPPORTUNISTIC_LEVEL_2_STANDARD_LEVEL_3_ADVANCED]
**Application Architecture:** [WEB_MOBILE_WEB_SERVICE_API_CLOUD_NATIVE]
**Security Requirements:** [BASELINE_STANDARD_ADVANCED_SECURITY_CONTROLS]
**Verification Approach:** [PENETRATION_TESTING_CODE_REVIEW_ARCHITECTURE_REVIEW]

**ASVS Verification Categories Assessment:**

**V1: Architecture, Design and Threat Modeling**
```
Verify Security Architecture:
Architecture Security:
- Secure architecture documentation
- Component trust boundary identification
- High-level security design validation
- Architecture change control procedures

Threat Modeling:
- Comprehensive threat model development
- Attack surface analysis completion
- Risk assessment and mitigation planning
- Regular threat model updates

Design Security:
- Security requirement specification
- Security control design validation
- Attack vector consideration
- Defense-in-depth implementation

Level-Specific Requirements:
Level 1: Basic architecture documentation
Level 2: Formal threat modeling process
Level 3: Advanced threat modeling with attack trees
```

**V2: Authentication**
```
Authentication Mechanism Verification:
Password Authentication:
- Strong password policy implementation
- Password storage security (bcrypt, scrypt, Argon2)
- Account lockout protection
- Credential recovery security

Multi-Factor Authentication:
- MFA implementation for sensitive operations
- Token-based authentication security
- Biometric authentication protection
- Authentication factor independence

Session Management:
- Secure session token generation
- Session lifecycle management
- Concurrent session control
- Session termination security

Level-Specific Testing:
Level 1: Basic authentication controls
Level 2: Advanced authentication mechanisms
Level 3: Cryptographic authentication protocols
```

**V3: Session Management**
```
Session Security Validation:
Session Token Security:
- Cryptographically secure token generation
- Session token entropy validation
- Token transmission protection
- Session fixation prevention

Session Lifecycle:
- Appropriate session timeout implementation
- Session invalidation procedures
- Session renewal mechanisms
- Cross-domain session handling

Session Storage:
- Secure session storage implementation
- Session data protection
- Session sharing prevention
- Session replay attack protection

Testing Framework:
- Session token predictability testing
- Session hijacking attempt validation
- Concurrent session management testing
- Session timeout verification
```

**V4: Access Control**
```
Authorization and Access Control Testing:
Access Control Design:
- Centralized access control implementation
- Default deny access policy
- Least privilege principle application
- Role-based access control (RBAC)

Authorization Testing:
- Horizontal privilege escalation prevention
- Vertical privilege escalation prevention
- Business logic access control
- Administrative interface protection

Resource Protection:
- Direct object reference protection
- File upload security controls
- API endpoint authorization
- Database access control

Advanced Access Control:
- Attribute-based access control (ABAC)
- Dynamic access control mechanisms
- Context-aware access decisions
- Access control policy engines
```

**V5: Validation, Sanitization and Encoding**
```
Input and Output Security Validation:
Input Validation:
- Comprehensive input validation
- Whitelist-based validation preferred
- Input length and format restrictions
- Character encoding validation

Output Encoding:
- Context-appropriate output encoding
- XSS prevention through encoding
- JSON and XML output security
- Template injection prevention

Data Sanitization:
- HTML sanitization implementation
- SQL injection prevention
- NoSQL injection protection
- Command injection mitigation

Validation Framework:
- Centralized validation library usage
- Validation rule consistency
- Error handling security
- Validation bypass prevention
```

**V6: Stored Cryptography**
```
Cryptographic Implementation Assessment:
Cryptographic Standards:
- Strong encryption algorithm usage
- Appropriate key length implementation
- Secure random number generation
- Cryptographic protocol security

Key Management:
- Secure key generation procedures
- Key storage protection
- Key rotation implementation
- Key escrow and recovery

Data Protection:
- Sensitive data encryption at rest
- Database encryption implementation
- File system encryption validation
- Backup encryption verification

Cryptographic Operations:
- Digital signature validation
- Certificate management procedures
- Hash function security
- Cryptographic API usage
```

**V7: Error Handling and Logging**
```
Error Handling and Logging Security:
Error Handling:
- Secure error message design
- Information disclosure prevention
- Exception handling security
- Error page customization

Security Logging:
- Comprehensive security event logging
- Log integrity protection
- Centralized log management
- Real-time monitoring implementation

Log Analysis:
- Automated log analysis tools
- Anomaly detection capabilities
- Correlation rule implementation
- Forensic analysis preparation

Monitoring and Alerting:
- Security incident detection
- Real-time alerting mechanisms
- Dashboard and reporting tools
- Compliance reporting automation
```

**V8: Data Protection**
```
Data Protection and Privacy Controls:
Data Classification:
- Sensitive data identification
- Data classification scheme implementation
- Data handling procedure definition
- Data retention policy compliance

Data Storage Security:
- Encryption at rest implementation
- Database security configuration
- File storage protection
- Cloud storage security

Data Transmission:
- Encryption in transit validation
- Secure communication protocols
- Certificate validation procedures
- Man-in-the-middle attack prevention

Privacy Controls:
- Personal data protection measures
- Consent management implementation
- Data subject rights facilitation
- Privacy impact assessment completion
```

**ASVS Verification Levels:**

**Level 1 - Opportunistic Security**
- Basic security hygiene implementation
- Automated scanning tool utilization
- Common vulnerability elimination
- Security awareness demonstration

**Level 2 - Standard Security**
- Comprehensive security control implementation
- Manual verification and testing
- Defense-in-depth architecture
- Regular security assessment

**Level 3 - Advanced Security**
- High-value application protection
- Advanced security architecture
- Comprehensive manual verification
- Security expertise demonstration

**ASVS Compliance Assessment Framework:**
- Requirement-by-requirement verification
- Evidence collection and documentation
- Gap analysis and remediation planning
- Certification readiness assessment
```

## Microsoft SDL (Security Development Lifecycle) Audit Prompts

### Microsoft SDL Implementation Assessment Prompt
```
Conduct comprehensive Microsoft Security Development Lifecycle audit:

**Development Organization:** [TEAM_SIZE_STRUCTURE_MATURITY_LEVEL]
**Software Portfolio:** [APPLICATIONS_SERVICES_PLATFORMS_SYSTEMS]
**SDL Implementation:** [TRADITIONAL_AGILE_DEVOPS_HYBRID_METHODOLOGY]
**Security Maturity:** [BASIC_INTERMEDIATE_ADVANCED_EXPERT_LEVEL]

**SDL Phase-Based Assessment Framework:**

**Phase 1: Training**
```
Assess Security Training Implementation:
Core Security Training:
- Mandatory security training for all developers
- Role-specific security education programs
- Secure coding practice training
- Threat modeling training completion

Training Content Areas:
- Security fundamentals and principles
- Common vulnerability categories (OWASP Top 10)
- Secure coding techniques and practices
- Platform-specific security features

Training Effectiveness Measurement:
- Training completion rate tracking
- Knowledge retention assessment
- Practical skill demonstration
- Continuous learning program implementation

Specialized Training:
- Cryptography and key management
- Authentication and authorization
- Secure architecture and design
- Incident response and forensics

Training Program Maturity:
- Regular training content updates
- Industry best practice integration
- Hands-on security exercises
- Mentoring and coaching programs
```

**Phase 2: Requirements**
```
Security Requirements Definition and Management:
Security Requirement Categories:
- Authentication and authorization requirements
- Data protection and encryption needs
- Input validation and output encoding
- Error handling and logging specifications

Requirements Engineering Process:
- Security requirement elicitation techniques
- Stakeholder security need identification
- Regulatory compliance requirement mapping
- Business impact and risk assessment

Documentation and Tracking:
- Security requirement specification format
- Traceability matrix maintenance
- Change management procedures
- Requirement verification planning

Quality Assurance:
- Security requirement review process
- Completeness and consistency validation
- Feasibility and testability assessment
- Stakeholder approval and sign-off
```

**Phase 3: Design**
```
Secure Design and Architecture Review:
Threat Modeling Implementation:
- Systematic threat identification process
- Attack surface analysis completion
- Trust boundary definition and validation
- Threat mitigation strategy development

Design Review Process:
- Security architecture review procedures
- Design pattern security assessment
- Component interaction security validation
- Third-party integration security analysis

Security Design Principles:
- Defense-in-depth implementation
- Least privilege principle application
- Fail-safe and fail-secure design
- Security by design and by default

Documentation Requirements:
- Security architecture documentation
- Threat model documentation and updates
- Security design decision rationale
- Risk acceptance and mitigation records
```

**Phase 4: Implementation**
```
Secure Coding Practice Assessment:
Secure Coding Standards:
- Coding guideline definition and enforcement
- Language-specific security practices
- Framework and library security usage
- Code review checklist implementation

Static Analysis Integration:
- SAST tool implementation and configuration
- Automated security scanning in CI/CD
- Custom rule development and tuning
- False positive management procedures

Code Review Process:
- Peer review security focus areas
- Security champion involvement
- Automated and manual review combination
- Review quality and coverage metrics

Implementation Controls:
- Input validation implementation
- Output encoding and sanitization
- Error handling and logging integration
- Cryptographic implementation validation
```

**Phase 5: Verification**
```
Security Testing and Validation:
Dynamic Security Testing:
- DAST tool implementation and results
- Penetration testing procedures
- Vulnerability assessment completion
- Security regression testing

Test Case Development:
- Security test case design and execution
- Negative test case implementation
- Boundary value and edge case testing
- Error condition and exception testing

Testing Automation:
- Automated security test integration
- Continuous security testing pipeline
- Test result analysis and reporting
- Defect tracking and management

Verification Coverage:
- Functional security requirement validation
- Non-functional security property testing
- Performance and scalability security impact
- Interoperability security assessment
```

**Phase 6: Release**
```
Security Release Management:
Pre-Release Security Review:
- Final security assessment completion
- Security testing result evaluation
- Risk assessment and acceptance
- Release readiness criteria validation

Security Documentation:
- Security guide and documentation
- Known issue and limitation disclosure
- Security configuration recommendations
- Incident response contact information

Release Process Security:
- Secure build and deployment pipeline
- Code signing and integrity verification
- Release artifact protection
- Distribution channel security

Go-Live Security Monitoring:
- Security monitoring system activation
- Incident response team readiness
- Performance and security baseline establishment
- User communication and support preparation
```

**Phase 7: Response**
```
Security Incident Response and Management:
Incident Response Planning:
- Security incident response plan development
- Response team roles and responsibilities
- Communication and escalation procedures
- Recovery and business continuity planning

Vulnerability Management:
- Vulnerability disclosure and handling
- Patch development and testing procedures
- Emergency response and hotfix deployment
- Customer communication and notification

Monitoring and Detection:
- Security monitoring system implementation
- Anomaly detection and alerting
- Log analysis and correlation procedures
- Threat intelligence integration

Continuous Improvement:
- Incident analysis and lessons learned
- Process improvement implementation
- Security metric collection and analysis
- Industry best practice adoption
```

**SDL Maturity Assessment Framework:**

**Level 1 - Basic SDL Implementation**
- Core security training completion
- Basic threat modeling implementation
- Essential security testing execution
- Fundamental incident response capability

**Level 2 - Standard SDL Implementation**
- Comprehensive security training program
- Systematic threat modeling process
- Integrated security testing pipeline
- Mature incident response procedures

**Level 3 - Advanced SDL Implementation**
- Specialized security expertise development
- Advanced threat modeling techniques
- Automated security testing optimization
- Proactive security monitoring and response

**Level 4 - Expert SDL Implementation**
- Security thought leadership and innovation
- Industry-leading threat modeling practices
- Cutting-edge security testing methodologies
- World-class incident response capabilities

**SDL Compliance Metrics:**
- Training completion rate (target: 100%)
- Threat model coverage (target: 100% of features)
- Security defect density (target: <0.5 per KLOC)
- Time to patch critical vulnerabilities (target: <30 days)

**SDL Tool Integration Requirements:**
- Static Application Security Testing (SAST)
- Dynamic Application Security Testing (DAST)
- Interactive Application Security Testing (IAST)
- Software Composition Analysis (SCA)
- Threat modeling tools and platforms
```

## ISO/IEC/IEEE 12207 Software Lifecycle Process Audit Prompts

### ISO 12207 Software Lifecycle Process Assessment Prompt
```
Conduct comprehensive ISO/IEC/IEEE 12207 software lifecycle process audit:

**Organization Scope:** [SOFTWARE_DEVELOPMENT_ORGANIZATION_PROFILE]
**Process Implementation:** [SYSTEM_SOFTWARE_SERVICE_LIFECYCLE_PROCESSES]
**Maturity Target:** [CAPABILITY_MATURITY_LEVEL_1_5]
**Assessment Method:** [PROCESS_ASSESSMENT_METHODOLOGY_15504_SPICE]

**Primary Lifecycle Process Assessment:**

**Agreement Processes**
```
Assess Agreement Management:
Acquisition Process:
- Supplier selection and evaluation procedures
- Contract negotiation and management
- Acceptance criteria definition and validation
- Supplier performance monitoring and control

Supply Process:
- Customer requirement analysis and response
- Proposal development and submission procedures
- Contract fulfillment and delivery management
- Customer relationship and communication management

Process Capability Assessment:
Level 1 - Performed: Basic process execution
Level 2 - Managed: Planned and monitored execution
Level 3 - Established: Defined and standardized process
Level 4 - Predictable: Quantitatively controlled process
Level 5 - Optimizing: Continuously improving process

Evidence Requirements:
- Process documentation and procedures
- Work product samples and deliverables
- Process performance measurements
- Improvement initiative documentation
```

**Organizational Project-Enabling Processes**
```
Evaluate Organizational Capabilities:
Life Cycle Model Management:
- Software lifecycle model definition
- Process tailoring and adaptation procedures
- Methodology selection and implementation
- Process improvement and evolution management

Infrastructure Management:
- Development environment establishment
- Tool selection and integration
- Configuration management system deployment
- Quality assurance infrastructure implementation

Portfolio Management:
- Project portfolio planning and oversight
- Resource allocation and optimization
- Risk management across portfolio
- Strategic alignment and prioritization

Human Resource Management:
- Competency framework development
- Training and development programs
- Performance management systems
- Knowledge management and retention

Process Assessment Framework:
- Process definition completeness
- Process implementation consistency
- Process measurement and control
- Process improvement effectiveness
```

**Technical Management Processes**
```
Assess Technical Management Implementation:
Project Planning:
- Project scope definition and management
- Work breakdown structure development
- Schedule and resource planning
- Risk identification and mitigation planning

Project Assessment and Control:
- Progress monitoring and reporting procedures
- Quality metrics collection and analysis
- Change control and configuration management
- Issue and problem resolution processes

Decision Management:
- Decision-making framework and criteria
- Alternative analysis and evaluation
- Stakeholder involvement and consultation
- Decision documentation and communication

Risk Management:
- Risk identification and assessment procedures
- Risk mitigation strategy development
- Risk monitoring and control mechanisms
- Contingency planning and response procedures

Configuration Management:
- Configuration item identification and control
- Baseline establishment and management
- Change control board procedures
- Version control and release management

Information Management:
- Information architecture and standards
- Data governance and quality management
- Information security and access control
- Knowledge capture and sharing mechanisms
```

**Technical Processes**
```
Evaluate Technical Process Implementation:
Business or Mission Analysis:
- Stakeholder need identification and analysis
- Business case development and validation
- System-of-interest definition and scope
- Life cycle concept development

Stakeholder Needs and Requirements Definition:
- Stakeholder identification and engagement
- Need elicitation and analysis techniques
- Requirement specification and documentation
- Requirement validation and acceptance

System/Software Requirements Definition:
- System requirement analysis and specification
- Software requirement derivation and allocation
- Requirement traceability and management
- Interface requirement definition and control

System/Software Architecture Definition:
- Architecture design and documentation
- Component identification and specification
- Interface design and definition
- Architecture evaluation and validation

Implementation:
- Code development and construction
- Unit testing and integration procedures
- Documentation development and maintenance
- Implementation standard compliance

Integration:
- Integration strategy and planning
- Interface testing and validation
- System integration and testing
- Integration issue resolution procedures

Verification:
- Verification planning and procedures
- Test case design and execution
- Result analysis and reporting
- Verification evidence collection

Transition:
- Deployment planning and execution
- User training and support provision
- System cutover and migration procedures
- Operational readiness assessment

Validation:
- Validation planning and procedures
- User acceptance testing coordination
- Stakeholder need satisfaction validation
- Validation evidence documentation

Operation:
- System operation and monitoring
- User support and maintenance
- Performance monitoring and optimization
- Operational issue resolution

Maintenance:
- Maintenance strategy and planning
- Change request processing and implementation
- System modification and enhancement
- Maintenance documentation and reporting

Disposal:
- Disposal planning and preparation
- Data migration and archival procedures
- System decommissioning and removal
- Disposal documentation and records
```

**Supporting Processes**
```
Assess Supporting Process Implementation:
Documentation Management:
- Documentation standards and templates
- Document lifecycle management procedures
- Review and approval processes
- Document control and distribution

Configuration Management:
- CM planning and implementation
- Configuration identification and control
- Status accounting and reporting
- Configuration audit and review

Quality Assurance:
- Quality planning and procedures
- Quality control and assessment
- Quality metrics and measurement
- Quality improvement initiatives

Verification and Validation:
- V&V planning and procedures
- Independent V&V activities
- V&V result analysis and reporting
- V&V process improvement

Joint Review:
- Review planning and scheduling
- Review conduct and facilitation
- Action item tracking and closure
- Review effectiveness assessment

Audit:
- Audit planning and preparation
- Audit execution and reporting
- Finding resolution and follow-up
- Audit program management

Problem Resolution:
- Problem identification and reporting
- Problem analysis and investigation
- Solution development and implementation
- Problem resolution tracking and closure
```

**Process Assessment and Improvement Framework:**

**Process Capability Determination**
```
Assess Process Capability Levels:
Level 0 - Incomplete:
- Process not implemented or fails to achieve purpose
- Little or no evidence of systematic achievement
- No identifiable work products or outcomes

Level 1 - Performed:
- Process achieves its purpose
- Work products are produced
- Purpose achievement may not be fully systematic

Level 2 - Managed:
- Process is planned and monitored
- Work products are controlled and maintained
- Performance is tracked and adjusted

Level 3 - Established:
- Process is defined and implemented systematically
- Standard process adapted for specific context
- Competent human resources deployed

Level 4 - Predictable:
- Process operates within defined limits
- Quantitative management and control implemented
- Process performance is predictable

Level 5 - Optimizing:
- Process continuously improved to meet goals
- Quantitative process improvement implemented
- Innovation and optimization culture established
```

**Assessment Evidence Collection**
```
Gather Process Assessment Evidence:
Direct Evidence:
- Process documentation and procedures
- Work product samples and artifacts
- Tool usage and configuration evidence
- Training and competency records

Indirect Evidence:
- Interview results and observations
- Process performance measurements
- Customer and stakeholder feedback
- Audit and review findings

Assessment Documentation:
- Process instance characterization
- Practice implementation rating
- Capability level determination
- Improvement opportunity identification

Validation and Verification:
- Evidence triangulation and confirmation
- Assessment result validation
- Stakeholder review and acceptance
- Assessment report finalization
```

**ISO 12207 Compliance Benefits:**
- Improved software quality and reliability
- Reduced development time and costs
- Enhanced customer satisfaction
- Better risk management and control
- Increased process maturity and capability

**Implementation Roadmap:**
- Current state assessment and gap analysis
- Process improvement planning and prioritization
- Pilot implementation and validation
- Organization-wide deployment and institutionalization
- Continuous monitoring and improvement
```

## Integrated Software Development Framework Dashboard Prompt
```
Create comprehensive software development security and quality dashboard:

**Framework Coverage:** OWASP Top 10 + OWASP ASVS + Microsoft SDL + ISO 12207
**Development Methodology:** [AGILE_DEVOPS_WATERFALL_HYBRID_APPROACH]
**Security Integration:** [DEVSECOPS_SHIFT_LEFT_CONTINUOUS_SECURITY]

**Unified Development Security Maturity Matrix:**

1. **Application Security Assessment (OWASP)**
   ```
   OWASP Top 10 Compliance:
   - Broken Access Control: [COMPLIANT/NON_COMPLIANT]
   - Cryptographic Failures: [COMPLIANT/NON_COMPLIANT]
   - Injection: [COMPLIANT/NON_COMPLIANT]
   - Insecure Design: [COMPLIANT/NON_COMPLIANT]
   - Security Misconfiguration: [COMPLIANT/NON_COMPLIANT]
   - Vulnerable Components: [COMPLIANT/NON_COMPLIANT]
   - Authentication Failures: [COMPLIANT/NON_COMPLIANT]
   - Software/Data Integrity: [COMPLIANT/NON_COMPLIANT]
   - Logging/Monitoring Failures: [COMPLIANT/NON_COMPLIANT]
   - Server-Side Request Forgery: [COMPLIANT/NON_COMPLIANT]
   
   OWASP ASVS Verification:
   - Level 1 Opportunistic: [SCORE]/100
   - Level 2 Standard: [SCORE]/100
   - Level 3 Advanced: [SCORE]/100
   ```

2. **Secure Development Lifecycle (Microsoft SDL)**
   ```
   SDL Phase Implementation:
   - Training: [SCORE]/100
   - Requirements: [SCORE]/100
   - Design: [SCORE]/100
   - Implementation: [SCORE]/100
   - Verification: [SCORE]/100
   - Release: [SCORE]/100
   - Response: [SCORE]/100
   
   SDL Maturity Level: [BASIC/STANDARD/ADVANCED/EXPERT]
   ```

3. **Process Maturity (ISO 12207)**
   ```
   Process Capability Assessment:
   - Primary Lifecycle Processes: [CAPABILITY_LEVEL_0_5]
   - Organizational Project-Enabling: [CAPABILITY_LEVEL_0_5]
   - Technical Management: [CAPABILITY_LEVEL_0_5]
   - Technical Processes: [CAPABILITY_LEVEL_0_5]
   - Supporting Processes: [CAPABILITY_LEVEL_0_5]
   
   Overall Process Maturity: [LEVEL_1_5]
   ```

**Cross-Framework Integration Analysis:**
- Security requirement integration across frameworks
- Process and control harmonization opportunities
- Tool chain optimization and automation
- Training and competency development alignment

**DevSecOps Implementation Readiness:**
- Security tool integration in CI/CD pipeline
- Automated security testing deployment
- Continuous monitoring and feedback loops
- Security culture and practices adoption

**Strategic Development Security Roadmap:**
- Phase 1: Foundation - Basic security hygiene and process establishment
- Phase 2: Integration - DevSecOps pipeline and automation deployment
- Phase 3: Optimization - Advanced security practices and continuous improvement
- Phase 4: Innovation - Security research, innovation, and industry leadership

**Key Performance Indicators:**
- Security defect density reduction
- Time to remediate vulnerabilities
- Security test coverage percentage
- Developer security training completion rate
- Customer security satisfaction score
```