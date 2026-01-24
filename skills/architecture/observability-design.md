---
name: engram-observability-design
description: "Design comprehensive observability systems with logging, metrics, tracing, alerting, and SLI/SLO/SLA definitions for services."
---

# Observability Design (Engram-Integrated)

## Overview

Design observability systems that provide visibility into system behavior through structured logging, metrics collection, distributed tracing, and intelligent alerting. Define Service Level Indicators (SLIs), Service Level Objectives (SLOs), and Service Level Agreements (SLAs) to measure and maintain service reliability. Store observability strategy and decisions in Engram for long-term reference.

## When to Use

Use this skill when:
- Designing observability for a new service or system
- Debugging production issues requires better visibility
- Need to establish reliability targets and measure them
- Migrating from reactive (waiting for user reports) to proactive monitoring
- Implementing SRE practices and error budgets
- Coordinating observability across multiple services
- Planning incident response and on-call runbooks

## The Pattern

### Step 1: Define Observability Requirements

Identify what needs to be observable:

```bash
engram context create \
  --title "Observability Requirements: [Service Name]" \
  --content "## Service Overview\n\n**Purpose:** [What this service does]\n**Criticality:** [Critical/High/Medium/Low - affects user experience directly?]\n**Dependencies:** [Upstream and downstream services]\n**Users:** [Who uses this service - external users, internal services, admins]\n\n## What Needs Observability\n\n**User Journey:**\n1. [Step 1] - [What user does, what service does]\n2. [Step 2] - [Next action, response]\n3. [Step 3] - [Final outcome]\n\n**Critical Paths:**\n- [Path 1]: e.g., User login → Token generation → Database query → Response\n- [Path 2]: e.g., Payment processing → Validation → External API → Database write\n- [Path 3]: e.g., Background job → Queue fetch → Processing → Completion\n\n**Failure Modes:**\n- [Failure 1]: e.g., Database timeout (affects: all requests)\n- [Failure 2]: e.g., Rate limit exceeded (affects: high-volume users)\n- [Failure 3]: e.g., Memory leak (affects: service stability over time)\n\n**Performance Requirements:**\n- Latency targets: [e.g., p50 < 100ms, p95 < 300ms, p99 < 1s]\n- Throughput targets: [e.g., 1000 requests/sec]\n- Error rate targets: [e.g., < 0.1% 5xx errors]\n\n**Compliance:**\n- Data retention: [e.g., logs 30 days, metrics 1 year]\n- PII handling: [e.g., no user data in logs, sanitize traces]\n- Audit requirements: [e.g., log all admin actions]" \
  --source "observability-design" \
  --tags "observability,requirements,[service-name]"
```

### Step 2: Design Logging Strategy

Define structured logging approach:

```bash
engram reasoning create \
  --title "Logging Strategy: [Service Name]" \
  --task-id [TASK_ID] \
  --content "## Logging Levels\n\n**ERROR:** Failures requiring immediate attention\n- Failed database connections\n- Unhandled exceptions\n- External API failures (after retries)\n- Data corruption detected\n- Rate: Target < 1 per minute (should be rare)\n\n**WARN:** Degraded behavior, potential issues\n- Slow queries (> 1s)\n- High retry counts\n- Deprecated API usage\n- Cache misses (abnormally high)\n- Rate: Acceptable 1-10 per minute\n\n**INFO:** Significant business events\n- User registration\n- Payment processed\n- Order placed\n- Job completed\n- Rate: Proportional to traffic\n\n**DEBUG:** Detailed diagnostic information\n- Function entry/exit\n- Variable values\n- Query plans\n- Rate: High (disabled in production unless needed)\n\n## Structured Logging Format\n\n**Standard Fields:**\n```json\n{\n  \"timestamp\": \"2026-01-24T10:15:30.123Z\",\n  \"level\": \"INFO\",\n  \"service\": \"[service-name]\",\n  \"version\": \"1.2.3\",\n  \"environment\": \"production\",\n  \"trace_id\": \"abc123...\",\n  \"span_id\": \"def456...\",\n  \"user_id\": \"user_789\" (sanitized if needed),\n  \"request_id\": \"req_xyz\",\n  \"message\": \"Order placed successfully\",\n  \"context\": {\n    \"order_id\": \"order_123\",\n    \"amount\": 99.99,\n    \"currency\": \"USD\"\n  },\n  \"duration_ms\": 234\n}\n```\n\n**For Errors:**\n```json\n{\n  \"level\": \"ERROR\",\n  \"message\": \"Database query failed\",\n  \"error\": {\n    \"type\": \"DatabaseConnectionError\",\n    \"message\": \"Connection timeout after 5s\",\n    \"stack\": \"[stack trace]\",\n    \"query\": \"[sanitized query]\"\n  },\n  \"context\": {\n    \"retry_count\": 3,\n    \"database_host\": \"db-primary-1\"\n  }\n}\n```\n\n## What to Log\n\n**Always Log:**\n- Request start/completion (with duration)\n- Authentication/authorization decisions\n- External API calls (with latency)\n- Database operations (with duration)\n- Errors and exceptions\n- State transitions (order created → processing → completed)\n\n**Never Log:**\n- Passwords, tokens, API keys\n- Credit card numbers, SSN\n- Personal health information\n- Raw user input (may contain sensitive data)\n\n**Sanitize Before Logging:**\n- Email addresses: [Redact if not needed]\n- IP addresses: [May be PII in some jurisdictions]\n- User agent strings: [May contain device identifiers]\n- Request bodies: [Filter sensitive fields]\n\n## Log Aggregation\n\n**Technology:** [e.g., ELK Stack, Loki, CloudWatch Logs]\n\n**Indexing Strategy:**\n- Index by: timestamp, service, level, trace_id, user_id\n- TTL: 30 days for all logs, 90 days for ERROR level\n- Sampling: None for ERROR/WARN, 10% for DEBUG in high traffic\n\n**Search Patterns:**\n- Find all errors for user: `user_id:\"user_789\" AND level:ERROR`\n- Find slow requests: `duration_ms:>1000`\n- Find trace: `trace_id:\"abc123\"`\n- Find pattern: `message:\"database\" AND level:ERROR`\n\n**Retention:**\n- Hot storage (searchable): 30 days\n- Cold storage (archive): 1 year\n- Delete after: 1 year (or per compliance requirements)\n\n## Confidence\n\n**Confidence:** [0.0-1.0]" \
  --confidence [0.0-1.0] \
  --tags "observability,logging,[service-name]"
```

### Step 3: Design Metrics Strategy

Define key metrics to collect:

```bash
engram reasoning create \
  --title "Metrics Strategy: [Service Name]" \
  --task-id [TASK_ID] \
  --content "## Metrics Categories\n\n### RED Metrics (Request-based services)\n\n**Rate:** Requests per second\n- Metric: `http_requests_total` (counter)\n- Labels: {service, endpoint, method, status_code}\n- Use: Track traffic patterns, detect spikes/drops\n\n**Errors:** Error rate\n- Metric: `http_errors_total` (counter)\n- Labels: {service, endpoint, error_type}\n- Calculate: error_rate = errors / total_requests\n- Use: Detect failures, SLI for reliability\n\n**Duration:** Response time percentiles\n- Metric: `http_request_duration_seconds` (histogram)\n- Labels: {service, endpoint}\n- Buckets: [0.01, 0.05, 0.1, 0.3, 0.5, 1.0, 3.0, 5.0]\n- Use: Track latency, SLI for performance\n\n### USE Metrics (Resource-based services)\n\n**Utilization:** Resource usage percentage\n- CPU: `process_cpu_usage_percent` (gauge)\n- Memory: `process_memory_usage_bytes` (gauge)\n- Disk: `disk_usage_percent` (gauge)\n- Use: Capacity planning, detect resource exhaustion\n\n**Saturation:** Queue depth, backlog\n- Metric: `queue_depth` (gauge)\n- Labels: {queue_name}\n- Use: Detect bottlenecks, scaling triggers\n\n**Errors:** Resource errors\n- File descriptor exhaustion\n- OOM kills\n- Disk full\n\n### Business Metrics\n\n**Key Business Events:**\n- `orders_placed_total` (counter)\n- `users_registered_total` (counter)\n- `payments_processed_total` (counter)\n- `revenue_total` (counter) {currency}\n\n**Operational Metrics:**\n- `database_connections_active` (gauge)\n- `cache_hit_rate` (gauge)\n- `external_api_calls_total` (counter) {api, status}\n- `job_processing_duration_seconds` (histogram) {job_type}\n\n## Metric Design Principles\n\n**Cardinality Control:**\n- ✓ Low cardinality labels: service, endpoint, status_code\n- ✗ High cardinality labels: user_id, request_id, trace_id\n- Max unique label combinations: 10K per metric\n\n**Naming Convention:**\n- Format: `[namespace]_[subsystem]_[name]_[unit]`\n- Example: `http_request_duration_seconds`\n- Counters suffix: `_total`\n- Gauges: no suffix\n- Histograms: `_bucket`, `_sum`, `_count`\n\n**Aggregation:**\n- Counters: rate(), increase()\n- Gauges: avg(), min(), max()\n- Histograms: histogram_quantile()\n\n## Metrics Collection\n\n**Technology:** [e.g., Prometheus, Datadog, CloudWatch]\n\n**Scrape Interval:** 15s (standard)\n**Retention:** 15 days (high resolution), 1 year (downsampled)\n\n**Exposition:**\n- Endpoint: `/metrics` (Prometheus format)\n- Push gateway: For short-lived jobs\n- StatsD: For language-agnostic metrics\n\n## Key Queries\n\n**Error Rate (last 5 min):**\n```promql\nrate(http_errors_total[5m]) / rate(http_requests_total[5m])\n```\n\n**p99 Latency:**\n```promql\nhistogram_quantile(0.99, \n  rate(http_request_duration_seconds_bucket[5m])\n)\n```\n\n**Traffic Rate:**\n```promql\nsum(rate(http_requests_total[5m])) by (endpoint)\n```\n\n**Confidence:** [0.0-1.0]" \
  --confidence [0.0-1.0] \
  --tags "observability,metrics,[service-name]"
```

### Step 4: Design Distributed Tracing

Define tracing strategy for request flows:

```bash
engram reasoning create \
  --title "Tracing Strategy: [Service Name]" \
  --task-id [TASK_ID] \
  --content "## Distributed Tracing Overview\n\n**Purpose:** Track requests across service boundaries to understand end-to-end latency and identify bottlenecks.\n\n**Technology:** [e.g., OpenTelemetry, Jaeger, Zipkin, AWS X-Ray]\n\n## Trace Structure\n\n**Trace:** Complete request journey\n- ID: Unique identifier for entire request\n- Duration: End-to-end time\n- Services: All services involved\n- Status: Success/Error\n\n**Span:** Single operation within trace\n- ID: Unique identifier for this operation\n- Parent ID: Link to parent span\n- Name: Operation name (e.g., \"GET /users/{id}\")\n- Duration: Operation time\n- Tags: Metadata (service, version, error)\n- Logs: Events within span\n\n## What to Trace\n\n**Critical Paths:**\n1. HTTP requests (API endpoints)\n2. RPC calls (gRPC, internal services)\n3. Database queries\n4. External API calls\n5. Queue operations (publish/consume)\n6. Cache operations (if significant latency)\n\n**Span Creation:**\n```\nClient Request\n└── API Gateway (span: api-gateway.handle)\n    ├── Auth Service (span: auth.verify-token)\n    │   └── Database (span: db.query \"SELECT user...\")\n    ├── User Service (span: users.get-profile)\n    │   ├── Cache (span: cache.get \"user:123\")\n    │   └── Database (span: db.query \"SELECT profile...\")\n    └── Response Formatting (span: api-gateway.format-response)\n```\n\n## Span Attributes\n\n**Standard Attributes:**\n- `service.name`: [service-name]\n- `service.version`: 1.2.3\n- `deployment.environment`: production\n- `http.method`: GET\n- `http.url`: /api/users/123\n- `http.status_code`: 200\n- `db.system`: postgresql\n- `db.statement`: SELECT * FROM users WHERE id = ?\n- `error`: true/false\n- `error.type`: DatabaseConnectionError\n- `error.message`: Connection timeout\n\n**Custom Attributes:**\n- `user.id`: user_789 (for debugging)\n- `order.id`: order_123\n- `cache.hit`: true/false\n- `retry.count`: 3\n\n## Sampling Strategy\n\n**Problem:** Tracing all requests is expensive at scale.\n\n**Solution: Adaptive Sampling**\n\n**Production (High Traffic):**\n- Baseline: 1% of all requests\n- Always trace: Errors (100%)\n- Always trace: Slow requests (p99 threshold)\n- Always trace: Specific users (for debugging)\n- Result: ~5% of requests traced\n\n**Staging:**\n- 100% tracing (low traffic)\n\n**Implementation:**\n```python\ndef should_sample(request):\n    # Always trace errors\n    if request.has_error():\n        return True\n    \n    # Always trace slow requests\n    if request.duration > p99_threshold:\n        return True\n    \n    # Always trace flagged users\n    if request.user_id in debug_users:\n        return True\n    \n    # Sample baseline\n    return random.random() < 0.01  # 1%\n```\n\n## Trace Analysis\n\n**Find Bottlenecks:**\n- Sort spans by duration\n- Identify longest operations\n- Check for sequential operations that could be parallel\n\n**Find Errors:**\n- Filter traces with error=true\n- Check error spans for error.type and error.message\n- Identify error propagation path\n\n**Find Latency Issues:**\n- Compare p50, p95, p99 latency by span\n- Identify spans with high variance\n- Check for cascading timeouts\n\n## Integration with Logs and Metrics\n\n**Correlation:**\n- All logs include `trace_id` and `span_id`\n- Metrics labeled with `trace_id` for exemplars\n- Jump from metric spike → trace → logs\n\n**Workflow:**\n1. Alert fires: High error rate\n2. Check metrics: Error spike at 10:15 AM\n3. Find traces: Filter by time range and error=true\n4. Examine trace: See Auth Service span failed\n5. Find logs: Search `trace_id:\"abc123\"` for detailed error\n\n**Confidence:** [0.0-1.0]" \
  --confidence [0.0-1.0] \
  --tags "observability,tracing,[service-name]"
```

### Step 5: Define SLIs, SLOs, SLAs

Establish reliability targets:

```bash
engram reasoning create \
  --title "SLI/SLO/SLA: [Service Name]" \
  --task-id [TASK_ID] \
  --content "## Service Level Indicators (SLIs)\n\n**Definition:** Measurable metrics that indicate service health.\n\n### Availability SLI\n\n**What:** Percentage of successful requests\n**Formula:** `successful_requests / total_requests`\n**Success Criteria:** HTTP status < 500, no timeout\n**Measurement:** Every request counted\n**Current:** [e.g., 99.95%]\n\n### Latency SLI\n\n**What:** Percentage of requests below latency threshold\n**Formula:** `requests_below_threshold / total_requests`\n**Threshold:** p95 < 300ms, p99 < 1s\n**Measurement:** Response time histogram\n**Current:** [e.g., 99.9% below 300ms]\n\n### Durability SLI (Data services)\n\n**What:** Percentage of data not lost\n**Formula:** `data_accessible / data_written`\n**Measurement:** Periodic validation checks\n**Current:** [e.g., 99.999%]\n\n### Correctness SLI\n\n**What:** Percentage of correct responses\n**Formula:** `correct_responses / total_responses`\n**Measurement:** End-to-end tests, canary checks\n**Current:** [e.g., 99.99%]\n\n## Service Level Objectives (SLOs)\n\n**Definition:** Target values for SLIs over time window.\n\n### Availability SLO\n\n**Target:** 99.9% over 30-day window\n**Error Budget:** 0.1% = 43 minutes downtime per month\n**Measurement Window:** Rolling 30 days\n**Burn Rate Alert:** If error budget consumed in < 3 days\n\n**Calculation:**\n```promql\n# Error budget remaining\n1 - (sum(rate(http_errors_total[30d])) / sum(rate(http_requests_total[30d])))\n```\n\n### Latency SLO\n\n**Target:** 99% of requests < 300ms over 7-day window\n**Error Budget:** 1% of requests can exceed threshold\n**Measurement Window:** Rolling 7 days\n\n**Calculation:**\n```promql\nhistogram_quantile(0.99, \n  rate(http_request_duration_seconds_bucket[7d])\n) < 0.3\n```\n\n### Composite SLO\n\n**Target:** Good requests / Total requests\n**Good Request:** Available AND Fast AND Correct\n- Available: status < 500\n- Fast: latency < 300ms\n- Correct: passes validation\n\n**Formula:**\n```\nGood Requests = requests where (\n  status_code < 500 AND\n  duration < 300ms AND\n  validation_passed == true\n)\n\nSLO = Good Requests / Total Requests >= 99%\n```\n\n## Service Level Agreements (SLAs)\n\n**Definition:** Contractual commitment to customers with consequences for breach.\n\n### SLA Terms (Example)\n\n**Availability SLA:** 99.5% uptime per month\n- Below 99.5%: 10% service credit\n- Below 99.0%: 25% service credit\n- Below 95.0%: 100% service credit\n\n**Latency SLA:** p95 < 500ms\n- Above 500ms: Documented incident required\n- Above 1s: Service credit applies\n\n**Support SLA:**\n- Critical (P0): 15 minute response time\n- High (P1): 1 hour response time\n- Medium (P2): 4 hour response time\n- Low (P3): 1 business day response time\n\n### SLA vs SLO Relationship\n\n**SLA:** 99.5% (customer promise)\n**SLO:** 99.9% (internal target)\n**Gap:** 0.4% buffer for margin\n\n**Reason:** SLO is stricter than SLA to ensure SLA is always met.\n\n## Error Budget Policy\n\n**When Error Budget > 0:**\n- Deploy frequently (automated CD)\n- Launch new features\n- Experiment with risky changes\n- Focus on feature velocity\n\n**When Error Budget < 25%:**\n- Slow down deployments\n- Increase testing\n- Code freeze for non-critical features\n- Review recent changes\n\n**When Error Budget Exhausted:**\n- STOP all feature work\n- Focus on reliability improvements\n- Root cause analysis for incidents\n- Improve monitoring/alerting\n- Resume only when budget replenishes\n\n## Monitoring SLOs\n\n**Dashboard:**\n- Current SLI value\n- SLO target line\n- Error budget remaining (absolute and percentage)\n- Error budget burn rate (current rate vs acceptable)\n- Time until budget exhausted (at current rate)\n\n**Alerts:**\n- SLO at risk: 50% of error budget consumed in 10% of window\n- SLO breached: SLI below target\n- Error budget exhausted: No budget remaining\n\n**Confidence:** [0.0-1.0]" \
  --confidence [0.0-1.0] \
  --tags "observability,slo,sla,[service-name]"
```

### Step 6: Design Alerting Strategy

Define intelligent alerting:

```bash
engram reasoning create \
  --title "Alerting Strategy: [Service Name]" \
  --task-id [TASK_ID] \
  --content "## Alerting Principles\n\n**Alert on Symptoms, Not Causes:**\n- ✓ Alert: High error rate affecting users\n- ✗ Alert: CPU at 80% (may not affect users)\n\n**Alert on SLOs:**\n- Primary alerts based on SLO burn rate\n- Catch issues before SLA breach\n\n**Actionable Alerts Only:**\n- Every alert must require human action\n- If no action needed, don't alert (log instead)\n- Alerts must have runbook link\n\n**Reduce Alert Fatigue:**\n- Group related alerts\n- Use smart escalation (wait before paging)\n- Auto-resolve when condition clears\n\n## Alert Severity Levels\n\n### P0 - Critical (Page Immediately)\n\n**Criteria:**\n- Service completely down (availability < 95%)\n- Data loss occurring\n- Security breach detected\n- SLA breach imminent (< 15 minutes)\n\n**Response Time:** 15 minutes\n**Escalation:** Page on-call immediately, escalate to senior after 15 min\n**Examples:**\n- All instances down\n- Database unreachable\n- Payment processing failed for all users\n\n### P1 - High (Page During Business Hours)\n\n**Criteria:**\n- Significant degradation (availability < 99%)\n- High error rate (> 5%)\n- Latency spike (p99 > 5s)\n- SLO burn rate high (budget exhausted in < 2 days)\n\n**Response Time:** 1 hour\n**Escalation:** Slack alert, then page if unacknowledged after 30 min\n**Examples:**\n- Single instance down (redundancy remains)\n- External API slow\n- High memory usage trending toward OOM\n\n### P2 - Medium (Notify, Don't Page)\n\n**Criteria:**\n- Minor degradation (availability 99-99.5%)\n- Moderate error rate (1-5%)\n- SLO burn rate moderate (budget exhausted in 2-7 days)\n\n**Response Time:** 4 hours during business hours\n**Notification:** Slack channel\n**Examples:**\n- Elevated error rate (but below critical)\n- Disk usage 80%\n- Cache hit rate dropped\n\n### P3 - Low (Log Only)\n\n**Criteria:**\n- Early warning signals\n- Informational\n- No immediate user impact\n\n**Response Time:** Best effort\n**Notification:** Dashboard only\n**Examples:**\n- SSL certificate expires in 30 days\n- Disk usage 60%\n- Unusual traffic pattern detected\n\n## Multi-Window Alerting\n\n**Problem:** Balance between fast detection and false positives.\n\n**Solution: Multiple burn rate windows**\n\n### Fast Burn (Page Immediately)\n\n**Window:** 1 hour\n**Threshold:** Error budget consumed at 14.4x rate\n**Meaning:** At this rate, budget exhausted in 2 days (for 30-day window)\n**Action:** Page on-call\n\n**Alert:**\n```yaml\nalert: FastBurnSLO\nexpr: |\n  (\n    sum(rate(http_errors_total[1h])) / \n    sum(rate(http_requests_total[1h]))\n  ) > (14.4 * (1 - 0.999))  # 14.4x error budget\nfor: 5m\nlabels:\n  severity: P0\nannotations:\n  summary: \"Fast SLO burn detected\"\n  description: \"Error budget will be exhausted in 2 days at current rate\"\n  runbook: https://runbooks.example.com/fast-burn\n```\n\n### Slow Burn (Alert, Don't Page)\n\n**Window:** 6 hours\n**Threshold:** Error budget consumed at 6x rate\n**Meaning:** At this rate, budget exhausted in 5 days\n**Action:** Slack notification\n\n**Alert:**\n```yaml\nalert: SlowBurnSLO\nexpr: |\n  (\n    sum(rate(http_errors_total[6h])) / \n    sum(rate(http_requests_total[6h]))\n  ) > (6 * (1 - 0.999))\nfor: 30m\nlabels:\n  severity: P2\nannotations:\n  summary: \"Slow SLO burn detected\"\n  description: \"Error budget will be exhausted in 5 days at current rate\"\n```\n\n## Alert Definitions\n\n### High Error Rate\n\n```yaml\nalert: HighErrorRate\nexpr: |\n  sum(rate(http_errors_total[5m])) / \n  sum(rate(http_requests_total[5m])) > 0.05\nfor: 2m\nlabels:\n  severity: P1\nannotations:\n  summary: \"Error rate above 5%\"\n  description: \"{{ $value | humanizePercentage }} of requests failing\"\n  dashboard: https://grafana.example.com/d/service-health\n  runbook: https://runbooks.example.com/high-error-rate\n```\n\n### High Latency\n\n```yaml\nalert: HighLatency\nexpr: |\n  histogram_quantile(0.99, \n    rate(http_request_duration_seconds_bucket[5m])\n  ) > 1.0\nfor: 5m\nlabels:\n  severity: P1\nannotations:\n  summary: \"p99 latency above 1 second\"\n  description: \"99th percentile: {{ $value }}s\"\n  runbook: https://runbooks.example.com/high-latency\n```\n\n### Service Down\n\n```yaml\nalert: ServiceDown\nexpr: up{service=\"[service-name]\"} == 0\nfor: 1m\nlabels:\n  severity: P0\nannotations:\n  summary: \"Service {{ $labels.instance }} is down\"\n  description: \"Instance has been down for 1 minute\"\n  runbook: https://runbooks.example.com/service-down\n```\n\n## Alert Runbooks\n\nEvery alert must link to runbook with:\n\n1. **Symptoms:** What the alert means\n2. **Impact:** How users are affected\n3. **Diagnosis:** How to investigate\n   - Relevant dashboards\n   - Log queries\n   - Trace examples\n4. **Mitigation:** How to fix (short-term)\n5. **Resolution:** How to fix (long-term)\n6. **Escalation:** When to escalate, who to contact\n\n## Alert Routing\n\n**Channels:**\n- P0: PagerDuty → Phone call + SMS\n- P1: PagerDuty → Push notification\n- P2: Slack #alerts-service-name\n- P3: Dashboard only\n\n**On-Call Schedule:**\n- Primary: 7-day rotation\n- Secondary: Backup escalation after 15 min\n- Manager: Final escalation for extended incidents\n\n**Quiet Hours:**\n- P2 alerts suppressed outside business hours\n- P1 alerts delayed 15 minutes outside business hours\n- P0 alerts always immediate\n\n**Confidence:** [0.0-1.0]" \
  --confidence [0.0-1.0] \
  --tags "observability,alerting,[service-name]"
```

### Step 7: Link All Observability Components

```bash
# Link all observability docs to task
for DOC_ID in $REQUIREMENTS $LOGGING $METRICS $TRACING $SLO $ALERTING; do
  engram relationship create \
    --source-id [TASK_ID] --source-type task \
    --target-id $DOC_ID --target-type reasoning \
    --relationship-type documents --agent default
done
```

## Example

User needs observability for a new payment processing service.

### Step 1: Requirements

```bash
REQ=$(engram context create \
  --title "Observability Requirements: Payment Service" \
  --content "## Service Overview\n\nPurpose: Process payments via Stripe API\nCriticality: Critical - affects revenue directly\nDependencies: Stripe API (external), Order Service (internal), Database\nUsers: E-commerce checkout flow\n\n## Critical Paths\n\n1. Payment intent creation → Stripe API → Response → Save to DB\n2. Payment confirmation webhook → Validate → Update order → Notify user\n\n## Failure Modes\n\n- Stripe API timeout (affects: payment flow blocked)\n- Database write failure (affects: order not confirmed)\n- Webhook signature invalid (affects: fraud risk)\n\n## Performance Requirements\n\n- Latency: p99 < 500ms (payment UX critical)\n- Throughput: 100 payments/minute peak\n- Error rate: < 0.01% (money is involved)" \
  --source "observability-design" \
  --tags "observability,requirements,payment-service" \
  --json | jq -r '.id')
```

### Step 2: Logging Strategy

```bash
LOG=$(engram reasoning create \
  --title "Logging Strategy: Payment Service" \
  --task-id payment-123 \
  --content "## Key Logs\n\nERROR: Stripe API failures, database write failures\nWARN: High retry counts, slow Stripe responses\nINFO: Payment created, payment confirmed, refund issued\n\n## Structured Format\n\n{\"timestamp\": \"...\", \"level\": \"INFO\", \"message\": \"Payment confirmed\", \"payment_id\": \"pay_123\", \"amount\": 99.99, \"stripe_charge_id\": \"ch_xyz\", \"duration_ms\": 234}\n\n## Never Log\n\n- Credit card numbers\n- CVV codes\n- Stripe API keys\n\n## Sanitize\n\n- Card last 4 digits only\n- Stripe charge IDs (ok to log)" \
  --confidence 0.90 \
  --tags "observability,logging,payment-service" \
  --json | jq -r '.id')
```

### Step 3: Metrics Strategy

```bash
MET=$(engram reasoning create \
  --title "Metrics Strategy: Payment Service" \
  --task-id payment-123 \
  --content "## Key Metrics\n\n- payment_requests_total {status}\n- payment_errors_total {error_type}\n- payment_duration_seconds (histogram)\n- stripe_api_calls_total {status}\n- stripe_api_duration_seconds (histogram)\n- payments_processed_total\n- revenue_total {currency}\n\n## SLI Metrics\n\n- Availability: payment_requests_total{status!=\"5xx\"} / payment_requests_total\n- Latency: histogram_quantile(0.99, payment_duration_seconds) < 0.5\n\n## Queries\n\nError rate: rate(payment_errors_total[5m]) / rate(payment_requests_total[5m])" \
  --confidence 0.90 \
  --tags "observability,metrics,payment-service" \
  --json | jq -r '.id')
```

### Step 4: SLO Definition

```bash
SLO=$(engram reasoning create \
  --title "SLI/SLO/SLA: Payment Service" \
  --task-id payment-123 \
  --content "## SLIs\n\nAvailability: 99.99% (payments succeed)\nLatency: 99% < 500ms\nDurability: 100% (no payment data loss)\n\n## SLOs\n\nAvailability: 99.95% over 30 days\n- Error budget: 0.05% = 21 minutes downtime/month\n\nLatency: 99% < 500ms over 7 days\n\n## SLAs\n\nAvailability: 99.5% per month\n- Below 99.5%: 10% credit\n- Below 99.0%: 25% credit\n\n## Error Budget Policy\n\nBudget exhausted → Stop deployments, fix reliability issues" \
  --confidence 0.90 \
  --tags "observability,slo,payment-service" \
  --json | jq -r '.id')
```

### Step 5: Alerting Strategy

```bash
ALERT=$(engram reasoning create \
  --title "Alerting Strategy: Payment Service" \
  --task-id payment-123 \
  --content "## Alerts\n\nP0 (Page immediately):\n- Service down (all instances unreachable)\n- Error rate > 10% (revenue impact)\n- Stripe API unreachable (payments blocked)\n\nP1 (Page business hours):\n- Error rate > 1%\n- p99 latency > 1s\n- Fast SLO burn (budget exhausted in < 2 days)\n\nP2 (Notify Slack):\n- Error rate > 0.1%\n- Slow SLO burn (budget exhausted in < 7 days)\n\n## Runbooks\n\nEach alert links to:\n- https://runbooks.example.com/payment-service-down\n- https://runbooks.example.com/payment-high-error-rate\n- https://runbooks.example.com/stripe-api-timeout" \
  --confidence 0.90 \
  --tags "observability,alerting,payment-service" \
  --json | jq -r '.id')
```

### Step 6: Link Everything

```bash
for ID in $REQ $LOG $MET $SLO $ALERT; do
  engram relationship create \
    --source-id payment-123 --source-type task \
    --target-id $ID \
    --relationship-type documents --agent default
done
```

## Querying Observability Design

```bash
# Get all observability documents for a service
engram relationship connected --entity-id [TASK_ID] | grep -E "Observability|Logging|Metrics|Tracing|SLO|Alerting"

# Get all SLO definitions
engram reasoning list | grep "SLI/SLO/SLA"

# Get alerting strategies across services
engram reasoning list | grep "Alerting Strategy"

# Search for specific observability topic
engram context list | grep -i "logging"
```

## Related Skills

This skill integrates with:
- `engram-system-design` - Design observability into system architecture
- `engram-runbooks` - Create operational runbooks for alerts and incidents
- `engram-incident-response` - Use observability data during incident response
- `engram-performance-optimization` - Use metrics and traces to find bottlenecks
- `engram-risk-assessment` - Assess risks of insufficient observability
- `engram-spike-investigation` - Test observability setup with spike solutions
