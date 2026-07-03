use std::path::Path;

/// Build the full Forge installer URL for a version string.
pub fn installer_url(version: &str) -> String {
    format!(
        "https://maven.minecraftforge.net/net/minecraftforge/forge/{version}/forge-{version}-installer.jar"
    )
}

/// Download a URL to a local file path.
pub async fn download(url: &str, dest: &Path) -> Result<(), String> {
    let response = reqwest::get(url)
        .await
        .map_err(|e| format!("Failed to start download: {e}"))?;

    let status = response.status();
    if !status.is_success() {
        return Err(format!("Download failed with HTTP {status}"));
    }

    let bytes = response
        .bytes()
        .await
        .map_err(|e| format!("Failed to read response body: {e}"))?;

    tokio::fs::write(dest, &bytes)
        .await
        .map_err(|e| format!("Failed to write file: {e}"))?;

    Ok(())
}

/// Run `java -jar <installer> --installServer` inside the server directory.
/// `installer_filename` is just the file name (not a path) because
/// `current_dir` is already set to the server directory.
pub async fn run_installer(server_dir: &Path, installer_filename: &str) -> Result<(), String> {
    let status = tokio::process::Command::new("java")
        .args(["-jar", installer_filename, "--installServer"])
        .current_dir(server_dir)
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .status()
        .await
        .map_err(|e| format!("Failed to spawn java: {e}"))?;

    if !status.success() {
        return Err("Forge installer exited with a non-zero status".into());
    }

    Ok(())
}
