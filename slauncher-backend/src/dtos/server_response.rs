use serde::Serialize;

use crate::entities::server::ServerType;

#[derive(Serialize)]
pub struct ServerResponse {
    pub id: i32,
    pub name: String,
    pub server_type: ServerType,
    pub minecraft_version: String,
    pub loader_version: String,
}
