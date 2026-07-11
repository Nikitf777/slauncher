pub mod fabric;
pub mod forge;
use crate::{entities::server::Model as ServerModel, services::FileDownloader};
use async_trait::async_trait;
use std::path::Path;
use tokio::process::Command;

const SERVERS_DIR: &str = "./servers";
const SERVER_FILENAME: &str = "server";

#[async_trait]
pub trait Server {
	fn new(name: &str) -> Self
	where
		Self: Sized;

	fn from_existing(model: &ServerModel) -> Self
	where
		Self: Sized,
	{
		let mut server = Self::new(&model.name);
		server.setup_launch_command(
			SERVER_FILENAME,
			&model.minecraft_version,
			&model.server_version,
		);
		server
	}

	fn dir(&self) -> &Path;

	fn launch_command(&mut self) -> &mut Command;

	async fn prepare_files(
		&mut self,
		_downloaded_filename: &str,
		_minecraft_version: &str,
		_server_version: &str,
	) -> anyhow::Result<()> {
		Ok(())
	}

	fn setup_launch_command(
		&mut self,
		downloaded_filename: &str,
		minecraft_version: &str,
		server_version: &str,
	);

	async fn first_run(&mut self) {
		let _ = self.run().await;
	}

	async fn install(
		&mut self,
		minecraft_version: &str,
		server_version: &str,
		downloader: &FileDownloader,
	) -> anyhow::Result<()>
	where
		Self: Sized,
	{
		let url = Self::build_download_url(minecraft_version, server_version);
		downloader
			.download_to_file(&url, &self.dir().join(SERVER_FILENAME))
			.await?;

		self.prepare_files(SERVER_FILENAME, minecraft_version, server_version)
			.await?;
		self.setup_launch_command(SERVER_FILENAME, minecraft_version, server_version);
		self.first_run().await;

		Ok(())
	}

	fn build_download_url(minecraft_version: &str, server_version: &str) -> String
	where
		Self: Sized;

	async fn run(&mut self) -> anyhow::Result<()> {
		self.launch_command().status().await.map(|_| ())?;
		Ok(())
	}
}
