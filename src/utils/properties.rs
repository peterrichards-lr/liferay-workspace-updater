use anyhow::Context;
use std::fs;
use std::path::Path;

/// Simple utility to read and write java-style properties files
pub struct PropertiesEditor {
    content: String,
}

impl PropertiesEditor {
    pub fn new(path: &Path) -> anyhow::Result<Self> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read properties file: {}", path.display()))?;
        Ok(Self { content })
    }

    pub fn get(&self, key: &str) -> Option<String> {
        for line in self.content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with('#') || trimmed.is_empty() {
                continue;
            }

            if let Some((k, v)) = trimmed.split_once('=') {
                if k.trim() == key {
                    return Some(v.trim().to_string());
                }
            }
        }
        None
    }

    pub fn set(&mut self, key: &str, value: &str) {
        let mut new_lines = Vec::new();
        let mut found = false;

        for line in self.content.lines() {
            let trimmed = line.trim();
            if !trimmed.starts_with('#') && !trimmed.is_empty() {
                if let Some((k, _)) = trimmed.split_once('=') {
                    if k.trim() == key {
                        new_lines.push(format!("{}={}", key, value));
                        found = true;
                        continue;
                    }
                }
            }
            new_lines.push(line.to_string());
        }

        if !found {
            if !new_lines.is_empty() && !new_lines.last().unwrap().is_empty() {
                new_lines.push("".to_string());
            }
            new_lines.push(format!("{}={}", key, value));
        }

        self.content = new_lines.join("\n");
        if !self.content.ends_with('\n') {
            self.content.push('\n');
        }
    }

    pub fn save(&self, path: &Path) -> anyhow::Result<()> {
        fs::write(path, &self.content)
            .with_context(|| format!("Failed to write properties file: {}", path.display()))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_properties_editor() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "key1=value1").unwrap();
        writeln!(file, "# comment").unwrap();
        writeln!(file, "key2 = value2").unwrap();

        let mut editor = PropertiesEditor::new(file.path()).unwrap();
        assert_eq!(editor.get("key1"), Some("value1".to_string()));
        assert_eq!(editor.get("key2"), Some("value2".to_string()));
        assert_eq!(editor.get("key3"), None);

        editor.set("key1", "new_value1");
        editor.set("key3", "value3");

        assert_eq!(editor.get("key1"), Some("new_value1".to_string()));
        assert_eq!(editor.get("key3"), Some("value3".to_string()));
    }
}
