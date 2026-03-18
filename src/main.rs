mod cli;
mod core;
mod utils;

use crate::cli::{App, AppCommands};
use crate::core::{LiferayProject, Workspace};
use crate::utils::fetch_latest_version;
use clap::Parser;
use std::io::{self, Write};
use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    let args = App::parse();

    match args.command {
        AppCommands::Update { yes, path } => {
            let current_dir = path.map(PathBuf::from).unwrap_or(std::env::current_dir()?);
            let project = LiferayProject { current_dir };
            let root = project.find_root()?;

            let local_version = project.get_workspace_plugin_version(&root)?;
            let remote_version = fetch_latest_version()?;

            if local_version == remote_version {
                println!(
                    "The Liferay Workspace plugin is already up to date (version {}).",
                    local_version
                );
            } else {
                println!("A new version of the Liferay Workspace plugin is available!");
                println!("Local version:  {}", local_version);
                println!("Latest version: {}", remote_version);

                let proceed = if yes {
                    true
                } else {
                    print!("\nDo you wish to proceed with the update? [y/N]: ");
                    io::stdout().flush()?;
                    let mut input = String::new();
                    io::stdin().read_line(&mut input)?;
                    input.trim().to_lowercase() == "y"
                };

                if proceed {
                    project.update_workspace_plugin_version(&root, &remote_version)?;
                    println!(
                        "Successfully updated to version {} in settings.gradle",
                        remote_version
                    );
                } else {
                    println!("Update cancelled.");
                }
            }
        }
        AppCommands::Version {
            remote,
            local,
            path,
        } => {
            if remote && local {
                let current_dir = path.map(PathBuf::from).unwrap_or(std::env::current_dir()?);
                let project = LiferayProject { current_dir };
                let root = project.find_root()?;
                let l_v = project.get_workspace_plugin_version(&root)?;
                let r_v = fetch_latest_version()?;
                println!("local: {}, remote: {}", l_v, r_v);
            } else if remote {
                let r_v = fetch_latest_version()?;
                println!("{}", r_v);
            } else if local {
                let current_dir = path.map(PathBuf::from).unwrap_or(std::env::current_dir()?);
                let project = LiferayProject { current_dir };
                let root = project.find_root()?;
                let l_v = project.get_workspace_plugin_version(&root)?;
                println!("{}", l_v);
            } else {
                // Default to showing both if no flag specified
                let current_dir = path.map(PathBuf::from).unwrap_or(std::env::current_dir()?);
                let project = LiferayProject { current_dir };
                let root = project.find_root()?;
                let l_v = project
                    .get_workspace_plugin_version(&root)
                    .unwrap_or_else(|_| "Unknown".to_string());
                let r_v = fetch_latest_version().unwrap_or_else(|_| "Unknown".to_string());
                println!("Liferay Workspace Plugin Version:");
                println!("  Local:  {}", l_v);
                println!("  Remote: {}", r_v);
            }
        }
    }

    Ok(())
}
