use anyhow::Context;
use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};

pub trait Workspace {
    /// Finds the root of the Liferay project (DXP, LXC, or Client Extension)
    fn find_root(&self) -> anyhow::Result<PathBuf>;

    /// Returns the workspace plugin version if detectable from settings.gradle
    fn get_workspace_plugin_version(&self, root: &Path) -> anyhow::Result<String>;

    /// Updates the workspace plugin version in settings.gradle
    fn update_workspace_plugin_version(&self, root: &Path, new_version: &str)
        -> anyhow::Result<()>;
}

pub struct LiferayProject {
    pub current_dir: PathBuf,
}

impl Workspace for LiferayProject {
    fn find_root(&self) -> anyhow::Result<PathBuf> {
        let mut path = self.current_dir.clone();
        loop {
            // Liferay Workspace (Traditional) - look for settings.gradle
            if path.join("settings.gradle").exists() {
                return Ok(path);
            }

            if !path.pop() {
                break;
            }
        }
        anyhow::bail!("Liferay project root (settings.gradle) not found.")
    }

    fn get_workspace_plugin_version(&self, root: &Path) -> anyhow::Result<String> {
        let settings_gradle = root.join("settings.gradle");
        if !settings_gradle.exists() {
            anyhow::bail!("settings.gradle not found in {}", root.display());
        }

        let content = fs::read_to_string(&settings_gradle)?;

        // Regex to find: version: "16.0.1" or version: '16.0.1' inside the workspace plugin context
        let re = Regex::new(r#"version:\s*["']([^"']+)["']"#)?;

        // We look for the one near com.liferay.gradle.plugins.workspace
        if let Some(caps) = re.captures(&content) {
            return Ok(caps[1].to_string());
        }

        anyhow::bail!("Could not find workspace plugin version in settings.gradle")
    }

    fn update_workspace_plugin_version(
        &self,
        root: &Path,
        new_version: &str,
    ) -> anyhow::Result<()> {
        let settings_gradle = root.join("settings.gradle");
        let content = fs::read_to_string(&settings_gradle)?;

        // Regex for name: "com.liferay.gradle.plugins.workspace", version: "16.0.1"
        // or name: 'com.liferay.gradle.plugins.workspace', version: '16.0.1'
        let re = Regex::new(
            r#"(name:\s*["']com\.liferay\.gradle\.plugins\.workspace["'],\s*version:\s*)(["'])([^"']+)(["'])"#,
        )?;

        if !re.is_match(&content) {
            anyhow::bail!("Could not find workspace plugin definition in settings.gradle");
        }

        let new_content = re.replace(&content, |caps: &regex::Captures| {
            format!("{}{}{}{}", &caps[1], &caps[2], new_version, &caps[4])
        });

        fs::write(&settings_gradle, new_content.to_string())
            .context("Failed to write updated settings.gradle")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_find_root() {
        let dir = tempdir().unwrap();
        let root = dir.path();
        fs::write(root.join("settings.gradle"), "").unwrap();

        let sub_dir = root.join("modules").join("my-module");
        fs::create_dir_all(&sub_dir).unwrap();

        let project = LiferayProject {
            current_dir: sub_dir,
        };
        let found = project.find_root().unwrap();
        assert_eq!(found, root);
    }

    #[test]
    fn test_get_workspace_plugin_version() {
        let dir = tempdir().unwrap();
        let root = dir.path();
        let content = r#"
buildscript {
	dependencies {
		classpath group: "com.liferay", name: "com.liferay.gradle.plugins.workspace", version: "16.0.1"
	}
}
"#;
        fs::write(root.join("settings.gradle"), content).unwrap();

        let project = LiferayProject {
            current_dir: root.to_path_buf(),
        };
        let version = project.get_workspace_plugin_version(root).unwrap();
        assert_eq!(version, "16.0.1");
    }

    #[test]
    fn test_update_workspace_plugin_version() {
        let dir = tempdir().unwrap();
        let root = dir.path();
        let content = r#"
buildscript {
	dependencies {
		classpath group: "com.liferay", name: "com.liferay.gradle.plugins.workspace", version: "16.0.1"
	}
}
"#;
        fs::write(root.join("settings.gradle"), content).unwrap();

        let project = LiferayProject {
            current_dir: root.to_path_buf(),
        };
        project
            .update_workspace_plugin_version(root, "17.0.0")
            .unwrap();

        let updated_content = fs::read_to_string(root.join("settings.gradle")).unwrap();
        assert!(updated_content.contains(r#"version: "17.0.0""#));
    }
}
