use async_trait::async_trait;
use std::path::{Path, PathBuf};
use tokio::process::Command;

use crate::server_types::{SERVERS_DIR, Server};

pub struct FabricServer {
	dir: PathBuf,
	launch_command: Command,
}

impl FabricServer {
	const INSTALLER_VERSION: &str = "1.1.1";
}

#[async_trait]
impl Server for FabricServer {
	fn new(id: i32) -> FabricServer {
		let dir = PathBuf::from(SERVERS_DIR).join(id.to_string());
		let mut launch_command = Command::new("java");
		launch_command.current_dir(dir.clone());
		launch_command.arg("-jar");
		FabricServer {
			dir,
			launch_command,
		}
	}

	fn dir(&self) -> &Path {
		self.dir.as_path()
	}

	fn launch_command(&mut self) -> &mut Command {
		&mut self.launch_command
	}

	fn setup_launch_command(
		&mut self,
		downloaded_filename: &str,
		_minecraft_version: &str,
		_server_version: &str,
	) {
		self.launch_command.args([downloaded_filename, "nogui"]);
	}

	fn build_download_url(minecraft_version: &str, server_version: &str) -> String {
		format!(
			"https://meta.fabricmc.net/v2/versions/loader/{minecraft_version}/{server_version}/{}/server/jar",
			Self::INSTALLER_VERSION
		)
	}
}
