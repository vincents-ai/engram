# Evidence Request Template Generator

## Standard Evidence Request Templates

This document provides standardized evidence request templates for common compliance gaps identified during automated audits.

## Template Usage

Each template includes:
- **Compliance Framework**: Which standard/regulation applies
- **Missing Evidence**: Specific documentation or technical proof needed
- **Business Impact**: Why this evidence is critical for compliance
- **Deadline**: Timeframe for providing evidence
- **Contact Information**: Who to send evidence to

---

## 🎮 iGaming Compliance Evidence Requests

### GLI RNG Certification Evidence Request

```markdown
## 📋 GLI-11 RNG Certification Evidence Request

**Audit Reference**: {AUDIT_ID}
**Date**: {DATE}
**Framework**: GLI-11 v3.0 Random Number Generator Standard
**Auditor**: Auditron AI Compliance System

### Missing Critical Evidence:

#### 1. GLI RNG Certification Certificate
- **Status**: ❌ Not Found
- **Required**: Current GLI-11 certification for production RNG
- **Location Expected**: `certificates/gli/rng-certification-2024.pdf`
- **Must Include**:
  - Valid certification dates (current and not expired)
  - RNG algorithm certification details
  - Laboratory signature and accreditation number

#### 2. RNG Technical Specification
- **Status**: ❌ Not Found  
- **Required**: Technical documentation of RNG implementation
- **Location Expected**: `docs/technical/rng-specification.md`
- **Must Include**:
  - Entropy source documentation (/dev/urandom, hardware RNG)
  - Seeding methodology and frequency
  - Scaling algorithm without modulo bias
  - Statistical test results (NIST SP 800-22)

#### 3. Statistical Randomness Test Reports
- **Status**: ❌ Not Found
- **Required**: Recent statistical analysis of RNG output
- **Location Expected**: `testing/rng/statistical-reports/2024/`
- **Must Include**:
  - Chi-square test results
  - Frequency distribution analysis
  - Runs test and autocorrelation analysis
  - Test date within last 90 days

### Compliance Impact:
**CRITICAL** - Operating without valid GLI certification violates gaming regulations and risks license suspension.

### Business Impact:
- Potential license revocation by gaming authorities
- Inability to operate in regulated jurisdictions  
- Loss of player trust and platform credibility
- Possible financial penalties from regulators

### Evidence Collection Deadline:
- **GLI Certificate**: 48 hours (if exists) or 30 days (if needs to be obtained)
- **Technical Documentation**: 5 business days
- **Statistical Reports**: 10 business days

### Next Steps:
1. If GLI certificate exists, please provide immediately
2. If no current certification, contact GLI testing laboratory for certification process
3. Document current RNG implementation for review
4. Schedule statistical testing if reports are outdated

### Contact Information:
- **Compliance Team**: compliance@company.com
- **Technical Review**: cto@company.com
- **Urgent Issues**: compliance-urgent@company.com

### Related Audit Findings:
- [Automated RNG scan results](./findings/gli-rng-scan.json)
- [Code analysis report](./findings/rng-implementation-review.md)
```

### UKGC Reality Check Evidence Request

```markdown
## 📋 UKGC RTS 14B Reality Check Evidence Request

**Audit Reference**: {AUDIT_ID}
**Date**: {DATE}
**Framework**: UKGC Remote Gambling and Software Technical Standards
**Auditor**: Auditron AI Compliance System

### Critical Non-Compliance Detected:

#### Automated Scan Results:
Our automated analysis detected that reality check modals may not properly interrupt gameplay:
- Reality check component found but no game pause mechanism detected
- Game state continues during modal display
- Potential for automated play to continue uninterrupted

#### Required Evidence:

#### 1. Reality Check Technical Implementation
- **Required**: Complete implementation showing game interruption
- **Must Demonstrate**:
  - Game engine pause when reality check displays
  - Prevention of all user input during modal
  - Session timer continues but gameplay stops
  - Proper resume functionality after acknowledgment

#### 2. Testing Evidence
- **Required**: Test results proving gameplay interruption
- **Must Include**:
  - Screenshots/videos of reality check during active gameplay
  - Evidence that spin/bet buttons are disabled
  - Proof that automated play stops during reality check
  - Test results across all game types

#### 3. Player Configuration Evidence
- **Required**: Proof players can configure reality check intervals
- **Must Show**:
  - Account settings page with time interval options
  - Minimum 15-minute intervals available
  - Player preference persistence across sessions

### Compliance Status:
**NON-COMPLIANT** - Current implementation violates UKGC RTS 14B requirements.

### Regulatory Risk:
- **HIGH** - UKGC license suspension risk
- Immediate regulatory action possible
- Player protection violation
- Potential fine up to £20 million or 10% of turnover

### Immediate Actions Required:
1. **Within 24 Hours**: Confirm current implementation status
2. **Within 48 Hours**: Implement game pause if not working
3. **Within 5 Days**: Provide evidence of compliant implementation
4. **Within 10 Days**: Conduct comprehensive testing

### Technical Solution Required:
```javascript
// Required implementation example:
function showRealityCheck() {
    gameEngine.pause(); // Must pause game
    disableGameControls(); // Block all inputs
    showModal(); // Display reality check
}
```

### Contact Information:
- **URGENT**: compliance-urgent@company.com
- **Technical Team**: development@company.com  
- **Regulatory Affairs**: regulatory@company.com
```

---

## 💻 IT/SaaS Compliance Evidence Requests

### SOC 2 Access Controls Evidence Request

```markdown
## 📋 SOC 2 Access Controls Evidence Request

**Audit Reference**: {AUDIT_ID}
**Date**: {DATE}
**Framework**: SOC 2 Type II Trust Services Criteria
**Controls**: CC6.1, CC6.2, CC6.3 (Access Controls)
**Auditor**: Auditron AI Compliance System

### Missing Control Documentation:

#### 1. Access Control Policy and Procedures
- **Status**: ❌ Not Found
- **Required**: Formal access control policy documentation
- **Location Expected**: `docs/policies/access-control-policy.md`
- **Must Include**:
  - User provisioning and deprovisioning procedures
  - Role-based access control matrix
  - Privileged access management procedures
  - Access review requirements (quarterly minimum)
  - Password and authentication standards

#### 2. Recent Access Review Evidence
- **Status**: ❌ Overdue (Last review: {LAST_REVIEW_DATE})
- **Required**: Completed access review within last 90 days
- **Must Include**:
  - Complete inventory of user access across all systems
  - Manager approval/certification of subordinate access
  - Documentation of excess access removal
  - Sign-off by IT and HR departments

#### 3. User Provisioning/Deprovisioning Records
- **Status**: ❌ Incomplete
- **Required**: Documentation of access lifecycle management
- **Must Include**:
  - New employee onboarding access records
  - Role change access modifications
  - Employee termination access removal records
  - Approval documentation for all access changes

### Automated Findings Summary:
- **Unprotected Admin Endpoints**: {COUNT} endpoints without authentication
- **Weak Session Configuration**: {COUNT} security issues detected
- **Database Access Issues**: {COUNT} configuration problems found

### SOC 2 Compliance Impact:
**HIGH** - Access control deficiencies are likely to result in qualified opinion from SOC 2 auditor.

### Control Testing Implications:
- CC6.1: Cannot demonstrate logical access security measures
- CC6.2: User access provisioning process not documented
- CC6.3: Access reviews not performed timely

### Evidence Collection Timeline:
- **Access Control Policy**: 5 business days (if exists) or 15 days (if needs creation)
- **Immediate Access Review**: 10 business days
- **Provisioning Records**: 3 business days (historical data)

### Recommended Actions:
1. **Immediate**: Conduct emergency access review
2. **Week 1**: Document access control procedures
3. **Week 2**: Implement automated access review process
4. **Week 3**: Train managers on access certification

### Contact Information:
- **SOC 2 Program Lead**: soc2@company.com
- **IT Operations**: itops@company.com
- **HR Partners**: hr@company.com
```

### PCI DSS Data Protection Evidence Request

```markdown
## 📋 PCI DSS Critical Compliance Gap - Data Protection

**Audit Reference**: {AUDIT_ID}
**Date**: {DATE}
**Framework**: PCI DSS v4.0 Requirement 3 (Protect Stored Cardholder Data)
**Auditor**: Auditron AI Compliance System

### 🚨 CRITICAL FINDINGS DETECTED:

#### Automated Scan Results:
- **Prohibited Data Found**: CVV/CVC storage patterns detected
- **Unencrypted PAN**: Potential cardholder data without encryption
- **Hardcoded Keys**: Encryption keys found in source code

#### Emergency Evidence Required:

#### 1. Cardholder Data Inventory (IMMEDIATE)
- **Deadline**: 24 hours
- **Required**: Complete inventory of all CHD storage
- **Must Include**:
  - Every system, database, file storing CHD
  - Data format (encrypted, tokenized, masked, plain text)
  - Business justification for storage
  - Retention period and disposal procedures

#### 2. Data Protection Implementation (IMMEDIATE)
- **Deadline**: 48 hours  
- **Required**: Evidence of CHD protection
- **Must Prove**:
  - All PAN data is encrypted with AES-256 or stronger
  - No CVV, Track, or PIN data is stored anywhere
  - Encryption keys are properly managed (not in code)
  - Database encryption (TDE) is enabled

#### 3. Network Segmentation (URGENT)
- **Deadline**: 72 hours
- **Required**: CDE isolation documentation
- **Must Include**:
  - Network diagrams showing CDE boundaries
  - Firewall rules protecting CHD systems
  - Network access controls and monitoring

### Violation Severity:
**CRITICAL** - Multiple PCI DSS violations detected that require immediate remediation.

### Business Risk:
- **Immediate**: Risk of card brand fines ($5,000-$100,000 per month)
- **Short-term**: Loss of payment processing capability
- **Long-term**: Damaged reputation and customer trust
- **Legal**: Potential liability for data breaches

### Required Immediate Actions:
1. **Hour 1**: Stop storing any prohibited data (CVV, Track, PIN)
2. **Hour 6**: Encrypt or tokenize all unprotected PAN data
3. **Day 1**: Implement emergency network controls
4. **Day 2**: Document current state and remediation plan

### Emergency Contacts:
- **PCI Compliance Officer**: pci@company.com (24/7)
- **CISO**: security@company.com
- **Payment Operations**: payments@company.com
- **Legal/Risk**: legal@company.com

### External Notifications Required:
- Acquiring bank notification within 72 hours
- QSA (Qualified Security Assessor) engagement
- Potential card brand notification
```

---

## 🛡️ Data Protection Evidence Requests

### GDPR Data Subject Rights Evidence Request

```markdown
## 📋 GDPR Data Subject Rights Implementation Evidence Request

**Audit Reference**: {AUDIT_ID}
**Date**: {DATE}
**Framework**: GDPR Articles 15-22 (Data Subject Rights)
**Auditor**: Auditron AI Compliance System

### Missing Data Subject Rights Implementation:

#### Automated Scan Results:
- **No Erasure Endpoint**: No API found for processing deletion requests
- **No Data Export**: No implementation found for data portability
- **Missing Consent Management**: No cookie consent implementation detected

#### Required Evidence:

#### 1. Data Subject Rights Portal
- **Status**: ❌ Not Found
- **Required**: Web interface for data subject requests
- **Must Include**:
  - Request forms for all GDPR rights (access, erasure, rectification, etc.)
  - Identity verification process
  - Request tracking system
  - 30-day response time tracking

#### 2. Technical Implementation
- **Status**: ❌ Not Found
- **Required**: Backend systems for rights fulfillment
- **Must Include**:
  - Data erasure implementation across all systems
  - Data export functionality (JSON/XML format)
  - Third-party notification system
  - Audit logging for all rights requests

#### 3. Data Processing Records
- **Status**: ❌ Incomplete
- **Required**: Article 30 processing records
- **Must Include**:
  - Complete data inventory and mapping
  - Legal basis for each processing activity
  - Data retention schedules
  - Third-party data sharing documentation

### Compliance Gap Analysis:
- **Article 15 (Access)**: No implementation found
- **Article 17 (Erasure)**: No implementation found
- **Article 20 (Portability)**: No implementation found
- **Article 21 (Objection)**: No implementation found

### Regulatory Risk Assessment:
**HIGH** - Complete absence of data subject rights implementation.

### Potential GDPR Fines:
- **Administrative Fines**: Up to €20 million or 4% of global turnover
- **Supervisory Action**: Formal warnings, audits, corrective orders
- **Reputation Risk**: Loss of customer trust and business

### Implementation Timeline:
- **Emergency Planning**: 48 hours
- **Basic Implementation**: 30 days
- **Full Compliance**: 60 days
- **Testing and Validation**: 90 days

### Required Immediate Actions:
1. **Week 1**: Design data subject rights workflow
2. **Week 2**: Implement basic request processing
3. **Week 3**: Develop technical deletion capabilities
4. **Week 4**: Test and validate implementation

### Contact Information:
- **Data Protection Officer**: dpo@company.com
- **Privacy Team**: privacy@company.com
- **Development Team**: dev@company.com
```

---

## 📋 Evidence Request Generation Script

```bash
#!/bin/bash

# Evidence Request Generator
# Automatically creates evidence requests based on audit findings

generate_evidence_request() {
    local framework=$1
    local finding_type=$2
    local severity=$3
    local audit_id=$4
    
    local template_file="evidence_requests/${framework}-${finding_type}-template.md"
    local output_file="evidence_requests/${audit_id}-${framework}-${finding_type}-request.md"
    
    # Replace template variables
    sed -e "s/{AUDIT_ID}/$audit_id/g" \
        -e "s/{DATE}/$(date)/g" \
        -e "s/{SEVERITY}/$severity/g" \
        "$template_file" > "$output_file"
    
    echo "Evidence request generated: $output_file"
}

# Usage examples:
# generate_evidence_request "gli" "rng_certification" "critical" "AUD-2025-001"
# generate_evidence_request "pci" "data_protection" "critical" "AUD-2025-002"
# generate_evidence_request "gdpr" "subject_rights" "high" "AUD-2025-003"
```

## 📧 Email Template for Evidence Requests

```markdown
Subject: URGENT: Compliance Evidence Required - {FRAMEWORK} {FINDING_TYPE}

Dear {RECIPIENT_NAME},

Our automated compliance audit (ID: {AUDIT_ID}) has identified missing evidence required for {FRAMEWORK} compliance.

**Severity**: {SEVERITY}
**Deadline**: {DEADLINE}
**Impact**: {BUSINESS_IMPACT}

Please review the attached detailed evidence request and provide the required documentation by the specified deadline.

For immediate questions or assistance, please contact our compliance team.

Best regards,
Auditron Compliance System
```