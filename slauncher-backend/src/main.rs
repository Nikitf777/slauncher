use actix_web::{App, HttpServer, web};

use crate::services::FileDownloader;

mod db;
mod dtos;
mod entities;
mod models;
mod repository;
mod routes;
mod server_types;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	env_logger::init();

	let database = db::init().await.map_err(|e| {
		eprintln!("Database initialisation failed: {e}");
		std::io::Error::new(std::io::ErrorKind::Other, e.to_string())
	})?;
	log::info!("Database initialised");

	let file_downloader = FileDownloader::new();

	HttpServer::new(move || {
		App::new()
			.app_data(web::Data::new(database.clone()))
			.app_data(web::Data::new(file_downloader.clone()))
			.service(routes::create_server)
			.service(routes::accept_eula_by_name)
			.service(routes::accept_eula_by_id)
			.service(routes::configure_properties_by_name)
			.service(routes::configure_properties_by_id)
			.service(routes::get_properties_by_name)
			.service(routes::get_properties_by_id)
	})
	.bind(("127.0.0.1", 8080))?
	.run()
	.await
}
