#!/bin/bash

# Comprehensive Compliance Automated Check Script
# Performs technical compliance checks across multiple frameworks

set -euo pipefail

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
AUDIT_TIMESTAMP=$(date +"%Y-%m-%d-%H%M")
REPORT_DIR="$PROJECT_ROOT/docs/compliance_checks/$AUDIT_TIMESTAMP"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Logging functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

# Initialize audit environment
init_audit() {
    log_info "Initializing compliance audit environment..."
    mkdir -p "$REPORT_DIR"/{findings,evidence,recommendations}
    
    # Create audit metadata
    cat > "$REPORT_DIR/audit-metadata.json" << EOF
{
    "auditTimestamp": "$AUDIT_TIMESTAMP",
    "auditorVersion": "Auditron v2.1",
    "projectRoot": "$PROJECT_ROOT",
    "frameworksAudited": [],
    "criticalFindings": 0,
    "totalFindings": 0
}
EOF
    
    log_success "Audit environment initialized at $REPORT_DIR"
}

# Security scanning functions
check_hardcoded_secrets() {
    log_info "🔍 Scanning for hardcoded secrets..."
    
    local findings_file="$REPORT_DIR/findings/hardcoded-secrets.json"
    local critical_findings=0
    
    # Common secret patterns
    local patterns=(
        "password\s*=\s*[\"'][^\"']+[\"']"
        "api[_-]?key\s*=\s*[\"'][^\"']+[\"']"
        "secret\s*=\s*[\"'][^\"']+[\"']"
        "token\s*=\s*[\"'][^\"']+[\"']"
        "private[_-]?key\s*=\s*[\"'][^\"']+[\"']"
        "aws[_-]?access[_-]?key"
        "aws[_-]?secret[_-]?key"
        "database[_-]?password"
        "db[_-]?password"
    )
    
    cat > "$findings_file" << EOF
{
    "check": "hardcoded-secrets",
    "timestamp": "$(date -Iseconds)",
    "findings": [
EOF
    
    local first_finding=true
    
    for pattern in "${patterns[@]}"; do
        # Search for pattern in source files
        while IFS= read -r match; do
            if [ "$first_finding" = true ]; then
                first_finding=false
            else
                echo "," >> "$findings_file"
            fi
            
            local file=$(echo "$match" | cut -d: -f1)
            local line=$(echo "$match" | cut -d: -f2)
            local content=$(echo "$match" | cut -d: -f3-)
            
            cat >> "$findings_file" << EOF
        {
            "type": "CRITICAL",
            "pattern": "$pattern",
            "file": "$file",
            "line": $line,
            "content": "$(echo "$content" | sed 's/"/\\"/g')",
            "recommendation": "Move secret to environment variable or secure vault"
        }
EOF
            critical_findings=$((critical_findings + 1))
            log_error "Hardcoded secret found in $file:$line"
            
        done < <(rg -n "$pattern" --type js --type ts --type py --type java --type yaml --type json . 2>/dev/null || true)
    done
    
    cat >> "$findings_file" << EOF
    ],
    "summary": {
        "totalFindings": $critical_findings,
        "criticalFindings": $critical_findings
    }
}
EOF
    
    if [ $critical_findings -gt 0 ]; then
        log_error "Found $critical_findings hardcoded secrets"
    else
        log_success "No hardcoded secrets detected"
    fi
    
    return $critical_findings
}

# Database security checks
check_database_security() {
    log_info "🔍 Checking database security configuration..."
    
    local findings_file="$REPORT_DIR/findings/database-security.json"
    local findings=0
    
    cat > "$findings_file" << EOF
{
    "check": "database-security",
    "timestamp": "$(date -Iseconds)",
    "findings": [
EOF
    
    local first_finding=true
    
    # Check for unencrypted database connections
    while IFS= read -r match; do
        local file=$(echo "$match" | cut -d: -f1)
        local line=$(echo "$match" | cut -d: -f2)
        
        # Check if SSL/TLS is disabled or missing
        if echo "$match" | grep -q "ssl.*false\|sslmode.*disable\|encrypt.*false"; then
            if [ "$first_finding" = true ]; then
                first_finding=false
            else
                echo "," >> "$findings_file"
            fi
            
            cat >> "$findings_file" << EOF
        {
            "type": "HIGH",
            "issue": "Unencrypted database connection",
            "file": "$file",
            "line": $line,
            "recommendation": "Enable SSL/TLS encryption for database connections"
        }
EOF
            findings=$((findings + 1))
            log_warn "Unencrypted database connection found in $file:$line"
        fi
        
    done < <(rg -n "database|db_|connection" --type js --type ts --type py --type yaml . 2>/dev/null || true)
    
    # Check for database credentials in config files
    while IFS= read -r match; do
        local file=$(echo "$match" | cut -d: -f1)
        
        if [ "$first_finding" = true ]; then
            first_finding=false
        else
            echo "," >> "$findings_file"
        fi
        
        cat >> "$findings_file" << EOF
        {
            "type": "CRITICAL",
            "issue": "Database credentials in configuration file",
            "file": "$file",
            "recommendation": "Use environment variables or secure credential store"
        }
EOF
        findings=$((findings + 1))
        log_error "Database credentials found in config file: $file"
        
    done < <(find . -name "*.env*" -o -name "*config*" | xargs rg -l "password.*=|user.*=|username.*=" 2>/dev/null || true)
    
    cat >> "$findings_file" << EOF
    ],
    "summary": {
        "totalFindings": $findings
    }
}
EOF
    
    if [ $findings -gt 0 ]; then
        log_warn "Found $findings database security issues"
    else
        log_success "Database security configuration looks good"
    fi
    
    return $findings
}

# PCI DSS specific checks
check_pci_dss_compliance() {
    log_info "🔍 Performing PCI DSS compliance checks..."
    
    local findings_file="$REPORT_DIR/findings/pci-dss.json"
    local critical_findings=0
    
    cat > "$findings_file" << EOF
{
    "check": "pci-dss-compliance",
    "timestamp": "$(date -Iseconds)",
    "findings": [
EOF
    
    local first_finding=true
    
    # Check for prohibited data storage (CVV, Track data, PIN)
    local prohibited_patterns=(
        "cvv|cvc"
        "track.*data|magnetic.*stripe"
        "pin.*block|pin.*verification"
    )
    
    for pattern in "${prohibited_patterns[@]}"; do
        while IFS= read -r match; do
            if [ "$first_finding" = true ]; then
                first_finding=false
            else
                echo "," >> "$findings_file"
            fi
            
            local file=$(echo "$match" | cut -d: -f1)
            local line=$(echo "$match" | cut -d: -f2)
            
            cat >> "$findings_file" << EOF
        {
            "type": "CRITICAL",
            "issue": "Prohibited cardholder data storage detected",
            "pattern": "$pattern",
            "file": "$file",
            "line": $line,
            "recommendation": "Remove prohibited data storage immediately - PCI DSS violation"
        }
EOF
            critical_findings=$((critical_findings + 1))
            log_error "PCI DSS violation: Prohibited data pattern '$pattern' in $file:$line"
            
        done < <(rg -ni "$pattern" --type js --type ts --type py --type sql . 2>/dev/null || true)
    done
    
    # Check for unencrypted PAN patterns
    while IFS= read -r match; do
        local file=$(echo "$match" | cut -d: -f1)
        local line=$(echo "$match" | cut -d: -f2)
        
        # Check if this appears to be test data or real PAN
        if ! echo "$file" | grep -q "test\|spec\|mock"; then
            if [ "$first_finding" = true ]; then
                first_finding=false
            else
                echo "," >> "$findings_file"
            fi
            
            cat >> "$findings_file" << EOF
        {
            "type": "CRITICAL",
            "issue": "Potential unencrypted PAN data",
            "file": "$file",
            "line": $line,
            "recommendation": "Verify if this is real PAN data and ensure proper encryption/tokenization"
        }
EOF
            critical_findings=$((critical_findings + 1))
            log_error "Potential PAN data found in $file:$line"
        fi
        
    done < <(rg -n "4[0-9]{12}(?:[0-9]{3})?|5[1-5][0-9]{14}|3[47][0-9]{13}" . 2>/dev/null || true)
    
    cat >> "$findings_file" << EOF
    ],
    "summary": {
        "totalFindings": $critical_findings,
        "criticalFindings": $critical_findings
    }
}
EOF
    
    if [ $critical_findings -gt 0 ]; then
        log_error "Found $critical_findings PCI DSS compliance issues"
    else
        log_success "No obvious PCI DSS violations detected"
    fi
    
    return $critical_findings
}

# GDPR compliance checks
check_gdpr_compliance() {
    log_info "🔍 Checking GDPR compliance implementation..."
    
    local findings_file="$REPORT_DIR/findings/gdpr.json"
    local findings=0
    
    cat > "$findings_file" << EOF
{
    "check": "gdpr-compliance",
    "timestamp": "$(date -Iseconds)",
    "findings": [
EOF
    
    local first_finding=true
    
    # Check for data subject rights implementation
    local required_endpoints=(
        "delete.*user|erase.*user|user.*delete"
        "export.*user|download.*data|user.*export"
        "privacy.*policy|cookie.*policy"
    )
    
    for endpoint_pattern in "${required_endpoints[@]}"; do
        local found=false
        if rg -q "$endpoint_pattern" --type js --type ts routes/ api/ 2>/dev/null; then
            found=true
        fi
        
        if [ "$found" = false ]; then
            if [ "$first_finding" = true ]; then
                first_finding=false
            else
                echo "," >> "$findings_file"
            fi
            
            cat >> "$findings_file" << EOF
        {
            "type": "HIGH",
            "issue": "Missing data subject rights endpoint",
            "pattern": "$endpoint_pattern",
            "recommendation": "Implement endpoint for data subject rights (GDPR Article 15-22)"
        }
EOF
            findings=$((findings + 1))
            log_warn "Missing GDPR endpoint pattern: $endpoint_pattern"
        fi
    done
    
    # Check for consent management
    if ! rg -q "consent|cookie.*accept|privacy.*accept" --type js --type ts . 2>/dev/null; then
        if [ "$first_finding" = true ]; then
            first_finding=false
        else
            echo "," >> "$findings_file"
        fi
        
        cat >> "$findings_file" << EOF
        {
            "type": "MEDIUM",
            "issue": "No consent management implementation found",
            "recommendation": "Implement consent management for cookies and data processing"
        }
EOF
        findings=$((findings + 1))
        log_warn "No consent management implementation detected"
    fi
    
    cat >> "$findings_file" << EOF
    ],
    "summary": {
        "totalFindings": $findings
    }
}
EOF
    
    if [ $findings -gt 0 ]; then
        log_warn "Found $findings GDPR compliance gaps"
    else
        log_success "GDPR implementation appears adequate"
    fi
    
    return $findings
}

# Access control checks
check_access_controls() {
    log_info "🔍 Analyzing access controls and authentication..."
    
    local findings_file="$REPORT_DIR/findings/access-controls.json"
    local findings=0
    
    cat > "$findings_file" << EOF
{
    "check": "access-controls",
    "timestamp": "$(date -Iseconds)",
    "findings": [
EOF
    
    local first_finding=true
    
    # Check for unprotected admin endpoints
    while IFS= read -r match; do
        local file=$(echo "$match" | cut -d: -f1)
        local line=$(echo "$match" | cut -d: -f2)
        local content=$(echo "$match" | cut -d: -f3-)
        
        # Check if endpoint has authentication middleware
        local route_content=$(grep -A 10 -B 2 "$content" "$file" 2>/dev/null || echo "")
        
        if ! echo "$route_content" | grep -q "auth\|authenticate\|protect\|middleware"; then
            if [ "$first_finding" = true ]; then
                first_finding=false
            else
                echo "," >> "$findings_file"
            fi
            
            cat >> "$findings_file" << EOF
        {
            "type": "CRITICAL",
            "issue": "Unprotected admin endpoint",
            "file": "$file",
            "line": $line,
            "endpoint": "$(echo "$content" | sed 's/"/\\"/g')",
            "recommendation": "Add authentication middleware to admin endpoints"
        }
EOF
            findings=$((findings + 1))
            log_error "Unprotected admin endpoint in $file:$line"
        fi
        
    done < <(rg -n "/admin|/manage|/dashboard" --type js --type ts routes/ api/ 2>/dev/null || true)
    
    # Check for weak session configuration
    while IFS= read -r match; do
        local file=$(echo "$match" | cut -d: -f1)
        
        if echo "$match" | grep -q "secure.*false\|httpOnly.*false\|sameSite.*none"; then
            if [ "$first_finding" = true ]; then
                first_finding=false
            else
                echo "," >> "$findings_file"
            fi
            
            cat >> "$findings_file" << EOF
        {
            "type": "MEDIUM",
            "issue": "Weak session security configuration",
            "file": "$file",
            "recommendation": "Enable secure, httpOnly, and sameSite session options"
        }
EOF
            findings=$((findings + 1))
            log_warn "Weak session configuration in $file"
        fi
        
    done < <(rg -n "session|cookie" --type js --type ts config/ . 2>/dev/null || true)
    
    cat >> "$findings_file" << EOF
    ],
    "summary": {
        "totalFindings": $findings
    }
}
EOF
    
    if [ $findings -gt 0 ]; then
        log_warn "Found $findings access control issues"
    else
        log_success "Access controls appear properly implemented"
    fi
    
    return $findings
}

# Generate comprehensive report
generate_report() {
    log_info "📊 Generating compliance audit report..."
    
    local total_critical=0
    local total_findings=0
    
    # Aggregate findings from all checks
    for findings_file in "$REPORT_DIR"/findings/*.json; do
        if [ -f "$findings_file" ]; then
            local file_critical=$(jq '.summary.criticalFindings // 0' "$findings_file" 2>/dev/null || echo 0)
            local file_total=$(jq '.summary.totalFindings // 0' "$findings_file" 2>/dev/null || echo 0)
            
            total_critical=$((total_critical + file_critical))
            total_findings=$((total_findings + file_total))
        fi
    done
    
    # Update audit metadata
    jq --arg critical "$total_critical" --arg total "$total_findings" \
       '.criticalFindings = ($critical | tonumber) | .totalFindings = ($total | tonumber)' \
       "$REPORT_DIR/audit-metadata.json" > "$REPORT_DIR/audit-metadata.tmp" && \
       mv "$REPORT_DIR/audit-metadata.tmp" "$REPORT_DIR/audit-metadata.json"
    
    # Generate summary report
    cat > "$REPORT_DIR/compliance-summary.md" << EOF
# Automated Compliance Check Summary

**Audit Date**: $(date)
**Auditor**: Auditron Automated Scanner v2.1

## Overall Results

- **Total Findings**: $total_findings
- **Critical Findings**: $total_critical
- **Risk Level**: $([ $total_critical -gt 0 ] && echo "HIGH" || [ $total_findings -gt 5 ] && echo "MEDIUM" || echo "LOW")

## Checks Performed

EOF
    
    # Add details for each check
    for findings_file in "$REPORT_DIR"/findings/*.json; do
        if [ -f "$findings_file" ]; then
            local check_name=$(basename "$findings_file" .json)
            local check_total=$(jq '.summary.totalFindings // 0' "$findings_file" 2>/dev/null || echo 0)
            local status="✅ PASS"
            
            if [ "$check_total" -gt 0 ]; then
                status="⚠️ ISSUES FOUND"
            fi
            
            echo "- **${check_name}**: $status ($check_total findings)" >> "$REPORT_DIR/compliance-summary.md"
        fi
    done
    
    cat >> "$REPORT_DIR/compliance-summary.md" << EOF

## Next Steps

EOF
    
    if [ $total_critical -gt 0 ]; then
        cat >> "$REPORT_DIR/compliance-summary.md" << EOF
### 🚨 IMMEDIATE ACTION REQUIRED

**$total_critical critical findings** need immediate attention:

1. Review all critical findings in the detailed reports
2. Prioritize security vulnerabilities and compliance violations
3. Create GitHub issues for tracking remediation
4. Implement fixes within 48 hours for critical items

EOF
    fi
    
    if [ $total_findings -gt $total_critical ]; then
        local non_critical=$((total_findings - total_critical))
        cat >> "$REPORT_DIR/compliance-summary.md" << EOF
### 📋 Additional Actions

**$non_critical additional findings** should be addressed:

1. Review medium and low priority findings
2. Plan remediation within 30 days
3. Update policies and procedures as needed
4. Schedule follow-up audit after fixes

EOF
    fi
    
    cat >> "$REPORT_DIR/compliance-summary.md" << EOF

## Detailed Reports

- [Findings Directory](./$AUDIT_TIMESTAMP/findings/)
- [Evidence Requests](./$AUDIT_TIMESTAMP/evidence/)
- [Recommendations](./$AUDIT_TIMESTAMP/recommendations/)

EOF
    
    log_success "Compliance report generated at $REPORT_DIR/compliance-summary.md"
    
    # Print summary to console
    echo ""
    echo "======================================"
    echo "  COMPLIANCE AUDIT SUMMARY"
    echo "======================================"
    echo "Total Findings: $total_findings"
    echo "Critical Findings: $total_critical"
    echo ""
    
    if [ $total_critical -gt 0 ]; then
        log_error "CRITICAL ISSUES DETECTED - Immediate action required"
    elif [ $total_findings -gt 5 ]; then
        log_warn "Multiple issues found - Review and remediate"
    else
        log_success "Compliance posture appears good"
    fi
    
    echo "Detailed report: $REPORT_DIR/compliance-summary.md"
    echo "======================================"
}

# Main execution
main() {
    echo "🤖 Auditron Automated Compliance Scanner v2.1"
    echo "=============================================="
    
    init_audit
    
    local exit_code=0
    
    # Run all compliance checks
    check_hardcoded_secrets || exit_code=1
    check_database_security || exit_code=1  
    check_pci_dss_compliance || exit_code=1
    check_gdpr_compliance || exit_code=1
    check_access_controls || exit_code=1
    
    generate_report
    
    echo ""
    echo "🏁 Automated compliance scan completed"
    echo "📁 Results saved to: $REPORT_DIR"
    
    exit $exit_code
}

# Run main function if script is executed directly
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    main "$@"
fi