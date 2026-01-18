use std::env;
use std::process::Command;

fn main() {
    let package_version = get_package_version();
    let git_info = extract_git_info();
    let build_timestamp = generate_build_timestamp();

    set_build_environment_variables(&package_version, &git_info, &build_timestamp);

    let full_version = create_version_string(&package_version, &git_info);
    println!("cargo:rustc-env=ENGRAM_FULL_VERSION={}", full_version);
}

fn get_package_version() -> String {
    env::var("CARGO_PKG_VERSION").unwrap_or_default()
}

#[derive(Debug, Clone)]
struct GitInfo {
    tag: String,
    commit_sha: String,
    commit_date: String,
    is_tagged_release: bool,
}

fn extract_git_info() -> GitInfo {
    // Try to get release tag first, fallback to commit
    let tag_info = get_git_tag();
    let commit_info = get_git_commit();

    GitInfo {
        tag: tag_info.clone(),
        commit_sha: commit_info.sha,
        commit_date: commit_info.date,
        is_tagged_release: !tag_info.is_empty(),
    }
}

fn get_git_tag() -> String {
    match Command::new("git")
        .args(&["describe", "--tags", "--abbrev=0", "--exact-match", "v*"])
        .output()
    {
        Ok(output) if output.status.success() => {
            let tag_output = String::from_utf8_lossy(&output.stdout);
            let tag = tag_output.trim();
            if tag.starts_with('v') {
                tag.to_string()
            } else {
                String::new()
            }
        }
        _ => String::new(),
    }
}

fn get_git_commit() -> CommitInfo {
    match Command::new("git")
        .args(&["log", "-1", "--format=%H %ci %s"])
        .output()
    {
        Ok(output) if output.status.success() => {
            let git_info = String::from_utf8_lossy(&output.stdout);
            let parts: Vec<&str> = git_info.split_whitespace().collect();
            if parts.len() >= 2 {
                CommitInfo {
                    sha: parts[0].to_string(),
                    date: parts[1].to_string(),
                }
            } else {
                CommitInfo {
                    sha: "unknown".to_string(),
                    date: "unknown".to_string(),
                }
            }
        }
        _ => CommitInfo {
            sha: "unknown".to_string(),
            date: "unknown".to_string(),
        },
    }
}

#[derive(Debug, Clone)]
struct CommitInfo {
    sha: String,
    date: String,
}

fn generate_build_timestamp() -> String {
    chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()
}

fn set_build_environment_variables(version: &str, git_info: &GitInfo, build_timestamp: &str) {
    println!("cargo:rustc-env=ENGRAM_VERSION={}", version);
    println!("cargo:rustc-env=ENGRAM_GIT_TAG={}", git_info.tag);
    println!("cargo:rustc-env=ENGRAM_COMMIT_SHA={}", git_info.commit_sha);
    println!(
        "cargo:rustc-env=ENGRAM_COMMIT_DATE={}",
        git_info.commit_date
    );
    println!("cargo:rustc-env=ENGRAM_BUILD_TIMESTAMP={}", build_timestamp);
    println!(
        "cargo:rustc-env=ENGRAM_IS_TAGGED_RELEASE={}",
        git_info.is_tagged_release
    );
}

fn create_version_string(package_version: &str, git_info: &GitInfo) -> String {
    if git_info.is_tagged_release {
        // For tagged releases, show just the version (stable)
        package_version.to_string()
    } else if !git_info.commit_sha.is_empty() && git_info.commit_sha != "unknown" {
        // For dev builds, show commit SHA (safely handle short SHAs)
        let sha_len = std::cmp::min(8, git_info.commit_sha.len());
        format!(
            "{} ({} {})",
            package_version,
            &git_info.commit_sha[..sha_len],
            git_info.commit_date
        )
    } else {
        // Fallback for builds without git
        package_version.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_git_info_structure() {
        let info = extract_git_info();
        // Should always create valid GitInfo
        assert!(!info.commit_sha.is_empty() || info.tag.is_empty());
    }
}
