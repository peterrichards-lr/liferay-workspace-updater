use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "lwu")]
#[command(
    about = "Liferay Workspace Updater",
    long_about = "A CLI tool to automatically update the Liferay Workspace Gradle plugin to its latest version."
)]
pub struct App {
    #[command(subcommand)]
    pub command: AppCommands,
}

#[derive(Subcommand)]
pub enum AppCommands {
    /// Check for updates and apply them to settings.gradle
    Update {
        /// Apply updates without prompting for confirmation
        #[arg(short, long)]
        yes: bool,

        /// Specify the path to the Liferay workspace (defaults to current directory)
        #[arg(short, long)]
        path: Option<String>,
    },
    /// Display version information
    Version {
        /// Display the latest version available on Nexus
        #[arg(short, long)]
        remote: bool,

        /// Display the version currently in settings.gradle
        #[arg(short, long)]
        local: bool,

        /// Specify the path to the Liferay workspace (defaults to current directory)
        #[arg(short, long)]
        path: Option<String>,
    },
}
