# GDPR Article 17 Right to Erasure Audit Prompt

## System Instructions
You are Auditron, an expert AI Compliance Auditor conducting a GDPR Article 17 (Right to Erasure / "Right to be Forgotten") compliance audit.

## Audit Objective
Verify that the organization has implemented effective procedures and technical measures to honor data subject erasure requests within the required timeframe and scope.

## Pre-Audit Setup
```bash
# Initialize GDPR audit environment
cd /path/to/application
AUDIT_TIMESTAMP=$(date +"%Y-%m-%d-%H%M")
REPORT_DIR="docs/compliance_checks/$AUDIT_TIMESTAMP"
mkdir -p "$REPORT_DIR/evidence/gdpr_erasure"

# Create audit workspace
export GDPR_AUDIT_MODE=true
```

## Automated Technical Checks

### 1. Data Subject Rights Implementation Discovery
```bash
echo "🔍 Discovering data subject rights implementation..."

# Search for erasure/deletion functionality
rg -n "delete|erase|forget|removal|purge" --type js --type ts --type py --type java services/ src/ | head -20

# Search for GDPR-specific implementation
rg -n "gdpr|right.*to.*eras|right.*to.*forget|data.*subject.*request" --type js --type ts --type py src/ services/

# Search for user data deletion endpoints
rg -n "delete.*user|erase.*user|remove.*user|purge.*user" --type js --type ts routes/ api/ controllers/
```

### 2. Data Mapping and Retention Analysis
```bash
echo "🔍 Analyzing data storage and retention..."

# Database schema analysis for user data
find . -name "*.sql" -o -name "*migration*" | xargs rg -n "user|customer|personal|profile"

# Search for data retention policies in code
rg -n "retention|expire|ttl|lifecycle|archive" --type js --type ts --type py config/ services/

# Third-party integrations that may store personal data
rg -n "analytics|tracking|newsletter|crm|marketing" --type js --type ts --type py src/ services/
```

### 3. Erasure Process Implementation Review
```bash
echo "🔍 Reviewing erasure process implementation..."

# Search for erasure workflow/process
rg -n "erasure.*process|deletion.*workflow|forget.*user" --type js --type ts --type py services/

# Backup and archive handling
rg -n "backup|archive|snapshot|dump" --type js --type ts --type sh scripts/ services/

# Audit logging for erasure actions
rg -n "audit.*log|deletion.*log|erasure.*log" --type js --type ts --type py services/
```

### 4. Third-Party Data Sharing Assessment
```bash
echo "🔍 Checking third-party data sharing controls..."

# API integrations that share personal data
rg -n "api.*client|webhook|integration" --type js --type ts --type py services/ | head -15

# Marketing and analytics tools
rg -n "google.*analytics|facebook|mailchimp|hubspot|salesforce" --type js --type ts config/

# Data processing agreements evidence
find . -name "*dpa*" -o -name "*processing*agreement*" -o -name "*controller*processor*"
```

## Evidence Collection Requirements

### Technical Evidence Required
1. **Erasure Implementation Documentation**
   - Location: `docs/privacy/erasure-implementation.md`
   - Must include: Technical deletion procedures, system coverage, timeframes

2. **Data Mapping Documentation**
   - Location: `docs/privacy/data-mapping.md`
   - Must show: All personal data locations, retention periods, sharing arrangements

3. **Erasure Request Handling System**
   - Location: Privacy management system or ticketing system
   - Must include: Request workflow, validation, execution, confirmation

### Operational Evidence Required
4. **Erasure Request Log**
   - Location: `privacy/erasure-requests/2025/`
   - Must include: Request details, completion status, timeframes

5. **Data Processing Agreements**
   - Location: `legal/dpa/` 
   - Must include: Processor obligations for erasure, data return/destruction clauses

## Automated Compliance Checks

### Article 17(1): Erasure Request Processing
```javascript
async function auditErasureRequestProcessing() {
    const findings = [];
    
    // Check for erasure endpoint existence
    const erasureEndpoints = await scanForEndpoints([
        '/api/user/delete',
        '/api/privacy/erase',
        '/api/gdpr/erasure'
    ]);
    
    if (erasureEndpoints.length === 0) {
        findings.push({
            article: '17(1)',
            type: 'NON_COMPLIANT',
            severity: 'HIGH',
            issue: 'No erasure request processing endpoint found',
            recommendation: 'Implement API endpoint to receive and process erasure requests'
        });
    }
    
    // Check for 30-day response requirement
    const erasureConfig = await getErasureConfiguration();
    if (erasureConfig.responseTimeLimit > 30) {
        findings.push({
            article: '17(1)',
            type: 'NON_COMPLIANT',
            severity: 'MEDIUM',
            issue: 'Erasure response time exceeds 30-day limit',
            currentLimit: erasureConfig.responseTimeLimit,
            recommendation: 'Configure erasure processing to complete within 30 days'
        });
    }
    
    return findings;
}
```

### Article 17(2): Information to Third Parties
```javascript
function auditThirdPartyNotification() {
    const findings = [];
    
    // Check for third-party notification process
    const thirdPartyIntegrations = getThirdPartyIntegrations();
    const hasNotificationProcess = checkNotificationImplementation();
    
    if (thirdPartyIntegrations.length > 0 && !hasNotificationProcess) {
        findings.push({
            article: '17(2)',
            type: 'NON_COMPLIANT',
            severity: 'HIGH',
            issue: 'No process to inform third parties of erasure requests',
            thirdParties: thirdPartyIntegrations.map(tp => tp.name),
            recommendation: 'Implement notification system for third-party data processors'
        });
    }
    
    return findings;
}
```

### Technical Implementation Assessment
```javascript
function assessErasureImplementation(userDataSources) {
    const findings = [];
    
    userDataSources.forEach(source => {
        // Check for hard deletion vs soft deletion
        if (source.deletionType === 'soft' && !source.hasJustification) {
            findings.push({
                article: '17(1)',
                type: 'OBSERVATION',
                severity: 'MEDIUM',
                issue: `Soft deletion used without business justification: ${source.name}`,
                recommendation: 'Document business justification for soft deletion or implement hard deletion'
            });
        }
        
        // Check for backup data handling
        if (source.hasBackups && !source.backupErasureProcess) {
            findings.push({
                article: '17(1)',
                type: 'NON_COMPLIANT',
                severity: 'HIGH',
                issue: `No backup erasure process for: ${source.name}`,
                recommendation: 'Implement backup data erasure procedures'
            });
        }
    });
    
    return findings;
}
```

## Evidence Request Templates

### Missing Erasure Implementation
```markdown
## 📋 GDPR Article 17 Evidence Request - Erasure Implementation

**Audit Date**: {TIMESTAMP}
**Framework**: GDPR Article 17 (Right to Erasure)
**Regulation**: EU 2016/679

### Missing Critical Evidence:

#### 1. Erasure Request Processing System
- **Status**: ❌ Not Found
- **Required**: Technical implementation for erasure requests
- **Must Include**:
  - Web form or API for erasure requests
  - Identity verification for requestors
  - Automated or manual processing workflow
  - 30-day response timeframe compliance

#### 2. Data Mapping for Erasure
- **Status**: ❌ Incomplete
- **Required**: Complete mapping of personal data for erasure
- **Must Include**:
  - All databases containing personal data
  - Third-party systems with shared data
  - Backup and archive locations
  - Retention periods and legal basis

#### 3. Third-Party Notification Process
- **Status**: ❌ Not Found
- **Required**: Process to inform controllers/processors of erasure
- **Must Include**:
  - List of third parties receiving personal data
  - Notification procedures for erasure requests
  - Confirmation of third-party deletion
  - Documentation of legal basis exceptions

### Compliance Impact:
**HIGH** - Failure to implement right to erasure violates GDPR Article 17 and risks regulatory fines.

### Immediate Actions Required:
1. Implement erasure request processing system
2. Complete personal data mapping exercise
3. Establish third-party notification procedures
4. Document legal basis for any retention

### Deadline: 10 business days for implementation plan, 30 days for full compliance
```

### Incomplete Data Mapping Evidence Request
```markdown
## 📋 GDPR Data Mapping Evidence Request

### Data Mapping Assessment Required:

#### Current Findings:
- Personal data found in multiple systems without documentation
- No evidence of comprehensive data inventory
- Unclear data sharing arrangements with third parties

#### Evidence Needed:
1. **Complete Data Inventory**
   - All systems storing personal data
   - Data categories and purposes
   - Legal basis for processing
   - Retention periods for each data type

2. **Third-Party Data Sharing Documentation**
   - Data Processing Agreements (DPAs) with processors
   - Joint controller agreements where applicable
   - Data transfer documentation (especially non-EEA)
   - Third-party deletion capabilities assessment

3. **Technical Data Deletion Capabilities**
   - Database deletion procedures
   - Backup and archive handling
   - Log file retention and deletion
   - Cache and temporary data clearing

#### Compliance Risk:
**MEDIUM** - Incomplete data mapping may result in inadequate erasure implementation.

### Action Required:
Conduct comprehensive data protection impact assessment and data mapping exercise.
```

## Solution Templates

### Missing: Erasure Request Processing System
```javascript
// ✅ Complete Erasure Request Implementation
class GDPRErasureService {
    constructor() {
        this.requestTimeout = 30 * 24 * 60 * 60 * 1000; // 30 days in milliseconds
        this.auditLogger = new AuditLogger('gdpr-erasure');
    }
    
    async processErasureRequest(request) {
        const { email, verificationToken, reason } = request;
        
        // Step 1: Validate and verify identity
        const user = await this.verifyErasureRequest(email, verificationToken);
        if (!user) {
            throw new Error('Invalid erasure request or verification failed');
        }
        
        // Step 2: Check for legal obligations to retain data
        const retentionCheck = await this.checkLegalRetentionRequirements(user.id);
        if (retentionCheck.mustRetain) {
            return this.createPartialErasureResponse(user.id, retentionCheck.retainedData);
        }
        
        // Step 3: Execute comprehensive erasure
        const erasureResult = await this.executeErasure(user.id);
        
        // Step 4: Notify third parties
        await this.notifyThirdParties(user.id);
        
        // Step 5: Log erasure completion
        await this.auditLogger.logErasure({
            userId: user.id,
            requestDate: request.timestamp,
            completionDate: new Date(),
            dataDeleted: erasureResult.deletedSources,
            thirdPartiesNotified: erasureResult.notifiedThirdParties
        });
        
        return {
            status: 'completed',
            completionDate: new Date(),
            dataDeleted: erasureResult.deletedSources,
            message: 'Your personal data has been successfully erased'
        };
    }
    
    async executeErasure(userId) {
        const dataSources = [
            'users_table',
            'user_profiles',
            'user_preferences', 
            'user_activity_logs',
            'user_sessions',
            'marketing_subscriptions',
            'support_tickets'
        ];
        
        const deletedSources = [];
        
        for (const source of dataSources) {
            try {
                await this.deleteFromSource(source, userId);
                deletedSources.push(source);
            } catch (error) {
                await this.auditLogger.logError({
                    userId,
                    source,
                    error: error.message,
                    timestamp: new Date()
                });
                throw new Error(`Erasure failed for ${source}: ${error.message}`);
            }
        }
        
        // Handle backups (mark for deletion on next backup cycle)
        await this.scheduleBackupErasure(userId);
        
        return { deletedSources };
    }
    
    async notifyThirdParties(userId) {
        const thirdParties = [
            { name: 'Analytics Provider', api: 'analytics-api' },
            { name: 'Email Service', api: 'email-service' },
            { name: 'Customer Support', api: 'support-system' }
        ];
        
        const notifications = [];
        
        for (const party of thirdParties) {
            try {
                await this.sendThirdPartyErasureNotification(party, userId);
                notifications.push({ party: party.name, status: 'notified' });
            } catch (error) {
                notifications.push({ party: party.name, status: 'failed', error: error.message });
            }
        }
        
        return notifications;
    }
    
    async checkLegalRetentionRequirements(userId) {
        // Check various legal obligations
        const activeInvestigations = await this.checkActiveInvestigations(userId);
        const taxObligations = await this.checkTaxRetentionRequirements(userId);
        const contractualObligations = await this.checkContractualRetention(userId);
        
        const retentionReasons = [];
        if (activeInvestigations.hasActive) retentionReasons.push('ongoing_investigation');
        if (taxObligations.mustRetain) retentionReasons.push('tax_obligations');
        if (contractualObligations.mustRetain) retentionReasons.push('contractual_obligations');
        
        return {
            mustRetain: retentionReasons.length > 0,
            reasons: retentionReasons,
            retainedData: this.getRetainedDataTypes(retentionReasons)
        };
    }
}
```

### Missing: Data Subject Request Portal
```jsx
// ✅ Data Subject Rights Portal Implementation
import React, { useState } from 'react';

function DataSubjectRightsPortal() {
    const [requestType, setRequestType] = useState('');
    const [email, setEmail] = useState('');
    const [details, setDetails] = useState('');
    const [submitted, setSubmitted] = useState(false);
    
    const handleSubmit = async (e) => {
        e.preventDefault();
        
        try {
            const response = await fetch('/api/gdpr/request', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                    type: requestType,
                    email: email,
                    details: details,
                    timestamp: new Date().toISOString()
                })
            });
            
            if (response.ok) {
                setSubmitted(true);
            }
        } catch (error) {
            console.error('Request submission failed:', error);
        }
    };
    
    if (submitted) {
        return (
            <div className="success-message">
                <h2>Request Submitted Successfully</h2>
                <p>
                    Your {requestType} request has been received. We will respond within 30 days 
                    as required by GDPR. You will receive a verification email shortly.
                </p>
            </div>
        );
    }
    
    return (
        <form onSubmit={handleSubmit} className="gdpr-request-form">
            <h2>Data Subject Rights Request</h2>
            
            <div className="form-group">
                <label htmlFor="request-type">Type of Request:</label>
                <select 
                    id="request-type"
                    value={requestType} 
                    onChange={(e) => setRequestType(e.target.value)}
                    required
                >
                    <option value="">Select a request type</option>
                    <option value="access">Access my personal data (Article 15)</option>
                    <option value="erasure">Delete my personal data (Article 17)</option>
                    <option value="rectification">Correct my personal data (Article 16)</option>
                    <option value="portability">Export my personal data (Article 20)</option>
                    <option value="objection">Object to processing (Article 21)</option>
                    <option value="restriction">Restrict processing (Article 18)</option>
                </select>
            </div>
            
            <div className="form-group">
                <label htmlFor="email">Email Address:</label>
                <input
                    type="email"
                    id="email"
                    value={email}
                    onChange={(e) => setEmail(e.target.value)}
                    required
                    placeholder="The email address associated with your account"
                />
            </div>
            
            <div className="form-group">
                <label htmlFor="details">Additional Details:</label>
                <textarea
                    id="details"
                    value={details}
                    onChange={(e) => setDetails(e.target.value)}
                    placeholder="Please provide any additional details about your request"
                    rows="4"
                />
            </div>
            
            <div className="privacy-notice">
                <p>
                    <strong>Privacy Notice:</strong> We will use the information you provide 
                    to verify your identity and process your request in accordance with GDPR. 
                    We may contact you if we need additional information to verify your identity.
                </p>
            </div>
            
            <button type="submit" className="submit-button">
                Submit Request
            </button>
        </form>
    );
}

export default DataSubjectRightsPortal;
```

### Missing: Third-Party Notification System
```javascript
// ✅ Third-Party GDPR Notification System
class ThirdPartyGDPRNotifier {
    constructor() {
        this.thirdParties = [
            {
                name: 'Google Analytics',
                type: 'analytics',
                api: 'https://www.googleapis.com/analytics/v3/management/',
                deleteMethod: this.deleteFromGoogleAnalytics.bind(this)
            },
            {
                name: 'Mailchimp',
                type: 'email_marketing',
                api: 'https://us1.api.mailchimp.com/3.0/',
                deleteMethod: this.deleteFromMailchimp.bind(this)
            },
            {
                name: 'Intercom',
                type: 'customer_support',
                api: 'https://api.intercom.io/',
                deleteMethod: this.deleteFromIntercom.bind(this)
            }
        ];
    }
    
    async notifyAllThirdParties(userId, userEmail) {
        const results = [];
        
        for (const thirdParty of this.thirdParties) {
            try {
                const result = await this.notifyThirdParty(thirdParty, userId, userEmail);
                results.push({
                    thirdParty: thirdParty.name,
                    status: 'success',
                    deletedAt: new Date(),
                    details: result
                });
            } catch (error) {
                results.push({
                    thirdParty: thirdParty.name,
                    status: 'failed',
                    error: error.message,
                    attemptedAt: new Date()
                });
            }
        }
        
        // Log all notification attempts
        await this.logNotificationResults(userId, results);
        
        return results;
    }
    
    async deleteFromGoogleAnalytics(userId, userEmail) {
        // Google Analytics User Deletion API
        const response = await fetch(`${this.thirdParties[0].api}userDeletion/userDeletionRequests`, {
            method: 'POST',
            headers: {
                'Authorization': `Bearer ${process.env.GOOGLE_ACCESS_TOKEN}`,
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({
                id: {
                    type: 'CLIENT_ID',
                    userId: userId
                },
                propertyIds: [process.env.GA_PROPERTY_ID]
            })
        });
        
        if (!response.ok) {
            throw new Error(`Google Analytics deletion failed: ${response.statusText}`);
        }
        
        return await response.json();
    }
    
    async deleteFromMailchimp(userId, userEmail) {
        // Mailchimp subscriber deletion
        const subscriberHash = this.md5(userEmail.toLowerCase());
        const response = await fetch(
            `${this.thirdParties[1].api}lists/${process.env.MAILCHIMP_LIST_ID}/members/${subscriberHash}`,
            {
                method: 'DELETE',
                headers: {
                    'Authorization': `Bearer ${process.env.MAILCHIMP_API_KEY}`
                }
            }
        );
        
        if (!response.ok && response.status !== 404) {
            throw new Error(`Mailchimp deletion failed: ${response.statusText}`);
        }
        
        return { deleted: true, email: userEmail };
    }
    
    async deleteFromIntercom(userId, userEmail) {
        // Intercom user deletion
        const response = await fetch(`${this.thirdParties[2].api}users`, {
            method: 'DELETE',
            headers: {
                'Authorization': `Bearer ${process.env.INTERCOM_ACCESS_TOKEN}`,
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({
                email: userEmail
            })
        });
        
        if (!response.ok) {
            throw new Error(`Intercom deletion failed: ${response.statusText}`);
        }
        
        return await response.json();
    }
}
```

## Compliance Assessment Matrix

| Requirement | Compliant Criteria | Non-Compliant Indicators | Observations |
|-------------|-------------------|---------------------------|--------------|
| **Article 17(1)** | Complete erasure within 30 days | No erasure implementation | Soft deletion only |
| **Article 17(2)** | Third parties notified | No third-party notification | Manual notification |
| **Article 17(3)** | Exceptions documented | Blanket retention claims | Unclear legal basis |

## Critical Action Items

### High Priority (Immediate)
1. **Implement Erasure System** - Core technical capability
2. **Complete Data Mapping** - Know what data exists where
3. **Third-Party Assessment** - Identify all data sharing arrangements

### Medium Priority (30 days)
1. **Document Legal Exceptions** - When erasure can be refused
2. **Backup Procedures** - Handle archived/backup data
3. **Staff Training** - Ensure proper request handling

### Ongoing Monitoring
1. **Request Response Times** - Track 30-day compliance
2. **Third-Party Confirmations** - Verify actual deletion
3. **Legal Basis Reviews** - Regular assessment of retention needs