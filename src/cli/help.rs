use clap::Subcommand;

/// Help and onboarding commands
#[derive(Subcommand)]
pub enum HelpCommands {
    /// Show onboarding information
    Onboarding,
    /// Get started guide
    GettingStarted,
    /// Show examples
    Examples,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_help_commands_variants() {
        // Test that variants exist - this is mostly a compilation check
        // but ensuring we can instantiate them confirms the API stability
        let _ = HelpCommands::Onboarding;
        let _ = HelpCommands::GettingStarted;
        let _ = HelpCommands::Examples;
    }
}
