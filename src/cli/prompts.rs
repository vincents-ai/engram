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
    },
    /// Show prompt details
    Show {
        /// Prompt name or path
        #[arg(help = "Prompt name or path")]
        name: String,
    },
}

/// Get prompts path from environment or default
pub fn get_prompts_path() -> PathBuf {
    std::env::var("ENGRAM_PROMPTS_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("./engram/prompts"))
}

/// List all prompts in the prompts directory
pub fn list_prompts(category: Option<&str>, format: &str) -> Result<(), std::io::Error> {
    let prompts_path = get_prompts_path();

    if !prompts_path.exists() {
        println!("Prompts directory not found: {:?}", prompts_path);
        println!("Set ENGRAM_PROMPTS_PATH environment variable.");
        return Ok(());
    }

    let entries = fs::read_dir(&prompts_path)?;

    match format {
        "short" | "s" => {
            println!("Available Prompts:");
            println!("==================");

            for entry in entries.flatten() {
                if entry.path().is_dir() {
                    let name = entry.file_name().to_string_lossy().into_owned();

                    if let Some(cat) = category {
                        if name.to_lowercase() != cat.to_lowercase() {
                            continue;
                        }
                    }

                    // Count files in subdirectory
                    let count = fs::read_dir(&entry.path())?.flatten().count();
                    println!("  - {} ({})", name, count);
                }
            }
        }
        "full" | "f" => {
            println!("Available Prompts:");
            println!("==================");

            for entry in entries.flatten() {
                if entry.path().is_dir() {
                    let name = entry.file_name().to_string_lossy().into_owned();

                    if let Some(cat) = category {
                        if name.to_lowercase() != cat.to_lowercase() {
                            continue;
                        }
                    }

                    println!("\n[{}]", name);
                    println!("---");

                    let subentries = fs::read_dir(&entry.path())?;
                    for subentry in subentries.flatten().take(10) {
                        let sub_name = subentry.file_name().to_string_lossy().into_owned();
                        println!("  - {}", sub_name);
                    }

                    let total: usize = fs::read_dir(&entry.path())?.flatten().count();
                    if total > 10 {
                        println!("  ... and {} more", total - 10);
                    }
                }
            }
        }
        _ => {
            println!("Unknown format: {}. Use 'short' or 'full'.", format);
        }
    }

    Ok(())
}

/// Show a specific prompt
pub fn show_prompt(name: &str) -> Result<(), std::io::Error> {
    let prompts_path = get_prompts_path();

    // Try to find the prompt file
    let prompt_path = prompts_path.join(name);

    if prompt_path.exists() {
        if prompt_path.is_file() {
            let content = fs::read_to_string(&prompt_path)?;
            println!("\nPrompt: {}", name);
            println!("========");
            println!("\n{}", content);
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
