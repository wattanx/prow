use anyhow::Result;
use tokio::process::Command;

#[derive(serde::Deserialize)]
struct  Release {
    tag_name: String,
    assets: Vec<Asset>,
}

#[derive(serde::Deserialize)]
struct Asset {
    name: String,
    url: String
}

const REPO: &str = "wattanx/prow";

async fn get_latest_release() -> Result<Release> {
    let output = Command::new("gh")
        .args(["api", &format!("repos/{REPO}/releases/latest")])
        .output()
        .await?;
    let stdout = String::from_utf8(output.stdout)?;
    let release: Release = serde_json::from_str(&stdout)?;
    Ok(release)
}



/// Self-update: check latest release, download platform binary, replace executable.
///
/// See: src/lib/updater.ts — selfUpdate()
pub async fn self_update() -> Result<()> {
    let release = get_latest_release().await?;
    let latest = release.tag_name.trim_start_matches('v');
    let current = env!("CARGO_PKG_VERSION");

    if latest == current {
        println!("Already up to date (v{current}).");
        return Ok(());
    }

    println!("Updating prow v{current} -> v{latest}...");

    let platform = get_platform();
    let asset_name = format!("prow-{platform}");
    let asset = release.assets.iter().find(|a| a.name == asset_name)
        .ok_or_else(|| anyhow::anyhow!("No binary found for {platform}"))?;

    let output = Command::new("gh")
        .args(["api", &asset.url, "-H", "Accept: application/octet-stream"])
        .output()
        .await?;

    let binary_path = std::env::current_exe()?;
    let tmp_path = binary_path.with_extension("tmp");

    std::fs::write(&tmp_path, &output.stdout)?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(&tmp_path)?.permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&tmp_path, perms)?;
    }

    std::fs::rename(&tmp_path, &binary_path)?;

    println!("Updated to v{latest}");
    Ok(())
}

/// Self-uninstall: remove the prow binary.
///
/// See: src/lib/updater.ts — selfUninstall()
pub async fn self_uninstall() -> Result<()> {
    let binary_path = std::env::current_exe()?;
    std::fs::remove_file(&binary_path)?;
    println!("Removed {}", binary_path.display());
    println!("prow has been uninstalled.");
    Ok(())
}

/// Detect current platform (e.g., "darwin-arm64", "linux-x64").
///
/// See: src/lib/updater.ts — getPlatform()
fn get_platform() -> &'static str {
    match (std::env::consts::OS, std::env::consts::ARCH) {
        ("macos", "aarch64") => "darwin-arm64",
        ("macos", "x86_64") => "darwin-x64",
        ("linux", "aarch64") => "linux-arm64",
        ("linux", "x86_64") => "linux-x64",
        _ => "unknown",
    }
}
