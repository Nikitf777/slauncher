use std::path::Path;
use std::process::Stdio;

use crate::forge;

/// Base URL for the Fabric meta API.
const FABRIC_META_BASE: &str = "https://meta.fabricmc.net/v2";

/// Fabric installer version to use (latest as of this writing).
const FABRIC_INSTALLER_VERSION: &str = "1.1.1";

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Build the download URL for a Fabric server jar given a Minecraft version
/// and a loader version.
pub fn server_jar_url(mc_version: &str, loader_version: &str) -> String {
    format!(
        "{FABRIC_META_BASE}/versions/loader/{mc_version}/{loader_version}/{FABRIC_INSTALLER_VERSION}/server/jar"
    )
}

/// Download a Fabric server jar into `server_dir`.
async fn download_server(
    server_dir: &Path,
    mc_version: &str,
    loader_version: &str,
) -> Result<String, String> {
    let url = server_jar_url(mc_version, loader_version);
    let jar_name = "fabric-server-launcher.jar";
    let dest = server_dir.join(jar_name);

    log::info!("Downloading Fabric server from {url} …");
    forge::download(&url, &dest).await?;

    Ok(jar_name.to_owned())
}

/// Run the Fabric server launcher once so it downloads its libraries and
/// initialises the server directory.  The process will exit on its own
/// because EULA hasn't been accepted yet — that is fine; libraries and
/// default files (server.properties, etc.) are generated before the
/// EULA check.
async fn run_server(server_dir: &Path, jar_name: &str) -> Result<(), String> {
    log::info!("Running Fabric server to initialise directory …");

    let status = tokio::process::Command::new("java")
        .args(["-jar", jar_name, "--nogui"])
        .current_dir(server_dir)
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .await
        .map_err(|e| format!("Failed to spawn java: {e}"))?;

    if !status.success() {
        // EULA-not-accepted is the expected non-zero exit — not an error.
        log::info!("Fabric server exited (expected: EULA not accepted)");
    }

    Ok(())
}

/// Download the Fabric server launcher and run it once to initialise the
/// server directory (downloads libraries, generates files).
pub async fn setup_server(
    server_dir: &Path,
    mc_version: &str,
    loader_version: &str,
) -> Result<(), String> {
    let jar_name = download_server(server_dir, mc_version, loader_version).await?;
    run_server(server_dir, &jar_name).await?;
    Ok(())
}
