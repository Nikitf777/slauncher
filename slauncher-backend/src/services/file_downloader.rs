use reqwest::Client;
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DownloadError {
	#[error("failed to connect to the server")]
	ConnectionError,
	#[error("the server responded with a non-success code")]
	StatusError(Option<u16>),
	#[error("failed to read response body")]
	ResponseError,
	#[error("failed to save a file: {0}")]
	IOError(std::io::Error),
}

#[derive(Clone)]
pub struct FileDownloader {
	client: Client,
}

impl FileDownloader {
	pub fn new() -> FileDownloader {
		FileDownloader {
			client: Client::new(),
		}
	}
	pub async fn download_to_file(&self, url: &str, dest: &Path) -> Result<(), DownloadError> {
		let response = self
			.client
			.get(url)
			.send()
			.await
			.map_err(|_| DownloadError::ConnectionError)?;

		let response = response
			.error_for_status()
			.map_err(|e| DownloadError::StatusError(e.status().map(|e| e.as_u16())))?;

		let bytes = response
			.bytes()
			.await
			.map_err(|_| DownloadError::ResponseError)?;

		tokio::fs::write(dest, &bytes)
			.await
			.map_err(|e| DownloadError::IOError(e))?;

		Ok(())
	}
}
