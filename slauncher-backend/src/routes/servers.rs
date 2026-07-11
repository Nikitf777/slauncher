use actix_web::{HttpResponse, get, post, web};
use sea_orm::DatabaseConnection;
use std::path::PathBuf;

use crate::dtos::{CreateServerRequest, ServerResponse};
use crate::entities::server::{self, ServerType};
use crate::models::ServerProperties;
use crate::repository;
use crate::server_types::Server;
use crate::server_types::{fabric::FabricServer, forge::ForgeServer};
use crate::services::FileDownloader;

/// Base directory where all server folders live.
const SERVERS_DIR: &str = "./servers";

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Convert a generic `DbErr` into a 500 response.
fn db_error(e: sea_orm::DbErr) -> HttpResponse {
	HttpResponse::InternalServerError().body(format!("Database error: {e}"))
}

/// Look up a server by name, returning the model or an appropriate HTTP error response.
async fn find_server_by_name(
	name: &str,
	db: &DatabaseConnection,
) -> Result<server::Model, HttpResponse> {
	match repository::find_by_name(db, name).await {
		Ok(Some(model)) => Ok(model),
		Ok(None) => Err(HttpResponse::NotFound().body("Server not found")),
		Err(e) => Err(db_error(e)),
	}
}

/// Look up a server by id, returning the model or an appropriate HTTP error response.
async fn find_server_by_id(
	id: i32,
	db: &DatabaseConnection,
) -> Result<server::Model, HttpResponse> {
	match repository::find_by_id(db, id).await {
		Ok(Some(model)) => Ok(model),
		Ok(None) => Err(HttpResponse::NotFound().body("Server not found")),
		Err(e) => Err(db_error(e)),
	}
}

/// Shared implementation: write eula.txt for a server whose Model is already loaded.
async fn write_eula(server: &server::Model) -> HttpResponse {
	let eula_path = PathBuf::from(SERVERS_DIR)
		.join(server.id.to_string())
		.join("eula.txt");
	match tokio::fs::write(&eula_path, "eula=true\n").await {
		Ok(_) => {
			log::info!("Accepted EULA for server '{}'", server.name);
			HttpResponse::Ok().body("EULA accepted")
		}
		Err(e) => {
			HttpResponse::InternalServerError().body(format!("Failed to write eula.txt: {e}"))
		}
	}
}

/// Install a server according to its type.  On failure returns an `HttpResponse`
/// that the caller should send back (the caller is responsible for cleanup).
async fn install_server(
	downloader: &FileDownloader,
	req: &CreateServerRequest,
	server_id: i32,
) -> Result<(), HttpResponse> {
	match req.server_type {
		ServerType::Forge => install_server_type::<ForgeServer>(downloader, req, server_id).await,
		ServerType::Fabric => install_server_type::<FabricServer>(downloader, req, server_id).await,
		_ => Err(HttpResponse::BadRequest()
			.body(format!("Unsupported server type: {:?}", req.server_type))),
	}
}

async fn install_server_type<S: Server + Send>(
	downloader: &FileDownloader,
	req: &CreateServerRequest,
	server_id: i32,
) -> Result<(), HttpResponse> {
	S::new(server_id)
		.install(&req.minecraft_version, &req.server_version, downloader)
		.await
		.map_err(|e| HttpResponse::InternalServerError().body(e.to_string()))?;
	Ok(())
}

// ---------------------------------------------------------------------------
// Route handlers
// ---------------------------------------------------------------------------

#[post("/api/servers")]
pub async fn create_server(
	body: web::Json<CreateServerRequest>,
	db: web::Data<DatabaseConnection>,
	downloader: web::Data<FileDownloader>,
) -> HttpResponse {
	match repository::exists_by_name(db.get_ref(), &body.name).await {
		Ok(true) => return HttpResponse::Conflict().body("A server with this name already exists"),
		Ok(false) => {}
		Err(e) => return db_error(e),
	}

	let server_type = body.server_type.clone();
	let model = match repository::create(
		db.get_ref(),
		&body.name,
		server_type,
		&body.minecraft_version,
		&body.server_version,
	)
	.await
	{
		Ok(model) => model,
		Err(e) => {
			log::error!("Failed to insert server into DB: {e}");
			return db_error(e);
		}
	};

	let server_dir = PathBuf::from(SERVERS_DIR).join(model.id.to_string());
	if let Err(e) = tokio::fs::create_dir_all(&server_dir).await {
		let _ = repository::delete(db.get_ref(), model.id).await;
		return HttpResponse::InternalServerError()
			.body(format!("Failed to create directory: {e}"));
	}

	if let Err(e) = install_server(downloader.get_ref(), &body, model.id).await {
		let _ = tokio::fs::remove_dir_all(&server_dir).await;
		let _ = repository::delete(db.get_ref(), model.id).await;
		return e;
	}

	log::info!("Created server '{}' (id={})", model.name, model.id);
	HttpResponse::Created().json(ServerResponse {
		id: model.id,
		name: model.name,
		server_type: model.server_type,
		minecraft_version: model.minecraft_version,
		server_version: model.server_version,
	})
}

#[post("/api/servers/by-name/{name}/eula")]
pub async fn accept_eula_by_name(
	path: web::Path<String>,
	db: web::Data<DatabaseConnection>,
) -> HttpResponse {
	let name = path.into_inner();
	let server = match find_server_by_name(&name, db.get_ref()).await {
		Ok(s) => s,
		Err(resp) => return resp,
	};
	write_eula(&server).await
}

#[post("/api/servers/by-id/{id}/eula")]
pub async fn accept_eula_by_id(
	path: web::Path<i32>,
	db: web::Data<DatabaseConnection>,
) -> HttpResponse {
	let id = path.into_inner();

	let server = match find_server_by_id(id, db.get_ref()).await {
		Ok(s) => s,
		Err(resp) => return resp,
	};

	write_eula(&server).await
}

// ---------------------------------------------------------------------------
// Configure server properties
// ---------------------------------------------------------------------------

/// Apply in-memory property updates to a `server.properties` file on disk.
///
/// Reads the existing file, overwrites values for keys present in `updates`,
/// and appends any keys that are not already in the file.
fn apply_properties(content: &str, updates: &[(&str, String)]) -> String {
	// quick lookup for keys we care about
	let update_map: std::collections::HashMap<&str, &str> =
		updates.iter().map(|(k, v)| (*k, v.as_str())).collect();

	let mut seen = std::collections::HashSet::new();
	let mut out = String::new();

	for line in content.lines() {
		let trimmed = line.trim();
		if trimmed.is_empty() || trimmed.starts_with('#') {
			out.push_str(line);
			out.push('\n');
			continue;
		}
		if let Some(eq_pos) = line.find('=') {
			let key = &line[..eq_pos];
			if let Some(new_val) = update_map.get(key) {
				seen.insert(key);
				out.push_str(key);
				out.push('=');
				out.push_str(new_val);
				out.push('\n');
			} else {
				out.push_str(line);
				out.push('\n');
			}
		} else {
			// malformed line – preserve as-is
			out.push_str(line);
			out.push('\n');
		}
	}

	// Append keys that were not found in the file
	for (key, val) in updates {
		if !seen.contains(key) {
			out.push_str(key);
			out.push('=');
			out.push_str(val);
			out.push('\n');
		}
	}

	out
}

async fn configure_properties(
	server: &server::Model,
	body: web::Json<ServerProperties>,
) -> HttpResponse {
	if let Err(e) = body.validate() {
		return HttpResponse::BadRequest().body(e);
	}

	let properties_path = PathBuf::from(SERVERS_DIR)
		.join(server.id.to_string())
		.join("server.properties");

	let content = match tokio::fs::read_to_string(&properties_path).await {
		Ok(c) => c,
		Err(e) => {
			return HttpResponse::InternalServerError()
				.body(format!("Failed to read server.properties: {e}"));
		}
	};

	let updates = body.to_updates();
	if updates.is_empty() {
		return HttpResponse::Ok().body("No properties to update");
	}

	let new_content = apply_properties(&content, &updates);

	if let Err(e) = tokio::fs::write(&properties_path, &new_content).await {
		return HttpResponse::InternalServerError()
			.body(format!("Failed to write server.properties: {e}"));
	}

	log::info!("Updated properties for server '{}'", server.name);
	HttpResponse::Ok().body("Properties updated")
}

#[post("/api/servers/by-name/{name}/properties")]
pub async fn configure_properties_by_name(
	path: web::Path<String>,
	db: web::Data<DatabaseConnection>,
	body: web::Json<ServerProperties>,
) -> HttpResponse {
	let name = path.into_inner();
	let server = match find_server_by_name(&name, db.get_ref()).await {
		Ok(s) => s,
		Err(resp) => return resp,
	};
	configure_properties(&server, body).await
}

#[post("/api/servers/by-id/{id}/properties")]
pub async fn configure_properties_by_id(
	path: web::Path<i32>,
	db: web::Data<DatabaseConnection>,
	body: web::Json<ServerProperties>,
) -> HttpResponse {
	let id = path.into_inner();

	let server = match find_server_by_id(id, db.get_ref()).await {
		Ok(s) => s,
		Err(resp) => return resp,
	};

	configure_properties(&server, body).await
}

// ---------------------------------------------------------------------------
// Get server properties
// ---------------------------------------------------------------------------

async fn read_properties(server: &server::Model) -> HttpResponse {
	let properties_path = PathBuf::from(SERVERS_DIR)
		.join(server.id.to_string())
		.join("server.properties");

	let content = match tokio::fs::read_to_string(&properties_path).await {
		Ok(c) => c,
		Err(e) => {
			return HttpResponse::InternalServerError()
				.body(format!("Failed to read server.properties: {e}"));
		}
	};

	let props = ServerProperties::from_str(&content);
	HttpResponse::Ok().json(props)
}

#[get("/api/servers/by-name/{name}/properties")]
pub async fn get_properties_by_name(
	path: web::Path<String>,
	db: web::Data<DatabaseConnection>,
) -> HttpResponse {
	let name = path.into_inner();
	let server = match find_server_by_name(&name, db.get_ref()).await {
		Ok(s) => s,
		Err(resp) => return resp,
	};
	read_properties(&server).await
}

#[get("/api/servers/by-id/{id}/properties")]
pub async fn get_properties_by_id(
	path: web::Path<i32>,
	db: web::Data<DatabaseConnection>,
) -> HttpResponse {
	let id = path.into_inner();

	let server = match find_server_by_id(id, db.get_ref()).await {
		Ok(s) => s,
		Err(resp) => return resp,
	};

	read_properties(&server).await
}
