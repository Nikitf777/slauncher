use serde::{Deserialize, Serialize};

use crate::entities::server::ServerType;

#[derive(Deserialize)]
pub struct CreateServerRequest {
    /// Display name – must match the folder name (alphanumeric + hyphens).
    pub name: String,
    /// Mod loader / server type.
    pub server_type: ServerType,
    /// Minecraft version, e.g. "1.20.1".
    pub minecraft_version: String,
    /// Loader / mod-loader version, e.g. "47.2.0" (Forge) or "0.19.3" (Fabric).
    pub loader_version: String,
}

#[derive(Serialize)]
pub struct ServerResponse {
    pub id: i32,
    pub name: String,
    pub server_type: ServerType,
    pub minecraft_version: String,
    pub loader_version: String,
}
