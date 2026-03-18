use crate::utils::network::ProductType;
use crate::utils::properties::PropertiesEditor;
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

    /// Returns the product version from gradle.properties
    fn get_product_version(&self, root: &Path) -> anyhow::Result<String>;

    /// Updates the product version in gradle.properties
    fn update_product_version(&self, root: &Path, new_version: &str) -> anyhow::Result<()>;

    /// Detects the product type based on the current product version
    fn detect_product_type(&self, root: &Path) -> anyhow::Result<ProductType>;

    /// Gets the Gradle Wrapper version
    fn get_gradle_wrapper_version(&self, root: &Path) -> anyhow::Result<String>;

    /// Gets infrastructure properties from gradle.properties
    fn get_infrastructure_properties(&self, root: &Path) -> anyhow::Result<Vec<(String, String)>>;

    /// Updates or adds a property in gradle.properties
    fn set_gradle_property(&self, root: &Path, key: &str, value: &str) -> anyhow::Result<()>;
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

        // Regex to find: version: "16.0.1" or version: '16.0.1' or version: "latest.release" inside the workspace plugin context
        let re = Regex::new(r#"version:\s*["']([^"']+)["']"#)?;

        // We look for the one near com.liferay.gradle.plugins.workspace
        if let Some(caps) = re.captures(&content) {
            let version = caps[1].to_string();
            if version == "latest.release" {
                return Ok("latest.release".to_string());
            }
            return Ok(version);
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

    fn get_product_version(&self, root: &Path) -> anyhow::Result<String> {
        let gradle_properties = root.join("gradle.properties");
        if !gradle_properties.exists() {
            anyhow::bail!("gradle.properties not found in {}", root.display());
        }

        let editor = PropertiesEditor::new(&gradle_properties)?;
        editor
            .get("liferay.workspace.product")
            .context("liferay.workspace.product property not found in gradle.properties")
    }

    fn update_product_version(&self, root: &Path, new_version: &str) -> anyhow::Result<()> {
        self.set_gradle_property(root, "liferay.workspace.product", new_version)
    }

    fn detect_product_type(&self, root: &Path) -> anyhow::Result<ProductType> {
        let version = self.get_product_version(root)?;
        if version.starts_with("portal") {
            Ok(ProductType::Portal)
        } else if version.starts_with("dxp") {
            Ok(ProductType::Dxp)
        } else {
            anyhow::bail!("Could not detect product type from version: {}", version)
        }
    }

    fn get_gradle_wrapper_version(&self, root: &Path) -> anyhow::Result<String> {
        let wrapper_properties = root.join("gradle/wrapper/gradle-wrapper.properties");
        if !wrapper_properties.exists() {
            anyhow::bail!(
                "gradle-wrapper.properties not found in {}",
                wrapper_properties.display()
            );
        }

        let editor = PropertiesEditor::new(&wrapper_properties)?;
        let url = editor
            .get("distributionUrl")
            .context("distributionUrl not found in gradle-wrapper.properties")?;

        // Extract version from: https\://services.gradle.org/distributions/gradle-8.5-bin.zip
        let re = Regex::new(r"gradle-([0-9.]+)-(bin|all)\.zip")?;
        if let Some(caps) = re.captures(&url) {
            return Ok(caps[1].to_string());
        }

        anyhow::bail!(
            "Could not parse Gradle version from distributionUrl: {}",
            url
        )
    }

    fn get_infrastructure_properties(&self, root: &Path) -> anyhow::Result<Vec<(String, String)>> {
        let gradle_properties = root.join("gradle.properties");
        if !gradle_properties.exists() {
            return Ok(Vec::new());
        }

        let editor = PropertiesEditor::new(&gradle_properties)?;
        let mut props = Vec::new();

        if let Some(v) = editor.get("liferay.workspace.bundle.download.read.timeout") {
            props.push((
                "liferay.workspace.bundle.download.read.timeout".to_string(),
                v,
            ));
        }

        if let Some(v) = editor.get("sourceCompatibility") {
            props.push(("sourceCompatibility".to_string(), v));
        }

        if let Some(v) = editor.get("targetCompatibility") {
            props.push(("targetCompatibility".to_string(), v));
        }

        Ok(props)
    }

    fn set_gradle_property(&self, root: &Path, key: &str, value: &str) -> anyhow::Result<()> {
        let gradle_properties = root.join("gradle.properties");
        // Create the file if it doesn't exist
        if !gradle_properties.exists() {
            fs::write(&gradle_properties, "")?;
        }

        let mut editor = PropertiesEditor::new(&gradle_properties)?;
        editor.set(key, value);
        editor.save(&gradle_properties)?;
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

    #[test]
    fn test_product_version() {
        let dir = tempdir().unwrap();
        let root = dir.path();
        fs::write(
            root.join("gradle.properties"),
            "liferay.workspace.product=portal-7.4-ga100",
        )
        .unwrap();

        let project = LiferayProject {
            current_dir: root.to_path_buf(),
        };

        assert_eq!(
            project.get_product_version(root).unwrap(),
            "portal-7.4-ga100"
        );
        assert_eq!(
            project.detect_product_type(root).unwrap(),
            ProductType::Portal
        );

        project
            .update_product_version(root, "dxp-2024.q1.0")
            .unwrap();
        assert_eq!(project.get_product_version(root).unwrap(), "dxp-2024.q1.0");
        assert_eq!(project.detect_product_type(root).unwrap(), ProductType::Dxp);
    }

    #[test]
    fn test_gradle_wrapper_version() {
        let dir = tempdir().unwrap();
        let root = dir.path();
        let wrapper_dir = root.join("gradle").join("wrapper");
        fs::create_dir_all(&wrapper_dir).unwrap();

        let content =
            "distributionUrl=https\\://services.gradle.org/distributions/gradle-8.5-bin.zip";
        fs::write(wrapper_dir.join("gradle-wrapper.properties"), content).unwrap();

        let project = LiferayProject {
            current_dir: root.to_path_buf(),
        };

        assert_eq!(project.get_gradle_wrapper_version(root).unwrap(), "8.5");
    }

    #[test]
    fn test_latest_release_support() {
        let dir = tempdir().unwrap();
        let root = dir.path();
        let content = r#"version: "latest.release""#;
        fs::write(root.join("settings.gradle"), content).unwrap();

        let project = LiferayProject {
            current_dir: root.to_path_buf(),
        };

        assert_eq!(
            project.get_workspace_plugin_version(root).unwrap(),
            "latest.release"
        );
    }
}
