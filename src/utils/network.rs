use anyhow::Context;
use roxmltree::Document;
use serde::Deserialize;

const NEXUS_METADATA_URL: &str = "https://repository-cdn.liferay.com/nexus/content/groups/public/com/liferay/com.liferay.gradle.plugins.workspace/maven-metadata.xml";
const RELEASES_JSON_URL: &str = "https://releases-cdn.liferay.com/releases.json";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProductType {
    Portal,
    Dxp,
}

impl std::fmt::Display for ProductType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProductType::Portal => write!(f, "portal"),
            ProductType::Dxp => write!(f, "dxp"),
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ReleaseEntry {
    product: String,
    release_key: String,
    promoted: String,
    // tags: Vec<String>, // Some entries have tags, some don't or it's a different type
}

/// Fetches the latest version from Liferay's Nexus metadata (for workspace plugin)
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

/// Fetches the latest product version (Portal or DXP) from Liferay's releases.json
pub fn fetch_latest_product_version(product_type: ProductType) -> anyhow::Result<String> {
    let response = reqwest::blocking::get(RELEASES_JSON_URL)
        .context("Failed to fetch releases.json from Liferay")?;

    if !response.status().is_success() {
        anyhow::bail!("Failed to fetch releases.json: HTTP {}", response.status());
    }

    let releases: Vec<ReleaseEntry> = response.json().context("Failed to parse releases.json")?;

    let product_str = product_type.to_string();

    // The releases.json is usually ordered newest first
    let latest = releases
        .iter()
        .find(|r| r.product == product_str && r.promoted == "true")
        .context(format!("Could not find a promoted {} release", product_str))?;

    Ok(latest.release_key.clone())
}
