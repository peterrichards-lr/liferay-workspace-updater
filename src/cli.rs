use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "lwu")]
#[command(
    about = "Liferay Workspace Updater",
    long_about = "A CLI tool to automatically update Liferay Workspace components to their latest versions."
)]
pub struct App {
    #[command(subcommand)]
    pub command: AppCommands,
}

#[derive(Subcommand)]
pub enum AppCommands {
    /// Check for updates and apply them
    Update {
        /// Apply updates without prompting for confirmation
        #[arg(short, long)]
        yes: bool,

        /// Update the Liferay Workspace Gradle plugin (default)
        #[arg(short, long)]
        plugin: bool,

        /// Update the liferay.workspace.product in gradle.properties
        #[arg(long)]
        product: bool,

        /// Specify the path to the Liferay workspace (defaults to current directory)
        #[arg(short, long)]
        path: Option<String>,
    },
    /// Run health checks and recommend improvements
    Doctor {
        /// Specify the path to the Liferay workspace (defaults to current directory)
        #[arg(short, long)]
        path: Option<String>,

        /// Automatically fix issues where possible
        #[arg(long)]
        fix: bool,
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
