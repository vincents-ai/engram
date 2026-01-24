---
name: engram-runbooks
description: "Create operational runbooks for deployment procedures, incident response, maintenance tasks, with links to code and infrastructure."
---

# Operational Runbooks (Engram-Integrated)

## Overview

Create comprehensive operational runbooks that document procedures for deployment, incident response, routine maintenance, and emergency operations. Link runbooks to relevant code, infrastructure, monitoring dashboards, and related tasks in Engram for easy access during operations.

## When to Use

Use this skill when:
- Setting up operations for a new service
- Documenting incident response procedures
- Creating deployment guides for complex systems
- Onboarding new team members to on-call rotation
- After an incident to codify learnings
- Standardizing operations across team
- Preparing for compliance audits (need documented procedures)
- Reducing MTTR (Mean Time To Recovery)

## The Pattern

### Step 1: Identify Operational Procedures

List what needs runbooks:

```bash
engram context create \
  --title "Runbook Requirements: [Service Name]" \
  --content "## Service Overview\n\n**Service:** [Name]\n**Purpose:** [What it does]\n**Criticality:** [Critical/High/Medium/Low]\n**On-Call Team:** [Team name]\n**Escalation:** [Who to escalate to]\n\n## Operational Procedures Needed\n\n### Deployment\n- Standard deployment (rolling, blue-green, canary)\n- Rollback procedure\n- Database migration deployment\n- Emergency hotfix deployment\n\n### Incident Response\n- Service down (all instances unreachable)\n- High error rate (> 5%)\n- High latency (p99 > 1s)\n- Database issues (connection failures, slow queries)\n- External dependency failure (API timeout)\n- Security incident (breach detected)\n\n### Maintenance\n- Database backup and restore\n- Log rotation and cleanup\n- Certificate renewal\n- Dependency updates\n- Scaling (add/remove instances)\n- Configuration changes\n\n### Monitoring\n- Dashboard locations\n- Key metrics to watch\n- Alert definitions\n- Log query examples\n\n## Links to Resources\n\n**Code:**\n- Repository: [URL]\n- Deployment scripts: [Path]\n- Configuration: [Path]\n\n**Infrastructure:**\n- Terraform/CDK: [Repository]\n- AWS Console: [URL]\n- Kubernetes: [Cluster name, namespace]\n\n**Monitoring:**\n- Grafana: [Dashboard URL]\n- CloudWatch: [Log group]\n- Sentry: [Project URL]\n- PagerDuty: [Service URL]\n\n**Documentation:**\n- Architecture diagram: [URL]\n- API docs: [URL]\n- System design: [Engram context ID]" \
  --source "runbooks" \
  --tags "runbooks,requirements,[service-name]"
```

### Step 2: Create Deployment Runbook

Document deployment procedures:

```bash
engram context create \
  --title "Runbook: Deploy [Service Name]" \
  --content "# Deployment Runbook: [Service Name]\n\n## Prerequisites\n\n**Access Required:**\n- AWS account access (role: deployer)\n- GitHub repository access\n- Kubernetes cluster access (namespace: [name])\n- PagerDuty access (for maintenance window)\n\n**Before Deploy:**\n- [ ] All tests passing on main branch\n- [ ] Code review approved\n- [ ] Database migrations tested in staging\n- [ ] Deployment window scheduled (for high-risk deploys)\n- [ ] On-call engineer notified\n- [ ] Rollback plan ready\n\n## Standard Deployment (Rolling Update)\n\n**When to Use:** Normal feature releases, low-risk changes\n**Downtime:** None (zero-downtime deployment)\n**Duration:** 10-15 minutes\n\n### Steps\n\n**1. Verify Current State**\n\n```bash\n# Check current version\nkubectl get deployment [service-name] -n [namespace] -o jsonpath='{.spec.template.spec.containers[0].image}'\n\n# Check all pods healthy\nkubectl get pods -n [namespace] -l app=[service-name]\n# All should be Running with READY 1/1\n\n# Check recent errors\nkubectl logs -n [namespace] -l app=[service-name] --tail=50 | grep ERROR\n# Should see no recent errors\n```\n\n**Link:** Dashboard showing current health: [URL]\n\n**2. Create Deployment**\n\n```bash\n# Tag release\ngit tag v1.2.3\ngit push origin v1.2.3\n\n# CI/CD will automatically:\n# - Build Docker image\n# - Push to ECR\n# - Update Kubernetes deployment\n\n# Or manual deploy:\nkubectl set image deployment/[service-name] \\\n  [service-name]=[ecr-url]:[tag] \\\n  -n [namespace]\n```\n\n**Link:** CI/CD pipeline: [URL]\n**Link:** Deployment code: [repository]/deploy/kubernetes/deployment.yaml:15\n\n**3. Monitor Rollout**\n\n```bash\n# Watch rollout status\nkubectl rollout status deployment/[service-name] -n [namespace]\n# Wait for: deployment \"[service-name]\" successfully rolled out\n\n# Check new pods starting\nkubectl get pods -n [namespace] -l app=[service-name] -w\n# New pods should reach Running status\n# Old pods should terminate gracefully\n```\n\n**4. Verify Health**\n\n```bash\n# Check pod logs for startup errors\nkubectl logs -n [namespace] -l app=[service-name] --tail=100\n# Should see: Server started on port 8080\n\n# Test health endpoint\nkubectl exec -n [namespace] deploy/[service-name] -- \\\n  curl -s http://localhost:8080/health\n# Should return: {\"status\":\"healthy\"}\n\n# Check metrics\ncurl -s https://[service-url]/metrics | grep http_requests_total\n# Should show requests being handled\n```\n\n**Link:** Grafana dashboard: [URL]\n\n**5. Smoke Test**\n\n```bash\n# Run automated smoke tests\n./tests/smoke/run.sh [environment]\n\n# Or manual test key endpoints:\ncurl -X GET https://[service-url]/api/health\ncurl -X GET https://[service-url]/api/users/test-user\ncurl -X POST https://[service-url]/api/orders -d '{...}'\n```\n\n**Link:** Smoke test code: [repository]/tests/smoke/run.sh:1\n\n**6. Monitor for 15 Minutes**\n\n**Watch these metrics:**\n- Error rate: Should be < 0.1% (same as before deploy)\n- Latency p99: Should be < 500ms (same as before deploy)\n- Traffic rate: Should be normal\n- Memory/CPU: Should stabilize after 5 minutes\n\n**Link:** Real-time metrics: [Grafana URL]\n\n**If any metric degrades → Rollback immediately (see below)**\n\n**7. Confirm Success**\n\n```bash\n# Verify all replicas ready\nkubectl get deployment [service-name] -n [namespace]\n# READY should show N/N (all replicas)\n\n# Check no recent errors\nkubectl logs -n [namespace] -l app=[service-name] --since=15m | grep ERROR\n# Should be clean or expected errors only\n```\n\n**8. Update Runbook**\n\n- Update \"Current Version\" in this runbook\n- Document any issues encountered\n- Update related Engram tasks\n\n## Rollback Procedure\n\n**When to Rollback:**\n- Error rate increased > 2x baseline\n- p99 latency increased > 2x baseline\n- New critical bug discovered\n- Deployment stuck (not progressing)\n\n**Steps:**\n\n```bash\n# 1. Immediately rollback\nkubectl rollout undo deployment/[service-name] -n [namespace]\n\n# 2. Verify rollback\nkubectl rollout status deployment/[service-name] -n [namespace]\n\n# 3. Check health\nkubectl get pods -n [namespace] -l app=[service-name]\n# All should be Running\n\n# 4. Verify metrics recovered\n# Check Grafana: error rate and latency should return to baseline\n\n# 5. Investigate issue\n# Check logs from failed deployment:\nkubectl logs -n [namespace] -l app=[service-name] \\\n  --previous --tail=500 > failed-deployment-logs.txt\n\n# 6. Create incident postmortem\nengram task create \\\n  --title \"Incident: Failed deployment of [service] v1.2.3\" \\\n  --description \"Error rate spiked to X%, rolled back to previous version\"\n```\n\n**Link:** Incident response runbook: [Engram context ID]\n\n## Database Migration Deployment\n\n**When to Use:** Deploy includes database schema changes\n**Downtime:** Depends on migration (aim for zero)\n**Duration:** 20-30 minutes\n\n### Steps\n\n**1. Prepare Migration**\n\n```bash\n# Review migration SQL\ncat migrations/002_add_user_email_index.sql\n\n# Test in staging\npsql -h [staging-db] -U [user] -d [database] < migrations/002_add_user_email_index.sql\n\n# Estimate duration (test with production-sized data)\n# If migration > 5 minutes, consider maintenance window\n```\n\n**Link:** Migration code: [repository]/migrations/002_add_user_email_index.sql:1\n\n**2. Backup Database**\n\n```bash\n# Create snapshot (AWS RDS)\naws rds create-db-snapshot \\\n  --db-instance-identifier [db-name] \\\n  --db-snapshot-identifier [db-name]-before-v1.2.3-$(date +%Y%m%d-%H%M)\n\n# Verify snapshot created\naws rds describe-db-snapshots \\\n  --db-snapshot-identifier [snapshot-name]\n# Status should be: available\n```\n\n**Link:** Restore procedure: [Engram context: Runbook: Restore Database]\n\n**3. Run Migration**\n\n```bash\n# Apply migration\nkubectl exec -n [namespace] deploy/[service-name] -- \\\n  npm run migrate:up\n\n# Or direct SQL:\npsql -h [db-host] -U [user] -d [database] < migrations/002_add_user_email_index.sql\n\n# Verify migration applied\npsql -h [db-host] -U [user] -d [database] -c \"\\d users\"\n# Should show new email_idx index\n```\n\n**4. Deploy Application**\n\n(Follow standard deployment steps above)\n\n**5. Verify Migration**\n\n```bash\n# Check migration status\npsql -h [db-host] -U [user] -d [database] -c \\\n  \"SELECT * FROM schema_migrations ORDER BY version DESC LIMIT 5;\"\n\n# Verify new code uses new schema\nkubectl logs -n [namespace] -l app=[service-name] --tail=100 | grep \"email query\"\n```\n\n## Emergency Hotfix Deployment\n\n**When to Use:** Critical bug in production, need immediate fix\n**Downtime:** Minimize (5 minutes acceptable)\n**Duration:** 30 minutes (expedited process)\n\n### Steps\n\n**1. Create Hotfix Branch**\n\n```bash\ngit checkout main\ngit pull\ngit checkout -b hotfix/critical-bug-fix\n\n# Make minimal fix\n# Add test\n# Commit\n```\n\n**2. Fast-Track Review**\n\n- Create PR with label: `priority:critical`\n- Get single reviewer approval (not usual 2)\n- Run automated tests (must pass)\n- Skip staging deployment (go straight to prod if critical)\n\n**3. Deploy Immediately**\n\n```bash\n# Merge to main\ngit checkout main\ngit pull\ngit merge hotfix/critical-bug-fix\ngit tag v1.2.4-hotfix\ngit push origin v1.2.4-hotfix\n\n# Deploy (follow standard deployment)\n```\n\n**4. Monitor Closely**\n\n- Watch metrics for 30 minutes (longer than standard)\n- Have rollback ready\n- Keep on-call engineer online\n\n**5. Backfill Process**\n\n- Schedule full PR review post-deploy\n- Update tests if needed\n- Document in incident postmortem\n\n## Post-Deployment Checklist\n\n- [ ] All pods Running and Ready\n- [ ] Error rate < 0.1%\n- [ ] Latency p99 < 500ms\n- [ ] Smoke tests passed\n- [ ] Monitored for 15 minutes\n- [ ] No alerts fired\n- [ ] Deployment logged in Slack #deployments\n- [ ] Version updated in runbook\n- [ ] Close deployment task in Engram\n\n## Common Issues\n\n### Issue: Pods in CrashLoopBackOff\n\n**Symptoms:** Pods restarting repeatedly, never reach Running\n\n**Diagnosis:**\n```bash\nkubectl logs -n [namespace] [pod-name] --previous\nkubectl describe pod -n [namespace] [pod-name]\n```\n\n**Common Causes:**\n- Missing environment variable\n- Database connection failure\n- Invalid configuration\n- OOM (out of memory)\n\n**Resolution:** Fix issue, redeploy or rollback\n\n**Link:** Troubleshooting guide: [Engram context ID]\n\n### Issue: Deployment Stuck (Not Progressing)\n\n**Symptoms:** New pods stuck in Pending or ContainerCreating\n\n**Diagnosis:**\n```bash\nkubectl describe pod -n [namespace] [pod-name]\n# Check Events section for errors\n```\n\n**Common Causes:**\n- Insufficient cluster resources (CPU/memory)\n- Image pull failure (wrong tag, auth issue)\n- Volume mount failure\n\n**Resolution:**\n- Scale down other services temporarily\n- Fix image tag\n- Rollback if blocking\n\n## Links\n\n**Code:** [repository]/README.md\n**Infrastructure:** [repository]/infrastructure/kubernetes/\n**Monitoring:** [Grafana dashboard URL]\n**Alerts:** [PagerDuty service URL]\n**Related Runbooks:**\n- Incident Response: [Engram context ID]\n- Database Restore: [Engram context ID]\n- Scaling: [Engram context ID]" \
  --source "runbooks" \
  --tags "runbooks,deployment,[service-name]"
```

### Step 3: Create Incident Response Runbook

Document incident procedures:

```bash
engram context create \
  --title "Runbook: Incident Response [Service Name]" \
  --content "# Incident Response Runbook: [Service Name]\n\n## General Incident Response Process\n\n**1. Acknowledge Alert**\n- Acknowledge in PagerDuty (stops escalation)\n- Post in Slack #incidents: \"Investigating [alert name]\"\n\n**2. Assess Severity**\n- **P0:** Complete outage, data loss, security breach\n- **P1:** Significant degradation, high error rate\n- **P2:** Minor degradation, isolated issue\n\n**3. Assemble Team (if P0/P1)**\n- Incident Commander: On-call engineer\n- Technical Lead: Service owner\n- Communications: Manager (for external communication)\n\n**4. Investigate and Mitigate**\n- Follow specific runbook below\n- Document actions in Slack thread\n- Focus on mitigation first, root cause later\n\n**5. Resolve**\n- Verify metrics returned to normal\n- Monitor for 15 minutes\n- Resolve alert in PagerDuty\n- Post resolution in Slack\n\n**6. Postmortem**\n- Create postmortem document (if P0/P1)\n- Schedule review meeting within 48 hours\n- Document in Engram\n\n---\n\n## Incident: Service Down (All Instances Unreachable)\n\n**Alert:** `ServiceDown` (P0)\n**Symptoms:** All health checks failing, 100% error rate\n**Impact:** Complete service outage, users cannot access\n**SLA Impact:** Yes (availability SLA breached if > 5 minutes)\n\n### Immediate Actions (First 5 Minutes)\n\n**1. Verify Outage**\n\n```bash\n# Check all instances\nkubectl get pods -n [namespace] -l app=[service-name]\n# If all pods show: Running → Check load balancer\n# If all pods show: CrashLoopBackOff → Check logs\n# If no pods → Check deployment\n\n# Test health endpoint\ncurl -v https://[service-url]/health\n# If timeout → Load balancer issue\n# If 502 Bad Gateway → All instances down\n# If 404 → Wrong URL (unlikely in incident)\n```\n\n**Link:** Service health dashboard: [Grafana URL]\n\n**2. Check Recent Changes**\n\n```bash\n# Check recent deployments\nkubectl rollout history deployment/[service-name] -n [namespace]\n\n# Check recent config changes\ngit log -n 5 --oneline config/production.yaml\n\n# Check recent infrastructure changes\ncd infrastructure && git log -n 5 --oneline\n```\n\n**If deployed in last 30 minutes → Rollback immediately (see Deployment Runbook)**\n\n**3. Check Dependencies**\n\n```bash\n# Check database\npsql -h [db-host] -U [user] -d [database] -c \"SELECT 1;\"\n# If fails → Database is down (see Database Incident below)\n\n# Check external APIs\ncurl -v https://[external-api]/health\n# If fails → External dependency down\n```\n\n**Link:** Dependency map: [Engram context: System Dependencies]\n\n### Investigation (Next 10 Minutes)\n\n**4. Check Logs**\n\n```bash\n# Get recent logs from all pods\nkubectl logs -n [namespace] -l app=[service-name] --tail=200 > incident-logs.txt\n\n# Search for errors\ngrep -i error incident-logs.txt\ngrep -i exception incident-logs.txt\ngrep -i fatal incident-logs.txt\n\n# Common issues:\n# - \"Cannot connect to database\" → Database issue\n# - \"Out of memory\" → OOM kill\n# - \"Port 8080 already in use\" → Config issue\n# - \"ECONNREFUSED\" → Dependency unreachable\n```\n\n**5. Check Resource Usage**\n\n```bash\n# Check cluster resources\nkubectl top nodes\n# If any node > 90% CPU or memory → Resource exhaustion\n\n# Check pod resources\nkubectl top pods -n [namespace] -l app=[service-name]\n\n# Check for evicted pods\nkubectl get pods -n [namespace] --field-selector=status.phase=Failed\n```\n\n**Link:** Cluster dashboard: [Grafana URL]\n\n### Mitigation\n\n**Scenario A: Recent Deployment Issue**\n\n```bash\n# Rollback to previous version\nkubectl rollout undo deployment/[service-name] -n [namespace]\n\n# Monitor recovery\nwatch kubectl get pods -n [namespace] -l app=[service-name]\n```\n\n**Scenario B: Database Connection Failure**\n\n```bash\n# Check database status\naws rds describe-db-instances --db-instance-identifier [db-name]\n\n# If database is down:\n# 1. Check AWS RDS console for maintenance\n# 2. Check recent config changes to connection string\n# 3. Check security groups (may have changed)\n# 4. Restart database if unresponsive (last resort)\n\n# Temporary mitigation: Enable read-only mode (if applicable)\nkubectl set env deployment/[service-name] READ_ONLY=true -n [namespace]\n```\n\n**Scenario C: Resource Exhaustion**\n\n```bash\n# Scale up replicas\nkubectl scale deployment/[service-name] --replicas=10 -n [namespace]\n\n# Or increase resource limits\nkubectl set resources deployment/[service-name] \\\n  --limits=memory=2Gi,cpu=1000m -n [namespace]\n```\n\n**Scenario D: External Dependency Down**\n\n```bash\n# Enable circuit breaker / fallback mode\nkubectl set env deployment/[service-name] \\\n  CIRCUIT_BREAKER_ENABLED=true -n [namespace]\n\n# Or deploy degraded mode (cached responses)\nkubectl set env deployment/[service-name] \\\n  FALLBACK_MODE=cached -n [namespace]\n```\n\n### Verification\n\n**6. Confirm Recovery**\n\n```bash\n# Check all pods Running\nkubectl get pods -n [namespace] -l app=[service-name]\n# All should be: Running and READY 1/1\n\n# Test health endpoint\ncurl https://[service-url]/health\n# Should return: 200 OK\n\n# Check metrics\n# Error rate should be < 1%\n# Latency should be < 500ms\n```\n\n**Link:** Service health dashboard: [Grafana URL]\n\n**7. Monitor for 15 Minutes**\n\nWatch for:\n- Error rate stays low\n- Latency returns to normal\n- No new alerts fire\n- Traffic rate returns to normal\n\n### Resolution\n\n**8. Document Incident**\n\n```bash\n# Create incident task\nengram task create \\\n  --title \"Incident: Service Down [Service Name] - $(date +%Y-%m-%d)\" \\\n  --description \"**Duration:** [start] to [end]\\n**Cause:** [Brief cause]\\n**Mitigation:** [What fixed it]\\n**Impact:** [User impact]\" \\\n  --priority high \\\n  --tags incident,service-down,[service-name]\n\n# Attach logs\nengram artifact create \\\n  --task-id [INCIDENT_TASK_ID] \\\n  --file incident-logs.txt \\\n  --description \"Logs from incident\"\n```\n\n**9. Communicate**\n\n```\nSlack #incidents:\n\"✅ RESOLVED: [Service Name] is back online.\nDuration: X minutes\nCause: [Brief]\nNext steps: Postmortem scheduled for [date]\"\n```\n\n**10. Create Postmortem**\n\nSee: [Engram context: Postmortem Template]\n\n---\n\n## Incident: High Error Rate\n\n**Alert:** `HighErrorRate` (P1)\n**Symptoms:** Error rate > 5%, service partially working\n**Impact:** Some users affected, others working\n**SLA Impact:** Maybe (depends on error budget)\n\n### Investigation\n\n**1. Identify Error Pattern**\n\n```bash\n# Check error breakdown\nkubectl logs -n [namespace] -l app=[service-name] --tail=500 | \\\n  grep ERROR | cut -d' ' -f5- | sort | uniq -c | sort -rn\n\n# Common patterns:\n# - All from one endpoint → Specific feature broken\n# - All one error type → Systemic issue\n# - Spread across endpoints → Dependency issue\n```\n\n**2. Check Traces**\n\nGo to Jaeger: [URL]\n- Filter by: service=[service-name], error=true, time=last 15 minutes\n- Examine slowest traces\n- Identify which span is failing\n\n**3. Check Recent Changes**\n\nSame as Service Down above\n\n### Mitigation\n\n**If error rate > 20%:** Rollback deployment\n\n**If error rate 5-20%:** Investigate specific cause\n- Feature flag: Disable broken feature\n- Circuit breaker: Open circuit to failing dependency\n- Rate limit: Reduce load if overload issue\n\n---\n\n## Incident: High Latency\n\n**Alert:** `HighLatency` (P1)\n**Symptoms:** p99 latency > 1s (target: < 500ms)\n**Impact:** Slow user experience\n**SLA Impact:** Yes (if sustained)\n\n### Investigation\n\n**1. Check Bottleneck**\n\n```bash\n# Check database query times\npsql -h [db-host] -U [user] -d [database] -c \\\n  \"SELECT query, mean_exec_time FROM pg_stat_statements \\\n   ORDER BY mean_exec_time DESC LIMIT 10;\"\n\n# Check external API latency\nkubectl logs -n [namespace] -l app=[service-name] | \\\n  grep \"external_api_duration\" | tail -100\n```\n\n**2. Check Traces**\n\nGo to Jaeger: [URL]\n- Sort by: duration (longest first)\n- Identify which span takes longest time\n- Database query? External API? CPU-intensive operation?\n\n### Mitigation\n\n- Slow database: Add read replicas, optimize query\n- Slow external API: Increase timeout, add circuit breaker\n- CPU bottleneck: Scale up replicas\n- Memory pressure: Increase memory limits\n\n---\n\n## Incident: Database Issues\n\n**Alert:** `DatabaseConnectionFailure` (P0)\n**Symptoms:** Cannot connect to database\n**Impact:** Complete service outage\n\n### Investigation\n\n```bash\n# Check database status\naws rds describe-db-instances --db-instance-identifier [db-name] | \\\n  jq '.DBInstances[0].DBInstanceStatus'\n# Should be: available\n\n# Test connection\npsql -h [db-host] -U [user] -d [database] -c \"SELECT 1;\"\n\n# Check connection pool\npsql -h [db-host] -U [user] -d [database] -c \\\n  \"SELECT count(*) FROM pg_stat_activity;\"\n# If at max connections → Connection leak\n```\n\n### Mitigation\n\n- Database down: Check AWS console, may need restart\n- Connection pool exhausted: Restart application, fix connection leak\n- Slow queries: Kill long-running queries, optimize\n\n**Link:** Database restore runbook: [Engram context ID]\n\n---\n\n## Links\n\n**Dashboards:**\n- Service Health: [Grafana URL]\n- Database Metrics: [Grafana URL]\n- Infrastructure: [Grafana URL]\n\n**Logs:**\n- Application Logs: [CloudWatch URL]\n- Database Logs: [CloudWatch URL]\n\n**Traces:**\n- Jaeger: [URL]\n\n**Alerts:**\n- PagerDuty: [Service URL]\n\n**Related Runbooks:**\n- Deployment: [Engram context ID]\n- Database Restore: [Engram context ID]\n- Maintenance: [Engram context ID]\n\n**Contacts:**\n- On-Call: See PagerDuty schedule\n- Manager: [Name] - [Contact]\n- Database Admin: [Name] - [Contact]" \
  --source "runbooks" \
  --tags "runbooks,incident-response,[service-name]"
```

### Step 4: Create Maintenance Runbook

Document routine maintenance:

```bash
engram context create \
  --title "Runbook: Maintenance [Service Name]" \
  --content "# Maintenance Runbook: [Service Name]\n\n## Database Backup\n\n**Frequency:** Daily (automated), weekly (manual verification)\n**Retention:** 30 days\n\n### Automated Backup\n\n```bash\n# Verify automated backup ran\naws rds describe-db-snapshots \\\n  --db-instance-identifier [db-name] \\\n  --snapshot-type automated \\\n  --max-records 5\n\n# Check latest backup timestamp\n# Should be within last 24 hours\n```\n\n### Manual Backup\n\n```bash\n# Create manual snapshot\naws rds create-db-snapshot \\\n  --db-instance-identifier [db-name] \\\n  --db-snapshot-identifier manual-backup-$(date +%Y%m%d)\n\n# Verify created\naws rds wait db-snapshot-completed \\\n  --db-snapshot-identifier manual-backup-$(date +%Y%m%d)\n```\n\n### Test Restore (Monthly)\n\n```bash\n# Restore to separate instance\naws rds restore-db-instance-from-db-snapshot \\\n  --db-instance-identifier [db-name]-restore-test \\\n  --db-snapshot-identifier [snapshot-id]\n\n# Connect and verify data\npsql -h [test-db-host] -U [user] -d [database] -c \\\n  \"SELECT count(*) FROM users;\"\n\n# Delete test instance\naws rds delete-db-instance \\\n  --db-instance-identifier [db-name]-restore-test \\\n  --skip-final-snapshot\n```\n\n## Certificate Renewal\n\n**Frequency:** Every 90 days (Let's Encrypt)\n**Alert:** 30 days before expiry\n\n```bash\n# Check certificate expiry\necho | openssl s_client -connect [domain]:443 2>/dev/null | \\\n  openssl x509 -noout -enddate\n\n# Renew certificate (cert-manager does this automatically)\nkubectl get certificate -n [namespace] [cert-name]\n# Status should be: True\n\n# If manual renewal needed:\ncertbot renew --cert-name [domain]\n\n# Verify new cert loaded\nkubectl rollout restart deployment/[service-name] -n [namespace]\n```\n\n## Log Cleanup\n\n**Frequency:** Weekly\n**Retention:** 30 days\n\n```bash\n# Check log volume\naws logs describe-log-groups --log-group-name-prefix /[service-name]\n\n# Logs auto-expire after 30 days (configured in Terraform)\n# No manual action needed unless troubleshooting\n```\n\n## Dependency Updates\n\n**Frequency:** Monthly (security patches), quarterly (minor versions)\n\n```bash\n# Check for updates\nnpm outdated  # Node.js\ncargo update --dry-run  # Rust\npip list --outdated  # Python\n\n# Update dependencies\nnpm update\ncargo update\npip install --upgrade -r requirements.txt\n\n# Run tests\nnpm test\n\n# Deploy to staging\n# Test in staging for 1 week\n# Deploy to production\n```\n\n**Link:** Dependency security scanning: [GitHub Dependabot]\n\n## Scaling\n\n**When to Scale Up:**\n- CPU usage > 70% sustained for 1 hour\n- Memory usage > 80%\n- Latency p99 > 500ms due to load\n- Error rate increased due to overload\n\n**When to Scale Down:**\n- CPU usage < 30% for 1 day\n- Memory usage < 40%\n- Cost optimization\n\n```bash\n# Scale up\nkubectl scale deployment/[service-name] --replicas=10 -n [namespace]\n\n# Monitor for 15 minutes\n# If metrics improve, keep new scale\n\n# Scale down\nkubectl scale deployment/[service-name] --replicas=3 -n [namespace]\n\n# Monitor for 30 minutes\n# If metrics stay good, keep new scale\n\n# Update Terraform to match\ncd infrastructure\n# Edit: desired_count = 10\nterraform plan\nterraform apply\n```\n\n## Configuration Changes\n\n**Types:**\n- Environment variables\n- Feature flags\n- Infrastructure settings\n\n```bash\n# Update environment variable\nkubectl set env deployment/[service-name] \\\n  NEW_SETTING=value -n [namespace]\n\n# Verify change\nkubectl get deployment/[service-name] -n [namespace] -o yaml | \\\n  grep -A5 env:\n\n# Monitor for issues\n# If problems, revert:\nkubectl set env deployment/[service-name] \\\n  NEW_SETTING- -n [namespace]  # Note the minus sign\n```\n\n**Link:** Configuration management: [repository]/config/\n\n## Links\n\n**Code:** [repository]\n**Infrastructure:** [repository]/infrastructure/\n**Monitoring:** [Grafana URL]\n**Backups:** AWS RDS Console → Snapshots" \
  --source "runbooks" \
  --tags "runbooks,maintenance,[service-name]"
```

### Step 5: Link Runbooks to Tasks

```bash
# Link runbooks to service task
engram relationship create \
  --source-id [SERVICE_TASK_ID] --source-type task \
  --target-id [DEPLOYMENT_RUNBOOK_ID] --target-type context \
  --relationship-type references --agent default

engram relationship create \
  --source-id [SERVICE_TASK_ID] --source-type task \
  --target-id [INCIDENT_RUNBOOK_ID] --target-type context \
  --relationship-type references --agent default

engram relationship create \
  --source-id [SERVICE_TASK_ID] --source-type task \
  --target-id [MAINTENANCE_RUNBOOK_ID] --target-type context \
  --relationship-type references --agent default
```

## Example

Create runbooks for a payment processing service.

### Deployment Runbook

```bash
DEPLOY=$(engram context create \
  --title "Runbook: Deploy Payment Service" \
  --content "# Deployment: Payment Service\n\n## Prerequisites\n- [ ] All tests passing\n- [ ] PCI compliance check passed\n- [ ] Staging tested for 48 hours\n\n## Steps\n1. Create maintenance window in PagerDuty\n2. Deploy to production during low-traffic period\n3. Monitor error rate and payment success rate\n4. Run smoke tests on real payment flow\n5. Monitor for 30 minutes (critical service)\n\n## Rollback\nIf payment error rate > 0.1%, rollback immediately\n\n**Links:**\n- Code: github.com/company/payment-service\n- Dashboard: grafana.company.com/payment-health\n- Alerts: pagerduty.com/services/payment" \
  --source "runbooks" \
  --tags "runbooks,deployment,payment-service" \
  --json | jq -r '.id')
```

### Incident Response Runbook

```bash
INCIDENT=$(engram context create \
  --title "Runbook: Incident Response Payment Service" \
  --content "# Incident Response: Payment Service\n\n## P0: Payments Not Processing\n\n**Immediate Actions:**\n1. Check Stripe API status: status.stripe.com\n2. Check database connection\n3. Check recent deployments\n4. Enable fallback payment processor if needed\n\n**Mitigation:**\n- Queue payments for retry if Stripe down\n- Switch to backup payment processor\n- Rollback if recent deploy\n\n**Communication:**\n- Post in #incidents immediately\n- Email finance team if > 5 min outage\n- Update status page\n\n**Links:**\n- Stripe status: status.stripe.com\n- Payment queue dashboard: [URL]\n- Backup processor runbook: [Engram ID]" \
  --source "runbooks" \
  --tags "runbooks,incident-response,payment-service" \
  --json | jq -r '.id')
```

### Link Everything

```bash
engram relationship create \
  --source-id payment-service-123 --source-type task \
  --target-id $DEPLOY --target-type context \
  --relationship-type references --agent default

engram relationship create \
  --source-id payment-service-123 --source-type task \
  --target-id $INCIDENT --target-type context \
  --relationship-type references --agent default
```

## Querying Runbooks

```bash
# Get all runbooks for a service
engram relationship connected --entity-id [SERVICE_TASK_ID] --relationship-type references | grep "Runbook"

# Find all deployment runbooks
engram context list | grep "Runbook: Deploy"

# Find all incident runbooks
engram context list | grep "Runbook: Incident"

# Search for specific procedure
engram context list | grep -i "database restore"
```

## Related Skills

This skill integrates with:
- `engram-observability-design` - Runbooks link to dashboards and alerts
- `engram-system-design` - Runbooks reference architecture diagrams
- `engram-risk-assessment` - Runbooks mitigate identified risks
- `engram-post-mortem` - Incidents lead to runbook improvements
