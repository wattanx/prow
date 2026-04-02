use anyhow::Result;

use crate::types::AppConfig;

/// Load config from ~/.config/prow/config.json
/// If the file doesn't exist, return defaults.
/// Handle legacy "mine" -> "all" migration for defaultSection.
///
/// See: src/hooks/useConfig.ts
pub fn load_config() -> Result<AppConfig> {
    todo!("Load config from ~/.config/prow/config.json")
}

/// Save the filtered repos list to config.
///
/// See: src/hooks/useConfig.ts — saveFilteredRepos()
pub fn save_filtered_repos(_repos: &[String]) -> Result<()> {
    todo!("Save filtered repos to config")
}

#[cfg(test)]
mod tests {
    // TODO: Test load/save with temp directories
    // TODO: Test "mine" -> "all" migration
}
