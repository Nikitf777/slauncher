use serde::{Deserialize, Serialize};

use crate::entities::server::ServerType;

#[derive(Deserialize)]
pub struct CreateServerRequest {
    /// Display name – must match the folder name (alphanumeric + hyphens).
    pub name: String,
    /// Mod loader / server type.
    pub server_type: ServerType,
    /// Version string, e.g. "1.20.1-47.2.0" (only relevant for Forge etc.).
    pub version: String,
}

#[derive(Serialize)]
pub struct ServerResponse {
    pub id: i32,
    pub name: String,
    pub server_type: ServerType,
    pub version: String,
}
