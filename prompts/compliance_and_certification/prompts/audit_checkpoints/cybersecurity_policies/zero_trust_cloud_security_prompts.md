# Zero Trust Architecture and Cloud Security Framework Audit Checkpoint Prompts


  
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


## Overview
This collection provides comprehensive audit checkpoint prompts for Zero Trust Architecture implementation per NIST SP 800-207, cloud security frameworks (CSA CCM, AWS/Azure/GCP security), and modern cloud-native security architectures.

---

## Zero Trust Architecture (NIST SP 800-207)

### Zero Trust Core Principles Assessment

#### Principle 1: Never Trust, Always Verify
```
PROMPT: Assess implementation of "never trust, always verify" principle across enterprise architecture.

Evaluate zero trust verification implementation per NIST SP 800-207:

IDENTITY VERIFICATION AND AUTHENTICATION:
• Continuous Authentication: Do we continuously verify user and device identity throughout sessions?
• Multi-Factor Authentication: Is MFA implemented universally across all access scenarios?
• Risk-Based Authentication: Do we adjust authentication requirements based on risk assessment?
• Identity Provider Integration: Are identity providers integrated to support continuous verification?

DEVICE VERIFICATION AND TRUST:
• Device Registration: Are all devices registered and verified before network access?
• Device Health Assessment: Do we continuously assess device security posture and compliance?
• Certificate-Based Authentication: Are device certificates used for authentication and authorization?
• Mobile Device Management: Are mobile devices managed and verified through MDM/UEM solutions?

APPLICATION AND SERVICE VERIFICATION:
• Application Authentication: Do applications authenticate to each other using strong cryptographic methods?
• Service Mesh Security: Are service-to-service communications secured and verified?
• API Security: Are APIs secured with authentication, authorization, and verification controls?
• Microservices Security: Are microservices architectures implementing zero trust principles?

VERIFICATION ENFORCEMENT POINTS:
• Policy Enforcement Points: Are PEPs deployed to enforce verification at all access points?
• Network Segmentation: Are networks segmented to require verification for cross-segment access?
• Least Privilege Access: Is access granted based on verified identity and minimum required privileges?
• Dynamic Access Control: Are access decisions made dynamically based on current risk assessment?

OUTPUT REQUIREMENTS:
1. Verification implementation maturity assessment
2. Trust assumption identification and elimination analysis
3. Continuous verification capability evaluation
4. Policy enforcement effectiveness assessment
5. Specific recommendations for "never trust, always verify" implementation

Include verification workflow diagrams and recommended verification technologies.
```

#### Principle 2: Assume Breach and Minimize Impact
```
PROMPT: Evaluate "assume breach" mentality and impact minimization strategies.

Assess breach assumption and containment per Zero Trust principles:

BREACH ASSUMPTION STRATEGY:
• Architectural Design: Is the architecture designed assuming adversaries are already present?
• Defense in Depth: Are multiple layers of security controls implemented throughout the architecture?
• Lateral Movement Prevention: Are controls in place to prevent lateral movement within the network?
• Blast Radius Limitation: Are systems designed to limit the impact of potential breaches?

NETWORK SEGMENTATION AND ISOLATION:
• Micro-segmentation: Is micro-segmentation implemented to isolate critical assets and workloads?
• Software-Defined Perimeter: Are SDP solutions used to create dynamic, encrypted micro-tunnels?
• Network Isolation: Are critical systems isolated from general corporate networks?
• East-West Traffic Inspection: Is lateral traffic inspected and controlled?

DATA PROTECTION AND CLASSIFICATION:
• Data Classification: Are data classification schemes implemented to prioritize protection?
• Encryption Everywhere: Is encryption implemented for data at rest, in transit, and in processing?
• Data Loss Prevention: Are DLP controls implemented to prevent data exfiltration?
• Data Access Controls: Are granular access controls implemented based on data sensitivity?

INCIDENT RESPONSE AND CONTAINMENT:
• Automated Response: Are automated response capabilities implemented for rapid containment?
• Forensic Readiness: Are systems configured for forensic investigation and evidence collection?
• Recovery Procedures: Are recovery procedures designed for rapid restoration from breaches?
• Tabletop Exercises: Are breach scenarios regularly exercised to test response capabilities?

OUTPUT REQUIREMENTS:
1. Breach assumption implementation assessment
2. Impact minimization strategy evaluation
3. Containment capability maturity analysis
4. Recovery and resilience assessment
5. Specific recommendations for breach assumption and impact minimization

Provide breach scenario analysis and recommended containment strategies.
```

#### Principle 3: Verify Explicitly Based on All Available Data Points
```
PROMPT: Assess explicit verification using comprehensive data sources and analytics.

Evaluate comprehensive verification implementation:

DATA SOURCE INTEGRATION:
• Identity Data: Are comprehensive identity data sources integrated for verification decisions?
• Device Data: Is device telemetry and security posture data integrated into verification?
• Network Data: Are network traffic patterns and anomalies considered in verification decisions?
• Application Data: Is application behavior and usage data incorporated into verification?
• Threat Intelligence: Is external threat intelligence integrated into verification processes?

CONTEXTUAL ANALYSIS:
• User Behavior Analytics: Are UBA solutions implemented to detect anomalous user behavior?
• Entity Behavior Analytics: Are EBA solutions monitoring device and application behavior?
• Geolocation Analysis: Is geolocation data used to assess access request legitimacy?
• Time-Based Analysis: Are temporal patterns analyzed for verification decisions?
• Risk Scoring: Are comprehensive risk scores calculated for access decisions?

MACHINE LEARNING AND AI INTEGRATION:
• Anomaly Detection: Are ML/AI solutions implemented for anomaly detection across data sources?
• Pattern Recognition: Are patterns in user and entity behavior recognized and analyzed?
• Predictive Analytics: Are predictive models used to anticipate and prevent security incidents?
• Adaptive Authentication: Do authentication requirements adapt based on ML/AI analysis?

DECISION ENGINE CAPABILITIES:
• Policy Decision Points: Are centralized PDPs implemented for access control decisions?
• Real-Time Processing: Are verification decisions made in real-time based on current data?
• Policy Management: Are verification policies centrally managed and consistently enforced?
• Audit and Logging: Are all verification decisions logged and auditable?

OUTPUT REQUIREMENTS:
1. Data source integration completeness assessment
2. Contextual analysis capability evaluation
3. ML/AI integration maturity analysis
4. Decision engine effectiveness assessment
5. Specific recommendations for explicit verification enhancement

Include data integration architecture and recommended analytics capabilities.
```

### Zero Trust Network Architecture (ZTNA)

#### Software-Defined Perimeter Implementation
```
PROMPT: Assess Software-Defined Perimeter (SDP) implementation for zero trust network access.

Evaluate SDP deployment per zero trust architecture requirements:

SDP ARCHITECTURE ASSESSMENT:
• SDP Controller: Is the SDP controller properly deployed and configured for policy enforcement?
• SDP Gateways: Are SDP gateways strategically placed to control resource access?
• SDP Client Software: Are SDP clients deployed and managed across all endpoints?
• Cryptographic Implementation: Are strong cryptographic protocols used for SDP communications?

DYNAMIC ACCESS CONTROL:
• Just-in-Time Access: Is JIT access implemented to minimize standing privileges?
• Application-Level Access: Is access controlled at the application level rather than network level?
• Dark Cloud Implementation: Are resources hidden from unauthorized users ("dark cloud")?
• Access Revocation: Can access be revoked dynamically based on risk changes?

SDP POLICY MANAGEMENT:
• Policy Definition: Are SDP policies defined based on zero trust principles?
• Policy Enforcement: Are policies consistently enforced across all SDP components?
• Policy Updates: Can policies be updated dynamically based on threat intelligence?
• Policy Auditing: Are SDP policies auditable and compliant with security requirements?

INTEGRATION AND SCALABILITY:
• Identity Provider Integration: Is SDP integrated with enterprise identity providers?
• SIEM Integration: Are SDP events integrated with security monitoring systems?
• Cloud Integration: Does SDP support cloud and hybrid environments?
• Scalability Testing: Has SDP scalability been tested for enterprise requirements?

OUTPUT REQUIREMENTS:
1. SDP implementation maturity assessment
2. Dynamic access control effectiveness evaluation
3. Policy management capability analysis
4. Integration and scalability assessment
5. Specific recommendations for SDP enhancement

Provide SDP architecture diagrams and recommended deployment strategies.
```

#### Network Microsegmentation
```
PROMPT: Evaluate network microsegmentation implementation for zero trust architecture.

Assess microsegmentation deployment and effectiveness:

SEGMENTATION STRATEGY:
• Segmentation Model: Is a comprehensive segmentation model implemented based on business requirements?
• Asset Classification: Are assets classified and grouped for appropriate segmentation?
• Trust Zones: Are trust zones defined based on security requirements and risk levels?
• Segmentation Policies: Are segmentation policies aligned with zero trust principles?

MICROSEGMENTATION IMPLEMENTATION:
• Software-Defined Networking: Is SDN used to implement dynamic microsegmentation?
• Virtual LANs: Are VLANs configured to support microsegmentation requirements?
• Firewall Rules: Are next-generation firewalls configured for microsegmentation enforcement?
• Container Segmentation: Are containerized workloads properly segmented?

TRAFFIC INSPECTION AND CONTROL:
• East-West Traffic Inspection: Is lateral traffic inspected and controlled between segments?
• Application-Layer Inspection: Are application protocols inspected for security threats?
• Encrypted Traffic Analysis: Can encrypted traffic be analyzed for threat detection?
• Traffic Analytics: Are traffic patterns analyzed for anomaly detection?

AUTOMATION AND ORCHESTRATION:
• Automated Policy Enforcement: Are segmentation policies automatically enforced?
• Dynamic Segmentation: Can segmentation adapt dynamically to changing conditions?
• Orchestration Integration: Is segmentation integrated with security orchestration platforms?
• Compliance Automation: Are compliance requirements automatically enforced through segmentation?

OUTPUT REQUIREMENTS:
1. Microsegmentation strategy assessment
2. Implementation effectiveness evaluation
3. Traffic inspection capability analysis
4. Automation and orchestration maturity assessment
5. Specific recommendations for microsegmentation enhancement

Include segmentation architecture and recommended implementation approaches.
```

### Zero Trust Identity and Access Management

#### Identity-Centric Security Model
```
PROMPT: Assess identity-centric security model implementation for zero trust architecture.

Evaluate identity-based security controls:

IDENTITY GOVERNANCE:
• Identity Lifecycle Management: Are identity lifecycles managed comprehensively from onboarding to offboarding?
• Role-Based Access Control: Is RBAC implemented consistently across all systems and applications?
• Attribute-Based Access Control: Is ABAC used for fine-grained access control decisions?
• Identity Federation: Are identity federation capabilities implemented for partner access?

PRIVILEGED IDENTITY MANAGEMENT:
• Privileged Account Discovery: Are all privileged accounts discovered and inventoried?
• Just-in-Time Privileged Access: Is JIT access implemented for privileged operations?
• Privileged Session Management: Are privileged sessions monitored and recorded?
• Credential Rotation: Are privileged credentials rotated regularly and automatically?

CONTINUOUS AUTHENTICATION:
• Adaptive Authentication: Do authentication requirements adapt based on risk assessment?
• Behavioral Biometrics: Are behavioral biometrics used for continuous user verification?
• Risk-Based MFA: Are MFA requirements adjusted based on contextual risk factors?
• Session Management: Are user sessions continuously monitored and validated?

IDENTITY ANALYTICS:
• User Behavior Analytics: Are UBA solutions implemented to detect identity-based threats?
• Identity Risk Scoring: Are identity risk scores calculated and used for access decisions?
• Anomaly Detection: Are identity-based anomalies detected and investigated?
• Identity Threat Hunting: Are threat hunting activities focused on identity-based attacks?

OUTPUT REQUIREMENTS:
1. Identity governance maturity assessment
2. Privileged identity management effectiveness evaluation
3. Continuous authentication capability analysis
4. Identity analytics implementation assessment
5. Specific recommendations for identity-centric security enhancement

Provide identity architecture diagrams and recommended identity security controls.
```

---

## Cloud Security Frameworks

### Cloud Security Alliance (CSA) Cloud Controls Matrix (CCM)

#### CCM Domain Assessment: Data Security and Privacy
```
PROMPT: Assess cloud data security and privacy controls per CSA CCM requirements.

Evaluate cloud data protection against CSA CCM domains:

DATA SECURITY AND LIFECYCLE MANAGEMENT (DSI/DSP):
• Data Classification: Are cloud data classification schemes implemented and enforced?
• Data Retention: Are data retention policies implemented in cloud environments?
• Data Disposal: Are secure data disposal procedures implemented for cloud storage?
• Data Location: Is data location tracked and controlled per regulatory requirements?
• Data Sovereignty: Are data sovereignty requirements addressed for international operations?

ENCRYPTION AND KEY MANAGEMENT (EKM):
• Encryption Standards: Are strong encryption standards implemented for cloud data?
• Key Management: Are encryption keys managed securely using cloud-native or external HSMs?
• Key Lifecycle: Are key lifecycle management procedures implemented?
• Key Escrow: Are key escrow arrangements established where required by regulation?
• Encryption at Rest: Is encryption at rest implemented for all sensitive cloud data?
• Encryption in Transit: Is encryption in transit implemented for all cloud communications?

DATA LOSS PREVENTION (DLP):
• Cloud DLP Implementation: Are DLP solutions implemented for cloud environments?
• Data Discovery: Are data discovery tools used to locate sensitive data in cloud storage?
• Data Monitoring: Is cloud data access monitored and analyzed for anomalies?
• Data Exfiltration Prevention: Are controls implemented to prevent unauthorized data exfiltration?

PRIVACY COMPLIANCE:
• Privacy by Design: Are privacy principles implemented in cloud architecture design?
• Consent Management: Are consent management mechanisms implemented for cloud services?
• Data Subject Rights: Can data subject rights be exercised in cloud environments?
• Privacy Impact Assessments: Are PIAs conducted for cloud service implementations?

OUTPUT REQUIREMENTS:
1. CSA CCM data security compliance assessment
2. Cloud encryption implementation evaluation
3. DLP effectiveness in cloud environments analysis
4. Privacy compliance capability assessment
5. Specific recommendations for cloud data security enhancement

Include cloud data protection architecture and recommended security controls.
```

#### CCM Domain Assessment: Identity and Access Management
```
PROMPT: Evaluate cloud identity and access management per CSA CCM IAM controls.

Assess cloud IAM implementation against CSA CCM:

IDENTITY AND ACCESS MANAGEMENT (IAM):
• Cloud IAM Strategy: Is a comprehensive cloud IAM strategy implemented?
• Identity Provider Integration: Are cloud services integrated with enterprise identity providers?
• Single Sign-On: Is SSO implemented for cloud service access?
• Identity Federation: Are identity federation capabilities implemented for cloud partners?

ACCESS CONTROL AND AUTHORIZATION (AAC):
• Role-Based Access Control: Is RBAC implemented consistently across cloud services?
• Attribute-Based Access Control: Is ABAC used for fine-grained cloud access control?
• Least Privilege: Are least privilege principles enforced in cloud environments?
• Access Reviews: Are cloud access rights reviewed regularly and systematically?

PRIVILEGED ACCESS MANAGEMENT (PAM):
• Cloud Privileged Accounts: Are privileged accounts managed securely in cloud environments?
• Administrative Access: Is administrative access to cloud services controlled and monitored?
• Break-Glass Procedures: Are emergency access procedures implemented for cloud services?
• Privileged Session Monitoring: Are privileged cloud sessions monitored and recorded?

MULTI-FACTOR AUTHENTICATION (MFA):
• MFA Implementation: Is MFA implemented for all cloud service access?
• Risk-Based Authentication: Are authentication requirements adjusted based on cloud access risk?
• Adaptive Authentication: Do cloud authentication systems adapt to user behavior and context?
• MFA Bypass Controls: Are MFA bypass procedures secured and auditable?

OUTPUT REQUIREMENTS:
1. Cloud IAM maturity assessment per CSA CCM
2. Access control implementation effectiveness evaluation
3. Privileged access management in cloud analysis
4. MFA implementation coverage assessment
5. Specific recommendations for cloud IAM enhancement

Provide cloud IAM architecture and recommended identity security controls.
```

### AWS Security Framework Assessment

#### AWS Well-Architected Security Pillar
```
PROMPT: Assess AWS security implementation per Well-Architected Framework Security Pillar.

Evaluate AWS security against Well-Architected principles:

IDENTITY AND ACCESS MANAGEMENT:
• AWS IAM Implementation: Is AWS IAM configured with least privilege principles?
• Role-Based Access: Are IAM roles used instead of long-term access keys where possible?
• Multi-Factor Authentication: Is MFA enabled for all IAM users and root accounts?
• Cross-Account Access: Is cross-account access secured with appropriate trust policies?
• Service-Linked Roles: Are service-linked roles used for AWS service permissions?

DETECTIVE CONTROLS:
• AWS CloudTrail: Is CloudTrail enabled for all regions and integrated with CloudWatch?
• Amazon GuardDuty: Is GuardDuty enabled for threat detection across AWS accounts?
• AWS Config: Is Config enabled to monitor resource compliance and configuration changes?
• VPC Flow Logs: Are VPC Flow Logs enabled for network traffic analysis?
• AWS Security Hub: Is Security Hub used for centralized security finding management?

INFRASTRUCTURE PROTECTION:
• Network Security: Are VPCs configured with appropriate subnets and security groups?
• Web Application Firewall: Is AWS WAF deployed to protect web applications?
• DDoS Protection: Is AWS Shield Advanced used for DDoS protection where appropriate?
• Edge Security: Is Amazon CloudFront configured with security best practices?
• Compute Protection: Are EC2 instances protected with appropriate security controls?

DATA PROTECTION:
• Encryption at Rest: Is encryption at rest enabled for all applicable AWS services?
• Encryption in Transit: Is encryption in transit configured for all data communications?
• Key Management: Is AWS KMS used for encryption key management?
• Data Classification: Are AWS resources tagged appropriately for data classification?
• Backup and Recovery: Are data backup and recovery procedures implemented and tested?

INCIDENT RESPONSE:
• Incident Response Plan: Is an AWS-specific incident response plan documented and tested?
• Automated Response: Are automated incident response capabilities implemented using AWS services?
• Forensic Capabilities: Are forensic investigation capabilities available for AWS environments?
• Recovery Procedures: Are recovery procedures documented and tested for AWS services?

OUTPUT REQUIREMENTS:
1. AWS Well-Architected Security Pillar compliance assessment
2. AWS security service utilization evaluation
3. Infrastructure protection effectiveness analysis
4. Data protection implementation assessment
5. Specific recommendations for AWS security enhancement

Include AWS security architecture diagrams and recommended service configurations.
```

#### AWS Security Service Integration
```
PROMPT: Evaluate AWS security service integration and optimization.

Assess AWS security service ecosystem implementation:

SECURITY MONITORING AND ANALYTICS:
• Amazon CloudWatch: Is CloudWatch configured for comprehensive security monitoring?
• AWS CloudTrail: Are CloudTrail logs analyzed for security events and anomalies?
• Amazon Macie: Is Macie used for sensitive data discovery and protection?
• AWS Access Analyzer: Is Access Analyzer used to review and validate access permissions?

THREAT DETECTION AND RESPONSE:
• Amazon GuardDuty: Is GuardDuty configured with appropriate threat intelligence feeds?
• AWS Security Hub: Are security findings aggregated and managed through Security Hub?
• Amazon Detective: Is Detective used for security investigation and analysis?
• AWS Systems Manager: Is Systems Manager used for patch management and compliance?

COMPLIANCE AND GOVERNANCE:
• AWS Config: Are Config rules implemented for compliance monitoring?
• AWS Control Tower: Is Control Tower used for multi-account governance?
• AWS Organizations: Are Organizations used for account management and SCPs?
• AWS Trusted Advisor: Are Trusted Advisor security recommendations implemented?

AUTOMATION AND ORCHESTRATION:
• AWS Lambda: Are Lambda functions used for automated security response?
• Amazon EventBridge: Is EventBridge used for security event routing and automation?
• AWS Step Functions: Are Step Functions used for complex security workflows?
• AWS CodePipeline: Are security controls integrated into CI/CD pipelines?

OUTPUT REQUIREMENTS:
1. AWS security service integration maturity assessment
2. Threat detection and response capability evaluation
3. Compliance and governance automation analysis
4. Security automation effectiveness assessment
5. Specific recommendations for AWS security service optimization

Provide AWS security service architecture and recommended integration patterns.
```

### Microsoft Azure Security Framework

#### Azure Security Center and Sentinel Integration
```
PROMPT: Assess Microsoft Azure security monitoring and SIEM integration.

Evaluate Azure security monitoring per Microsoft security framework:

AZURE SECURITY CENTER IMPLEMENTATION:
• Security Posture Management: Is Azure Security Center used for continuous security posture assessment?
• Policy Compliance: Are Azure Policy definitions implemented for security compliance?
• Regulatory Compliance: Are regulatory compliance dashboards monitored and managed?
• Secure Score: Is Azure Secure Score used to track and improve security posture?
• Just-in-Time Access: Is JIT VM access implemented to reduce attack surface?

AZURE SENTINEL SIEM CAPABILITIES:
• Data Connectors: Are appropriate data connectors configured for comprehensive log collection?
• Analytics Rules: Are analytics rules configured to detect security threats and incidents?
• Threat Intelligence: Is threat intelligence integrated into Sentinel for enhanced detection?
• SOAR Capabilities: Are Security Orchestration, Automation, and Response capabilities implemented?
• Investigation and Response: Are investigation and response workflows optimized in Sentinel?

THREAT PROTECTION SERVICES:
• Microsoft Defender for Cloud: Is Defender for Cloud enabled for workload protection?
• Microsoft Defender for Identity: Is Defender for Identity deployed for AD threat detection?
• Microsoft Defender for Office 365: Is Defender for Office 365 implemented for email security?
• Microsoft Defender for Endpoint: Is Defender for Endpoint deployed for endpoint protection?

IDENTITY AND ACCESS PROTECTION:
• Azure Active Directory: Is Azure AD configured with appropriate security features?
• Conditional Access: Are Conditional Access policies implemented based on risk assessment?
• Privileged Identity Management: Is PIM used for privileged access management?
• Identity Protection: Is Azure AD Identity Protection enabled for risk-based authentication?

OUTPUT REQUIREMENTS:
1. Azure security monitoring maturity assessment
2. Sentinel SIEM effectiveness evaluation
3. Threat protection service integration analysis
4. Identity protection implementation assessment
5. Specific recommendations for Azure security enhancement

Include Azure security architecture and recommended security service configurations.
```

### Google Cloud Platform (GCP) Security

#### GCP Security Command Center and Chronicle
```
PROMPT: Evaluate Google Cloud Platform security monitoring and threat detection.

Assess GCP security implementation per Google security framework:

SECURITY COMMAND CENTER:
• Asset Discovery: Is Security Command Center used for comprehensive asset inventory?
• Vulnerability Management: Are vulnerability findings managed through Security Command Center?
• Threat Detection: Are security threats detected and managed through SCC?
• Compliance Monitoring: Is compliance status monitored through Security Command Center?

CHRONICLE SIEM PLATFORM:
• Data Ingestion: Is Chronicle configured for comprehensive security data ingestion?
• Threat Detection: Are threat detection rules configured in Chronicle?
• Investigation Capabilities: Are investigation workflows optimized in Chronicle?
• Threat Intelligence: Is threat intelligence integrated into Chronicle for enhanced analysis?

GCP SECURITY SERVICES:
• Cloud Identity and Access Management: Is Cloud IAM configured with least privilege principles?
• Cloud Key Management Service: Is Cloud KMS used for encryption key management?
• Cloud Security Scanner: Is Security Scanner used for web application vulnerability assessment?
• Binary Authorization: Is Binary Authorization used for container image security?

NETWORK SECURITY:
• VPC Security: Are VPC networks configured with appropriate security controls?
• Cloud Armor: Is Cloud Armor used for DDoS protection and WAF capabilities?
• Cloud NAT: Is Cloud NAT configured securely for outbound internet access?
• Private Google Access: Is Private Google Access configured for secure API access?

OUTPUT REQUIREMENTS:
1. GCP security monitoring maturity assessment
2. Chronicle SIEM effectiveness evaluation
3. GCP security service utilization analysis
4. Network security implementation assessment
5. Specific recommendations for GCP security enhancement

Include GCP security architecture and recommended service configurations.
```

---

## Container and Kubernetes Security

### Container Security Assessment
```
PROMPT: Assess container security implementation across the container lifecycle.

Evaluate container security per cloud-native security best practices:

CONTAINER IMAGE SECURITY:
• Image Vulnerability Scanning: Are container images scanned for vulnerabilities before deployment?
• Base Image Management: Are secure, minimal base images used for container builds?
• Image Signing: Are container images signed and signature verification enforced?
• Image Registry Security: Are container registries secured with appropriate access controls?
• Supply Chain Security: Is container supply chain security managed and monitored?

CONTAINER RUNTIME SECURITY:
• Runtime Protection: Are runtime protection solutions implemented for container environments?
• Behavioral Monitoring: Is container behavior monitored for anomalies and threats?
• Resource Constraints: Are resource limits and constraints enforced for container security?
• Privilege Management: Are containers run with minimal privileges and capabilities?
• Network Segmentation: Are containers segmented appropriately at the network level?

CONTAINER ORCHESTRATION SECURITY:
• Kubernetes Security: Are Kubernetes clusters configured with security best practices?
• Pod Security Standards: Are Pod Security Standards implemented and enforced?
• Network Policies: Are Kubernetes Network Policies implemented for micro-segmentation?
• RBAC Implementation: Is Kubernetes RBAC configured with least privilege principles?
• Secrets Management: Are Kubernetes secrets managed securely?

DEVOPS SECURITY INTEGRATION:
• Shift-Left Security: Are security controls integrated into CI/CD pipelines?
• Infrastructure as Code: Are IaC templates secured and validated?
• Security Testing: Are automated security tests integrated into container deployment pipelines?
• Compliance as Code: Are compliance requirements automated in container deployments?

OUTPUT REQUIREMENTS:
1. Container security maturity assessment
2. Runtime protection effectiveness evaluation
3. Orchestration security implementation analysis
4. DevOps security integration assessment
5. Specific recommendations for container security enhancement

Include container security architecture and recommended security controls.
```

### Service Mesh Security
```
PROMPT: Evaluate service mesh security implementation for microservices architectures.

Assess service mesh security per zero trust and cloud-native principles:

SERVICE MESH ARCHITECTURE SECURITY:
• Mutual TLS: Is mTLS implemented for all service-to-service communications?
• Identity Management: Are service identities managed and verified through the service mesh?
• Certificate Management: Are certificates managed automatically through the service mesh?
• Traffic Encryption: Is all service mesh traffic encrypted in transit?

POLICY ENFORCEMENT:
• Access Control Policies: Are fine-grained access control policies implemented in the service mesh?
• Traffic Policies: Are traffic routing and load balancing policies secured?
• Rate Limiting: Are rate limiting policies implemented to prevent abuse?
• Circuit Breaking: Are circuit breaking patterns implemented for resilience?

OBSERVABILITY AND MONITORING:
• Traffic Monitoring: Is service mesh traffic monitored and analyzed for security events?
• Distributed Tracing: Is distributed tracing implemented for security investigation?
• Metrics Collection: Are security-relevant metrics collected from the service mesh?
• Audit Logging: Are service mesh activities logged for audit and compliance?

INTEGRATION AND AUTOMATION:
• CI/CD Integration: Is service mesh security integrated into CI/CD pipelines?
• Policy as Code: Are service mesh policies managed as code?
• Automated Response: Are automated security responses implemented in the service mesh?
• Compliance Automation: Are compliance requirements automated through service mesh policies?

OUTPUT REQUIREMENTS:
1. Service mesh security architecture assessment
2. Policy enforcement effectiveness evaluation
3. Observability and monitoring capability analysis
4. Integration and automation maturity assessment
5. Specific recommendations for service mesh security enhancement

Include service mesh security architecture and recommended implementation patterns.
```

---

## Cloud-Native Security Assessment

### Cloud Security Posture Management (CSPM)
```
PROMPT: Assess Cloud Security Posture Management implementation and effectiveness.

Evaluate CSPM capabilities across multi-cloud environments:

CONFIGURATION MONITORING:
• Multi-Cloud Coverage: Does CSPM cover all cloud platforms in use (AWS, Azure, GCP, etc.)?
• Configuration Baselines: Are security configuration baselines established for all cloud services?
• Drift Detection: Are configuration drifts detected and reported automatically?
• Remediation Automation: Are configuration issues remediated automatically where appropriate?

COMPLIANCE MONITORING:
• Regulatory Frameworks: Are regulatory compliance requirements monitored through CSPM?
• Custom Policies: Are custom security policies implemented and monitored?
• Compliance Reporting: Are compliance reports generated automatically for stakeholders?
• Exception Management: Are compliance exceptions managed and tracked appropriately?

RISK ASSESSMENT AND PRIORITIZATION:
• Risk Scoring: Are cloud security risks scored and prioritized based on business impact?
• Threat Context: Is threat intelligence integrated into risk assessment?
• Asset Criticality: Are asset criticality levels considered in risk prioritization?
• Remediation Prioritization: Are remediation efforts prioritized based on risk assessment?

INTEGRATION AND WORKFLOW:
• SIEM Integration: Are CSPM findings integrated with SIEM platforms?
• Ticketing Integration: Are security findings automatically created as tickets in ITSM systems?
• Developer Workflow: Are CSPM findings integrated into developer workflows?
• Executive Reporting: Are executive dashboards provided for cloud security posture?

OUTPUT REQUIREMENTS:
1. CSPM implementation maturity assessment
2. Configuration monitoring effectiveness evaluation
3. Compliance monitoring capability analysis
4. Risk assessment and prioritization effectiveness assessment
5. Specific recommendations for CSPM enhancement

Include CSPM architecture and recommended implementation strategies.
```

---

## Integration Requirements
These prompts should be used in conjunction with:
- NIST CSF prompts for framework alignment
- ISO 27002 and CIS Controls for technical control implementation
- Compliance framework prompts for regulatory integration
- Industry-specific security requirements

---

## Usage Guidelines
1. **Maturity-Based Assessment**: Progress through zero trust maturity levels systematically
2. **Cloud-First Security**: Prioritize cloud-native security solutions where appropriate
3. **Continuous Monitoring**: Implement ongoing monitoring for all assessed areas
4. **Integration Planning**: Ensure integration with existing security and compliance programs
5. **Risk-Based Implementation**: Prioritize implementations based on organizational risk assessment

---

## Output Standards
All assessments should provide:
- Current zero trust implementation maturity ratings
- Cloud security posture assessment results
- Gap analysis with specific findings
- Risk-prioritized recommendations
- Implementation roadmaps with timelines
- Success metrics and monitoring procedures
- Integration points with other security frameworks