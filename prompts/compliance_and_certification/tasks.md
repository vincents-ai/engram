These prompts are structured to be used by an AI agent with access to your file system and the GitHub CLI (`gh`).

-----

### Agent System Prompt (Persona)

This foundational prompt defines the agent's role, capabilities, and core operational workflow. It should be included at the beginning of every session to set the context for the agent.

```
You are an expert AI Compliance Auditor. Your designation is "Auditron".

Your primary function is to perform continuous, methodical audits of our organization's systems, source code, documentation, and operational processes. You will assess compliance against the following frameworks: GLI, MGA, UKGC, G4, SoC 2, and ISO 27001.

**Capabilities:**
* You have read-only access to all company source code repositories, documentation wikis, infrastructure-as-code configurations, and application logs.
* You are authorized to execute shell commands to navigate the file system (`ls`, `cd`), search for content (`grep`, `find`), and read files (`cat`, `less`).
* You will use the GitHub CLI (`gh`) to interact with our project management system. Specifically, you will use `gh issue create` and `gh issue edit` to report findings and track remediation.

**Core Workflow:**
1.  **Analyze Request:** You will be given a specific task to audit a control from a designated standard.
2.  **Gather Evidence:** Locate and identify all relevant files, code blocks, configuration settings, or log entries that serve as evidence for the control's implementation.
3.  **Evaluate Evidence:** Scrutinize the gathered evidence against the explicit requirements of the control. Your analysis must be objective and fact-based.
4.  **Formulate Conclusion:** For each control, you must provide a clear conclusion: **Compliant**, **Non-Compliant**, or **Observation**.
    * **Compliant**: All requirements of the control are met.
    * **Non-Compliant**: A requirement of the control is not met, posing a compliance risk. This requires immediate action.
    * **Observation**: The control is technically compliant, but improvements are recommended to enhance robustness or mitigate potential future risks.
5.  **Report and Remediate:**
    * For every finding, generate a report detailing the **Control ID**, **Requirement**, **Evidence Examined**, **Findings**, and your **Conclusion**.
    * For every **"Non-Compliant"** finding, you MUST create a GitHub issue using the `gh issue create` command. The issue must be assigned to the relevant project, tagged with the compliance standard (e.g., `soc2`, `ukgc`), and contain a detailed description of the deficiency and a clear recommendation for remediation.
    * For every **"Observation"**, create a GitHub issue with a lower priority and an `observation` tag.

Always cite the exact file paths and line numbers for your evidence. If evidence is ambiguous or cannot be located, this itself is a finding that must be reported.
```

-----

### GLI (Gaming Laboratories International) Prompts

**Focus**: Technical standards for gaming software, RNG, and system integrity.

#### **Prompt: GLI-11 RNG Seeding and Scaling Audit**

```
**Standard**: GLI-11, v3.0
**Control**: 2.3 - Random Number Generator (RNG)
**Objective**: Audit the source code of the 'blackjack-game-engine' to verify its RNG implementation meets GLI-11 requirements for seeding and scaling.

**Tasks**:
1.  **Locate Evidence**:
    * Navigate to the `monorepo/services/blackjack-game-engine/src/rng/` directory.
    * Identify the source file responsible for the RNG's initialization (seeding).
    * Identify the source file responsible for converting the raw RNG output into card values (scaling).
    * Locate the technical documentation for this service, expected at `docs/services/blackjack-engine.md`.

2.  **Analyze Evidence**:
    * **Seeding**: Review the initialization code. Confirm that the initial seed is acquired from a non-deterministic, high-entropy source (e.g., a hardware RNG or the operating system's entropy pool like `/dev/urandom`). Document the exact function call.
    * **Reseeding**: Check the code and documentation to confirm that a reseeding strategy is implemented to prevent state compromise.
    * **Scaling**: Analyze the scaling algorithm that maps a random integer to a specific card (e.g., Ace of Spades). Verify that the mapping is unbiased and that all outcomes are equally probable over a large sample. Look for common flaws like modulo bias.

3.  **Report and Remediate**:
    * Summarize your findings for both the seeding and scaling processes.
    * If the seeding source is predictable or the scaling method is biased, declare the control **"Non-Compliant"**.
    * Create a GitHub issue in the 'Gaming-Engine' project (Project #3):
        * `gh issue create --project "Gaming-Engine" --title "GLI-11 RNG Non-Compliance: Insecure Seeding in Blackjack Engine" --body "The RNG in 'blackjack-game-engine' uses a predictable seed source [LINK TO CODE]. This violates GLI-11 section 2.3. The seed must be replaced with a call to a CSPRNG source." --label "gli,security,bug"`
```

-----

### MGA / UKGC Prompts

**Focus**: Player protection, responsible gaming (RG), data security, and transaction logging.

#### **Prompt: UKGC RTS 14B - Reality Check Implementation**

```
**Standard**: UKGC Remote Gambling and Software Technical Standards (RTS)
**Control**: RTS 14B - Time-based reality checks
**Objective**: Verify that the "Reality Check" functionality is implemented according to RTS 14B requirements for all real-money games.

**Tasks**:
1.  **Locate Evidence**:
    * Locate the front-end source code responsible for displaying the reality check modal/pop-up. Search for "RealityCheck" in the `monorepo/frontend/` directory.
    * Locate the back-end service (`player-session-service`) that tracks session duration and triggers the reality check event.
    * Examine the application logs for a test user to trace the reality check events being fired and acknowledged.
    * Review the Responsible Gaming policy at `docs/policies/responsible-gaming.md`.

2.  **Analyze Evidence**:
    * **Display**: The reality check must be displayed clearly and interrupt gameplay. Verify that the UI component is a modal that overlays the game.
    * **Content**: The modal must show (a) time elapsed since the session started, (b) total money won/lost in the session. Verify the back-end event sends this data and the front-end displays it correctly.
    * **Action**: The player must acknowledge the message to continue. Verify that the only actions are to "Continue" or "Exit Game". Confirm that the game remains paused until an action is taken.
    * **Configuration**: Confirm that the time interval for the reality check is configurable by the player from their account settings page, as per the policy.

3.  **Report and Remediate**:
    * Document the end-to-end user flow for the reality check.
    * If the check can be bypassed, does not display the required information, or does not pause gameplay, declare the control **"Non-Compliant"**.
    * Create a GitHub issue in the 'Player-Services' project (Project #4):
        * `gh issue create --project "Player-Services" --title "UKGC RTS 14B Non-Compliance: Reality Check is Not Interrupting Gameplay" --body "The Reality Check modal in the game client does not pause the underlying game session, allowing automated play to continue. This violates RTS 14B. The game state must be frozen until the player acknowledges the notification. [LINK TO CODE]" --label "ukgc,player-protection,bug"`
```

-----

### G4 (Global Gambling Guidance Group) Prompts

**Focus**: Responsible gaming policies, advertising, and staff training.

#### **Prompt: G4 Advertising and Promotion Audit**

```
**Standard**: G4 Accreditation Criteria
**Control**: Section 4 - Advertising and Promotion
**Objective**: Review recent marketing materials to ensure they do not target minors, are not misleading, and include responsible gaming messaging.

**Tasks**:
1.  **Locate Evidence**:
    * Access the repository of approved marketing assets for the last quarter, located at `/marketing/assets/2025/Q3/`.
    * Locate the Marketing and Advertising Code of Conduct policy document at `docs/policies/marketing-conduct.md`.
    * Gather a list of URLs for our 5 most recent promotional landing pages from `/marketing/campaigns-q3-2025.txt`.

2.  **Analyze Evidence**:
    * **Content Review**: Manually (or with VLM assistance) review the assets and web pages. Check for:
        * Imagery or themes that could appeal to individuals under the legal gambling age.
        * Misleading language regarding the likelihood of winning (e.g., "risk-free", "guaranteed profit").
        * Compliance with the Code of Conduct.
    * **RG Messaging**: Verify that all marketing materials and landing pages prominently display a responsible gaming message and a link to our RG tools/help page, as required by the policy.
    * **Terms and Conditions**: For each promotion, ensure the significant terms and conditions are clearly and transparently stated.

3.  **Report and Remediate**:
    * List all assets reviewed and note any that violate the policy.
    * If any asset is found to target minors or contains misleading information, declare the control **"Non-Compliant"**.
    * Create a GitHub issue in the 'Marketing-Compliance' project (Project #11):
        * `gh issue create --project "Marketing-Compliance" --title "G4 Non-Compliance: Missing RG Message on 'SummerSlam25' Landing Page" --body "The promotional landing page at [URL] is missing the mandatory responsible gaming footer and link. This violates Section 4 of the G4 criteria and our internal policy. The page must be updated immediately." --label "g4,marketing,compliance-gap"`
```

-----

### SoC 2 Prompts

**Focus**: Trust Services Criteria (Security, Availability, Confidentiality, etc.).

#### **Prompt: SoC 2 CC7.1 - Vulnerability Management**

```
**Standard**: SoC 2 - Trust Services Criteria
**Control**: CC7.1 - To detect and respond to vulnerabilities
**Objective**: Verify that a vulnerability scanning process is in place for production systems and that identified vulnerabilities are tracked and remediated in a timely manner.

**Tasks**:
1.  **Locate Evidence**:
    * Locate the configuration files for our vulnerability scanner (e.g., Trivy, Snyk). This should be in the CI/CD pipeline configuration, `monorepo/.github/workflows/security-scan.yml`.
    * Fetch the latest vulnerability scan reports, expected in an S3 bucket or similar artifact storage at `/ci-artifacts/scans/latest.json`.
    * Find the Vulnerability Management Policy at `docs/policies/vulnerability-management.md` to identify the SLAs for patching Critical, High, Medium, and Low vulnerabilities.

2.  **Analyze Evidence**:
    * **Scanning Process**: Confirm that the `security-scan.yml` workflow is configured to run automatically on all code merges to the main branch and on a scheduled basis (e.g., nightly).
    * **Report Analysis**: Parse the `latest.json` scan report. Identify the 5 most recent 'Critical' or 'High' severity vulnerabilities discovered.
    * **Remediation Tracking**: For each of those 5 vulnerabilities, search GitHub issues for a corresponding remediation ticket. Check the ticket's creation date against the vulnerability's discovery date.
    * **SLA Compliance**: Compare the time-to-remediation (from discovery to ticket closure) against the SLAs defined in the policy. For example, the policy might state 'Critical vulnerabilities must be patched within 15 days'.

3.  **Report and Remediate**:
    * Report on the status of the vulnerability scanning process.
    * If any identified Critical/High vulnerability does not have a corresponding remediation ticket, or if the remediation of a ticket has exceeded its SLA, declare the control **"Non-Compliant"**.
    * Create a GitHub issue in the 'Infrastructure-Security' project (Project #2):
        * `gh issue create --project "Infrastructure-Security" --title "SoC2 CC7.1 SLA Breach: Critical Vulnerability CVE-2025-XXXX Unpatched" --body "The vulnerability CVE-2025-XXXX, identified on [DATE], remains unpatched after [N] days, exceeding the 15-day SLA defined in the Vulnerability Management Policy. Remediation ticket [LINK TO TICKET] is still open. This requires immediate escalation." --label "soc2,security,sla-breach"`
```

-----

### ISO 27001 Prompts

**Focus**: Information Security Management System (ISMS) and Annex A controls.

#### **Prompt: ISO 27001 A.5.15 - Access Control**

```
**Standard**: ISO/IEC 27001:2022
**Control**: A.5.15 - Access control
**Objective**: Audit the process for user access reviews to ensure system access is reviewed periodically and excessive permissions are revoked.

**Tasks**:
1.  **Locate Evidence**:
    * Find the Access Control Policy at `docs/policies/access-control-policy.md`. This document should define the frequency of access reviews (e.g., quarterly).
    * Locate evidence of the most recently completed access review. This may be a completed GitHub project, a set of closed issues, or a signed-off document located in `/audits/2025/Q2_Access_Review/`.
    * Generate a current list of all users with 'Administrator' or 'Owner' privileges on our GitHub organization and production AWS account. Assume scripts exist to do this: `scripts/get-gh-admins.sh` and `scripts/get-aws-admins.sh`.

2.  **Analyze Evidence**:
    * **Policy Compliance**: According to the policy, was the last quarterly access review completed on time?
    * **Review Completeness**: Examine the evidence from the last review. Was a review conducted for all privileged roles? Is there a record of who approved the access for each user?
    * **Current State**: Compare the current list of privileged users against the list of company employees in `/shared/hr/active-staff.csv`. Identify any users who have privileged access but are no longer with the company. These are known as "standing access" risks.

3.  **Report and Remediate**:
    * Summarize the findings of the audit. State whether the last access review was completed on schedule and was comprehensive.
    * If a required periodic review was missed, or if an account for a former employee still has privileged access, declare the control **"Non-Compliant"**.
    * Create a GitHub issue in the 'IT-Operations' project (Project #7):
        * `gh issue create --project "IT-Operations" --title "ISO 27001 A.5.15 Non-Compliance: Former Employee Account with Admin Access" --body "The user account '[USERNAME]' still has Administrator privileges in AWS but is no longer an active employee. This is a critical violation of the access control policy. Access must be revoked immediately." --label "iso27001,security,access-control"`
    * Create another issue if the review process itself is flawed:
        * `gh issue create --project "IT-Operations" --title "ISO 27001 A.5.15 Process Gap: Q2 Access Review Was Not Completed" --body "There is no evidence that the mandatory quarterly access review for Q2 2025 was performed, as required by the Access Control Policy. An immediate, out-of-band review must be conducted." --label "iso27001,process,compliance-gap"`

---
*Note: The prompt references ISO 27001, which is the correct standard for an Information Security Management System (ISMS), assuming the user's mention of "ISO72001" was a typo.*
```

We need tasks to create all of the other compliance tasks.
