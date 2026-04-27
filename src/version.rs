use std::env;

/// Version information for Engram with git tag-based release management
#[derive(Debug, Clone)]
pub struct BuildInfo {
    pub package_version: String,
    pub git_tag: String,
    pub commit_sha: String,
    pub commit_date: String,
    pub build_timestamp: String,
    pub is_tagged_release: bool,
}

impl BuildInfo {
    pub fn get() -> Self {
        Self {
            package_version: env::var("CARGO_PKG_VERSION")
                .unwrap_or_else(|_| "unknown".to_string()),
            git_tag: env::var("ENGRAM_GIT_TAG").unwrap_or_else(|_| get_runtime_git_tag()),
            commit_sha: env::var("ENGRAM_COMMIT_SHA").unwrap_or_else(|_| get_runtime_git_sha()),
            commit_date: env::var("ENGRAM_COMMIT_DATE").unwrap_or_else(|_| get_runtime_git_date()),
            build_timestamp: env::var("ENGRAM_BUILD_TIMESTAMP")
                .unwrap_or_else(|_| get_current_timestamp()),
            is_tagged_release: env::var("ENGRAM_IS_TAGGED_RELEASE")
                .map(|val| val == "true")
                .unwrap_or(false),
        }
    }

    pub fn version_string(&self) -> String {
        if self.is_tagged_release {
            self.package_version.clone() // Clean version for tagged releases
        } else if !self.commit_sha.is_empty() && self.commit_sha.len() >= 8 {
            format!(
                "{} ({} {})",
                self.package_version,
                &self.commit_sha[..8],
                self.commit_date
            )
        } else {
            self.package_version.clone()
        }
    }
}

fn get_runtime_git_tag() -> String {
    let cwd = std::env::current_dir().unwrap_or_default();
    let repo = match gix::open(&cwd) {
        Ok(r) => r,
        Err(_) => return String::new(),
    };
    let head_id = match repo.head_id() {
        Ok(id) => id,
        Err(_) => return String::new(),
    };
    // Check if HEAD is at a tag
    let refs = match repo.references() {
        Ok(r) => r,
        Err(_) => return String::new(),
    };
    let tags = match refs.tags() {
        Ok(t) => t,
        Err(_) => return String::new(),
    };
    for tag_ref in tags {
        if let Ok(tag) = tag_ref {
            let target_id = match tag.try_id() {
                Some(id) => id,
                None => continue,
            };
            if target_id == head_id {
                // Return the tag name (strip refs/tags/ prefix)
                let name = tag.name().shorten();
                return name.to_string();
            }
        }
    }
    String::new()
}

fn get_runtime_git_sha() -> String {
    let cwd = std::env::current_dir().unwrap_or_default();
    match gix::open(&cwd) {
        Ok(repo) => repo.head_id()
            .map(|id| id.to_string())
            .unwrap_or_else(|_| "unknown".to_string()),
        Err(_) => "unknown".to_string(),
    }
}

fn get_runtime_git_date() -> String {
    let cwd = std::env::current_dir().unwrap_or_default();
    let repo = match gix::open(&cwd) {
        Ok(r) => r,
        Err(_) => return "unknown".to_string(),
    };
    let head_id = match repo.head_id() {
        Ok(id) => id,
        Err(_) => return "unknown".to_string(),
    };
    // Do a revwalk to get commit_time
    let walk = match repo
        .rev_walk([head_id])
        .sorting(gix::revision::walk::Sorting::ByCommitTime(
            gix::traverse::commit::simple::CommitTimeOrder::NewestFirst,
        ))
        .all()
    {
        Ok(w) => w,
        Err(_) => return "unknown".to_string(),
    };
    if let Some(Ok(info)) = walk.into_iter().next() {
        let secs = info.commit_time.unwrap_or(0);
        return chrono::DateTime::from_timestamp(secs, 0)
            .map(|dt| dt.format("%Y-%m-%d").to_string())
            .unwrap_or_else(|| "unknown".to_string());
    }
    "unknown".to_string()
}

fn get_current_timestamp() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    format!("{}", duration.as_secs())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_info_always_has_package_version() {
        let info = BuildInfo::get();
        assert!(!info.package_version.is_empty());
    }

    #[test]
    fn build_info_version_string_format() {
        let info = BuildInfo::get();
        let version = info.version_string();
        assert!(!version.is_empty());
        // Should contain either just version or version with commit info
        assert!(version.contains(&info.package_version));
    }
}
