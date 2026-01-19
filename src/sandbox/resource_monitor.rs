use crate::entities::ResourceLimits;
use crate::sandbox::{SandboxError, SandboxRequest, SandboxResult};
use std::collections::HashMap;
use std::time::{Duration, Instant};

pub struct ResourceMonitor {
    agent_usage: HashMap<String, AgentResourceUsage>,
    start_time: Instant,
}

#[derive(Debug, Clone)]
struct AgentResourceUsage {
    memory_mb: f64,
    cpu_percentage: f64,
    disk_space_mb: f64,
    active_operations: u32,
    network_requests_count: u32,
    network_requests_window_start: Instant,
    execution_start: Option<Instant>,
}

impl AgentResourceUsage {
    fn new() -> Self {
        Self {
            memory_mb: 0.0,
            cpu_percentage: 0.0,
            disk_space_mb: 0.0,
            active_operations: 0,
            network_requests_count: 0,
            network_requests_window_start: Instant::now(),
            execution_start: None,
        }
    }
}

impl ResourceMonitor {
    pub fn new() -> Self {
        Self {
            agent_usage: HashMap::new(),
            start_time: Instant::now(),
        }
    }

    pub async fn check_limits(
        &mut self,
        agent_id: &str,
        request: &SandboxRequest,
        limits: &ResourceLimits,
    ) -> SandboxResult<()> {
        let usage = self
            .agent_usage
            .entry(agent_id.to_string())
            .or_insert_with(AgentResourceUsage::new);

        self.update_current_usage(agent_id).await?;

        if usage.memory_mb > limits.max_memory_mb as f64 {
            return Err(SandboxError::ResourceLimitExceeded(format!(
                "Memory usage {:.1}MB exceeds limit {}MB",
                usage.memory_mb, limits.max_memory_mb
            )));
        }

        if usage.cpu_percentage > limits.max_cpu_percentage as f64 {
            return Err(SandboxError::ResourceLimitExceeded(format!(
                "CPU usage {:.1}% exceeds limit {}%",
                usage.cpu_percentage, limits.max_cpu_percentage
            )));
        }

        if usage.disk_space_mb > limits.max_disk_space_mb as f64 {
            return Err(SandboxError::ResourceLimitExceeded(format!(
                "Disk usage {:.1}MB exceeds limit {}MB",
                usage.disk_space_mb, limits.max_disk_space_mb
            )));
        }

        if usage.active_operations >= limits.max_concurrent_operations {
            return Err(SandboxError::ResourceLimitExceeded(format!(
                "Active operations {} exceeds limit {}",
                usage.active_operations, limits.max_concurrent_operations
            )));
        }

        if request.operation == "network_request" {
            self.check_network_rate_limit(agent_id, limits).await?;
        }

        if matches!(
            request.operation.as_str(),
            "execute_command" | "execute_workflow"
        ) {
            if let Some(start_time) = usage.execution_start {
                let execution_duration = start_time.elapsed();
                let max_duration =
                    Duration::from_secs(limits.max_execution_time_minutes as u64 * 60);

                if execution_duration > max_duration {
                    return Err(SandboxError::ResourceLimitExceeded(format!(
                        "Execution time {:?} exceeds limit {:?}",
                        execution_duration, max_duration
                    )));
                }
            }
        }

        if let Some(file_size) = request.parameters.get("file_size_mb") {
            if let Some(size) = file_size.as_f64() {
                if size > limits.max_file_size_mb as f64 {
                    return Err(SandboxError::ResourceLimitExceeded(format!(
                        "File size {:.1}MB exceeds limit {}MB",
                        size, limits.max_file_size_mb
                    )));
                }
            }
        }

        Ok(())
    }

    async fn update_current_usage(&mut self, agent_id: &str) -> SandboxResult<()> {
        let usage = self
            .agent_usage
            .entry(agent_id.to_string())
            .or_insert_with(AgentResourceUsage::new);

        usage.memory_mb = self.get_memory_usage(agent_id).await?;
        usage.cpu_percentage = self.get_cpu_usage(agent_id).await?;
        usage.disk_space_mb = self.get_disk_usage(agent_id).await?;

        Ok(())
    }

    async fn get_memory_usage(&self, _agent_id: &str) -> SandboxResult<f64> {
        #[cfg(target_os = "linux")]
        {
            if let Ok(status) = std::fs::read_to_string("/proc/self/status") {
                for line in status.lines() {
                    if line.starts_with("VmRSS:") {
                        if let Some(kb_str) = line.split_whitespace().nth(1) {
                            if let Ok(kb) = kb_str.parse::<f64>() {
                                return Ok(kb / 1024.0);
                            }
                        }
                    }
                }
            }
        }

        Ok(0.0)
    }

    async fn get_cpu_usage(&self, _agent_id: &str) -> SandboxResult<f64> {
        Ok(0.0)
    }

    async fn get_disk_usage(&self, _agent_id: &str) -> SandboxResult<f64> {
        Ok(0.0)
    }

    async fn check_network_rate_limit(
        &mut self,
        agent_id: &str,
        limits: &ResourceLimits,
    ) -> SandboxResult<()> {
        let usage = self
            .agent_usage
            .entry(agent_id.to_string())
            .or_insert_with(AgentResourceUsage::new);

        let now = Instant::now();
        let window_duration = Duration::from_secs(60);

        if now.duration_since(usage.network_requests_window_start) > window_duration {
            usage.network_requests_count = 0;
            usage.network_requests_window_start = now;
        }

        if usage.network_requests_count >= limits.max_network_requests_per_minute {
            return Err(SandboxError::ResourceLimitExceeded(format!(
                "Network requests {} per minute exceeds limit {}",
                usage.network_requests_count, limits.max_network_requests_per_minute
            )));
        }

        usage.network_requests_count += 1;
        Ok(())
    }

    pub fn start_operation(&mut self, agent_id: &str, operation: &str) {
        let usage = self
            .agent_usage
            .entry(agent_id.to_string())
            .or_insert_with(AgentResourceUsage::new);

        usage.active_operations += 1;

        if matches!(operation, "execute_command" | "execute_workflow") {
            usage.execution_start = Some(Instant::now());
        }
    }

    pub fn end_operation(&mut self, agent_id: &str, operation: &str) {
        if let Some(usage) = self.agent_usage.get_mut(agent_id) {
            if usage.active_operations > 0 {
                usage.active_operations -= 1;
            }

            if matches!(operation, "execute_command" | "execute_workflow") {
                usage.execution_start = None;
            }
        }
    }

    pub fn get_current_usage(&self, agent_id: &str) -> Option<ResourceUsageSnapshot> {
        self.agent_usage
            .get(agent_id)
            .map(|usage| ResourceUsageSnapshot {
                memory_mb: usage.memory_mb,
                cpu_percentage: usage.cpu_percentage,
                disk_space_mb: usage.disk_space_mb,
                active_operations: usage.active_operations,
                network_requests_this_minute: usage.network_requests_count,
            })
    }

    pub fn clear_agent_data(&mut self, agent_id: &str) {
        self.agent_usage.remove(agent_id);
    }

    pub fn get_all_agents(&self) -> Vec<String> {
        self.agent_usage.keys().cloned().collect()
    }
}

#[derive(Debug, Clone)]
pub struct ResourceUsageSnapshot {
    pub memory_mb: f64,
    pub cpu_percentage: f64,
    pub disk_space_mb: f64,
    pub active_operations: u32,
    pub network_requests_this_minute: u32,
}

impl Default for ResourceMonitor {
    fn default() -> Self {
        Self::new()
    }
}
