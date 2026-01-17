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
