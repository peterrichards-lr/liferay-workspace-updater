use anyhow::Context;
use roxmltree::Document;

const NEXUS_METADATA_URL: &str = "https://repository-cdn.liferay.com/nexus/content/groups/public/com/liferay/com.liferay.gradle.plugins.workspace/maven-metadata.xml";

/// Fetches the latest version from Liferay's Nexus metadata
pub fn fetch_latest_version() -> anyhow::Result<String> {
    let response = reqwest::blocking::get(NEXUS_METADATA_URL)
        .context("Failed to fetch Maven metadata from Liferay Nexus")?;

    if !response.status().is_success() {
        anyhow::bail!("Failed to fetch Maven metadata: HTTP {}", response.status());
    }

    let xml = response.text().context("Failed to read response body")?;
    let doc = Document::parse(&xml).context("Failed to parse Maven metadata XML")?;

    let version = doc
        .descendants()
        .find(|n| n.has_tag_name("release"))
        .and_then(|n| n.text())
        .map(|s| s.to_string())
        .or_else(|| {
            doc.descendants()
                .find(|n| n.has_tag_name("latest"))
                .and_then(|n| n.text())
                .map(|s| s.to_string())
        })
        .context("Could not find latest or release version in metadata")?;

    Ok(version)
}
