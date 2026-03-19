mod cli;
mod core;
mod utils;

use crate::cli::{App, AppCommands};
use crate::core::{LiferayProject, Workspace};
use crate::utils::network::{fetch_latest_product_version, fetch_latest_version};
use clap::Parser;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

fn main() -> anyhow::Result<()> {
    let args = App::parse();

    match args.command {
        AppCommands::Update {
            yes,
            plugin,
            product,
            path,
        } => {
            let current_dir = path.map(PathBuf::from).unwrap_or(std::env::current_dir()?);
            let project = LiferayProject { current_dir };
            let root = project.find_root()?;

            // If no flag is specified, default to plugin update
            let should_update_plugin = plugin || !product;
            let should_update_product = product;

            if should_update_plugin {
                update_plugin(&project, &root, yes)?;
            }

            if should_update_product {
                update_product(&project, &root, yes)?;
            }
        }
        AppCommands::Doctor { path, fix } => {
            let current_dir = path.map(PathBuf::from).unwrap_or(std::env::current_dir()?);
            let project = LiferayProject { current_dir };
            let root = project.find_root()?;

            println!("Running Liferay Workspace Doctor...");
            println!("Project root: {}\n", root.display());

            if fix {
                println!("Auto-fix enabled. Applying improvements...\n");
            }

            // 1. Workspace Plugin
            match project.get_workspace_plugin_version(&root) {
                Ok(v) => {
                    let remote_v = fetch_latest_version()?;
                    if v == remote_v {
                        println!("  [OK]   Workspace Plugin: {} (up to date)", v);
                    } else {
                        println!(
                            "  [WARN] Workspace Plugin: {} (outdated, latest is {})",
                            v, remote_v
                        );
                        if fix && v != "latest.release" {
                            match project.update_workspace_plugin_version(&root, &remote_v) {
                                Ok(_) => println!("         [FIXED] Updated to {}", remote_v),
                                Err(e) => {
                                    println!("         [ERROR] Failed to update plugin: {}", e)
                                }
                            }
                        }
                    }
                }
                Err(e) => println!("  [FAIL] Workspace Plugin: {}", e),
            }

            // 2. Product Version
            match project.get_product_version(&root) {
                Ok(v) => {
                    let product_type = project.detect_product_type(&root)?;
                    let remote_v = fetch_latest_product_version(product_type)?;
                    if v == remote_v {
                        println!("  [OK]   Product ({}): {} (up to date)", product_type, v);
                    } else {
                        println!(
                            "  [WARN] Product ({}): {} (outdated, latest is {})",
                            product_type, v, remote_v
                        );
                        if fix {
                            match project.update_product_version(&root, &remote_v) {
                                Ok(_) => println!("         [FIXED] Updated to {}", remote_v),
                                Err(e) => {
                                    println!("         [ERROR] Failed to update product: {}", e)
                                }
                            }
                        }
                    }
                }
                Err(e) => println!("  [FAIL] Product version: {}", e),
            }

            // 3. Gradle Wrapper
            match project.get_gradle_wrapper_version(&root) {
                Ok(v) => {
                    let major: u32 = v.split('.').next().unwrap_or("0").parse().unwrap_or(0);
                    if major >= 8 {
                        println!("  [OK]   Gradle Wrapper: {}", v);
                    } else {
                        println!(
                            "  [WARN] Gradle Wrapper: {} (Liferay 7.4 recommends Gradle 8.5+)",
                            v
                        );
                        // Automating Gradle Wrapper update is complex (needs shell commands), skipping --fix for now
                    }
                }
                Err(e) => println!("  [WARN] Gradle Wrapper: {}", e),
            }

            // 4. Infrastructure properties
            match project.get_infrastructure_properties(&root) {
                Ok(props) => {
                    let mut has_timeout = false;
                    let mut source_compat = "Unknown".to_string();

                    for (k, v) in props {
                        if k == "liferay.workspace.bundle.download.read.timeout" {
                            has_timeout = true;
                        } else if k == "sourceCompatibility" {
                            source_compat = v;
                        }
                    }

                    if has_timeout {
                        println!("  [OK]   Download timeout configured");
                    } else {
                        println!("  [WARN] Download timeout not configured (recommend adding liferay.workspace.bundle.download.read.timeout=120000)");
                        if fix {
                            match project.set_gradle_property(
                                &root,
                                "liferay.workspace.bundle.download.read.timeout",
                                "120000",
                            ) {
                                Ok(_) => println!("         [FIXED] Added liferay.workspace.bundle.download.read.timeout=120000"),
                                Err(e) => println!("         [ERROR] Failed to add timeout: {}", e),
                            }
                        }
                    }

                    if source_compat == "11" || source_compat == "17" || source_compat == "21" {
                        println!("  [OK]   Java sourceCompatibility: {}", source_compat);
                    } else {
                        if source_compat == "Unknown" {
                            println!("  [WARN] Java sourceCompatibility not found in gradle.properties (recommend setting to 11, 17, or 21)");
                        } else {
                            println!("  [WARN] Java sourceCompatibility: {} (Liferay 7.4 supports 11, 17, and 21)", source_compat);
                        }

                        if fix {
                            // Default to 17 if missing or unusual
                            match project.set_gradle_property(&root, "sourceCompatibility", "17") {
                                Ok(_) => println!("         [FIXED] Set sourceCompatibility=17"),
                                Err(e) => {
                                    println!(
                                        "         [ERROR] Failed to set sourceCompatibility: {}",
                                        e
                                    )
                                }
                            }
                        }
                    }
                }
                Err(e) => println!("  [FAIL] Infrastructure properties: {}", e),
            }

            println!("\nDoctor check complete.");
        }

        AppCommands::Version {
            remote,
            local,
            path,
        } => {
            let current_dir = path.map(PathBuf::from).unwrap_or(std::env::current_dir()?);
            let project = LiferayProject { current_dir };
            let root = project.find_root()?;

            if remote && local {
                let l_v = project.get_workspace_plugin_version(&root)?;
                let r_v = fetch_latest_version()?;
                println!("local: {}, remote: {}", l_v, r_v);
            } else if remote {
                let r_v = fetch_latest_version()?;
                println!("{}", r_v);
            } else if local {
                let l_v = project.get_workspace_plugin_version(&root)?;
                println!("{}", l_v);
            } else {
                let l_v = project
                    .get_workspace_plugin_version(&root)
                    .unwrap_or_else(|_| "Unknown".to_string());
                let r_v = fetch_latest_version().unwrap_or_else(|_| "Unknown".to_string());

                println!("Liferay Workspace Updater (lwu):");
                println!("  Version: {}", env!("CARGO_PKG_VERSION"));
                println!("\nLiferay Workspace Plugin Version:");
                println!("  Local:  {}", l_v);
                println!("  Remote: {}", r_v);

                if let Ok(p_v) = project.get_product_version(&root) {
                    println!("\nLiferay Product Version:");
                    println!("  Local:  {}", p_v);
                    if let Ok(p_t) = project.detect_product_type(&root) {
                        let r_p_v = fetch_latest_product_version(p_t)
                            .unwrap_or_else(|_| "Unknown".to_string());
                        println!("  Remote: {}", r_p_v);
                    }
                }
            }
        }
    }

    Ok(())
}

fn update_plugin(project: &LiferayProject, root: &Path, yes: bool) -> anyhow::Result<()> {
    let local_version = project.get_workspace_plugin_version(root)?;
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

        if confirm_proceed(yes)? {
            project.update_workspace_plugin_version(root, &remote_version)?;
            println!(
                "Successfully updated to version {} in settings.gradle",
                remote_version
            );
        } else {
            println!("Update cancelled.");
        }
    }
    Ok(())
}

fn update_product(project: &LiferayProject, root: &Path, yes: bool) -> anyhow::Result<()> {
    let local_version = project.get_product_version(root)?;
    let product_type = project.detect_product_type(root)?;
    let remote_version = fetch_latest_product_version(product_type)?;

    if local_version == remote_version {
        println!(
            "The Liferay Product version ({}) is already up to date ({}).",
            product_type, local_version
        );
    } else {
        println!(
            "A new version of Liferay {} ({}) is available!",
            product_type, remote_version
        );
        println!("Local version:  {}", local_version);
        println!("Latest version: {}", remote_version);

        if confirm_proceed(yes)? {
            project.update_product_version(root, &remote_version)?;
            println!(
                "Successfully updated to version {} in gradle.properties",
                remote_version
            );
        } else {
            println!("Update cancelled.");
        }
    }
    Ok(())
}

fn confirm_proceed(yes: bool) -> anyhow::Result<bool> {
    if yes {
        return Ok(true);
    }

    print!("\nDo you wish to proceed with the update? [y/N]: ");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_lowercase() == "y")
}
