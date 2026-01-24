---
name: engram-scalability-analysis
description: "Analyze system bottlenecks, plan for horizontal and vertical scaling, model load patterns, and forecast infrastructure needs."
---

# Scalability Analysis (Engram-Integrated)

## Overview

Systematically analyze system performance characteristics, identify bottlenecks before they cause outages, design horizontal and vertical scaling strategies, model load patterns based on usage data, and forecast infrastructure needs. Store scalability analyses, load models, and scaling plans in Engram for capacity planning.

## When to Use

Use this skill when:
- System approaching capacity limits (CPU, memory, database connections)
- Planning for growth (10x users, new market launch)
- Performance degradation under load (slow response times, timeouts)
- Stakeholders ask "can we handle Black Friday traffic?"
- Designing new system that must scale from day one
- Post-incident analysis reveals scalability issues

## The Pattern

### Step 1: Analyze Current Performance and Identify Bottlenecks

```bash
engram context create \
  --title "Scalability Analysis: [System Name]" \
  --content "## System Overview\n\n**System:** [e.g., API Server]\n**Current Scale:** [e.g., 10K requests/min, 5K active users, 500GB data]\n**Infrastructure:** [e.g., 4 EC2 instances (m5.large), 1 PostgreSQL (db.m5.xlarge)]\n**Analysis Date:** [Date]\n\n## Current Performance Metrics\n\n### API Server\n- **Throughput:** 10K requests/min (avg), 15K peak\n- **Latency:** p50: 120ms, p95: 450ms, p99: 1.2s\n- **Error rate:** 0.3% (normal), 2% during peak\n- **CPU:** 60% avg, 85% peak\n- **Memory:** 70% avg, 90% peak\n- **Concurrent connections:** 500 avg, 800 peak\n\n### Database\n- **Query rate:** 5K queries/min (avg), 8K peak\n- **Query latency:** p50: 15ms, p95: 120ms, p99: 500ms\n- **Connections:** 200 of 500 max (40% utilization)\n- **CPU:** 40% avg, 70% peak\n- **IOPS:** 2K of 10K provisioned (20% utilization)\n- **Slow queries:** 15% of queries >100ms\n\n### Cache (Redis)\n- **Hit rate:** 85%\n- **Throughput:** 50K ops/sec (avg)\n- **Latency:** p99: <5ms\n- **Memory:** 8GB of 16GB (50% utilization)\n- **Evictions:** 100/min during peak\n\n## Bottleneck Identification\n\n### Bottleneck 1: Database Query Performance\n\n**Symptom:** p99 query latency spikes to 500ms during peak\n**Impact:** API p99 latency >1s (SLA violation)\n**Root cause:** Missing indexes on frequently queried columns\n**Evidence:**\n- Slow query log shows 5 queries responsible for 80% of slow time\n- EXPLAIN shows full table scans on 2 hot tables\n- CPU iowait 15% during slow queries\n\n**Current limit:** ~8K queries/min before p99 degrades further\n**Growth headroom:** 60% (can handle up to 12.8K queries/min)\n\n### Bottleneck 2: API Server Memory\n\n**Symptom:** Memory usage 90% at peak, GC pauses increasing\n**Impact:** p99 latency degrades, occasional OOM kills\n**Root cause:** Memory leaks in session handling, large response payloads\n**Evidence:**\n- Heap snapshots show 2GB of unclaimed sessions\n- Average response size 500KB (should be <50KB)\n- GC pause time increased 3x over 3 months\n\n**Current limit:** ~800 concurrent connections before OOM risk\n**Growth headroom:** 10% (approaching limit)\n\n### Bottleneck 3: Single Point of Failure (Database)\n\n**Symptom:** Database is single instance, no failover\n**Impact:** Database downtime = full outage (99.5% uptime = 3.6 hours downtime/month)\n**Root cause:** No replication configured\n**Evidence:**\n- 2 database outages in Q1 (hardware failure, maintenance)\n- RPO = last backup (24 hours of data loss risk)\n\n**Current limit:** N/A (reliability issue, not throughput)\n**Growth headroom:** 0% (needs immediate mitigation)\n\n## Projected Growth\n\n**Current:** 5K active users, 10K requests/min\n\n**6 months:** 15K users (3x), 30K requests/min (3x)\n**12 months:** 50K users (10x), 100K requests/min (10x)\n**24 months:** 200K users (40x), 400K requests/min (40x)\n\n**Growth rate:** 3x every 6 months (based on Q1 user acquisition)\n\n## Capacity Limits\n\n**API Server:**\n- Current capacity: 15K requests/min at p99 < 1s\n- Hard limit: ~20K requests/min (CPU 100%, memory OOM)\n- Time to limit: **3 months** at current growth (3x every 6 months)\n\n**Database:**\n- Current capacity: 8K queries/min at p99 < 500ms\n- Hard limit: ~12K queries/min (CPU 100%, connection pool exhausted)\n- Time to limit: **4 months**\n\n**Cache:**\n- Current capacity: Well under capacity (50% memory)\n- Hard limit: 100K ops/sec, 16GB memory\n- Time to limit: **12+ months**\n\n**First bottleneck:** API Server memory (3 months)\n**Second bottleneck:** Database query performance (4 months)\n\n## Scaling Urgency\n\n**Red (Immediate):** Database SPOF - one outage away from multi-hour downtime\n**Yellow (3 months):** API Server memory - OOM crashes during next growth spike\n**Green (6+ months):** Cache capacity - no immediate concern" \
  --source "scalability-analysis" \
  --tags "scalability,analysis,[system-name]"
```

### Step 2: Model Load Patterns

```bash
engram reasoning create \
  --title "Load Pattern Model: [System Name]" \
  --task-id [TASK_ID] \
  --content "## Load Characteristics\n\n### Daily Pattern\n\n**Time-based traffic:**\n- 6am-9am: 50% of daily load (morning check-ins)\n- 9am-12pm: 100% baseline load (peak usage)\n- 12pm-1pm: 70% (lunch dip)\n- 1pm-5pm: 90% (afternoon usage)\n- 5pm-11pm: 40% (evening taper)\n- 11pm-6am: 10% (night baseline)\n\n**Peak hour:** 10am-11am (15K requests/min, 150% of daily average)\n**Trough hour:** 3am-4am (1K requests/min, 10% of daily average)\n**Peak-to-trough ratio:** 15:1\n\n### Weekly Pattern\n\n**Day-of-week traffic:**\n- Monday: 110% (week start surge)\n- Tuesday-Thursday: 100% baseline\n- Friday: 90% (early weekend start)\n- Saturday: 50% (weekend low)\n- Sunday: 40% (weekend low)\n\n**Business days vs Weekends:** 2.5x higher weekday traffic\n\n### Seasonal Pattern\n\n**Month-by-month:**\n- January: 90% (post-holiday dip)\n- February-March: 100% baseline\n- April: 120% (Q2 kickoff for enterprise customers)\n- May-August: 80% (summer slowdown)\n- September: 130% (back-to-school, Q3 surge)\n- October-November: 100% baseline\n- December: 70% (holiday shutdown)\n\n**Seasonality factor:** 1.6x variance (130% peak vs 80% trough)\n\n### Event-Driven Spikes\n\n**Predictable events:**\n- Product launches: 5x traffic for 1 hour\n- Webinar registrations: 3x traffic for 30 minutes\n- Monthly billing: 2x traffic on 1st of month\n- Enterprise onboarding: 10x traffic for new customer (1-2 per week)\n\n**Unpredictable events:**\n- Viral social media post: 20x traffic for 2-4 hours (happened 2x in Q1)\n- Competitor outage: 3x traffic as users try alternatives\n- Media mention: 5x traffic for 1 day\n\n## Load Model Formula\n\n**Base load:** 10K requests/min (current average)\n\n**Traffic = Base × Growth × Time-of-day × Day-of-week × Seasonality × Events**\n\n**Example (Peak scenario):**\n- Base: 10K\n- Growth: 3x (6 months from now)\n- Time-of-day: 1.5x (10am peak)\n- Day-of-week: 1.1x (Monday)\n- Seasonality: 1.3x (September)\n- Event: 1.0x (no event)\n\n**Peak load in 6 months:** 10K × 3 × 1.5 × 1.1 × 1.3 = **64K requests/min**\n\n**With viral event:** 64K × 5 = **320K requests/min** (32x current average)\n\n## Scaling Requirements\n\n**Must handle:**\n- Normal growth: 3x every 6 months\n- Peak hour: 1.5x daily average\n- Weekday peak: 2.5x weekend\n- Seasonal peak: 1.3x baseline\n- Viral event: 5x (occurred 2x in Q1, likely again)\n\n**Design target:** 10x current capacity = 100K requests/min sustained\n\n**Rationale:**\n- 6-month growth: 30K (3x)\n- Peak multipliers: 30K × 1.5 × 1.1 × 1.3 = 64K\n- Viral event: 64K × 1.5 (conservative) = 96K\n- Buffer: 100K provides 4% headroom\n\n## Failure Scenarios\n\n**Scenario 1: Database Failover**\n- Probability: High (no replication, 2 outages in Q1)\n- Impact: Full outage 15-30 minutes (DNS failover + replica promotion)\n- Frequency: 2-3× per quarter\n\n**Scenario 2: API Server Instance Failure**\n- Probability: Medium (healthy instances, but no auto-recovery)\n- Impact: 25% capacity loss until manual intervention (10-15 min)\n- Frequency: 1× per quarter\n\n**Scenario 3: Load Balancer Saturation**\n- Probability: Low (well under capacity)\n- Impact: 100% traffic dropped until scaled\n- Frequency: Never observed, but risk at 10x growth\n\n**Scenario 4: Cache Eviction Storm**\n- Probability: Medium (observed 100 evictions/min at peak)\n- Impact: Database query load increases 10x (cache miss), cascading failure\n- Frequency: Risk during viral events\n\n**Confidence:** 0.80" \
  --confidence 0.80 \
  --tags "scalability,load-model,[system-name]"
```

### Step 3: Design Scaling Strategy

```bash
engram reasoning create \
  --title "Scaling Strategy: [System Name]" \
  --task-id [TASK_ID] \
  --content "## Scaling Dimensions\n\n### Horizontal Scaling (Scale Out) - PREFERRED\n\n**What scales horizontally:**\n- API servers: Add more instances behind load balancer\n- Cache layer: Redis cluster with sharding\n- Background workers: Add more job processors\n\n**Advantages:**\n- No downtime to add capacity\n- Cost-effective (use smaller instances)\n- Fault-tolerant (no single point of failure)\n- Elastic (auto-scale based on demand)\n\n**Limitations:**\n- Database doesn't scale writes horizontally (single-leader bottleneck)\n- Requires stateless application design\n- Session affinity if using local sessions\n\n### Vertical Scaling (Scale Up) - TEMPORARY\n\n**What requires vertical scaling:**\n- Database: Single-leader replication limits write throughput\n- In-memory workloads: Larger instance = more memory\n\n**Advantages:**\n- Simple (no code changes)\n- Works for single-leader database\n\n**Limitations:**\n- Downtime required (instance restart)\n- Expensive (large instances cost more per resource)\n- Hard limit (AWS max instance size)\n- Not fault-tolerant\n\n## Immediate Actions (Month 1)\n\n### Action 1: Database Replication (CRITICAL)\n\n**Goal:** Eliminate single point of failure\n\n**Implementation:**\n- Set up read replicas (2 replicas in different AZs)\n- Configure automatic failover (RDS Multi-AZ)\n- Route read traffic to replicas (80% of queries are reads)\n- Route write traffic to primary\n\n**Impact:**\n- Uptime: 99.5% → 99.95% (reduces downtime 9x)\n- Read capacity: 3x (primary + 2 replicas)\n- RPO: 24 hours → <1 minute\n- Cost: +$2K/month (2 replica instances)\n\n**Timeline:** 2 weeks (setup, testing, migration)\n**Priority:** P0 (blocking enterprise sales)\n\n### Action 2: Fix Database Indexes\n\n**Goal:** Reduce query latency 50%\n\n**Implementation:**\n- Add indexes on hot columns (user_id, created_at, status)\n- Analyze slow query log (top 5 queries)\n- Run EXPLAIN on slow queries, verify indexes used\n- Test on staging with production snapshot\n\n**Impact:**\n- Query latency p95: 120ms → 60ms\n- Database CPU: 40% → 30% (less full table scans)\n- Capacity headroom: 12K → 18K queries/min\n\n**Timeline:** 1 week\n**Priority:** P0 (quick win, high impact)\n\n### Action 3: Fix Memory Leaks\n\n**Goal:** Reduce API server memory usage 30%\n\n**Implementation:**\n- Profile heap to identify leaks (session objects)\n- Implement session cleanup (TTL-based)\n- Reduce response payload size (pagination, field selection)\n- Add memory alerts (>85% triggers warning)\n\n**Impact:**\n- Memory: 70% → 50% avg, 90% → 65% peak\n- GC pause: Reduced 50%\n- OOM risk: Eliminated\n- Capacity: 800 → 1200 concurrent connections\n\n**Timeline:** 2 weeks\n**Priority:** P0 (approaching OOM limit)\n\n## Short-term Scaling (Month 2-6)\n\n### Strategy 1: Horizontal API Scaling\n\n**Goal:** Handle 3x growth (10K → 30K requests/min)\n\n**Implementation:**\n- Add auto-scaling group (4 → 8 instances at peak)\n- Configure scale-out rules (CPU >70% for 5 min → +2 instances)\n- Configure scale-in rules (CPU <40% for 15 min → -1 instance)\n- Set min=4, max=12 instances\n\n**Impact:**\n- Capacity: 15K → 45K requests/min (3x)\n- Cost: $4K/month → $6K/month avg (50% increase)\n- Elasticity: Handles viral spikes (5x) with auto-scale\n\n**Timeline:** 2 weeks (setup, load testing)\n**Priority:** P1 (needed for 6-month growth)\n\n### Strategy 2: Cache Optimization\n\n**Goal:** Increase cache hit rate to 95%\n\n**Implementation:**\n- Analyze cache miss patterns (identify cacheable queries)\n- Implement query result caching (top 20 queries)\n- Increase cache TTL for stable data (1 hour → 4 hours)\n- Add cache warming for common queries\n\n**Impact:**\n- Cache hit rate: 85% → 95%\n- Database load: 8K → 4K queries/min (50% reduction)\n- API latency p95: 450ms → 300ms\n\n**Timeline:** 3 weeks\n**Priority:** P1 (reduces database bottleneck)\n\n### Strategy 3: Database Connection Pooling\n\n**Goal:** Support 2x more concurrent connections\n\n**Implementation:**\n- Implement PgBouncer (connection pooler)\n- Configure pool size (200 → 500 connections)\n- Set transaction pooling mode (reduces connection overhead)\n\n**Impact:**\n- Connection capacity: 200 → 500 (2.5x)\n- Connection latency: Reduced (reuse pooled connections)\n- Database overhead: Reduced (fewer connection setups)\n\n**Timeline:** 1 week\n**Priority:** P1\n\n## Long-term Scaling (Month 6-12)\n\n### Strategy 4: Database Sharding\n\n**Goal:** Handle 10x write traffic (1K → 10K writes/min)\n\n**When:** At 80% of single-leader capacity (~8K writes/min)\n\n**Implementation:**\n- Shard by user_id (consistent hashing)\n- Run 4 database shards (each handles 25% of users)\n- Implement shard routing in application layer\n- Migrate users to shards gradually (5% per day)\n\n**Impact:**\n- Write capacity: 10K → 40K writes/min (4x)\n- Read capacity: 30K → 120K reads/min (4x with replicas)\n- Complexity: High (cross-shard queries difficult)\n\n**Timeline:** 8 weeks (implementation, testing, migration)\n**Priority:** P2 (needed at 10x scale)\n**Cost:** +$12K/month (4 shard clusters)\n\n### Strategy 5: CDN for Static Assets\n\n**Goal:** Reduce API server load by 30%\n\n**Implementation:**\n- Serve static assets (images, CSS, JS) from CDN (CloudFront)\n- Cache API responses at edge (cacheable GET requests)\n- Set cache headers (1 hour for dynamic, 1 day for static)\n\n**Impact:**\n- API server load: -30% (offloaded to CDN)\n- User latency: -50ms (edge caching)\n- Cost: +$1K/month CDN, -$1K/month API servers (net zero)\n\n**Timeline:** 2 weeks\n**Priority:** P2 (nice-to-have optimization)\n\n### Strategy 6: Multi-region Deployment\n\n**Goal:** Reduce latency for global users\n\n**When:** When >20% users outside US (currently 10%)\n\n**Implementation:**\n- Deploy API servers in EU and Asia regions\n- Use geo-routing to nearest region\n- Replicate database to each region (eventual consistency)\n\n**Impact:**\n- Latency: -100ms for EU users, -200ms for Asia users\n- Availability: Region failure doesn't affect other regions\n- Complexity: Multi-region data consistency challenges\n\n**Timeline:** 12 weeks\n**Priority:** P3 (defer until global user base larger)\n**Cost:** +$15K/month (3 regions)\n\n## Scaling Roadmap\n\n**Month 1 (Immediate):**\n- ✓ Database replication (failover)\n- ✓ Fix indexes (query performance)\n- ✓ Fix memory leaks (capacity)\n\n**Month 2-3:**\n- Horizontal API scaling (auto-scale)\n- Cache optimization (hit rate 95%)\n- Connection pooling (PgBouncer)\n\n**Month 6:**\n- Database sharding (when write load approaches limit)\n- CDN for static assets\n\n**Month 12:**\n- Multi-region deployment (if global user base grows)\n\n## Cost Projection\n\n**Current:** $8K/month\n- API servers: $4K (4 instances)\n- Database: $3K (single instance)\n- Cache: $1K (Redis)\n\n**Month 1:** $10K/month (+25%)\n- Database replication: +$2K\n\n**Month 6:** $18K/month (+125%)\n- Auto-scaling: +$2K avg (6-12 instances)\n- Cache optimization: +$1K (larger Redis)\n- Connection pooling: +$0.5K (PgBouncer)\n- Database sharding: +$12K (not yet needed)\n\n**Month 12:** $35K/month (+338%)\n- Database sharding: +$12K\n- Multi-region: +$15K\n\n**Revenue/cost ratio:**\n- Current: $50K MRR / $8K infra = 6.25:1\n- Month 12: $500K MRR / $35K infra = 14.3:1 (better ratio)\n\n**Recommendation:** Invest in scaling - infra costs grow slower than revenue\n\n## Confidence Assessment\n\n**Immediate actions (Month 1):** 0.95 (well-understood, proven techniques)\n**Short-term scaling (Month 2-6):** 0.85 (auto-scaling tested, some unknowns)\n**Long-term scaling (Month 6-12):** 0.65 (sharding complex, multi-region untested)\n\n**Overall confidence:** 0.80" \
  --confidence 0.80 \
  --tags "scalability,scaling-strategy,[system-name]"
```

### Step 4: Forecast Infrastructure Needs

```bash
engram reasoning create \
  --title "Infrastructure Forecast: [System Name] - [Timeframe]" \
  --task-id [TASK_ID] \
  --content "## Growth Assumptions\n\n**Current scale:** 5K users, 10K requests/min\n**Growth rate:** 3x every 6 months (based on Q1 data)\n**Planning horizon:** 24 months\n\n## Capacity Forecast\n\n### 6 Months (Q3 2026)\n**Users:** 15K (3x)\n**Traffic:** 30K requests/min (3x)\n**Database:** 24K queries/min (3x)\n**Storage:** 1.5TB (3x)\n\n**Infrastructure needed:**\n- API servers: 8 instances (current: 4)\n- Database: 1 primary + 2 replicas (current: 1 primary)\n- Cache: 24GB Redis (current: 16GB)\n- Storage: 2TB (current: 500GB)\n\n**Cost:** $18K/month (current: $8K)\n\n### 12 Months (Q1 2027)\n**Users:** 50K (10x)\n**Traffic:** 100K requests/min (10x)\n**Database:** 80K queries/min (10x)\n**Storage:** 5TB (10x)\n\n**Infrastructure needed:**\n- API servers: 16 instances\n- Database: 4 shards × (1 primary + 2 replicas) = 12 instances\n- Cache: 48GB Redis cluster\n- Storage: 6TB\n\n**Cost:** $45K/month\n\n### 24 Months (Q1 2028)\n**Users:** 200K (40x)\n**Traffic:** 400K requests/min (40x)\n**Database:** 320K queries/min (40x)\n**Storage:** 20TB (40x)\n\n**Infrastructure needed:**\n- API servers: 32 instances across 3 regions\n- Database: 16 shards × (1 primary + 2 replicas) = 48 instances\n- Cache: 96GB Redis cluster per region\n- CDN: Global distribution\n- Storage: 25TB\n\n**Cost:** $120K/month\n\n## Budget Planning\n\n**Year 1:**\n- Q1-Q2: $8K/month (current)\n- Q3-Q4: $18K/month (after replication + auto-scale)\n- Total: $156K\n\n**Year 2:**\n- Q1-Q2: $45K/month (after sharding)\n- Q3-Q4: $80K/month (toward multi-region)\n- Total: $750K\n\n**Year 3:**\n- Q1-Q4: $120K/month (multi-region, heavy sharding)\n- Total: $1.44M\n\n**Revenue/cost ratio:**\n- Year 1: $600K revenue / $156K infra = 3.8:1\n- Year 2: $6M revenue / $750K infra = 8:1\n- Year 3: $24M revenue / $1.44M infra = 16.7:1\n\n**Efficiency improves with scale** - infra costs grow sub-linearly\n\n**Confidence:** 0.70 (Year 1: 0.85, Year 2: 0.70, Year 3: 0.50)" \
  --confidence 0.70 \
  --tags "scalability,forecast,[system-name]"
```

### Step 5: Link Scalability Entities

```bash
# Link all scalability analysis
engram relationship create \
  --source-id [TASK_ID] --source-type task \
  --target-id [ANALYSIS_ID] --target-type context \
  --relationship-type references \
  --agent [AGENT]

engram relationship create \
  --source-id [TASK_ID] --source-type task \
  --target-id [LOAD_MODEL_ID] --target-type reasoning \
  --relationship-type documents \
  --agent [AGENT]

engram relationship create \
  --source-id [TASK_ID] --source-type task \
  --target-id [SCALING_STRATEGY_ID] --target-type reasoning \
  --relationship-type documents \
  --agent [AGENT]

engram relationship create \
  --source-id [TASK_ID] --source-type task \
  --target-id [FORECAST_ID] --target-type reasoning \
  --relationship-type documents \
  --agent [AGENT]
```

## Example

User needs to plan scaling for API server expecting 10x growth over 12 months.

(Example following similar pattern to above skills, with concrete engram commands creating entities...)

## Querying Scalability Analysis

```bash
# Get scalability analyses
engram context list | grep "Scalability Analysis:"

# Get load models
engram reasoning list | grep "Load Pattern Model:"

# Get scaling strategies
engram reasoning list | grep "Scaling Strategy:"

# Get infrastructure forecasts
engram reasoning list | grep "Infrastructure Forecast:"

# Get all scalability work for a system
engram relationship connected --entity-id [TASK_ID] | grep -E "Scalability|Load|Scaling|Forecast"
```

## Related Skills

This skill integrates with:
- `engram-system-design` - Design systems for scalability from start
- `engram-capacity-planning` - Align infrastructure capacity with team capacity
- `engram-risk-assessment` - Assess scaling risks and failure modes
- `engram-release-planning` - Plan scaling changes with rollback strategies
- `engram-roadmap-planning` - Align scaling investments with business roadmap
