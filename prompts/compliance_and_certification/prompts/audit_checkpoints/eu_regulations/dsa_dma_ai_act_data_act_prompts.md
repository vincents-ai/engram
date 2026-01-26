# EU Digital & AI Framework Audit Checkpoint Prompts


  
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


## DSA (Digital Services Act) Audit Prompts

### DSA Platform Compliance Audit Prompt
```
Conduct comprehensive Digital Services Act compliance audit:

**Platform Classification:** [INTERMEDIARY_HOSTING_ONLINE_PLATFORM_VLOP]
**User Base:** [EU_USERS_COUNT_AND_CLASSIFICATION_THRESHOLDS]
**Services Scope:** [DIGITAL_SERVICES_COVERED_UNDER_DSA]
**Risk Assessment:** [SYSTEMIC_RISK_EVALUATION_FOR_VLOPS]

**Article 3-12 - General Provisions and Definitions:**

1. **Service Provider Classification Assessment**
   ```
   Determine:
   - Mere conduit services (Article 3)
   - Caching services (Article 4)
   - Hosting services (Article 5)
   - Online platform services (Article 6)
   
   Classification Impact:
   - Liability exemption applicability
   - Due diligence obligation scope
   - Risk management requirement levels
   - Transparency reporting obligations
   ```

2. **EU Establishment and Legal Representative**
   ```
   Verify:
   - EU establishment requirements (Article 10)
   - Legal representative appointment (Article 11)
   - Contact point designation (Article 12)
   - Regulatory authority communication
   
   Compliance Validation:
   - EU presence documentation
   - Representative authority and responsibilities
   - Contact information accessibility
   - Authority communication protocols
   ```

**Chapter III - Due Diligence Obligations:**

**Article 14-15 - Terms and Conditions and Transparency**
```
Assess:
- Clear and unambiguous terms and conditions
- Plain and intelligible language usage
- Information on content moderation policies
- Algorithmic systems disclosure

Terms Assessment:
- Legal clarity and comprehensibility
- Content policy transparency
- Algorithmic decision-making disclosure
- User rights and obligations clarity
```

**Article 16 - Notice and Action Mechanisms**
```
Test:
- Illegal content notification systems
- Electronic notice processing
- Expeditious action upon notification
- Notice acknowledgment and feedback

Mechanism Testing:
- Notification system accessibility
- Processing speed and effectiveness
- Action taken documentation
- User communication procedures
```

**Article 17 - Statement of Reasons**
```
Verify:
- Content restriction reasoning
- Decision communication to users
- Appeal process information
- Redress mechanism availability

Reasoning Assessment:
- Decision justification adequacy
- Communication clarity and timeliness
- Appeal process accessibility
- Redress mechanism effectiveness
```

**Article 20 - Trusted Flaggers**
```
Evaluate:
- Trusted flagger designation process
- Priority treatment implementation
- Flagger expertise verification
- Feedback and cooperation mechanisms

Trusted Flagger Framework:
- Selection criteria application
- Priority processing validation
- Expertise assessment procedures
- Collaboration effectiveness measurement
```

**Chapter IV - Additional Obligations for Online Platforms:**

**Article 24 - Transparency Reporting**
```
Examine:
- Annual transparency report publication
- Content moderation statistics
- Automated tools deployment information
- Human review process documentation

Reporting Assessment:
- Report completeness and accuracy
- Statistical data reliability
- Tool disclosure adequacy
- Process transparency validation
```

**Article 26 - Risk Assessment (for VLOPs/VLOSEs)**
```
Conduct:
- Systemic risk identification
- Risk assessment methodology validation
- Mitigation measure effectiveness
- Annual assessment compliance

Risk Framework:
- Illegal content dissemination risks
- Fundamental rights protection
- Public security and civic discourse
- Electoral process and public debate protection
```

**Article 27 - Risk Mitigation Measures**
```
Test:
- Risk mitigation implementation
- Algorithmic system adaptations
- Content curation adjustments
- Crisis response mechanisms

Mitigation Validation:
- Measure effectiveness assessment
- Implementation timeline compliance
- Adaptive response capability
- Crisis preparedness testing
```

**Article 28 - Crisis Response Mechanism**
```
Verify:
- Crisis situation identification
- Response protocol activation
- Coordinated response implementation
- Recovery and lessons learned

Crisis Testing:
- Response time measurement
- Coordination effectiveness
- Communication protocol validation
- Recovery process assessment
```

**DSA Compliance Scoring Framework:**
- **Critical Non-Compliance:** Immediate enforcement action risk
- **Major Gaps:** Formal proceedings and penalties
- **Minor Issues:** Warning notices and improvement requests
- **Compliant:** Monitoring and maintenance requirements

**Very Large Online Platform (VLOP) Additional Requirements:**
- Independent audit obligations (Article 28)
- Recommender system transparency (Article 27)
- External researcher data access (Article 31)
- Risk management and mitigation (Articles 26-27)
```

## DMA (Digital Markets Act) Audit Prompts

### DMA Gatekeeper Compliance Audit Prompt
```
Conduct Digital Markets Act gatekeeper compliance audit:

**Gatekeeper Status:** [DESIGNATED_GATEKEEPER_CLASSIFICATION]
**Core Platform Services:** [CPS_CATEGORIES_AND_SERVICES_PROVIDED]
**Market Position:** [SIGNIFICANT_IMPACT_ASSESSMENT]
**Compliance Timeline:** [6_MONTH_IMPLEMENTATION_PERIOD]

**Article 3 - Gatekeeper Designation Criteria:**

1. **Quantitative Thresholds Assessment**
   ```
   Verify:
   - Annual EEA turnover ≥€7.5 billion (3 years)
   - Market capitalization ≥€75 billion
   - Monthly active users ≥45 million EU users
   - Annual active business users ≥10,000
   
   Threshold Validation:
   - Financial performance verification
   - User count accuracy assessment
   - Business user definition compliance
   - Geographic scope validation
   ```

2. **Core Platform Services Identification**
   ```
   Assess:
   - Intermediation services (marketplaces)
   - Online search engines
   - Social networking services
   - Video-sharing platform services
   - Operating systems and cloud computing
   - Online advertising services
   
   Service Classification:
   - CPS category determination
   - Service integration assessment
   - Bundling and tying practices
   - Cross-service data usage
   ```

**Article 5-6 - Obligations for Gatekeepers:**

**Article 5 - Prohibited Practices**
```
Audit Compliance:
- No combining personal data across services without consent
- No self-preferencing in ranking and indexing
- No pre-installation of own software applications
- No restriction of user access to acquired outside platform services

Prohibition Testing:
- Data combination practice verification
- Ranking algorithm fairness assessment
- Pre-installation policy review
- Access restriction identification
```

**Article 6 - Required Facilitations**
```
Test Implementation:
- Third-party interoperability facilitation
- Business user access to generated data
- Advertising performance measurement tools
- Portability of user-generated content

Facilitation Assessment:
- Interoperability standard compliance
- Data access mechanism functionality
- Measurement tool accuracy and availability
- Portability tool effectiveness
```

**Article 6a - Additional Obligations**
```
Verify:
- Fair and reasonable general conditions
- Effective internal complaint-handling system
- Clear description of differentiated treatment
- Most favored nation clause restrictions

Obligation Validation:
- Terms and conditions fairness review
- Complaint system effectiveness testing
- Treatment transparency assessment
- MFN clause compliance verification
```

**Article 13-14 - Effective Implementation and Compliance:**

**Article 13 - Compliance Function**
```
Assess:
- Independent compliance function establishment
- Adequate resources and authority provision
- Senior management reporting structure
- Regular compliance assessment conduct

Compliance Function Evaluation:
- Independence verification
- Resource adequacy assessment
- Authority scope validation
- Assessment quality review
```

**Article 14 - External Audit**
```
Review:
- Independent auditor selection
- Audit scope and methodology
- Finding accuracy and completeness
- Corrective action implementation

Audit Quality Assessment:
- Auditor independence verification
- Methodology rigor evaluation
- Finding substantiation review
- Implementation tracking
```

**DMA Enforcement and Penalty Risk:**
- **Systematic Non-Compliance:** Up to 20% of worldwide turnover
- **Persistent Non-Compliance:** Structural and behavioral remedies
- **Procedural Violations:** Up to 1% of worldwide turnover
- **Information Provision Failures:** Periodic penalty payments

**Market Investigation and Designation Updates:**
- Emerging service designation procedures
- Quantitative threshold evolution
- Market dynamics assessment
- Regulatory dialogue maintenance
```

## AI Act Audit Prompts

### EU AI Act Risk-Based Compliance Audit Prompt
```
Conduct comprehensive EU AI Act compliance audit:

**AI System Classification:** [PROHIBITED_HIGH_RISK_LIMITED_RISK_MINIMAL_RISK]
**Deployment Context:** [USE_CASE_AND_APPLICATION_DOMAIN]
**Risk Assessment:** [RISK_LEVEL_DETERMINATION_METHODOLOGY]
**Conformity Assessment:** [CE_MARKING_AND_DOCUMENTATION_REQUIREMENTS]

**Title II - Prohibited AI Practices (Article 5):**

1. **Prohibited AI System Assessment**
   ```
   Identify and Eliminate:
   - Subliminal or manipulative techniques
   - Exploiting vulnerabilities (age, disability, social/economic situation)
   - Social scoring by public authorities
   - Real-time biometric identification in public spaces
   
   Prohibition Validation:
   - System purpose and design review
   - Target population vulnerability assessment
   - Social impact evaluation
   - Public authority usage restrictions
   ```

**Title III - High-Risk AI Systems (Articles 6-51):**

**Article 6-7 - Classification Rules**
```
Determine High-Risk Classification:
- Annex III safety components (machinery, toys, medical devices)
- Annex III standalone AI systems
- Biometric identification and categorization
- Critical infrastructure management
- Education and vocational training
- Employment and worker management
- Essential services access and management
- Law enforcement systems
- Migration, asylum, and border control
- Administration of justice and democratic processes

Classification Assessment:
- Use case mapping to Annex III
- Risk level determination methodology
- Safety component integration analysis
- Standalone system risk evaluation
```

**Article 8-15 - Conformity Assessment and CE Marking**
```
Verify:
- Conformity assessment procedure completion
- Technical documentation preparation
- CE marking affixing
- EU declaration of conformity

Assessment Validation:
- Conformity assessment body involvement
- Technical documentation completeness
- CE marking compliance
- Declaration accuracy and completeness
```

**Article 16-29 - Obligations for Providers**
```
Test Provider Compliance:
- Quality management system implementation
- Technical documentation maintenance
- Automatic logging system deployment
- Human oversight mechanism integration
- Accuracy, robustness, and cybersecurity measures

Provider Obligation Testing:
- QMS effectiveness assessment
- Documentation adequacy review
- Logging system functionality verification
- Oversight mechanism validation
- Security and performance testing
```

**Article 30-51 - User and Other Party Obligations**
```
Assess User Responsibilities:
- Instructions for use compliance
- Human oversight implementation
- Input data monitoring
- Incident and malfunction response

User Compliance Validation:
- Instruction adherence verification
- Oversight mechanism effectiveness
- Data quality management
- Incident response capability
```

**Title IV - Transparency Obligations (Article 52):**

**Article 52 - Transparency Requirements**
```
Verify:
- AI system disclosure to natural persons
- Emotion recognition and biometric categorization notification
- Deep fake content labeling
- AI-generated content detection measures

Transparency Assessment:
- Disclosure mechanism effectiveness
- Notification clarity and accessibility
- Labeling accuracy and visibility
- Detection system reliability
```

**Title V - Measures in Support of Innovation (Articles 53-55):**

**Article 53-55 - Regulatory Sandboxes and Innovation Support**
```
Evaluate:
- Regulatory sandbox participation opportunities
- Innovation support measure utilization
- Testing and development framework compliance
- Real-world testing procedures

Innovation Framework Assessment:
- Sandbox requirement compliance
- Support measure effectiveness
- Testing protocol adherence
- Development milestone tracking
```

**AI Act Governance and Risk Management:**

**Risk Management System (Article 9)**
```
Implement:
- Risk management process establishment
- Risk identification and analysis
- Risk evaluation and mitigation
- Testing, validation, and monitoring

Risk Management Validation:
- Process comprehensiveness assessment
- Identification accuracy verification
- Mitigation effectiveness testing
- Monitoring system functionality
```

**Data and Data Governance (Article 10)**
```
Assess:
- Training, validation, and testing dataset quality
- Data governance and management measures
- Bias detection and mitigation
- Data representativeness and completeness

Data Quality Framework:
- Dataset quality metrics
- Governance process effectiveness
- Bias identification and correction
- Representativeness validation
```

**AI Act Compliance Maturity Model:**
- **Level 1:** Basic risk assessment and classification
- **Level 2:** Systematic compliance implementation
- **Level 3:** Comprehensive risk management integration
- **Level 4:** Innovation-driven compliance optimization
- **Level 5:** Industry leadership and best practice development

**Enforcement and Penalty Framework:**
- **Prohibited AI Practices:** Up to €35M or 7% annual turnover
- **High-Risk System Non-Compliance:** Up to €15M or 3% annual turnover
- **Documentation and Information Obligations:** Up to €7.5M or 1.5% annual turnover
```

## Data Act Audit Prompts

### Data Act Compliance Assessment Prompt
```
Conduct EU Data Act compliance audit for data sharing and access:

**Organization Role:** [DATA_HOLDER_USER_RECIPIENT_PROCESSOR]
**Product Portfolio:** [CONNECTED_PRODUCTS_AND_RELATED_SERVICES]
**Data Sharing Framework:** [B2B_B2G_USER_ACCESS_SCENARIOS]
**Compliance Timeline:** [IMPLEMENTATION_DEADLINES_AND_PHASES]

**Chapter II - Making Data Available to Users (Articles 3-4):**

**Article 3 - Right of Access to Data**
```
Assess:
- User data access right implementation
- Connected product data generation transparency
- Data access request handling procedures
- Data format and interoperability standards

User Access Framework:
- Access mechanism functionality
- Data comprehensiveness and accuracy
- Request response timeliness
- Format standardization compliance
```

**Article 4 - Obligations of Data Holders**
```
Verify:
- Data availability without undue delay
- Accessible and user-friendly format provision
- Continuous and real-time access facilitation
- Data quality and completeness assurance

Data Holder Compliance:
- Availability mechanism effectiveness
- Format accessibility validation
- Real-time access capability testing
- Quality assurance process verification
```

**Chapter III - Making Data Available to Third Parties (Articles 5-7):**

**Article 5 - Data Sharing with Third Parties**
```
Test:
- User consent-based data sharing
- Third-party data access facilitation
- Data sharing agreement templates
- Dispute resolution mechanism implementation

Sharing Framework Validation:
- Consent mechanism effectiveness
- Third-party access process efficiency
- Agreement template adequacy
- Dispute resolution accessibility
```

**Article 6 - Compensation and Article 7 - Unfair Terms**
```
Evaluate:
- Reasonable compensation determination
- Cost-based pricing methodology
- Unfair contractual term identification
- Negotiation process fairness

Commercial Term Assessment:
- Compensation reasonableness verification
- Pricing transparency evaluation
- Term fairness analysis
- Negotiation balance assessment
```

**Chapter V - Business-to-Government Data Sharing (Articles 15-21):**

**Article 15-18 - Public Emergency and Article 19-21 - Public Interest**
```
Assess:
- Emergency response data sharing capability
- Public interest data provision mechanisms
- Proportionality and necessity compliance
- Compensation and safeguard implementation

B2G Sharing Validation:
- Emergency response readiness
- Public interest data identification
- Proportionality assessment procedures
- Safeguard effectiveness verification
```

**Chapter VI - Switching Between Data Processing Services (Articles 23-30):**

**Article 23-26 - Cloud Switching Facilitation**
```
Verify:
- Data portability mechanism implementation
- Switching assistance provision
- Technical and contractual barriers removal
- Minimum notice period compliance

Switching Framework Testing:
- Portability tool functionality
- Assistance effectiveness measurement
- Barrier identification and removal
- Notice period adherence verification
```

**Chapter VII - Interoperability Standards (Articles 31-34):**

**Article 31-34 - Standard Development and Implementation**
```
Test:
- Interoperability standard compliance
- Data space participation readiness
- Technical specification adherence
- Common European data space integration

Interoperability Assessment:
- Standard implementation verification
- Data space integration capability
- Specification compliance testing
- Integration effectiveness measurement
```

**Data Act Risk and Compliance Framework:**

**Data Protection and Security Measures**
```
Validate:
- GDPR compliance maintenance
- Trade secret protection implementation
- Cybersecurity measure deployment
- International transfer safeguards

Protection Framework:
- Privacy compliance verification
- Trade secret safeguard effectiveness
- Security control implementation
- Transfer mechanism validation
```

**Enforcement and Remedy Mechanisms**
```
Assess:
- National competent authority cooperation
- Dispute resolution system effectiveness
- Penalty and fine framework readiness
- Alternative dispute resolution availability

Enforcement Readiness:
- Authority communication protocols
- Resolution system accessibility
- Penalty risk assessment
- ADR mechanism effectiveness
```

**Data Act Compliance Roadmap:**
- **Phase 1:** Data access right implementation (12 months)
- **Phase 2:** Third-party sharing framework (18 months)
- **Phase 3:** B2G sharing capability (24 months)
- **Phase 4:** Full interoperability integration (36 months)

**Strategic Compliance Benefits:**
- Enhanced data monetization opportunities
- Improved innovation and competitiveness
- Strengthened public-private partnerships
- Accelerated digital transformation
```

## Integrated EU Digital & AI Compliance Dashboard Prompt
```
Create comprehensive EU digital transformation compliance dashboard:

**Regulatory Coverage:** DSA + DMA + AI Act + Data Act
**Platform Assessment:** [COMPLETE_DIGITAL_SERVICE_ECOSYSTEM]
**Compliance Harmonization:** [CROSS_REGULATION_SYNERGY_ANALYSIS]

**Unified Digital Compliance Matrix:**

1. **Digital Services and Platform Obligations**
   ```
   DSA Compliance Status:
   - Due Diligence Obligations: [SCORE]/100
   - Risk Assessment (VLOP): [SCORE]/100
   - Transparency Reporting: [SCORE]/100
   - Crisis Response Readiness: [SCORE]/100
   
   DMA Gatekeeper Compliance:
   - Prohibited Practices Elimination: [SCORE]/100
   - Facilitation Requirements: [SCORE]/100
   - Compliance Function Effectiveness: [SCORE]/100
   - External Audit Readiness: [SCORE]/100
   ```

2. **AI System Risk Management**
   ```
   AI Act Classification Accuracy:
   - Prohibited Practice Identification: [COMPLETE/INCOMPLETE]
   - High-Risk System Assessment: [SCORE]/100
   - Transparency Obligation Compliance: [SCORE]/100
   - Innovation Support Utilization: [SCORE]/100
   
   Risk Management Maturity:
   - Risk Assessment Process: [MATURITY_LEVEL_1_5]
   - Data Governance Framework: [MATURITY_LEVEL_1_5]
   - Human Oversight Implementation: [MATURITY_LEVEL_1_5]
   ```

3. **Data Access and Sharing Framework**
   ```
   Data Act Implementation:
   - User Access Rights: [SCORE]/100
   - Third-Party Sharing: [SCORE]/100
   - B2G Data Provision: [SCORE]/100
   - Interoperability Standards: [SCORE]/100
   
   Data Ecosystem Readiness:
   - Connected Product Integration: [SCORE]/100
   - Data Space Participation: [SCORE]/100
   - Switching Facilitation: [SCORE]/100
   ```

**Cross-Regulation Integration Analysis:**
- Data sharing alignment (Data Act + GDPR + DSA)
- AI system transparency (AI Act + DSA + DMA)
- Platform governance harmonization (DSA + DMA)
- Innovation framework coordination (AI Act + Data Act)

**EU Digital Single Market Readiness:**
- Cross-border service provision capability
- Single market access optimization
- Regulatory fragmentation mitigation
- Digital sovereignty compliance

**Strategic Implementation Timeline:**
- **Immediate (0-6 months):** Critical compliance gaps
- **Short-term (6-18 months):** System integration and optimization
- **Medium-term (18-36 months):** Advanced feature development
- **Long-term (36+ months):** Innovation and market leadership

**Stakeholder Engagement Framework:**
- European Commission liaison protocols
- National competent authority coordination
- Industry association participation
- Academic and research collaboration

**Continuous Monitoring and Adaptation:**
- Regulatory development tracking
- Implementation guidance updates
- Best practice sharing networks
- Compliance maturity benchmarking
```