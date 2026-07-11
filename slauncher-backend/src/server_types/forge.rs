use crate::server_types::{SERVERS_DIR, Server};
use anyhow::bail;
use async_trait::async_trait;
use std::path::{Path, PathBuf};
use tokio::process::Command;

pub struct ForgeServer {
	dir: PathBuf,
	launch_command: Command,
}

impl ForgeServer {
	/// Run `java -jar <installer> --installServer` inside the server directory.
	/// `installer_filename` is just the file name (not a path) because
	/// `current_dir` is already set to the server directory.
	pub async fn run_installer(&self, installer_filename: &str) -> anyhow::Result<()> {
		let status = tokio::process::Command::new("java")
			.args(["-jar", installer_filename, "--installServer"])
			.current_dir(self.dir())
			.status()
			.await?;

		if !status.success() {
			bail!("Forge installer exited with a non-zero status");
		}

		Ok(())
	}
}

#[async_trait]
impl Server for ForgeServer {
	fn new(name: &str) -> Self {
		let dir = PathBuf::from(SERVERS_DIR).join(name);
		let mut launch_command = Command::new("java");
		launch_command.current_dir(dir.clone());
		Self {
			dir,
			launch_command,
		}
	}

	fn dir(&self) -> &Path {
		self.dir.iter().as_path()
	}

	fn launch_command(&mut self) -> &mut Command {
		&mut self.launch_command
	}

	async fn prepare_files(
		&mut self,
		downloaded_filename: &str,
		_minecraft_version: &str,
		_server_version: &str,
	) -> anyhow::Result<()> {
		self.run_installer(downloaded_filename).await?;
		Ok(())
	}

	fn setup_launch_command(
		&mut self,
		_downloaded_filename: &str,
		minecraft_version: &str,
		server_version: &str,
	) {
		self.launch_command.current_dir(&self.dir).args([
			format!(
				"@libraries/net/minecraftforge/forge/{minecraft_version}-{server_version}/unix_args.txt"
			),
			"nogui".to_string(),
		]);
	}

	fn build_download_url(minecraft_version: &str, server_version: &str) -> String {
		let version = format!("{minecraft_version}-{server_version}");
		format!(
			"https://maven.minecraftforge.net/net/minecraftforge/forge/{version}/forge-{version}-installer.jar"
		)
	}
}
