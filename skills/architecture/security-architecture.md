---
name: engram-security-architecture
description: "Design security controls including authentication, authorization, encryption, audit logging, and least privilege. Perform threat modeling and attack surface analysis."
---

# Security Architecture (Engram-Integrated)

## Overview

Design comprehensive security controls by systematically identifying threats, defining trust boundaries, implementing defense-in-depth, and enforcing least privilege principles. Store threat models, security decisions, and attack surface analysis in Engram for compliance and audit purposes.

## When to Use

Use this skill when:
- Designing a new system or feature with security implications
- Adding authentication or authorization mechanisms
- Handling sensitive data (PII, credentials, financial data)
- Exposing new APIs or external interfaces
- Preparing for security audit or compliance certification
- Responding to a security incident or vulnerability
- Planning encryption strategy (at-rest, in-transit, end-to-end)

## The Pattern

### Step 1: Define Security Requirements

Start by documenting what needs protection and threat actors:

```bash
engram context create \
  --title "Security Requirements: [System Name]" \
  --content "## Assets to Protect\n\n**Data:**\n1. [Asset 1] - Sensitivity: [Public/Internal/Confidential/Secret]\n2. [Asset 2] - Sensitivity: [Classification]\n\n**Systems:**\n1. [System 1] - Criticality: [High/Medium/Low]\n2. [System 2] - Criticality: [Level]\n\n## Threat Actors\n\n**External:**\n- Unauthenticated attackers (internet-facing services)\n- Malicious users (authenticated but hostile)\n- Competitors (industrial espionage)\n\n**Internal:**\n- Compromised employees (insider threat)\n- Negligent users (accidental exposure)\n- Compromised supply chain (dependencies)\n\n## Compliance Requirements\n\n- [GDPR]: Data privacy, right to deletion\n- [SOC 2]: Access controls, audit logging\n- [HIPAA]: Healthcare data encryption\n- [PCI DSS]: Payment card data protection\n\n## Security Objectives\n\n**Confidentiality:**\n- [Data X] must only be accessible to [authorized roles]\n- [API Y] must use encrypted channels (TLS 1.3+)\n\n**Integrity:**\n- [Data Z] must be immutable once written\n- [Operation W] must be tamper-evident (audit log)\n\n**Availability:**\n- System must resist DDoS (rate limiting, WAF)\n- Graceful degradation under attack\n\n**Non-Repudiation:**\n- [Critical actions] must be cryptographically signed\n- [Audit logs] must be tamper-proof" \
  --source "security-architecture" \
  --tags "security,requirements,[system-name]"
```

### Step 2: Perform Threat Modeling (STRIDE)

Systematically identify threats using STRIDE methodology:

```bash
engram reasoning create \
  --title "Threat Model (STRIDE): [System/Component Name]" \
  --task-id [TASK_ID] \
  --content "## STRIDE Analysis\n\n### S - Spoofing (Authentication)\n\n**Threat:** Attacker impersonates legitimate user\n\n**Attack Scenarios:**\n1. Stolen credentials: User password compromised via phishing\n2. Session hijacking: Cookie stolen via XSS or network sniffing\n3. Token forgery: JWT signed with weak key or none algorithm\n\n**Mitigations:**\n- Use strong password hashing (Argon2, bcrypt with high work factor)\n- Implement MFA for sensitive operations\n- Sign JWTs with RS256 (asymmetric), rotate keys regularly\n- Use HttpOnly, Secure, SameSite cookies\n- Short-lived tokens (15 min access token, refresh token rotation)\n\n**Residual Risk:** Medium (MFA reduces but doesn't eliminate risk)\n\n### T - Tampering (Integrity)\n\n**Threat:** Attacker modifies data in transit or at rest\n\n**Attack Scenarios:**\n1. MITM attack: Intercept and modify API requests\n2. Database injection: SQL injection modifies records\n3. Message tampering: Modify message queue payloads\n\n**Mitigations:**\n- TLS 1.3 for all communication (enforce, no downgrade)\n- Parameterized queries, ORM for database access\n- Sign message payloads (HMAC or digital signature)\n- Immutable audit logs with cryptographic hashing\n\n**Residual Risk:** Low (multiple layers of protection)\n\n### R - Repudiation (Non-Repudiation)\n\n**Threat:** User denies performing action they actually did\n\n**Attack Scenarios:**\n1. Unauthorized transaction: User claims they didn't transfer funds\n2. Data deletion: User claims they didn't delete critical data\n\n**Mitigations:**\n- Comprehensive audit logging (who, what, when, where, why)\n- Cryptographic signatures for critical operations\n- Immutable audit log (append-only, tamper-evident)\n- Timestamp with trusted time source\n\n**Residual Risk:** Low (strong audit trail)\n\n### I - Information Disclosure (Confidentiality)\n\n**Threat:** Unauthorized access to sensitive data\n\n**Attack Scenarios:**\n1. API enumeration: Guess valid IDs to access other users' data\n2. Error messages: Stack traces leak system internals\n3. Logs: Sensitive data logged in plaintext\n4. Backup exposure: Database backup publicly accessible\n\n**Mitigations:**\n- Authorization checks on every resource access (not just authentication)\n- Use UUIDs instead of sequential IDs\n- Generic error messages to external users, detailed logs internal only\n- Redact sensitive data in logs (passwords, tokens, PII)\n- Encrypt backups, restrict access (IAM policies)\n- Data classification and handling policies\n\n**Residual Risk:** Medium (complex to enforce consistently)\n\n### D - Denial of Service (Availability)\n\n**Threat:** Attacker makes system unavailable to legitimate users\n\n**Attack Scenarios:**\n1. Volume attack: Overwhelming traffic (DDoS)\n2. Resource exhaustion: Expensive queries consume all database connections\n3. Application-level DoS: Trigger slow code paths repeatedly\n\n**Mitigations:**\n- Rate limiting per IP/user (token bucket, sliding window)\n- Web Application Firewall (WAF) for layer 7 protection\n- CloudFlare / AWS Shield for DDoS mitigation\n- Query timeouts and connection pooling\n- Circuit breakers for downstream dependencies\n- Graceful degradation (shed load, return cached data)\n\n**Residual Risk:** Medium (determined attackers can still cause degradation)\n\n### E - Elevation of Privilege (Authorization)\n\n**Threat:** Attacker gains higher privileges than authorized\n\n**Attack Scenarios:**\n1. IDOR: Modify user_id parameter to access admin functions\n2. Path traversal: Access files outside intended directory\n3. Privilege escalation: Exploit vulnerability to gain root access\n\n**Mitigations:**\n- Enforce least privilege (default deny, explicit allow)\n- Role-based access control (RBAC) with fine-grained permissions\n- Attribute-based access control (ABAC) for complex rules\n- Input validation and sanitization\n- Separate admin interfaces (different network zone)\n- Regular security audits and penetration testing\n\n**Residual Risk:** High (most common vulnerability class)\n\n## Overall Risk Assessment\n\n**Critical Risks:**\n1. Elevation of Privilege (High likelihood, High impact)\n2. Information Disclosure (Medium likelihood, High impact)\n\n**Action Items:**\n1. Implement comprehensive authorization checks (every endpoint)\n2. Add automated security testing (OWASP ZAP, static analysis)\n3. Conduct code review with security focus\n\n**Confidence:** [0.0-1.0]" \
  --confidence [0.0-1.0] \
  --tags "security,threat-model,stride,[system-name]"
```

### Step 3: Define Trust Boundaries and Attack Surface

Map where trust boundaries cross and where attacks can enter:

```bash
engram reasoning create \
  --title "Trust Boundaries and Attack Surface: [System Name]" \
  --task-id [TASK_ID] \
  --content "## Trust Boundaries\n\n**Definition:** Points where data crosses from untrusted to trusted zone\n\n### Boundary 1: Internet → API Gateway\n\n**Untrusted:** Any client on internet\n**Trusted:** Internal API Gateway\n\n**Controls at Boundary:**\n- TLS termination (enforce HTTPS)\n- Rate limiting (per IP, per token)\n- WAF (SQL injection, XSS filtering)\n- Input validation (JSON schema)\n- Authentication (verify JWT signature)\n\n**Risk:** Highest attack surface - fully exposed to internet\n\n### Boundary 2: API Gateway → Internal Services\n\n**Untrusted:** API Gateway (potentially compromised)\n**Trusted:** Internal microservices\n\n**Controls at Boundary:**\n- Mutual TLS (mTLS) for service-to-service auth\n- Service mesh (Istio, Linkerd) for traffic control\n- Network policies (only allow gateway → service traffic)\n- Re-validate authorization (don't trust gateway completely)\n\n**Risk:** Medium - assumes gateway is mostly trusted but defense-in-depth\n\n### Boundary 3: Application → Database\n\n**Untrusted:** Application code (may have vulnerabilities)\n**Trusted:** Database\n\n**Controls at Boundary:**\n- Parameterized queries only (no string concatenation)\n- ORM with SQL injection protection\n- Least privilege database credentials (read-only vs read-write)\n- Network isolation (database in private subnet)\n- Audit logging of all queries\n\n**Risk:** Medium - SQL injection is common vulnerability class\n\n### Boundary 4: Application → External APIs\n\n**Untrusted:** External services (may be compromised or malicious)\n**Trusted:** Application\n\n**Controls at Boundary:**\n- TLS certificate validation (pin certificates if possible)\n- Input validation on responses (don't trust external data)\n- Timeout and circuit breakers (prevent slow attacks)\n- Rate limiting outbound requests (prevent abuse of our credentials)\n- Rotate API keys regularly\n\n**Risk:** Medium - supply chain attacks increasing\n\n## Attack Surface Analysis\n\n### Entry Points (Attack Vectors)\n\n**1. Public API Endpoints**\n\n**Surface:**\n- [N] REST endpoints\n- [M] GraphQL queries\n- [P] WebSocket connections\n\n**Exposure:** High - internet-accessible\n\n**Mitigations:**\n- Authentication required (except public endpoints)\n- Authorization on every request\n- Input validation\n- Rate limiting\n- API versioning (deprecate insecure endpoints)\n\n**2. Web Application**\n\n**Surface:**\n- User input forms\n- URL parameters\n- File uploads\n- Client-side JavaScript\n\n**Exposure:** High - user-controlled input\n\n**Mitigations:**\n- Content Security Policy (CSP) headers\n- XSS prevention (escape output, DOMPurify)\n- CSRF tokens\n- File upload validation (type, size, scan for malware)\n- Subresource Integrity (SRI) for CDN assets\n\n**3. Admin Interface**\n\n**Surface:**\n- Admin dashboard\n- Database management tools\n- Deployment pipelines\n\n**Exposure:** Medium - restricted access but high privilege\n\n**Mitigations:**\n- Separate network (VPN required)\n- MFA mandatory\n- IP allowlist\n- Audit all admin actions\n- Time-based access (JIT privilege escalation)\n\n**4. Third-Party Dependencies**\n\n**Surface:**\n- [N] npm packages\n- [M] Docker base images\n- [P] OS packages\n\n**Exposure:** High - supply chain attacks\n\n**Mitigations:**\n- Dependency scanning (Snyk, Dependabot)\n- Pin versions (lock files)\n- Verify signatures\n- Minimal base images (distroless, Alpine)\n- Regular updates and patching\n\n## Attack Surface Reduction\n\n**Remove:**\n- Unused endpoints (DELETE /debug)\n- Unnecessary features (feature flags)\n- Deprecated protocols (SSLv3, TLS 1.0)\n\n**Restrict:**\n- Admin interface to VPN only\n- Database ports not exposed externally\n- Metrics/monitoring only on private network\n\n**Harden:**\n- Disable directory listing\n- Remove server version headers\n- Use security-focused defaults\n\n**Confidence:** [0.0-1.0]" \
  --confidence [0.0-1.0] \
  --tags "security,attack-surface,trust-boundaries,[system-name]"
```

### Step 4: Design Authentication and Authorization

Define identity and access control mechanisms:

```bash
engram reasoning create \
  --title "Authentication & Authorization Design: [System Name]" \
  --task-id [TASK_ID] \
  --content "## Authentication (Identity)\n\n### Strategy: JWT with Refresh Tokens\n\n**Flow:**\n1. User submits credentials (email + password)\n2. Server verifies password hash (Argon2)\n3. Server issues access token (JWT, 15 min expiry)\n4. Server issues refresh token (opaque, 7 day expiry, stored in DB)\n5. Client stores access token (memory), refresh token (HttpOnly cookie)\n6. Client includes access token in Authorization header\n7. When access token expires, use refresh token to get new one\n\n**JWT Structure:**\n\`\`\`json\n{\n  \"sub\": \"user-id\",\n  \"email\": \"user@example.com\",\n  \"roles\": [\"user\"],\n  \"permissions\": [\"read:own:documents\", \"write:own:documents\"],\n  \"iat\": 1234567890,\n  \"exp\": 1234568790\n}\n\`\`\`\n\n**Security Properties:**\n- Signed with RS256 (private key on auth service only)\n- Short-lived (15 min) limits exposure if stolen\n- Refresh token rotation (invalidate old on use)\n- Revocation via token blacklist (Redis, rare case)\n\n### Multi-Factor Authentication\n\n**When Required:**\n- Admin access (always)\n- High-value operations (money transfer, delete account)\n- Login from new device or location\n\n**Methods:**\n- TOTP (Time-based One-Time Password, e.g., Google Authenticator)\n- SMS (fallback, less secure)\n- WebAuthn (hardware keys, most secure)\n\n**Implementation:**\n- Store MFA secret encrypted (per-user key derived from password)\n- Require MFA within 5 minutes for sensitive operations\n- Backup codes for account recovery\n\n## Authorization (Access Control)\n\n### Strategy: RBAC + Attribute-Based (Hybrid)\n\n**Role-Based Access Control (RBAC):**\n\n**Roles:**\n- `user`: Basic authenticated user\n- `premium`: Paid subscriber (extends user)\n- `moderator`: Content moderation (extends user)\n- `admin`: System administrator (full access)\n\n**Role Hierarchy:**\n\`\`\`\nadmin → moderator → premium → user\n\`\`\`\n\n**Permissions by Role:**\n- `user`: read:own:*, write:own:*, delete:own:*\n- `premium`: read:own:*, write:own:*, delete:own:*, export:own:*\n- `moderator`: read:any:posts, write:any:posts, delete:any:posts\n- `admin`: *:*:*\n\n**Attribute-Based Access Control (ABAC):**\n\nFor complex rules that RBAC can't express:\n\n**Policy Example:**\n\`\`\`\nALLOW read:document IF:\n  (document.owner_id == user.id) OR\n  (document.visibility == \"public\") OR\n  (user.id IN document.shared_with)\n\`\`\`\n\n**Policy Example 2:**\n\`\`\`\nALLOW write:document IF:\n  (document.owner_id == user.id) AND\n  (document.locked == false) AND\n  (time.now < document.expires_at)\n\`\`\`\n\n**Implementation:**\n- Policy engine (Open Policy Agent, Casbin)\n- Policies stored as code, versioned\n- Evaluated on every request (cache decisions)\n\n### Least Privilege Principles\n\n**Default Deny:**\n- All access denied unless explicitly allowed\n- New roles start with zero permissions\n\n**Minimal Scope:**\n- Permissions scoped to resource type and action\n- read:own:documents (not just read:documents)\n\n**Time-Limited:**\n- Admin access expires after 8 hours\n- Emergency access auto-revokes after 1 hour\n\n**Just-In-Time (JIT):**\n- Elevate privileges only when needed\n- Require approval + MFA for privilege escalation\n- Audit all escalations\n\n## Enforcement Points\n\n**API Gateway:**\n- Verify JWT signature and expiry\n- Extract user_id and roles from JWT\n- Reject if token invalid or expired\n\n**Service Layer:**\n- Re-validate authorization (defense-in-depth)\n- Query policy engine with (user, action, resource)\n- Log decision (allowed/denied) to audit log\n\n**Database Layer:**\n- Row-level security (PostgreSQL RLS)\n- Separate read-only vs read-write credentials\n- Encrypt sensitive columns\n\n## Security Testing\n\n**Unit Tests:**\n- Test permission checks for each endpoint\n- Test that unauthorized access is denied\n- Test privilege escalation attempts fail\n\n**Integration Tests:**\n- Test full auth flow (login → token → request)\n- Test token expiry and refresh\n- Test MFA flow\n\n**Penetration Testing:**\n- Test IDOR (change user_id in requests)\n- Test JWT manipulation (change claims, remove signature)\n- Test session fixation and hijacking\n\n**Confidence:** [0.0-1.0]" \
  --confidence [0.0-1.0] \
  --tags "security,authentication,authorization,[system-name]"
```

### Step 5: Design Encryption Strategy

Define encryption for data at-rest, in-transit, and end-to-end:

```bash
engram reasoning create \
  --title "Encryption Strategy: [System Name]" \
  --task-id [TASK_ID] \
  --content "## Encryption At-Rest\n\n### Database Encryption\n\n**Strategy: Transparent Data Encryption (TDE) + Column-Level**\n\n**TDE (Full-Disk Encryption):**\n- PostgreSQL: Use encrypted volumes (LUKS, AWS EBS encryption)\n- Key management: AWS KMS, HashiCorp Vault\n- Protects against: Physical theft, snapshot exposure\n- Does NOT protect against: SQL injection, compromised app\n\n**Column-Level Encryption:**\n\nFor highly sensitive data (PII, financial):\n\n**Fields to Encrypt:**\n- SSN, passport numbers\n- Credit card numbers (PCI DSS requirement)\n- Health records (HIPAA requirement)\n- Private messages\n\n**Implementation:**\n\`\`\`sql\nCREATE TABLE users (\n  id UUID PRIMARY KEY,\n  email TEXT NOT NULL,\n  name TEXT NOT NULL,\n  ssn_encrypted BYTEA,  -- Encrypted with user-specific key\n  created_at TIMESTAMP\n);\n\`\`\`\n\n**Encryption Keys:**\n- Master key: Stored in KMS (AWS KMS, GCP KMS)\n- Data encryption keys (DEK): Per-user or per-record\n- DEK encrypted with master key (envelope encryption)\n- Rotate master key annually, re-encrypt DEKs\n\n**Algorithm:** AES-256-GCM (authenticated encryption)\n\n### File Storage Encryption\n\n**Strategy: Server-Side Encryption (SSE)**\n\n**S3 / Object Storage:**\n- Use SSE-KMS (AWS KMS managed keys)\n- Encrypt on upload, decrypt on download\n- Access control via IAM policies\n\n**Alternative: Client-Side Encryption**\n\nFor end-to-end encryption (zero-knowledge):\n\n**Flow:**\n1. Client generates encryption key (derived from user password)\n2. Client encrypts file locally\n3. Client uploads encrypted file to S3\n4. Server cannot decrypt (doesn't have key)\n\n**Trade-offs:**\n- Pro: True end-to-end encryption (server breach doesn't expose data)\n- Con: Cannot process files server-side (thumbnails, search)\n- Con: Password reset requires re-encryption or key escrow\n\n## Encryption In-Transit\n\n### TLS Configuration\n\n**Version:** TLS 1.3 (or minimum TLS 1.2)\n\n**Cipher Suites:**\n\`\`\`\nTLS_AES_256_GCM_SHA384\nTLS_CHACHA20_POLY1305_SHA256\nTLS_AES_128_GCM_SHA256\n\`\`\`\n\n**Certificate:**\n- Use Let's Encrypt or commercial CA\n- Wildcard cert for subdomains\n- Automate renewal (certbot)\n- Monitor expiry (alert 30 days before)\n\n**HSTS (HTTP Strict Transport Security):**\n\`\`\`\nStrict-Transport-Security: max-age=31536000; includeSubDomains; preload\n\`\`\`\n\n**Certificate Pinning:**\n\nFor mobile apps (optional, reduces flexibility):\n\n\`\`\`json\n{\n  \"pins\": [\n    {\"hostname\": \"api.example.com\", \"sha256\": \"base64-hash\"}\n  ]\n}\n\`\`\`\n\n### Service-to-Service Encryption\n\n**Mutual TLS (mTLS):**\n\n**Flow:**\n1. Service A presents client certificate\n2. Service B verifies certificate (signed by internal CA)\n3. Service B presents server certificate\n4. Service A verifies certificate\n5. Encrypted channel established\n\n**Benefits:**\n- Authentication (both parties verified)\n- Encryption (traffic protected)\n- No shared secrets (certificates instead)\n\n**Implementation:**\n- Service mesh (Istio, Linkerd) handles automatically\n- Certificate rotation managed by mesh\n\n## End-to-End Encryption\n\n### Use Cases\n\n**When Required:**\n- Private messaging (Signal protocol)\n- Encrypted document storage (zero-knowledge)\n- Sensitive file sharing\n\n**Flow:**\n\`\`\`\nAlice                   Server                     Bob\n  |                       |                         |\n  |-- [Bob's public key]->|                         |\n  |                       |-- [Bob's public key] -->|\n  |                       |                         |\n  |-- Encrypt(msg, Bob_pub) ----------------------->|\n  |                       |                         |\n  |                       |          Decrypt(msg, Bob_priv)\n  |                       |                         |\n\`\`\`\n\n**Server Role:**\n- Store encrypted messages (cannot read)\n- Route to recipient\n- Manage public key directory\n\n**Key Management:**\n- Generate key pair on device\n- Upload public key to server\n- Store private key locally (encrypted with device key)\n- Backup private key encrypted with recovery key\n\n## Key Management Best Practices\n\n**Separation of Duties:**\n- Encryption keys separate from data\n- KMS access separate from application access\n- Require two parties to access master key\n\n**Rotation:**\n- Master keys: Rotate annually\n- Data encryption keys: Rotate on suspected compromise\n- TLS certificates: Rotate every 90 days\n- API keys: Rotate every 6 months\n\n**Backup and Recovery:**\n- Backup encryption keys securely (encrypted backup)\n- Store recovery keys offline (paper, safe)\n- Test recovery process quarterly\n\n**Audit:**\n- Log all key access (KMS audit logs)\n- Alert on unusual patterns (key used from new IP)\n- Review key access logs monthly\n\n**Confidence:** [0.0-1.0]" \
  --confidence [0.0-1.0] \
  --tags "security,encryption,[system-name]"
```

### Step 6: Design Audit Logging

Define what to log for security monitoring and compliance:

```bash
engram reasoning create \
  --title "Audit Logging Design: [System Name]" \
  --task-id [TASK_ID] \
  --content "## Audit Log Requirements\n\n### What to Log\n\n**Authentication Events:**\n- Login success/failure (who, when, from where, IP, user agent)\n- Logout (who, when)\n- Password change (who, when, IP)\n- MFA enabled/disabled (who, when)\n- Token issued/refreshed/revoked (who, token_id, when)\n\n**Authorization Events:**\n- Permission denied (who tried, what resource, what action, why denied)\n- Permission granted (who, what, when - only for sensitive resources)\n- Role change (who changed, target user, old role, new role)\n- Policy change (who, what policy, diff)\n\n**Data Access:**\n- Read sensitive data (who, what resource, when, IP)\n- Modify data (who, what resource, old value, new value, when)\n- Delete data (who, what resource, when, can it be recovered)\n- Export data (who, what data, how much, when)\n\n**Admin Actions:**\n- User created/deleted (who did it, target user, when)\n- Configuration change (who, what setting, old/new value)\n- Deployment (who, what version, when, success/failure)\n- Emergency access (who, why, when started, when ended)\n\n**Security Events:**\n- Suspicious activity (rate limit exceeded, multiple login failures)\n- Security scan results (vulnerability found, severity)\n- Certificate expiry warnings\n- Anomalous access patterns (access from new country)\n\n### Log Format\n\n**Structured Logging (JSON):**\n\n\`\`\`json\n{\n  \"timestamp\": \"2026-01-24T12:34:56.789Z\",\n  \"event_type\": \"authentication\",\n  \"event_name\": \"login_success\",\n  \"actor\": {\n    \"user_id\": \"550e8400-e29b-41d4-a716-446655440000\",\n    \"email\": \"user@example.com\",\n    \"ip\": \"203.0.113.42\",\n    \"user_agent\": \"Mozilla/5.0...\"\n  },\n  \"resource\": {\n    \"type\": \"session\",\n    \"id\": \"session-xyz\"\n  },\n  \"result\": \"success\",\n  \"metadata\": {\n    \"mfa_used\": true,\n    \"login_method\": \"password\"\n  }\n}\n\`\`\`\n\n**Required Fields:**\n- timestamp (ISO 8601, UTC)\n- event_type (category)\n- event_name (specific event)\n- actor (who)\n- resource (what)\n- result (success/failure)\n- trace_id (for correlation with application logs)\n\n### Log Storage\n\n**Strategy: Immutable Append-Only**\n\n**Implementation:**\n- Write to write-only table (PostgreSQL, no UPDATE/DELETE grants)\n- Or use dedicated audit log service (AWS CloudTrail, Panther)\n- Cryptographic chaining (each log entry includes hash of previous)\n\n**Retention:**\n- Security logs: 1 year minimum (compliance requirement)\n- Admin actions: 7 years (regulatory requirement)\n- Data access: 90 days (balance storage cost vs investigation window)\n\n**Access Control:**\n- Read-only access for security team\n- No access for regular developers\n- Alerts on audit log access (someone is investigating)\n\n### Tamper Evidence\n\n**Cryptographic Chain:**\n\n\`\`\`\nlog[n].hash = SHA256(log[n].data + log[n-1].hash)\n\`\`\`\n\nIf attacker modifies log[i], all hashes after break (detection).\n\n**Periodic Signing:**\n\nEvery hour, sign hash chain with private key:\n\n\`\`\`\nsignature = RSA_sign(log[N].hash, private_key)\n\`\`\`\n\nStore signature externally (cannot be modified by attacker).\n\n### Log Monitoring and Alerting\n\n**Real-Time Alerts:**\n- Multiple login failures (>5 in 5 minutes) → potential brute force\n- Admin action from new IP → potential compromised account\n- Permission denied spike → potential attack or misconfiguration\n- Large data export → potential data exfiltration\n- Security scan failed → new vulnerability introduced\n\n**Daily Reports:**\n- Summary of authentication events\n- Failed authorization attempts\n- Admin actions performed\n- Unusual access patterns\n\n**Implementation:**\n- SIEM (Security Information and Event Management): Splunk, ELK, Panther\n- Stream logs to SIEM via syslog or API\n- Define detection rules in SIEM\n- Alert to Slack, PagerDuty, email\n\n### Privacy Considerations\n\n**PII in Logs:**\n- Do NOT log passwords (even hashed)\n- Do NOT log full credit card numbers (last 4 digits only)\n- Redact PII if compliance requires (GDPR right to deletion)\n\n**Log Anonymization:**\n\nFor analytics on audit logs without exposing PII:\n\n\`\`\`\nuser_id → hash(user_id + salt)\n\`\`\`\n\nConsistent within session but not linkable across days.\n\n## Example: Login Flow Audit\n\n**Step 1: Login attempt**\n\n\`\`\`json\n{\"event_name\": \"login_attempt\", \"email\": \"user@example.com\", \"ip\": \"203.0.113.42\"}\n\`\`\`\n\n**Step 2: Password verified**\n\n\`\`\`json\n{\"event_name\": \"password_verified\", \"user_id\": \"550e8400...\", \"result\": \"success\"}\n\`\`\`\n\n**Step 3: MFA challenge sent**\n\n\`\`\`json\n{\"event_name\": \"mfa_challenge_sent\", \"user_id\": \"550e8400...\", \"method\": \"totp\"}\n\`\`\`\n\n**Step 4: MFA verified**\n\n\`\`\`json\n{\"event_name\": \"mfa_verified\", \"user_id\": \"550e8400...\", \"result\": \"success\"}\n\`\`\`\n\n**Step 5: Token issued**\n\n\`\`\`json\n{\"event_name\": \"token_issued\", \"user_id\": \"550e8400...\", \"token_id\": \"abc123\", \"expiry\": \"2026-01-24T12:49:56Z\"}\n\`\`\`\n\n**Step 6: Login success**\n\n\`\`\`json\n{\"event_name\": \"login_success\", \"user_id\": \"550e8400...\", \"session_id\": \"session-xyz\"}\n\`\`\`\n\n**Investigation:**\n\nIf user reports unauthorized login, query audit logs:\n\n\`\`\`sql\nSELECT * FROM audit_logs\nWHERE user_id = '550e8400...'\n  AND event_type = 'authentication'\n  AND timestamp BETWEEN '2026-01-20' AND '2026-01-25'\nORDER BY timestamp DESC;\n\`\`\`\n\n**Confidence:** [0.0-1.0]" \
  --confidence [0.0-1.0] \
  --tags "security,audit-logging,[system-name]"
```

### Step 7: Link All Security Entities

```bash
# Link all security design documents to task
engram relationship create \
  --source-id [TASK_ID] --source-type task \
  --target-id [SECURITY_REQUIREMENTS_ID] --target-type context \
  --relationship-type references --agent [AGENT]

engram relationship create \
  --source-id [TASK_ID] --source-type task \
  --target-id [THREAT_MODEL_ID] --target-type reasoning \
  --relationship-type documents --agent [AGENT]

engram relationship create \
  --source-id [TASK_ID] --source-type task \
  --target-id [ATTACK_SURFACE_ID] --target-type reasoning \
  --relationship-type documents --agent [AGENT]

engram relationship create \
  --source-id [TASK_ID] --source-type task \
  --target-id [AUTH_DESIGN_ID] --target-type reasoning \
  --relationship-type documents --agent [AGENT]

engram relationship create \
  --source-id [TASK_ID] --source-type task \
  --target-id [ENCRYPTION_ID] --target-type reasoning \
  --relationship-type documents --agent [AGENT]

engram relationship create \
  --source-id [TASK_ID] --source-type task \
  --target-id [AUDIT_LOG_ID] --target-type reasoning \
  --relationship-type documents --agent [AGENT]
```

## Example

User wants to add payment processing to an e-commerce system.

### Step 1: Security Requirements

```bash
SECURITY_REQ=$(engram context create \
  --title "Security Requirements: Payment Processing" \
  --content "## Assets to Protect\n\n**Data:**\n1. Credit card numbers (PAN) - Sensitivity: Secret (PCI DSS Level 1)\n2. Customer PII (name, address) - Sensitivity: Confidential (GDPR)\n3. Transaction history - Sensitivity: Confidential\n\n**Systems:**\n1. Payment gateway integration - Criticality: High (business-critical)\n2. Order processing - Criticality: High\n\n## Threat Actors\n\n**External:**\n- Unauthenticated attackers (steal card data)\n- Malicious users (fraud, chargebacks)\n\n**Internal:**\n- Compromised employees (insider theft)\n- Negligent developers (accidental exposure in logs)\n\n## Compliance Requirements\n\n- PCI DSS: Do NOT store CVV, encrypt PAN, annual audit\n- GDPR: Data minimization, right to deletion\n- SOC 2: Access controls, encryption, audit logging\n\n## Security Objectives\n\n**Confidentiality:**\n- Credit card numbers never stored plaintext\n- Payment forms served over HTTPS only\n\n**Integrity:**\n- Transaction amounts cannot be tampered\n- Order IDs cannot be guessed (use UUIDs)\n\n**Availability:**\n- Payment downtime < 0.1% (accept alternative methods)\n- Graceful degradation (queue orders for later processing)\n\n**Non-Repudiation:**\n- All transactions cryptographically signed\n- Audit log immutable" \
  --source "security-architecture" \
  --tags "security,requirements,payment-processing" \
  --json | jq -r '.id')
```

### Step 2: Threat Model (STRIDE)

```bash
THREAT_MODEL=$(engram reasoning create \
  --title "Threat Model (STRIDE): Payment Processing" \
  --task-id 8b2e4f67-9876-5432-10ab-cdef87654321 \
  --content "## STRIDE Analysis\n\n### S - Spoofing\n\n**Threat:** Attacker submits payment as different user\n\n**Mitigations:**\n- Require authentication before payment form\n- Match user_id in session to user_id in order\n- Implement 3D Secure (cardholder verification)\n\n**Residual Risk:** Low\n\n### T - Tampering\n\n**Threat:** Attacker modifies payment amount in transit\n\n**Attack Scenario:**\nUser submits $100 payment but intercepts request and changes to $1.\n\n**Mitigations:**\n- HTTPS for all payment forms (TLS 1.3)\n- Server-side price calculation (never trust client)\n- Sign payment request (HMAC of order_id + amount + timestamp)\n\n**Residual Risk:** Low\n\n### R - Repudiation\n\n**Threat:** Customer claims they didn't make purchase\n\n**Mitigations:**\n- Store IP address, user agent, timestamp\n- Require email confirmation before processing\n- 3D Secure provides liability shift\n- Immutable audit log of all transactions\n\n**Residual Risk:** Low (protected by evidence)\n\n### I - Information Disclosure\n\n**Threat:** Credit card data exposed\n\n**Attack Scenarios:**\n1. Logs: Card number logged in plaintext\n2. Error messages: Full card shown in error\n3. SQL injection: Attacker extracts card DB\n4. XSS: Steal card from payment form\n\n**Mitigations:**\n- Use Stripe/PayPal (they store card, not us)\n- If storing: PCI DSS compliance (encrypt, segment network)\n- Never log full card (last 4 digits only)\n- Parameterized queries (prevent SQL injection)\n- CSP headers (prevent XSS)\n- Tokenization (store token not card)\n\n**Residual Risk:** Medium (HIGH PRIORITY)\n\n### D - Denial of Service\n\n**Threat:** Attacker prevents legitimate transactions\n\n**Attack Scenarios:**\n1. Submit many failed transactions (exhaust rate limit)\n2. DDOS payment endpoint\n\n**Mitigations:**\n- Rate limit per IP (10 transactions/hour)\n- CAPTCHA after 3 failed attempts\n- WAF / CloudFlare for DDoS\n- Async processing (queue transactions)\n\n**Residual Risk:** Medium\n\n### E - Elevation of Privilege\n\n**Threat:** Attacker processes payment without authorization\n\n**Attack Scenarios:**\n1. IDOR: Change order_id to charge someone else's card\n2. Price manipulation: Submit negative amount for refund\n\n**Mitigations:**\n- Verify user owns order_id before charging\n- Server-side price calculation\n- Refunds require admin approval\n- Separate API endpoint for refunds (different auth)\n\n**Residual Risk:** Medium\n\n## Critical Risks\n\n1. **Information Disclosure (HIGH):** Card data exposure = PCI DSS violation + fines\n2. **Elevation of Privilege (MEDIUM):** Unauthorized charges = fraud + chargebacks\n\n## Action Items\n\n1. Use Stripe/PayPal (offload PCI compliance)\n2. Never store CVV (PCI DSS requirement)\n3. Implement rate limiting and CAPTCHA\n4. Audit all payment code with security focus\n5. Penetration test payment flow\n\n**Confidence:** 0.90" \
  --confidence 0.90 \
  --tags "security,threat-model,stride,payment-processing" \
  --json | jq -r '.id')
```

### Step 3: Attack Surface

```bash
ATTACK_SURFACE=$(engram reasoning create \
  --title "Attack Surface: Payment Processing" \
  --task-id 8b2e4f67-9876-5432-10ab-cdef87654321 \
  --content "## Trust Boundaries\n\n### Boundary 1: Browser → Payment Gateway (Stripe)\n\n**Untrusted:** User's browser\n**Trusted:** Stripe\n\n**Controls:**\n- Stripe.js handles card input (PCI DSS compliant)\n- Card never touches our server\n- Stripe returns token (not card)\n\n**Risk:** Low - Stripe is PCI Level 1 certified\n\n### Boundary 2: Our Server → Stripe API\n\n**Untrusted:** Our application\n**Trusted:** Stripe\n\n**Controls:**\n- TLS for all API calls\n- Stripe API key stored in secrets manager (not code)\n- Rotate API keys quarterly\n- Verify webhook signatures (HMAC)\n\n**Risk:** Low\n\n## Attack Surface\n\n### Entry Point 1: Payment Form\n\n**Exposure:** High - user input\n\n**Mitigations:**\n- Stripe hosted form (card never touches our server)\n- CSP headers prevent XSS\n- HTTPS only\n\n### Entry Point 2: POST /api/payments\n\n**Exposure:** High - accepts payment token\n\n**Mitigations:**\n- Require authentication\n- Verify user owns order_id\n- Rate limit (10 payments/hour per user)\n- Idempotency key (prevent double charges)\n- Server-side amount calculation\n\n### Entry Point 3: Stripe Webhooks\n\n**Exposure:** Medium - external service calls us\n\n**Mitigations:**\n- Verify webhook signature (Stripe signs with secret)\n- Process asynchronously (don't block)\n- Idempotent processing (webhook may retry)\n\n## Attack Surface Reduction\n\n**Remove:**\n- Direct card storage (use Stripe tokens)\n- CVV storage (PCI DSS violation)\n\n**Restrict:**\n- Payment API only accessible to authenticated users\n- Refund API only accessible to admins\n\n**Harden:**\n- Minimize PII in logs\n- Encrypt payment tokens at rest\n- Segment payment processing (separate service)\n\n**Confidence:** 0.85" \
  --confidence 0.85 \
  --tags "security,attack-surface,payment-processing" \
  --json | jq -r '.id')
```

### Step 4: Link Everything

```bash
engram relationship create \
  --source-id 8b2e4f67-9876-5432-10ab-cdef87654321 --source-type task \
  --target-id $SECURITY_REQ --target-type context \
  --relationship-type references --agent default

engram relationship create \
  --source-id 8b2e4f67-9876-5432-10ab-cdef87654321 --source-type task \
  --target-id $THREAT_MODEL --target-type reasoning \
  --relationship-type documents --agent default

engram relationship create \
  --source-id 8b2e4f67-9876-5432-10ab-cdef87654321 --source-type task \
  --target-id $ATTACK_SURFACE --target-type reasoning \
  --relationship-type documents --agent default
```

## Querying Security Architecture

After creating security architecture, agents can retrieve:

```bash
# Get all security documents for a system
engram reasoning list | grep -i security

# Get threat models
engram reasoning list | grep "Threat Model"

# Get attack surface analyses
engram reasoning list | grep "Attack Surface"

# Get all security reasoning for a task
engram relationship connected --entity-id [TASK_ID] --relationship-type documents | grep -i security
```

## Related Skills

This skill integrates with:
- `engram-system-design` - Security is part of system architecture
- `engram-risk-assessment` - Security threats are risks to mitigate
- `engram-check-compliance` - Security controls provide compliance evidence
- `engram-audit-trail` - Audit logs stored in Engram for compliance
- `engram-api-design` - API security (authentication, rate limiting)
