use actix_web::{web, App, HttpServer};

mod db;
mod dtos;
mod entities;
mod fabric;
mod forge;
mod models;
mod repository;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let database = db::init().await.map_err(|e| {
        eprintln!("Database initialisation failed: {e}");
        std::io::Error::new(std::io::ErrorKind::Other, e.to_string())
    })?;
    log::info!("Database initialised");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(database.clone()))
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
