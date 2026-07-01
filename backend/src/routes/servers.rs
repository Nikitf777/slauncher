use actix_web::{post, web, HttpResponse};
use sea_orm::DatabaseConnection;
use std::path::PathBuf;

use crate::entities::server::{self, ServerType};
use crate::{fabric, forge};
use crate::models::{CreateServerRequest, ServerResponse};
use crate::repository;

/// Base directory where all server folders live.
const SERVERS_DIR: &str = "./servers";

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Sanitise a server name so it can safely be used as a folder name.
fn sanitise_name(name: &str) -> Result<String, String> {
    let ok = name
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '-');
    if !ok {
        return Err(
            "Server name may only contain ASCII letters, digits, and hyphens".into(),
        );
    }
    if name.is_empty() {
        return Err("Server name cannot be empty".into());
    }
    Ok(name.to_owned())
}

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
        .join(&server.name)
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
    req: &CreateServerRequest,
    server_dir: &PathBuf,
) -> Result<(), HttpResponse> {
    match req.server_type {
        ServerType::Forge => install_forge(req, server_dir).await,
        ServerType::Fabric => install_fabric(req, server_dir).await,
        _ => Err(HttpResponse::BadRequest()
            .body(format!("Unsupported server type: {:?}", req.server_type))),
    }
}

async fn install_forge(req: &CreateServerRequest, server_dir: &PathBuf) -> Result<(), HttpResponse> {
    let combined = format!("{}-{}", req.minecraft_version, req.loader_version);
    let installer_filename = format!("forge-{}-installer.jar", combined);
    let installer_path = server_dir.join(&installer_filename);

    let url = forge::installer_url(&combined);
    log::info!("Downloading {url} …");
    forge::download(&url, &installer_path)
        .await
        .map_err(|e| HttpResponse::BadGateway().body(format!("Download failed: {e}")))?;

    log::info!("Running Forge installer …");
    forge::run_installer(server_dir, &installer_filename)
        .await
        .map_err(|e| HttpResponse::InternalServerError().body(format!("Installation failed: {e}")))?;

    // Remove the installer jar (keeps the server dir tidy)
    let _ = tokio::fs::remove_file(&installer_path).await;
    Ok(())
}

async fn install_fabric(req: &CreateServerRequest, server_dir: &PathBuf) -> Result<(), HttpResponse> {
    fabric::setup_server(server_dir, &req.minecraft_version, &req.loader_version)
        .await
        .map_err(|e| HttpResponse::BadGateway().body(e))
}

// ---------------------------------------------------------------------------
// Route handlers
// ---------------------------------------------------------------------------

#[post("/api/servers")]
pub async fn create_server(
    body: web::Json<CreateServerRequest>,
    db: web::Data<DatabaseConnection>,
) -> HttpResponse {
    // 1. Validate name
    let name = match sanitise_name(&body.name) {
        Ok(n) => n,
        Err(e) => return HttpResponse::BadRequest().body(e),
    };

    // 2. Check server does not already exist
    match repository::exists_by_name(db.get_ref(), &name).await {
        Ok(true) => return HttpResponse::Conflict().body("A server with this name already exists"),
        Ok(false) => {}
        Err(e) => return db_error(e),
    }

    // 3. Create the server directory
    let server_dir = PathBuf::from(SERVERS_DIR).join(&name);
    if let Err(e) = tokio::fs::create_dir_all(&server_dir).await {
        return HttpResponse::InternalServerError()
            .body(format!("Failed to create directory: {e}"));
    }

    // 4. Install the server according to its type
    if let Err(e) = install_server(&body, &server_dir).await {
        let _ = tokio::fs::remove_dir_all(&server_dir).await;
        return e;
    }

    // 5. Insert into database
    let server_type = body.server_type.clone();
    match repository::create(
        db.get_ref(),
        &name,
        server_type,
        &body.minecraft_version,
        &body.loader_version,
    )
    .await
    {
        Ok(model) => {
            log::info!("Created server '{}' (id={})", model.name, model.id);
            HttpResponse::Created().json(ServerResponse {
                id: model.id,
                name: model.name,
                server_type: model.server_type,
                minecraft_version: model.minecraft_version,
                loader_version: model.loader_version,
            })
        }
        Err(e) => {
            log::error!("Failed to insert server into DB: {e}");
            db_error(e)
        }
    }
}

#[post("/api/servers/by-name/{name}/eula")]
pub async fn accept_eula_by_name(
    path: web::Path<String>,
    db: web::Data<DatabaseConnection>,
) -> HttpResponse {
    let name = match sanitise_name(&path.into_inner()) {
        Ok(n) => n,
        Err(e) => return HttpResponse::BadRequest().body(e),
    };

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
