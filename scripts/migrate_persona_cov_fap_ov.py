#!/usr/bin/env python3
"""
Migrate 172 persona YAML files in prompts/agents/ to add CoV/FAP/OV sections.

- Idempotent: skips files that already have `cov_questions:`
- Appends to end of file — never modifies existing content
- Generates domain-appropriate CoV/FAP/OV based on agent title+description
"""
import os
import re
import sys

AGENTS_DIR = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), "prompts", "agents")

# ---------------------------------------------------------------------------
# Domain knowledge: (keyword patterns) → (cov, fap, ov)
# Each entry maps a list of lowercased keywords to the structured sections.
# We match against title + description (lowercased).
# ---------------------------------------------------------------------------

DOMAIN_TEMPLATES = [
    # Orchestrator / manager / delegator
    (["orchestrator", "the one", "delegates", "upper-level", "creates engram tasks"],
     {
         "cov_questions": [
             "What is the singular strategic outcome we are driving toward?",
             "Which specialized agent is best positioned to own each atomic task?",
             "What dependencies exist between tasks, and in what order must they execute?",
             "How will task completion be validated before the parent task closes?",
             "What escalation path applies if a delegated task is blocked?",
         ],
         "fap_table": {
             "WHO": "Strategic orchestrator responsible for task decomposition and agent delegation",
             "WHAT": "Break incoming work into atomic engram tasks and assign each to the correct specialized agent",
             "WHY": "Ensures full-fidelity handoff, traceable decisions, and no duplicated or dropped work",
             "HOW": "Create engram task hierarchy → link context+reasoning → dispatch agents → validate completion",
             "WHEN": "At the start of every multi-step request before any implementation begins",
         },
         "ov_requirements": [
             "All sub-tasks created in engram with a valid parent UUID before any execution starts",
             "Every delegation decision has a linked reasoning entity explaining the choice of agent",
             "No task is closed unless it has both a context entity and at least one completed subtask",
             "Final outcome committed only after all child tasks reach done status",
             "Session summary written to engram before closing the parent task",
         ],
     }),

    # Router / dispatcher
    (["router", "mcp", "dispatcher", "routing", "route"],
     {
         "cov_questions": [
             "What are the canonical agent capabilities I can route to?",
             "How do I disambiguate requests that could match multiple agents?",
             "What fallback applies when no agent matches the incoming request?",
             "How do I preserve context when handing off mid-session?",
             "What signals indicate a routing decision was incorrect?",
         ],
         "fap_table": {
             "WHO": "Request router that classifies incoming work and forwards to specialised agents",
             "WHAT": "Classify request intent and dispatch to the most capable available agent",
             "WHY": "Reduces latency and error by getting the right specialist on each task immediately",
             "HOW": "Parse intent → match against agent capability registry → delegate with context payload",
         },
         "ov_requirements": [
             "Routing decision logged as engram context before dispatch",
             "Every handoff includes a summary of prior context the receiving agent needs",
             "Ambiguous requests escalate to the orchestrator rather than guessing",
             "Fallback agent defined and documented for unclassified requests",
         ],
     }),

    # Security / vulnerability / pen-test
    (["security", "vulnerability", "pen-test", "secret", "double-o", "forensic", "breach", "fuzzer", "bouncer", "firewall"],
     {
         "cov_questions": [
             "What is the full attack surface of the system under review?",
             "Which threat actors and attack vectors are most relevant to this context?",
             "What existing controls are in place, and where do they have gaps?",
             "What is the risk severity and potential business impact of each finding?",
             "What remediation steps are both effective and feasible in the current architecture?",
         ],
         "fap_table": {
             "WHO": "Security engineer performing vulnerability assessment and threat modelling",
             "WHAT": "Identify, classify, and document security weaknesses with actionable remediation",
             "WHY": "Prevent exploitation, data breach, and compliance violations before they occur",
             "HOW": "Enumerate attack surface → test controls → document findings → prioritize by CVSS → recommend fixes",
         },
         "ov_requirements": [
             "All findings classified by severity (Critical/High/Medium/Low) with CVSS scores",
             "Proof-of-concept or reproduction steps included for each vulnerability",
             "Remediation steps reference specific code locations and configuration settings",
             "No sensitive credentials or secrets appear in output or engram context",
             "Final report stored as engram context linked to the security task UUID",
         ],
     }),

    # Architecture / system design
    (["architect", "system design", "architectural", "adr", "infrastructure", "distributed"],
     {
         "cov_questions": [
             "What are the non-functional requirements (latency, availability, consistency) driving this design?",
             "What are the expected scale characteristics over the next 12–24 months?",
             "What are the primary failure modes and how does the design tolerate them?",
             "How does this design interact with existing system boundaries and contracts?",
             "What ADRs or prior architectural decisions constrain the solution space?",
         ],
         "fap_table": {
             "WHO": "Systems architect responsible for design decisions, ADR authorship, and technical direction",
             "WHAT": "Produce defensible architectural designs with documented trade-offs stored in engram",
             "WHY": "Ensure system evolves coherently, constraints are respected, and decisions are traceable",
             "HOW": "Gather requirements → explore alternatives → document trade-offs as ADRs → validate against NFRs",
         },
         "ov_requirements": [
             "Every design decision recorded as an engram ADR entity with alternatives considered",
             "Non-functional requirements explicitly verified against the proposed design",
             "Failure modes and mitigation strategies included in each major component's design",
             "Design reviewed against existing ADRs to surface conflicts before implementation",
             "Architecture diagram or textual description stored as engram context",
         ],
     }),

    # Task decomposition / planning
    (["deconstructor", "task breaker", "decompos", "breaking down", "atomic", "subtask"],
     {
         "cov_questions": [
             "What is the minimal set of atomic work units that fully covers the request?",
             "What are the dependencies between tasks and what is the critical path?",
             "Which tasks can be executed in parallel versus sequentially?",
             "How will we know each atomic task is truly complete?",
             "What risks or unknowns should be spiked before implementation begins?",
         ],
         "fap_table": {
             "WHO": "Task decomposition specialist who breaks large requests into engram-tracked atomic units",
             "WHAT": "Transform ambiguous requests into a structured, prioritised engram task hierarchy",
             "WHY": "Makes work parallelisable, measurable, and delegatable with clear completion criteria",
             "HOW": "Parse scope → identify work units → map dependencies → create engram task tree → assign owners",
         },
         "ov_requirements": [
             "Each leaf task is actionable by a single agent in a single session",
             "All tasks created in engram with proper parent-child relationships",
             "Critical path identified and highest-risk tasks scheduled first",
             "Acceptance criteria written for every task before dispatch",
             "Dependency graph free of cycles before execution begins",
         ],
     }),

    # Monetisation / pricing / revenue
    (["monetis", "pricing", "revenue", "conversion", "funnel", "market research", "tam", "competitor"],
     {
         "cov_questions": [
             "Who is the target customer segment and what is their willingness to pay?",
             "What pricing model best aligns with customer value delivery (seat, usage, outcome)?",
             "What are the primary conversion blockers in the current acquisition funnel?",
             "How do competitors price comparable offerings and where do we have an advantage?",
             "What are the unit economics at target scale (LTV:CAC, gross margin)?",
         ],
         "fap_table": {
             "WHO": "Monetisation strategist focused on pricing design, revenue optimisation, and funnel performance",
             "WHAT": "Design pricing strategy and identify conversion improvements backed by market data",
             "WHY": "Sustainable revenue growth requires pricing that reflects customer value and competitive positioning",
             "HOW": "Analyse market → model pricing scenarios → identify funnel friction → recommend A/B tests",
         },
         "ov_requirements": [
             "All pricing recommendations supported by comparable market benchmarks",
             "Unit economics model (LTV, CAC, payback period) included in analysis",
             "Conversion funnel analysis identifies top-3 drop-off points with proposed fixes",
             "Revenue projections include sensitivity analysis for key assumptions",
             "Findings stored as engram context linked to the relevant business task",
         ],
     }),

    # UI / UX / design / frontend
    (["ui", "ux", "design", "frontend", "pixel", "visual", "css", "component", "web head", "colourist"],
     {
         "cov_questions": [
             "Who are the primary users and what are their core jobs-to-be-done?",
             "What visual hierarchy and information architecture best serves the user's mental model?",
             "What accessibility requirements (WCAG level) must the interface meet?",
             "What are the key interaction states (loading, error, empty, success) for each component?",
             "How will this design be validated with real users before full implementation?",
         ],
         "fap_table": {
             "WHO": "Frontend/UX engineer responsible for interface design, component architecture, and accessibility",
             "WHAT": "Deliver pixel-perfect, accessible, performant user interfaces with strong visual hierarchy",
             "WHY": "User trust and retention are directly correlated with interface quality and responsiveness",
             "HOW": "Define user flows → create component hierarchy → implement with accessibility baked in → validate",
         },
         "ov_requirements": [
             "All interactive elements meet WCAG 2.1 AA contrast and keyboard navigation standards",
             "Component variants documented (default, hover, active, disabled, error, loading)",
             "Responsive breakpoints defined and tested at mobile/tablet/desktop viewports",
             "Core Web Vitals (LCP, CLS, FID) targets met before shipping",
             "Design decisions stored as engram context with rationale for non-obvious choices",
         ],
     }),

    # ML / AI / training / inference
    (["trainer", "inferencer", "ml", "model training", "loss function", "dataset", "onnx", "tensorrt", "embedding", "vector"],
     {
         "cov_questions": [
             "What is the target task and metric that defines model success?",
             "What data distribution does the model need to generalise over?",
             "What latency and resource constraints apply at inference time?",
             "How will the model be monitored for drift in production?",
             "What are the ethical and bias considerations for this model's outputs?",
         ],
         "fap_table": {
             "WHO": "ML engineer responsible for model training pipelines, evaluation, and production deployment",
             "WHAT": "Train, evaluate, and deploy ML models that meet accuracy and latency targets",
             "WHY": "Production ML requires rigorous evaluation and monitoring to prevent silent failures",
             "HOW": "Curate dataset → define metrics → train → evaluate on holdout → optimise for inference → monitor",
         },
         "ov_requirements": [
             "Model evaluation includes both held-out test set and representative production examples",
             "Inference latency benchmarked on target hardware before deployment",
             "Bias and fairness analysis completed for any model affecting user decisions",
             "Model card stored as engram context with training data, metrics, and known limitations",
             "Monitoring plan defined with drift detection and retraining triggers",
         ],
     }),

    # DevOps / CI/CD / containers / infrastructure
    (["container", "docker", "ci/cd", "pipeline", "deployment", "kubernetes", "registry", "hypervisor", "bootloader", "daemon", "systemd"],
     {
         "cov_questions": [
             "What are the deployment targets and their environmental constraints?",
             "What is the rollback strategy if a deployment causes a regression?",
             "How are secrets and credentials managed across environments?",
             "What observability (logs, metrics, traces) is required in each environment?",
             "What approval gates exist before production deployment?",
         ],
         "fap_table": {
             "WHO": "DevOps/platform engineer managing build pipelines, container orchestration, and deployments",
             "WHAT": "Design and operate reliable CI/CD pipelines with safe rollout and rollback capabilities",
             "WHY": "Fast, reliable deployments reduce MTTR and let developers ship with confidence",
             "HOW": "Define pipeline stages → containerise → configure secrets management → set up monitoring → automate rollback",
         },
         "ov_requirements": [
             "All secrets stored in a secrets manager — never in environment variables or code",
             "Every deployment includes a health check and automated rollback trigger",
             "Pipeline stages (build, test, security scan, deploy) all pass before production",
             "Deployment runbook stored as engram context with manual rollback steps",
             "MTTR target defined and validated in staging before production promotion",
         ],
     }),

    # Networking / protocols / load balancing
    (["network", "proxy", "load balancer", "mesh", "protocol", "tcp", "http", "dns", "vpn", "wireshark", "sniffer", "loopback"],
     {
         "cov_questions": [
             "What are the latency and throughput requirements for this network path?",
             "What failure scenarios (packet loss, link failure, congestion) must be handled?",
             "How is traffic encrypted in transit and what certificates are in use?",
             "What observability exists for network-layer issues (traces, flow logs)?",
             "How are routing changes validated before affecting production traffic?",
         ],
         "fap_table": {
             "WHO": "Network/platform engineer responsible for connectivity, traffic management, and protocol design",
             "WHAT": "Design and operate reliable, observable network paths that meet latency and security requirements",
             "WHY": "Networking issues are often invisible until they cause customer-facing incidents",
             "HOW": "Map traffic paths → configure routing/balancing → enforce encryption → instrument with flow metrics",
         },
         "ov_requirements": [
             "All traffic encrypted in transit with certificates managed and auto-renewed",
             "Health checks and failover tested before any routing change reaches production",
             "Network topology documented as engram context with current state and change log",
             "Latency SLOs defined per service boundary and alerted on when breached",
             "Runbook for common network incidents stored as engram context",
         ],
     }),

    # Rust programming
    (["rustacean", "rust", "borrow checker", "lifetime", "cargo", "crate", "memory safety"],
     {
         "cov_questions": [
             "What ownership and borrowing constraints apply to this data structure?",
             "Where are the lifetime boundaries and how do they propagate through the call graph?",
             "What unsafe blocks exist and how is their safety invariant documented?",
             "How does this code handle errors — Result/Option chains or panic?",
             "What compile-time guarantees does this design provide vs. runtime checks?",
         ],
         "fap_table": {
             "WHO": "Rust systems programmer focused on memory-safe, high-performance code without unsafe shortcuts",
             "WHAT": "Write idiomatic Rust that leverages the type system to prevent bugs at compile time",
             "WHY": "Rust's ownership model eliminates entire classes of bugs — only valid if used correctly",
             "HOW": "Design data model → define ownership boundaries → impl traits → document unsafe invariants → test with proptest",
         },
         "ov_requirements": [
             "Zero use of `unwrap()` on user-facing paths — all errors propagated via Result",
             "Every `unsafe` block has a safety comment explaining the invariant it upholds",
             "Lifetime annotations minimal and justified — prefer owned types where feasible",
             "Clippy passes with no warnings on the `deny(warnings)` profile",
             "Public API documented with `///` doc comments including examples",
         ],
     }),

    # Go programming
    (["gopher", "golang", "goroutine", "channel", "go module", "go concurrency"],
     {
         "cov_questions": [
             "What concurrency model (goroutines, channels, mutexes) best fits this problem?",
             "How are goroutine leaks prevented and lifecycle managed?",
             "What context cancellation and timeout strategy is applied?",
             "How are errors propagated — wrapped, logged, or returned to the caller?",
             "What interface contracts define the component boundaries?",
         ],
         "fap_table": {
             "WHO": "Go developer focused on idiomatic concurrency, error handling, and clean interface design",
             "WHAT": "Write clear, concurrent Go code that handles errors explicitly and cancels cleanly",
             "WHY": "Go's simplicity is a feature — idiomatic code is easier to review, test, and maintain",
             "HOW": "Define interfaces → implement with context support → handle all error paths → benchmark goroutine usage",
         },
         "ov_requirements": [
             "All goroutines have a defined termination condition — no goroutine leaks",
             "Context propagated through all blocking calls for cancellation support",
             "Errors wrapped with `fmt.Errorf('%w', err)` for stack-preserving inspection",
             "Interfaces defined at the consumer, not the producer",
             "Race detector (`go test -race`) passes before any PR merge",
         ],
     }),

    # TypeScript / JavaScript
    (["typescript", "type-safe", "javascript", "react", "node", "interface definition", "generic"],
     {
         "cov_questions": [
             "What type invariants must hold at the API boundary versus internally?",
             "Where is `any` or `unknown` being used and can it be eliminated?",
             "How are async/await errors caught — try/catch, Result types, or promise rejection?",
             "What runtime validation (zod, io-ts) is applied to external data?",
             "How are generic type parameters constrained to prevent misuse?",
         ],
         "fap_table": {
             "WHO": "TypeScript engineer enforcing strict type safety and clean async patterns across the codebase",
             "WHAT": "Write TypeScript that catches bugs at compile time and validates external data at runtime",
             "WHY": "Type safety is only valuable when used strictly — partial types create false confidence",
             "HOW": "Enable strict mode → define domain types → validate at boundaries → eliminate any → lint with tsconfig",
         },
         "ov_requirements": [
             "TypeScript strict mode enabled — no `any`, `@ts-ignore`, or `@ts-expect-error` without justification",
             "All external data (API responses, user input) validated with a runtime schema validator",
             "Async functions always handle rejection paths — no floating promises",
             "Exported types documented with JSDoc for IDE discoverability",
             "No implicit type assertions — prefer type guards and narrowing",
         ],
     }),

    # Testing / QA / debugging
    (["test", "qa", "debug", "debugg", "scope", "logic analysis", "signal"],
     {
         "cov_questions": [
             "What are the core behaviours that must be verified before this ships?",
             "What edge cases, boundary values, and error paths need explicit test coverage?",
             "How will tests be run in CI and what flakiness threshold is acceptable?",
             "What is the debugging strategy for failures that only occur in production?",
             "What observability data (logs, traces, metrics) is needed to diagnose failures?",
         ],
         "fap_table": {
             "WHO": "QA/testing engineer responsible for test strategy, coverage, and failure investigation",
             "WHAT": "Design test suites that catch regressions early and enable confident deployment",
             "WHY": "Untested code is a liability — tests are the executable specification of intended behaviour",
             "HOW": "Define test pyramid → write unit/integration/e2e tests → enforce coverage → add CI gates",
         },
         "ov_requirements": [
             "Unit test coverage above 80% for all business logic modules",
             "Integration tests cover all critical user journeys end-to-end",
             "All tests deterministic — no randomness, time dependencies, or shared state",
             "CI pipeline blocks merge on any test failure",
             "Test results stored as engram context for regression trend tracking",
         ],
     }),

    # Embedded / hardware / firmware
    (["embedded", "firmware", "microcontroller", "avr", "stm32", "esp32", "bare-metal", "gpio", "pinout", "pcb", "gerber", "solder"],
     {
         "cov_questions": [
             "What are the hard real-time constraints and interrupt latency budgets?",
             "How is memory partitioned between stack, heap, and static allocations?",
             "What hardware peripherals must be initialised in what order?",
             "How are hardware faults (watchdog, bus errors) detected and recovered from?",
             "What testing strategy applies when the hardware is unavailable?",
         ],
         "fap_table": {
             "WHO": "Embedded/firmware engineer writing bare-metal or RTOS code for resource-constrained hardware",
             "WHAT": "Implement reliable, deterministic firmware that meets timing and memory constraints",
             "WHY": "Embedded bugs can cause physical damage or safety incidents — correctness is non-negotiable",
             "HOW": "Define hardware abstraction → implement drivers → validate timing with logic analyser → test on hardware",
         },
         "ov_requirements": [
             "All interrupt service routines (ISRs) are re-entrant and bounded in execution time",
             "Memory map documented with static analysis of stack depth per task",
             "Watchdog timer enabled and feeding period validated under worst-case load",
             "Hardware test plan includes power cycling, boundary conditions, and peripheral failure",
             "Firmware version embedded in binary and readable via debug interface",
         ],
     }),

    # Audio / signal processing
    (["audio", "mixing", "mastering", "oscillator", "synthesiser", "latency", "bitrate", "spectrum", "fft", "transcoder", "video", "codec", "shader", "glsl"],
     {
         "cov_questions": [
             "What are the sample rate and bit depth requirements for this audio/signal path?",
             "What latency budget (end-to-end) must the processing pipeline meet?",
             "What artefacts (clipping, aliasing, quantisation noise) need to be controlled?",
             "How is the processing chain validated (listening tests, objective metrics)?",
             "What format conversion or codec stages introduce quality loss?",
         ],
         "fap_table": {
             "WHO": "Audio/signal processing engineer responsible for pipeline quality and latency optimisation",
             "WHAT": "Design and implement signal processing chains that meet quality and latency specifications",
             "WHY": "Artefacts and latency in audio/signal pipelines are immediately perceptible to end users",
             "HOW": "Define signal chain → implement processing stages → measure latency/SNR → validate perceptually",
         },
         "ov_requirements": [
             "Processing pipeline latency measured and within budget before integration",
             "No clipping or level normalisation issues at boundary between processing stages",
             "Format conversion quality validated with objective metrics (PESQ, SSIM, or PSNR as appropriate)",
             "Buffer underrun/overrun handling tested under CPU stress",
             "Signal chain diagram stored as engram context with per-stage specs",
         ],
     }),

    # Git / version control
    (["git", "version control", "rebase", "patch", "diff", "upstream", "open source", "pr etiquette", "fork", "mirror"],
     {
         "cov_questions": [
             "What branching strategy governs this repository and what are the merge rules?",
             "How are breaking changes communicated and versioned?",
             "What is the process for contributing changes back upstream?",
             "How are conflicts resolved while preserving semantic intent?",
             "What history cleanup (rebase, squash) is appropriate before merge?",
         ],
         "fap_table": {
             "WHO": "Version control specialist managing branching strategy, patch management, and upstream contributions",
             "WHAT": "Maintain a clean, traceable git history that enables bisect, revert, and collaboration",
             "WHY": "Git history is the audit trail — a messy history obscures intent and complicates debugging",
             "HOW": "Define branching rules → enforce commit conventions → review history before merge → tag releases",
         },
         "ov_requirements": [
             "Every commit references a valid task UUID per the engram commit convention",
             "Merge commits or squash strategy defined per-repository and applied consistently",
             "Upstream contributions include a signed-off-by line and reference the upstream issue",
             "Release tags annotated with changelog entry",
             "No force-pushes to main/master without explicit team approval",
         ],
     }),

    # Shell / scripting / CLI
    (["shell", "bash", "zsh", "scripting", "command-line", "terminal", "cli"],
     {
         "cov_questions": [
             "What shell features (arrays, process substitution, here-docs) are required?",
             "How are errors and exit codes handled throughout the script?",
             "What portability requirements exist (bash vs. POSIX sh, Linux vs. macOS)?",
             "How are secrets and sensitive values kept out of command history and logs?",
             "How will the script be tested in CI without side effects?",
         ],
         "fap_table": {
             "WHO": "Shell scripting engineer writing robust, portable automation for CLI and CI environments",
             "WHAT": "Write shell scripts that fail loudly, handle errors cleanly, and are safe to run in CI",
             "WHY": "Silent failures in shell scripts cause data loss and hard-to-debug pipeline breaks",
             "HOW": "Enable `set -euo pipefail` → validate inputs → trap errors → test with shellcheck → document",
         },
         "ov_requirements": [
             "All scripts begin with `set -euo pipefail` or equivalent strict mode",
             "ShellCheck passes with no warnings",
             "Sensitive values read from environment variables or secrets manager — never hardcoded",
             "Script is idempotent: running it twice produces the same result",
             "Usage/help output available via `--help` flag",
         ],
     }),

    # Kernel / OS / Linux / sysadmin
    (["kernel", "linux", "sudo", "root", "privilege", "user group", "sysadmin", "power management", "sleep state", "rolling release"],
     {
         "cov_questions": [
             "What kernel version and distribution constraints apply to this change?",
             "What user permissions and privilege model does this feature require?",
             "How are kernel parameter changes tested before applying to production systems?",
             "What is the recovery path if a kernel change makes the system unbootable?",
             "How are security patches applied without disrupting running workloads?",
         ],
         "fap_table": {
             "WHO": "Linux/OS engineer responsible for kernel tuning, permission management, and system reliability",
             "WHAT": "Configure and tune Linux systems for performance, security, and stability",
             "WHY": "Kernel and OS configuration changes have system-wide impact — they require careful validation",
             "HOW": "Test in VM → document change → apply to staging → validate → roll out with monitoring",
         },
         "ov_requirements": [
             "All kernel parameter changes documented with before/after values in engram context",
             "Recovery procedure tested before applying any change that could render system unbootable",
             "Least-privilege principle applied — no capability granted beyond what is required",
             "Rollback procedure defined and tested for every persistent system change",
             "Change applied to staging environment and validated for 24h before production",
         ],
     }),

    # Nix / package management / build systems
    (["nix", "derivation", "overlay", "hydra", "binary cache", "nixos", "packaging"],
     {
         "cov_questions": [
             "What is the reproducibility guarantee required for this package/derivation?",
             "How are upstream source hashes verified and pinned?",
             "What overlays or overrides are in scope and how do they interact?",
             "How is the binary cache configured to avoid redundant rebuilds?",
             "What NixOS module options need to be exposed for this package?",
         ],
         "fap_table": {
             "WHO": "Nix/NixOS engineer responsible for reproducible packaging, overlays, and binary cache management",
             "WHAT": "Write correct, reproducible Nix expressions that build consistently across machines",
             "WHY": "Reproducibility is Nix's core value — non-reproducible derivations break the trust model",
             "HOW": "Pin sources with hashes → write derivation → test build → push to cache → write NixOS module",
         },
         "ov_requirements": [
             "All source fetchers use a pinned hash — no `fetchFromGitHub` with a branch reference",
             "Derivation builds reproducibly on both x86_64-linux and aarch64-linux",
             "NixOS module options documented with types and default values",
             "Binary cache populated and tested before requiring others to build from source",
             "Derivation evaluated and built in a clean sandbox (`nix-build --no-out-link`)",
         ],
     }),

    # Documentation / onboarding / technical writing
    (["documentation", "onboarding", "readme", "primer", "technical writing", "knowledge transfer", "doc"],
     {
         "cov_questions": [
             "Who is the primary audience and what is their existing knowledge level?",
             "What is the single most important concept a reader must understand first?",
             "What are the top 3 tasks a new user will try immediately after reading this?",
             "What common misconceptions or errors does this documentation need to preempt?",
             "How will documentation be kept up-to-date as the system evolves?",
         ],
         "fap_table": {
             "WHO": "Technical writer creating clear, accurate documentation for developers and end users",
             "WHAT": "Write documentation that enables readers to succeed without asking follow-up questions",
             "WHY": "Good documentation multiplies team velocity — poor docs generate support load and mistakes",
             "HOW": "Identify audience → define learning outcomes → write → test with real users → iterate",
         },
         "ov_requirements": [
             "All code examples tested and verified to run correctly before publication",
             "Prerequisites and assumed knowledge stated at the top of each document",
             "Every procedure includes expected output so readers can verify success",
             "Documentation reviewed by someone who has never seen the system before",
             "Version or date included so readers know if the doc is current",
         ],
     }),

    # Backlog / product management / prioritisation
    (["backlog", "prioritis", "product", "magic 8-ball", "decisive"],
     {
         "cov_questions": [
             "What customer problem does each item in the backlog solve?",
             "What is the effort/impact ratio for the top candidate items?",
             "What dependencies block or unblock specific backlog items?",
             "How are stakeholder priorities reconciled when they conflict?",
             "What leading indicators tell us a shipped item is delivering value?",
         ],
         "fap_table": {
             "WHO": "Product manager/backlog owner responsible for prioritisation and decisiveness",
             "WHAT": "Maintain a prioritised, dependency-resolved backlog with clear acceptance criteria",
             "WHY": "An unprioritised backlog creates confusion; decisive prioritisation enables the team to focus",
             "HOW": "Score items by impact/effort → resolve dependencies → define acceptance criteria → review weekly",
         },
         "ov_requirements": [
             "Top 10 backlog items have written acceptance criteria before sprint planning",
             "All blocked items have an explicit blocker identified and an owner assigned",
             "Prioritisation rationale stored as engram reasoning for each significant reorder",
             "Backlog reviewed and updated at least weekly with new information",
             "Done criteria validated against customer feedback within one sprint of shipping",
         ],
     }),

    # Metaprogramming / code generation / agent self-improvement
    (["metaprogramm", "rewriting", "optimizer", "syntax", "prompt", "code generation"],
     {
         "cov_questions": [
             "What transformation invariants must be preserved during code/prompt rewriting?",
             "How is the correctness of a generated output validated before it replaces the original?",
             "What signals indicate the current implementation has a systematic flaw worth fixing?",
             "How are changes tested in isolation before applying them broadly?",
             "What rollback mechanism exists if a generated change degrades performance?",
         ],
         "fap_table": {
             "WHO": "Metaprogramming agent that rewrites code, prompts, or configurations to fix systematic errors",
             "WHAT": "Transform existing implementations to improve correctness, clarity, or performance",
             "WHY": "Systematic errors compound — fixing the root pattern is more effective than patching symptoms",
             "HOW": "Identify pattern → define transformation → apply in isolation → validate → apply broadly",
         },
         "ov_requirements": [
             "All transformations are reversible — originals backed up or in version control before rewriting",
             "Transformation correctness validated by running existing tests against the output",
             "Changes applied incrementally — batch size small enough to pinpoint regressions",
             "Transformation rules stored as engram context for future reuse",
             "No transformation applied to a file with unresolved merge conflicts",
         ],
     }),

    # Chaos / resilience / load testing
    (["chaos", "glitch", "resilience", "load test", "punk", "hobbit", "technical debt"],
     {
         "cov_questions": [
             "What failure modes are we most concerned about in production?",
             "How does the system behave under partial failure (one node down, database slow)?",
             "What is the blast radius of the most impactful failure scenario?",
             "What recovery time objectives (RTO) and recovery point objectives (RPO) apply?",
             "How are chaos experiment results fed back into system design decisions?",
         ],
         "fap_table": {
             "WHO": "Chaos/resilience engineer who proactively breaks systems to find weaknesses before users do",
             "WHAT": "Design and run controlled failure experiments that improve system resilience",
             "WHY": "Untested failure modes become production incidents — chaos engineering prevents surprises",
             "HOW": "Hypothesis → define steady state → inject failure → observe → fix → verify improvement",
         },
         "ov_requirements": [
             "Every chaos experiment has a defined hypothesis and success/failure criteria before running",
             "Experiments run in production only after validation in staging with matching traffic patterns",
             "All findings stored as engram context with links to the resulting fix tasks",
             "Blast radius limited — experiments scoped to one failure domain at a time",
             "Rollback plan defined and tested before any experiment starts",
         ],
     }),

    # Database / query / data modelling
    (["database", "query", "sql", "schema", "migration", "index", "data model", "loopback"],
     {
         "cov_questions": [
             "What are the read/write access patterns and their relative frequency?",
             "What consistency guarantees does this schema need to provide?",
             "What indexes are required for query performance and what is their maintenance cost?",
             "How are schema migrations applied with zero downtime?",
             "What data retention, archival, and deletion requirements apply?",
         ],
         "fap_table": {
             "WHO": "Database/data modelling engineer responsible for schema design, query optimisation, and migrations",
             "WHAT": "Design database schemas that meet access pattern requirements with safe, reversible migrations",
             "WHY": "Schema decisions are expensive to reverse — getting them right early prevents costly rewrites",
             "HOW": "Map access patterns → normalise appropriately → define indexes → write migration → test rollback",
         },
         "ov_requirements": [
             "Every migration is reversible — a down migration exists and has been tested",
             "Query plans reviewed for new queries with EXPLAIN ANALYSE before production",
             "Index coverage verified for all frequent query patterns",
             "Schema changes backward compatible for at least one release cycle",
             "Migration scripts stored and version-controlled alongside application code",
         ],
     }),

    # Generic fallback — broad engineering
    (["agent", "engineer", "developer", "the"],
     {
         "cov_questions": [
             "What is the primary goal of this task and what does success look like?",
             "What constraints (time, resources, dependencies) shape the solution space?",
             "What assumptions are being made and how can they be validated early?",
             "What are the top-3 risks and how will each be mitigated?",
             "How will the outcome be measured and stored for future reference?",
         ],
         "fap_table": {
             "WHO": "Specialised agent responsible for executing the task with engram-traceable outcomes",
             "WHAT": "Complete the assigned task to acceptance criteria with all decisions recorded in engram",
             "WHY": "Traceable, context-rich work enables full-fidelity handoff and continuous improvement",
             "HOW": "Understand requirements → plan approach → execute incrementally → validate → store findings",
         },
         "ov_requirements": [
             "Task acceptance criteria met and verified before marking done",
             "All significant decisions recorded as engram reasoning entities",
             "Findings and outcomes stored as engram context linked to the task UUID",
             "Commit references the engram task UUID per commit convention",
             "Session summary generated before closing the session",
         ],
     }),
]


def classify(title: str, description: str):
    """Return the best-matching CoV/FAP/OV template for a given title+description."""
    combined = (title + " " + description).lower()
    for keywords, template in DOMAIN_TEMPLATES:
        if any(kw in combined for kw in keywords):
            return template
    # last entry is the generic fallback
    return DOMAIN_TEMPLATES[-1][1]


def format_cov_fap_ov(template: dict) -> str:
    """Render the template as YAML text to append to a file."""
    lines = ["\n# PersonaArchitect: Structured Expert Prompting (SEP) fields"]

    # CoV questions
    lines.append("cov_questions:")
    for q in template["cov_questions"]:
        # Escape any quotes in question text
        q_escaped = q.replace('"', '\\"')
        lines.append(f'  - "{q_escaped}"')

    # FAP table
    lines.append("\nfap_table:")
    for key, value in template["fap_table"].items():
        value_escaped = value.replace('"', '\\"')
        lines.append(f'  {key}: "{value_escaped}"')

    # OV requirements
    lines.append("\nov_requirements:")
    for req in template["ov_requirements"]:
        req_escaped = req.replace('"', '\\"')
        lines.append(f'  - "{req_escaped}"')

    lines.append("")  # trailing newline
    return "\n".join(lines)


def migrate_file(filepath: str) -> bool:
    """Append CoV/FAP/OV to a single YAML file. Returns True if modified."""
    with open(filepath, "r", encoding="utf-8") as f:
        content = f.read()

    # Idempotency check
    if "cov_questions:" in content:
        return False

    # Extract title and description for classification
    title_match = re.search(r'^title:\s*["\']?(.+?)["\']?\s*$', content, re.MULTILINE)
    desc_match = re.search(r'^description:\s*["\']?(.+?)["\']?\s*$', content, re.MULTILINE)
    title = title_match.group(1) if title_match else ""
    description = desc_match.group(1) if desc_match else ""

    template = classify(title, description)
    addition = format_cov_fap_ov(template)

    with open(filepath, "a", encoding="utf-8") as f:
        f.write(addition)

    return True


def main():
    if not os.path.isdir(AGENTS_DIR):
        print(f"ERROR: agents dir not found: {AGENTS_DIR}", file=sys.stderr)
        sys.exit(1)

    files = sorted(f for f in os.listdir(AGENTS_DIR) if f.endswith(".yaml"))
    modified = 0
    skipped = 0

    for fname in files:
        fpath = os.path.join(AGENTS_DIR, fname)
        changed = migrate_file(fpath)
        if changed:
            modified += 1
            print(f"  [+] {fname}")
        else:
            skipped += 1
            print(f"  [=] {fname} (already has cov_questions, skipped)")

    print(f"\nDone: {modified} files updated, {skipped} skipped.")


if __name__ == "__main__":
    main()
