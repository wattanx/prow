use anyhow::Result;
use std::fs;

use crate::types::{AppConfig, SectionType};

/// Load config from ~/.config/prow/config.json
/// If the file doesn't exist, return defaults.
/// Handle legacy "mine" -> "all" migration for defaultSection.
///
/// See: src/hooks/useConfig.ts
pub fn load_config() -> Result<AppConfig> {
    let home = std::env::var("HOME")?;
    let config_path = std::path::PathBuf::from(home).join(".config/prow/config.json");

    let text = match fs::read_to_string(&config_path) {
        Ok(text) => text,
        Err(_) => return Ok(AppConfig::default())
    };
    let json: serde_json::Value = serde_json::from_str(&text)?;
    let poll_interval = json["pollInterval"].as_u64().unwrap_or(60);
    let default_section = json["defaultSection"].as_str().unwrap_or("all");
    let filtered_repos = json["filteredRepos"]
        .as_array()
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(String::from)).collect()
        })
        .unwrap_or_default();

    let default_section = match  default_section {
        "mine" => "all",
        other => other
    };

    let default_section = match default_section {
        "new" => SectionType::New,
        "stale" => SectionType::Stale,
        "authored" => SectionType::Authored,
        _ => SectionType::All
    };
    Ok(AppConfig {
        poll_interval,
        default_section,
        filtered_repos,
    })
}

/// Save the filtered repos list to config.
///
/// See: src/hooks/useConfig.ts — saveFilteredRepos()
pub fn save_filtered_repos(repos: &[String]) -> Result<()> {
    let home = std::env::var("HOME")?;
    let config_path = std::path::PathBuf::from(home).join(".config/prow/config.json");

    let mut json: serde_json::Value = match fs::read_to_string(&config_path) {
        Ok(text) => serde_json::from_str(&text)?,
        Err(_) => serde_json::json!({})
    };
    
    json["filteredRepos"] = serde_json::json!(repos);
    fs::write(&config_path, serde_json::to_string_pretty(&json)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    // TODO: Test load/save with temp directories
    // TODO: Test "mine" -> "all" migration
}
