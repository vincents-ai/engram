# SOX (Sarbanes-Oxley Act) Compliance Report

**Framework**: SOX (Sarbanes-Oxley Act) Section 404  
**Date**: 2025-08-22  
**Time**: 11:00  
**Auditor**: Auditron AI

## Navigation
- [← Back to Main Report](./index.md)
- [→ Next: COSO Framework Report](./coso.md)

## SOX Sections Audited

### Section 302: Corporate Responsibility for Financial Reports

#### Control: CEO/CFO Certification of Financial Statements

**Status**: ✅ **COMPLIANT**

**Requirements Checked**:
- Executive certification procedures in place
- Quarterly and annual certification process
- Internal controls evaluation documented

**Evidence Examined**:
- `legal/sox/executive-certifications/2025-Q2-certifications.pdf`
- `finance/reporting/quarterly-close-procedures.md`
- `compliance/sox/section-302-checklist.xlsx`

**Findings**:
- CEO and CFO certifications completed for Q2 2025
- Certification process includes review of internal controls
- No material weaknesses identified in executive assessments

**Conclusion**: Executive certification requirements fully met.

---

### Section 404: Management Assessment of Internal Controls

#### Control: IT General Controls (ITGC) - Access Controls

**Status**: ⚠️ **OBSERVATION**

**Requirements Checked**:
- User access management for financial systems
- Segregation of duties in financial processes
- Regular access reviews and certifications

**Evidence Examined**:
- `systems/erp/access-control-matrix.xlsx`
- `audits/sox/itgc/access-review-2025-Q2.pdf`
- `hr/joiner-mover-leaver/financial-systems-access.md`

**Findings**:
- Access controls properly implemented for financial systems
- Quarterly access reviews completed but 3 reviews were 2 days late
- Segregation of duties maintained across all critical processes

**Conclusion**: Minor process improvement needed in access review timing.

**GitHub Issue**: [#242 - SOX ITGC: Improve Access Review Timeliness](https://github.com/org/it-ops/issues/242)

---

#### Control: IT General Controls (ITGC) - Change Management

**Status**: ⚠️ **OBSERVATION**

**Requirements Checked**:
- Change approval process for financial systems
- Emergency change procedures
- Change documentation and testing

**Evidence Examined**:
- `systems/erp/change-management/procedures.md`
- `development/financial-systems/deployment-log-2025.json`
- `compliance/sox/change-management-audit-2025-Q2.xlsx`

**Findings**:
- Formal change management process in place
- All changes properly approved and documented
- 2 emergency changes lacked complete post-implementation documentation

**Conclusion**: Change management effective but documentation needs improvement.

**GitHub Issue**: [#244 - SOX: Complete Emergency Change Documentation](https://github.com/org/it-ops/issues/244)

---

### Section 409: Real-time Financial Disclosures

#### Control: Material Event Disclosure Process

**Status**: ✅ **COMPLIANT**

**Requirements Checked**:
- Process for identifying material events
- Timely disclosure procedures (4 business days)
- Legal review and approval workflow

**Evidence Examined**:
- `legal/disclosures/material-events-2025.log`
- `procedures/sox/material-event-assessment.md`
- `finance/sec-filings/8k-filings-2025.xlsx`

**Findings**:
- All material events identified and disclosed within required timeframe
- Legal review process ensures accuracy and completeness
- No late or missed disclosures in 2025

**Conclusion**: Material event disclosure process operating effectively.

## SOX Compliance Summary

### Internal Controls Assessment
- **Control Environment**: Effective
- **Risk Assessment**: Effective  
- **Control Activities**: Effective with minor observations
- **Information & Communication**: Effective
- **Monitoring**: Effective

### ITGC Assessment Results
| Control Area | Status | Issues |
|--------------|--------|--------|
| Access Controls | ⚠️ Minor Observations | 1 |
| Change Management | ⚠️ Minor Observations | 1 |
| Computer Operations | ✅ Effective | 0 |
| Security Administration | ✅ Effective | 0 |

### Management Certification Status
- **Q1 2025**: ✅ Certified (No Material Weaknesses)
- **Q2 2025**: ✅ Certified (No Material Weaknesses)
- **Next Certification**: Q3 2025 (Due September 15)

## Summary

- **Total Controls**: 4
- **Compliant**: 2
- **Non-Compliant**: 0
- **Observations**: 2

## External Auditor Status
- **Auditor**: Deloitte & Touche LLP
- **Last SOX 404 Opinion**: Effective (2024 Annual Report)
- **Management Letter Comments**: 2 (both resolved)
- **Next SOX 404 Audit**: Q4 2025

## GitHub Issues Created
- [#242 - SOX ITGC: Improve Access Review Timeliness](https://github.com/org/it-ops/issues/242)
- [#244 - SOX: Complete Emergency Change Documentation](https://github.com/org/it-ops/issues/244)

## Navigation
- [← Back to Main Report](./index.md)
- [→ Next: COSO Framework Report](./coso.md)