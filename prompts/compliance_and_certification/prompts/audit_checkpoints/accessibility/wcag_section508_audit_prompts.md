# WCAG and Section 508 Web Accessibility Audit Prompts


  
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


This collection provides comprehensive audit checkpoint prompts for WCAG 2.1 and Section 508 web accessibility compliance assessment.

## Context and Background

### WCAG 2.1 (Web Content Accessibility Guidelines)
**Effective Date**: June 5, 2018 (current version)  
**Scope**: Web content, applications, and digital documents  
**Key Principles**: Perceivable, Operable, Understandable, Robust (POUR)  
**Structure**: 13 guidelines with success criteria at three levels (A, AA, AAA)  
**Benefits**: Inclusive design, legal compliance, improved user experience, broader audience reach

### Section 508 (Rehabilitation Act Amendments)
**Effective Date**: January 18, 2018 (updated standards)  
**Scope**: Federal agency websites and digital content  
**Key Requirements**: Software applications, web-based intranet and internet information, telecommunications products  
**Structure**: Technical standards covering accessibility requirements  
**Benefits**: Federal compliance, accessibility for people with disabilities, improved usability

## Regulatory Context
- **WCAG 2.1**: W3C recommendation for web accessibility
- **Section 508**: U.S. federal law requiring accessible technology
- **Legal Compliance**: Required for many organizations under disability discrimination laws
- **International Standards**: WCAG widely adopted globally

## WCAG 2.1 Comprehensive Accessibility Audit Prompt

Conduct comprehensive WCAG 2.1 web accessibility audit:

**Website Type:** [PUBLIC_COMMERCIAL_GOVERNMENT_EDUCATIONAL]
**Content Type:** [STATIC_DYNAMIC_MULTIMEDIA_INTERACTIVE]
**Target Compliance Level:** [A_AA_AAA]
**User Testing:** [AUTOMATED_MANUAL_COMBINED]

**WCAG 2.1 Compliance Assessment:**

1. **Principle 1: Perceivable**
   - Evaluate text alternatives for non-text content
   - Assess time-based media alternatives
   - Review adaptable content presentation
   - Validate distinguishable content

2. **Principle 2: Operable**
   - Assess keyboard accessibility
   - Review sufficient time provisions
   - Validate seizures and physical reactions
   - Confirm navigable content

3. **Principle 3: Understandable**
   - Evaluate readable text
   - Assess predictable content
   - Review input assistance
   - Validate understandable content

4. **Principle 4: Robust**
   - Assess compatible content
   - Review parsing and interpretation
   - Validate name, role, value requirements
   - Confirm content compatibility

**WCAG 2.1 Compliance Scoring:**
- **Perceivable**: [SCORE]/100
- **Operable**: [SCORE]/100
- **Understandable**: [SCORE]/100
- **Robust**: [SCORE]/100

## Section 508 Comprehensive Accessibility Audit Prompt

Conduct comprehensive Section 508 accessibility audit:

**Federal Agency:** [AGENCY_NAME_PROGRAM_OFFICE]
**Content Type:** [WEBSITE_APPLICATION_DOCUMENT_MULTIMEDIA]
**Procurement Status:** [NEW_ACQUISITION_EXISTING_SYSTEM]
**Conformance Level:** [FULL_PARTIAL_NON_CONFORMANT]

**Section 508 Compliance Assessment:**

1. **Software Applications and Operating Systems**
   - Evaluate keyboard navigation support
   - Assess screen reader compatibility
   - Review timing function controls
   - Validate color and contrast requirements

2. **Web-Based Intranet and Internet Information**
   - Assess text equivalent alternatives
   - Review multimedia presentation synchronization
   - Validate color and contrast usage
   - Confirm server-side image maps

3. **Telecommunications Products**
   - Evaluate video content captions
   - Assess audio content transcripts
   - Review volume control mechanisms
   - Validate hearing aid compatibility

4. **Self-Contained, Closed Products**
   - Assess private listening features
   - Review tactilely discernible keys
   - Validate visual output accessibility
   - Confirm color coding alternatives

5. **Desktop and Portable Computers**
   - Evaluate mechanically operated controls
   - Assess key repeat and sticky keys
   - Review toggle keys and sound sourcing
   - Validate adjustable response rate

**Section 508 Compliance Scoring:**
- **Software Applications**: [SCORE]/100
- **Web-Based Information**: [SCORE]/100
- **Telecommunications**: [SCORE]/100
- **Self-Contained Products**: [SCORE]/100
- **Computers**: [SCORE]/100

## Accessibility Testing Methodology

**Automated Testing Tools:**
- **Coverage**: [PERCENTAGE_OF_ISSUES_DETECTED]
- **False Positives**: [NUMBER_OF_FALSE_POSITIVES]
- **False Negatives**: [NUMBER_OF_FALSE_NEGATIVES]
- **Tools Used**: [TOOL_NAMES_VERSIONS]

**Manual Testing Procedures:**
- **Keyboard Navigation**: [PASS_FAIL_PARTIAL]
- **Screen Reader Testing**: [PASS_FAIL_PARTIAL]
- **Color Contrast Analysis**: [PASS_FAIL_PARTIAL]
- **User Testing**: [NUMBER_OF_PARTICIPANTS_FINDINGS]

## Accessibility Integration Assessment

**Multi-Framework Compliance:**
- WCAG + Section 508: [ALIGNMENT_SCORE]/100
- WCAG + EN 301 549: [ALIGNMENT_SCORE]/100
- Section 508 + ADA: [ALIGNMENT_SCORE]/100

**Digital Accessibility Framework:**
1. **Accessibility Policy and Governance**
2. **Inclusive Design and Development**
3. **Testing and Validation Procedures**
4. **Remediation and Continuous Improvement**
5. **Training and Awareness Programs**