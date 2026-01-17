use std::env;
use std::process::Command;

fn main() {
    let package_version = get_package_version();
    let (commit_sha, commit_date) = extract_git_commit_info()
        .unwrap_or_else(|_| ("unknown".to_string(), "unknown".to_string()));
    let build_timestamp = generate_build_timestamp();

    set_build_environment_variables(
        &package_version,
        &commit_sha,
        &commit_date,
        &build_timestamp,
    );

    let full_version = create_version_string(&package_version, &commit_sha, &commit_date);
    println!("cargo:rustc-env=ENGRAM_FULL_VERSION={}", full_version);
}

fn get_package_version() -> String {
    env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| "unknown".to_string())
}

fn extract_git_commit_info() -> Result<(String, String), Box<dyn std::error::Error>> {
    let git_log_output = Command::new("git")
        .args(&["log", "-1", "--format=%H %ci %s"])
        .output()?;

    if !git_log_output.status.success() {
        return Err("Failed to execute git log".into());
    }

    parse_git_log_output(&git_log_output.stdout)
}

fn parse_git_log_output(output: &[u8]) -> Result<(String, String), Box<dyn std::error::Error>> {
    let git_info = String::from_utf8(output.to_vec())?;
    let parts: Vec<&str> = git_info.split_whitespace().collect();

    if parts.len() >= 2 {
        let commit_sha = parts[0].to_string();
        let commit_date = parts[1].to_string();
        Ok((commit_sha, commit_date))
    } else {
        Err("Git log output format invalid".into())
    }
}

fn generate_build_timestamp() -> String {
    chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()
}

fn set_build_environment_variables(
    version: &str,
    commit_sha: &str,
    commit_date: &str,
    build_timestamp: &str,
) {
    println!("cargo:rustc-env=ENGRAM_VERSION={}", version);
    println!("cargo:rustc-env=ENGRAM_COMMIT_SHA={}", commit_sha);
    println!("cargo:rustc-env=ENGRAM_COMMIT_DATE={}", commit_date);
    println!("cargo:rustc-env=ENGRAM_BUILD_TIMESTAMP={}", build_timestamp);
}

fn create_version_string(package_version: &str, commit_sha: &str, commit_date: &str) -> String {
    if commit_sha != "unknown" {
        format!("{} ({} {})", package_version, &commit_sha[..8], commit_date)
    } else {
        package_version.to_string()
    }
}
