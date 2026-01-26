# GLI (Gaming Laboratories International) Audit Checkpoint Prompts


  
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


## GLI-11 Gaming Device Standards

### Initial Assessment Prompt
```
You are conducting a GLI-11 compliance audit for gaming devices. Review the following technical specifications and implementation:

**System Context:** [GAMING_PLATFORM_TYPE]
**Audit Scope:** [DEVICES/SOFTWARE_MODULES]
**Previous Findings:** [PRIOR_AUDIT_RESULTS]

Evaluate compliance with GLI-11 standards across these critical areas:

1. **Random Number Generation (RNG)**
   - Analyze entropy sources and seeding mechanisms
   - Verify statistical randomness and unpredictability
   - Check for proper scaling and range mapping
   - Validate absence of exploitable patterns

2. **Game Logic Integrity**
   - Review game mathematics and probability calculations
   - Verify Return to Player (RTP) accuracy
   - Check for proper handling of edge cases
   - Validate game state persistence and recovery

3. **Security Controls**
   - Assess tamper detection and response mechanisms
   - Review access controls and authentication
   - Verify data encryption and secure communications
   - Check audit trail completeness and integrity

4. **Hardware Interface Standards**
   - Validate peripheral device communications
   - Check for proper error handling and recovery
   - Verify compliance with electrical specifications
   - Assess physical security measures

**Evidence Required:**
- Technical documentation and specifications
- Source code for critical game components
- Test results and statistical analysis
- Security assessment reports
- Hardware validation certificates

**Deliverable:** Provide a detailed compliance assessment with specific findings, risk ratings, and remediation recommendations aligned with GLI-11 requirements.
```

### RNG Certification Prompt
```
Conduct a comprehensive GLI-11 Random Number Generator certification audit:

**RNG Implementation Details:** [RNG_TYPE_AND_VERSION]
**Statistical Testing Period:** [TEST_DURATION]
**Gaming Application:** [GAME_TYPES_SERVED]

**Audit Checklist:**

1. **Entropy Source Validation**
   - [ ] Hardware-based entropy sources documented
   - [ ] Entropy collection methods verified
   - [ ] Minimum entropy rate calculations confirmed
   - [ ] Entropy source failure detection implemented

2. **Algorithm Compliance**
   - [ ] NIST-approved algorithms used
   - [ ] Proper seeding procedures implemented
   - [ ] Secure key management practices
   - [ ] Algorithm implementation verified against specifications

3. **Statistical Testing Results**
   - [ ] NIST SP 800-22 test suite results (all 15 tests)
   - [ ] Diehard battery test results
   - [ ] TestU01 BigCrush test results
   - [ ] Custom gaming-specific statistical tests

4. **Output Analysis**
   - [ ] Uniform distribution verification
   - [ ] Independence of successive values
   - [ ] Absence of exploitable patterns
   - [ ] Proper scaling to game ranges

5. **Security Assessment**
   - [ ] Protection against state compromise
   - [ ] Secure storage of seeds and keys
   - [ ] Access control to RNG functions
   - [ ] Tamper detection mechanisms

**Risk Assessment Framework:**
- Critical: Any statistical test failure or security vulnerability
- High: Implementation deviations from approved algorithms
- Medium: Documentation gaps or procedural weaknesses
- Low: Minor configuration or monitoring issues

**Certification Decision Tree:**
IF all statistical tests pass AND security controls adequate AND implementation compliant
THEN recommend certification
ELSE identify specific remediation requirements
```

### Game Logic Audit Prompt
```
Perform GLI-11 game logic and mathematics audit:

**Game Portfolio:** [GAME_TYPES_AND_VARIANTS]
**Mathematical Models:** [PAYTABLE_AND_PROBABILITY_MODELS]
**Audit Timeframe:** [AUDIT_PERIOD]

**Mathematical Verification Process:**

1. **Paytable Analysis**
   ```
   For each game variant:
   - Verify theoretical RTP calculations
   - Validate probability distributions
   - Check for mathematical errors in payout logic
   - Confirm compliance with jurisdictional RTP requirements
   ```

2. **Game Flow Validation**
   ```
   Trace execution paths for:
   - Normal gameplay scenarios
   - Bonus feature triggers
   - Progressive jackpot mechanics
   - Error conditions and recovery
   ```

3. **Statistical Correlation Testing**
   ```
   Analyze 1M+ game rounds for:
   - RTP convergence patterns
   - Volatility measurements
   - Bonus frequency validation
   - Progressive contribution accuracy
   ```

4. **Edge Case Testing**
   ```
   Test boundary conditions:
   - Maximum/minimum bet scenarios
   - Progressive jackpot resets
   - Concurrent player interactions
   - System failure recovery
   ```

**Code Review Checklist:**
- [ ] Game mathematics implementation matches approved models
- [ ] No hidden or undocumented features
- [ ] Proper handling of fractional currency units
- [ ] Accurate game history and replay functionality
- [ ] Compliance with maximum payout limitations

**Documentation Requirements:**
1. Mathematical analysis reports
2. Source code annotations and comments
3. Test case results and statistical summaries
4. Paytable verification certificates
5. Progressive jackpot contribution calculations

**Approval Criteria:**
- RTP variance < 0.01% from theoretical
- All statistical tests within acceptable ranges
- No exploitable game logic vulnerabilities
- Complete and accurate documentation
```

## GLI-19 Interactive Gaming Systems

### Platform Security Audit Prompt
```
Conduct GLI-19 compliance audit for interactive gaming platform:

**Platform Architecture:** [SYSTEM_ARCHITECTURE_OVERVIEW]
**User Base:** [CONCURRENT_USERS_AND_GEOGRAPHIC_SCOPE]
**Integration Points:** [PAYMENT_PROCESSORS_AND_THIRD_PARTY_SERVICES]

**Security Assessment Framework:**

1. **Authentication and Authorization**
   ```
   Evaluate:
   - Multi-factor authentication implementation
   - Session management and timeout controls
   - Role-based access control (RBAC) effectiveness
   - Account lockout and fraud prevention
   ```

2. **Data Protection**
   ```
   Verify:
   - Encryption at rest and in transit (AES-256 minimum)
   - PCI DSS compliance for payment data
   - Player data privacy controls
   - Secure data backup and recovery
   ```

3. **Network Security**
   ```
   Test:
   - Firewall configurations and rules
   - Intrusion detection/prevention systems
   - DDoS protection mechanisms
   - Secure communication protocols (TLS 1.3+)
   ```

4. **Application Security**
   ```
   Assess:
   - Input validation and sanitization
   - SQL injection prevention
   - Cross-site scripting (XSS) protection
   - CSRF token implementation
   ```

**Penetration Testing Scope:**
- External network infrastructure
- Web application attack vectors
- Mobile application security
- API endpoint vulnerabilities
- Social engineering susceptibility

**Compliance Verification:**
- [ ] Player registration and KYC processes
- [ ] Geolocation verification accuracy
- [ ] Responsible gaming tools implementation
- [ ] Anti-money laundering controls
- [ ] Game integrity monitoring systems

**Risk Classification:**
- **Critical:** Direct impact on player funds or personal data
- **High:** Potential for fraud or regulatory violations
- **Medium:** Operational disruption or minor data exposure
- **Low:** Limited impact on system or user experience
```

### Player Protection Audit Prompt
```
Audit GLI-19 player protection and responsible gaming controls:

**Operator Profile:** [OPERATOR_LICENSE_AND_JURISDICTIONS]
**Player Demographics:** [USER_BASE_CHARACTERISTICS]
**Responsible Gaming Tools:** [IMPLEMENTED_PROTECTION_MEASURES]

**Player Protection Assessment:**

1. **Self-Exclusion Systems**
   ```
   Verify:
   - Multi-channel exclusion options (time-based, permanent)
   - Cross-platform exclusion enforcement
   - Third-party exclusion database integration
   - Exclusion override prevention controls
   ```

2. **Deposit and Spending Limits**
   ```
   Test:
   - Daily/weekly/monthly limit setting
   - Cooling-off periods for limit increases
   - Real-time limit enforcement
   - Limit bypass prevention mechanisms
   ```

3. **Reality Check Features**
   ```
   Evaluate:
   - Configurable session time reminders
   - Spending amount notifications
   - Loss limit warnings
   - Popup frequency and effectiveness
   ```

4. **Player Behavior Monitoring**
   ```
   Assess:
   - Automated risk indicator detection
   - Escalation procedures for at-risk players
   - Intervention effectiveness tracking
   - Staff training on problem gambling
   ```

**Age Verification Process:**
- [ ] Document verification requirements
- [ ] Real-time identity validation
- [ ] Ongoing monitoring for fraudulent accounts
- [ ] Minor access prevention controls

**Financial Protection Controls:**
- [ ] Segregated player fund accounts
- [ ] Withdrawal processing timeframes
- [ ] Dispute resolution procedures
- [ ] Fraud detection and prevention

**Audit Trail Requirements:**
- Complete player interaction logs
- Responsible gaming tool usage tracking
- Staff intervention documentation
- Regulatory reporting capabilities

**Compliance Scoring:**
Calculate weighted scores across:
- Technical implementation (40%)
- Process effectiveness (30%)
- Staff training and awareness (20%)
- Documentation and reporting (10%)
```

## GLI-33 Event Wagering Systems

### Sports Betting Platform Audit Prompt
```
Conduct GLI-33 compliance audit for event wagering systems:

**Betting Platform:** [SPORTSBOOK_SOFTWARE_AND_VERSION]
**Sports Coverage:** [SUPPORTED_SPORTS_AND_MARKETS]
**Integration Partners:** [ODDS_PROVIDERS_AND_DATA_FEEDS]

**Core System Validation:**

1. **Odds Management and Distribution**
   ```
   Verify:
   - Real-time odds updating mechanisms
   - Market suspension and reopening controls
   - Maximum exposure and liability management
   - Arbitrage opportunity detection and prevention
   ```

2. **Bet Acceptance and Processing**
   ```
   Test:
   - Bet placement validation rules
   - Stake limit enforcement
   - In-play betting controls
   - Bet cancellation and voiding procedures
   ```

3. **Settlement and Payout Systems**
   ```
   Audit:
   - Result data source validation
   - Automated settlement accuracy
   - Manual settlement procedures
   - Dispute resolution workflows
   ```

4. **Risk Management Controls**
   ```
   Assess:
   - Player profiling and monitoring
   - Suspicious betting pattern detection
   - Market manipulation prevention
   - Regulatory reporting capabilities
   ```

**Data Integrity Verification:**
- [ ] Official data feed redundancy
- [ ] Data source authentication
- [ ] Result verification procedures
- [ ] Historical data retention and archival

**Market Operations Testing:**
- [ ] Pre-match market creation and management
- [ ] Live betting delay and suspension controls
- [ ] Cash-out functionality accuracy
- [ ] Partial settlement processing

**Integration Security:**
- [ ] Third-party API authentication
- [ ] Data encryption in transit
- [ ] Feed interruption handling
- [ ] Backup system activation procedures

**Compliance Reporting:**
Generate detailed assessment covering:
- Technical infrastructure adequacy
- Operational procedure effectiveness
- Risk management framework robustness
- Regulatory compliance status
```

## Comprehensive GLI Audit Summary Prompt
```
Compile comprehensive GLI compliance audit summary:

**Audit Scope:** [ALL_GLI_STANDARDS_COVERED]
**Assessment Period:** [AUDIT_DURATION_AND_DATES]
**Systems Evaluated:** [COMPLETE_SYSTEM_INVENTORY]

**Executive Summary Framework:**

1. **Overall Compliance Rating**
   ```
   Calculate composite score:
   - GLI-11 Gaming Devices: [SCORE]/100
   - GLI-19 Interactive Gaming: [SCORE]/100
   - GLI-33 Event Wagering: [SCORE]/100
   - Weighted Average: [OVERALL_SCORE]/100
   ```

2. **Critical Findings Summary**
   ```
   Prioritize by impact:
   - Immediate certification blockers
   - High-risk compliance gaps
   - Medium-priority improvements
   - Low-impact recommendations
   ```

3. **Remediation Roadmap**
   ```
   Phase 1 (0-30 days): Critical security and compliance fixes
   Phase 2 (30-90 days): Process improvements and documentation
   Phase 3 (90-180 days): System enhancements and optimization
   Phase 4 (180+ days): Strategic improvements and upgrades
   ```

4. **Certification Recommendation**
   ```
   IF overall_score >= 85 AND critical_findings == 0
   THEN recommend_certification
   ELSE provide_conditional_approval_with_requirements
   ```

**Regulatory Communication Package:**
- Executive summary for senior management
- Technical findings for development teams
- Compliance gaps for legal and regulatory affairs
- Timeline and budget for remediation activities

**Ongoing Monitoring Plan:**
- Quarterly compliance assessments
- Continuous security monitoring
- Annual recertification requirements
- Incident response and reporting procedures
```