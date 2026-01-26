//! Prompts management commands
//!
//! Provides commands for listing and showing prompts from ENGRAM_PROMPTS_PATH.

use clap::Subcommand;
use std::fs;
use std::path::PathBuf;

/// Prompts commands
#[derive(Debug, Subcommand)]
pub enum PromptsCommands {
    /// List all available prompts
    List {
        /// Category (agents, ai, compliance)
        #[arg(long, short)]
        category: Option<String>,

        /// Format output (short, full)
        #[arg(long, short, default_value = "short")]
        format: String,

        /// Verbose output (show search paths and filtering)
        #[arg(long, short)]
        verbose: bool,
    },
    /// Show prompt details
    Show {
        /// Prompt name or path
        #[arg(help = "Prompt name or path")]
        name: String,
    },
    /// Validate all prompts for evidence-based validation requirements
    Validate {
        /// Category to validate (agents, ai, compliance)
        #[arg(long, short)]
        category: Option<String>,

        /// Fix common issues automatically
        #[arg(long, short)]
        fix: bool,
    },
}

/// Get prompts path from environment or default
pub fn get_prompts_path() -> PathBuf {
    // 1. Try environment variable
    if let Ok(path_str) = std::env::var("ENGRAM_PROMPTS_PATH") {
        let path = PathBuf::from(&path_str);
        if path.exists() {
            return path;
        }
        // If set but not found, we might want to warn, but for now let's fall through
        // or we could return it to let the caller fail on it.
        // However, the requirement is "Default to looking in ... if ... point to invalid paths"
    }

    // 2. Try .engram/prompts in CWD
    let cwd_prompts = PathBuf::from(".engram/prompts");
    if cwd_prompts.exists() {
        return cwd_prompts;
    }

    // 3. Try engram/prompts in CWD
    let local_prompts = PathBuf::from("./engram/prompts");
    if local_prompts.exists() {
        return local_prompts;
    }

    // 4. Fallback to default
    PathBuf::from(".engram/prompts")
}

use crate::cli::utils::create_table;
use prettytable::row;

/// List all prompts in the prompts directory
pub fn list_prompts(
    category: Option<&str>,
    format: &str,
    root: Option<PathBuf>,
    verbose: bool,
) -> Result<(), std::io::Error> {
    let prompts_path = root.unwrap_or_else(get_prompts_path);
    let abs_path = std::fs::canonicalize(&prompts_path).unwrap_or_else(|_| prompts_path.clone());

    if verbose {
        println!("🔎 Prompts configuration:");
        println!("  • Target path: {:?}", prompts_path);
        println!("  • Absolute path: {:?}", abs_path);
        if let Ok(env_path) = std::env::var("ENGRAM_PROMPTS_PATH") {
            println!("  • ENGRAM_PROMPTS_PATH: {}", env_path);
        } else {
            println!("  • ENGRAM_PROMPTS_PATH: (not set)");
        }
    }

    if !prompts_path.exists() {
        if verbose {
            println!("❌ Directory does not exist");
        }
        // Only show error if we are not in verbose mode (already showed status)
        // or ensure the message is clear.
        println!("Prompts directory not found at: {:?}", abs_path);
        println!(
            "Current working directory: {:?}",
            std::env::current_dir().unwrap_or_default()
        );
        println!("\nTo fix this:");
        println!("1. Run 'engram setup prompts' to install default prompts");
        println!("2. Or set ENGRAM_PROMPTS_PATH to your prompts directory");
        return Ok(());
    }

    let entries = fs::read_dir(&prompts_path)?;
    let mut table = create_table();
    let mut found_any = false;

    match format {
        "short" | "s" => {
            table.set_titles(row!["Category", "Prompt Count"]);

            for entry in entries.flatten() {
                if entry.path().is_dir() {
                    let name = entry.file_name().to_string_lossy().into_owned();

                    if let Some(cat) = category {
                        if name.to_lowercase() != cat.to_lowercase() {
                            if verbose {
                                println!("  Skipping category '{}' (filtered by '{}')", name, cat);
                            }
                            continue;
                        }
                    }

                    // Count files in subdirectory
                    let count = fs::read_dir(&entry.path())?.flatten().count();
                    table.add_row(row![name, count]);
                    found_any = true;
                } else if verbose {
                    let path = entry.path();
                    let is_hidden = path
                        .file_name()
                        .map(|s| s.to_string_lossy().starts_with('.'))
                        .unwrap_or(false);

                    if !is_hidden {
                        println!("  Skipping file in root: {:?} (Prompts should be organized in subdirectories/categories, or use 'show' for specific files)", path);
                    }
                }
            }
            if found_any {
                table.printstd();
            } else if verbose {
                println!("No prompts found in {:?}", prompts_path);
            }
        }
        "full" | "f" => {
            table.set_titles(row!["Category", "Prompt Name"]);

            for entry in entries.flatten() {
                if entry.path().is_dir() {
                    let name = entry.file_name().to_string_lossy().into_owned();

                    if let Some(cat) = category {
                        if name.to_lowercase() != cat.to_lowercase() {
                            if verbose {
                                println!("  Skipping category '{}' (filtered by '{}')", name, cat);
                            }
                            continue;
                        }
                    }

                    let subentries = fs::read_dir(&entry.path())?;
                    let mut file_names = Vec::new();

                    for subentry in subentries.flatten() {
                        let sub_name = subentry.file_name().to_string_lossy().into_owned();
                        file_names.push(sub_name);
                    }
                    file_names.sort();

                    if file_names.is_empty() {
                        table.add_row(row![name, "-"]);
                        found_any = true;
                    } else {
                        for (i, file_name) in file_names.iter().enumerate() {
                            if i == 0 {
                                table.add_row(row![name, file_name]);
                            } else {
                                table.add_row(row!["", file_name]);
                            }
                            found_any = true;
                        }
                    }
                } else if verbose {
                    let path = entry.path();
                    let is_hidden = path
                        .file_name()
                        .map(|s| s.to_string_lossy().starts_with('.'))
                        .unwrap_or(false);

                    if !is_hidden {
                        println!("  Skipping file in root: {:?} (Prompts should be organized in subdirectories/categories, or use 'show' for specific files)", path);
                    }
                }
            }
            if found_any {
                table.printstd();
            } else if verbose {
                println!("No prompts found in {:?}", prompts_path);
            }
        }
        _ => {
            println!("Unknown format: {}. Use 'short' or 'full'.", format);
        }
    }

    Ok(())
}

/// Validate prompt for evidence-based validation requirements
fn validate_prompt_evidence_requirements(content: &str, prompt_name: &str) -> Vec<String> {
    let mut warnings = Vec::new();

    // Check for evidence-based validation keywords
    let evidence_keywords = [
        "evidence-based validation",
        "provide evidence-based validation",
        "instead of unsubstantiated assertions",
        "concrete evidence",
        "verifiable measurements",
        "specific examples",
        "quantifiable metrics",
    ];

    let has_evidence_requirements = evidence_keywords
        .iter()
        .any(|keyword| content.to_lowercase().contains(keyword));

    if !has_evidence_requirements {
        warnings.push(format!(
            "⚠️  Missing evidence-based validation requirements in prompt: {}",
            prompt_name
        ));
        warnings.push(
            "   Add: 'provide evidence-based validation for all final claims instead of unsubstantiated assertions'".to_string()
        );
    }

    // Check for prohibited unsubstantiated assertion patterns
    let prohibited_patterns = [
        "the code is more efficient",
        "this improves security",
        "the refactoring is better",
        "fixed the bug",
        "improved performance",
    ];

    for pattern in &prohibited_patterns {
        if content.to_lowercase().contains(pattern) {
            warnings.push(format!(
                "⚠️  Found potentially unsubstantiated assertion pattern: '{}'",
                pattern
            ));
        }
    }

    // Check for evidence collection instructions
    let evidence_collection_patterns = [
        "code reference:",
        "test results:",
        "execution log:",
        "documentation:",
        "measurement evidence:",
        "evidence collection instructions:",
        "## claim:",
        "### evidence:",
    ];

    let has_evidence_collection = evidence_collection_patterns
        .iter()
        .any(|pattern| content.to_lowercase().contains(pattern));

    if has_evidence_requirements && !has_evidence_collection {
        warnings.push(
            "⚠️  Prompt mentions evidence-based validation but lacks specific evidence collection instructions".to_string()
        );
    }

    warnings
}

/// Show a specific prompt
pub fn show_prompt(name: &str, root: Option<PathBuf>) -> Result<(), std::io::Error> {
    let prompts_path = root.unwrap_or_else(get_prompts_path);

    // Try to find the prompt file
    let prompt_path = prompts_path.join(name);

    if prompt_path.exists() {
        if prompt_path.is_file() {
            let content = fs::read_to_string(&prompt_path)?;
            println!("\nPrompt: {}", name);
            println!("========");
            println!("\n{}", content);

            // Validate evidence-based validation requirements
            let warnings = validate_prompt_evidence_requirements(&content, name);
            if !warnings.is_empty() {
                println!("\n🔍 Evidence-Based Validation Check:");
                for warning in warnings {
                    println!("{}", warning);
                }
                println!("\n💡 See: ./engram/prompts/_evidence-based-validation-template.yaml");
            }
        } else {
            // It's a directory, list contents
            println!("\nPrompt Directory: {}", name);
            println!("===================");

            let entries = fs::read_dir(&prompt_path)?;
            for entry in entries.flatten() {
                let file_name = entry.file_name().to_string_lossy().into_owned();
                let file_type = if entry.path().is_dir() {
                    "[DIR]"
                } else {
                    "[FILE]"
                };
                println!("  {} {}", file_type, file_name);
            }
        }
    } else {
        // Search for matching file
        println!("Searching for: {}", name);

        let search_name = name.to_lowercase();
        let entries = fs::read_dir(&prompts_path)?;
        for entry in entries.flatten() {
            if entry.path().is_dir() {
                let entry_name = entry.file_name().to_string_lossy().into_owned();
                let subentries = fs::read_dir(&entry.path())?;
                for subentry in subentries.flatten() {
                    let sub_name = subentry.file_name().to_string_lossy().into_owned();
                    if sub_name.to_lowercase().contains(&search_name) && subentry.path().is_file() {
                        println!("\nFound: {}/{}", entry_name, sub_name);
                        let content = fs::read_to_string(&subentry.path())?;
                        println!("\n{}", content);

                        // Validate evidence-based validation requirements
                        let warnings = validate_prompt_evidence_requirements(
                            &content,
                            &format!("{}/{}", entry_name, sub_name),
                        );
                        if !warnings.is_empty() {
                            println!("\n🔍 Evidence-Based Validation Check:");
                            for warning in warnings {
                                println!("{}", warning);
                            }
                            println!("\n💡 See: ./engram/prompts/_evidence-based-validation-template.yaml");
                        }

                        return Ok(());
                    }
                }
            }
        }

        println!("Prompt not found: {}", name);
        println!("Searched in: {:?}", prompts_path);
    }

    Ok(())
}

/// Validate all prompts for evidence-based validation requirements
pub fn validate_prompts(
    category: Option<&str>,
    fix: bool,
    root: Option<PathBuf>,
) -> Result<(), std::io::Error> {
    let prompts_path = root.unwrap_or_else(get_prompts_path);

    if !prompts_path.exists() {
        println!("Prompts directory not found at: {:?}", prompts_path);
        return Ok(());
    }

    println!("🔍 Validating evidence-based validation requirements...\n");

    let entries = fs::read_dir(&prompts_path)?;
    let mut total_files = 0;
    let mut valid_files = 0;
    let mut warnings_found = 0;

    for entry in entries.flatten() {
        if entry.path().is_dir() {
            let dir_name = entry.file_name().to_string_lossy().into_owned();

            if let Some(cat) = category {
                if dir_name.to_lowercase() != cat.to_lowercase() {
                    continue;
                }
            }

            println!("📁 Validating category: {}", dir_name);

            let subentries = fs::read_dir(&entry.path())?;
            for subentry in subentries.flatten() {
                if subentry.path().is_file() {
                    let file_name = subentry.file_name().to_string_lossy().into_owned();
                    if file_name.ends_with(".yaml") || file_name.ends_with(".md") {
                        total_files += 1;

                        let content = match fs::read_to_string(&subentry.path()) {
                            Ok(c) => c,
                            Err(_) => {
                                println!("  ❌ Failed to read: {}", file_name);
                                continue;
                            }
                        };

                        let warnings = validate_prompt_evidence_requirements(
                            &content,
                            &format!("{}/{}", dir_name, file_name),
                        );

                        if warnings.is_empty() {
                            valid_files += 1;
                            println!("  ✅ {}", file_name);
                        } else {
                            warnings_found += warnings.len();
                            println!("  ⚠️  {} ({} issues)", file_name, warnings.len());
                            for warning in warnings {
                                println!("    {}", warning);
                            }

                            if fix {
                                println!("  💡 Auto-fix not yet implemented for: {}", file_name);
                            }
                        }
                    }
                }
            }
            println!();
        }
    }

    println!("📊 Validation Summary:");
    println!("  Total files: {}", total_files);
    println!("  Valid files: {}", valid_files);
    println!("  Warnings found: {}", warnings_found);

    if warnings_found > 0 {
        println!("\n🛠️  To fix issues:");
        println!("  1. Review prompts that failed validation");
        println!("  2. Add evidence-based validation requirements");
        println!("  3. Reference: ./engram/prompts/_evidence-based-validation-template.yaml");
        println!("  4. Use: engram prompts validate --fix (when implemented)");
    } else {
        println!("\n🎉 All prompts meet evidence-based validation requirements!");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn test_prompts_commands_variants() {
        let _ = PromptsCommands::List {
            category: None,
            format: "short".to_string(),
            verbose: false,
        };
        let _ = PromptsCommands::Show {
            name: "test".to_string(),
        };
    }

    #[test]
    fn test_get_prompts_path_default() {
        // We can't easily test the env var logic in parallel tests without side effects,
        // but we can verify it returns a path
        let path = get_prompts_path();
        assert!(path.to_string_lossy().len() > 0);
    }

    #[test]
    fn test_list_prompts_empty() {
        let temp_dir = TempDir::new().unwrap();
        // Now we can properly test with a custom root
        let result = list_prompts(None, "short", Some(temp_dir.path().to_path_buf()), false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_show_prompt_not_found() {
        let temp_dir = TempDir::new().unwrap();
        // Should not panic or error, just prints "Prompt not found"
        let result = show_prompt("nonexistent", Some(temp_dir.path().to_path_buf()));
        assert!(result.is_ok());
    }

    #[test]
    fn test_list_prompts_with_files() {
        let temp_dir = TempDir::new().unwrap();
        let cat_dir = temp_dir.path().join("category1");
        fs::create_dir(&cat_dir).unwrap();

        let file_path = cat_dir.join("prompt1.txt");
        let mut file = File::create(file_path).unwrap();
        writeln!(file, "content").unwrap();

        // Capture stdout would be ideal but for now we just check no error
        let result = list_prompts(None, "short", Some(temp_dir.path().to_path_buf()), false);
        assert!(result.is_ok());

        // Test filtering
        let result_cat = list_prompts(
            Some("category1"),
            "short",
            Some(temp_dir.path().to_path_buf()),
            false,
        );
        assert!(result_cat.is_ok());
    }
}
