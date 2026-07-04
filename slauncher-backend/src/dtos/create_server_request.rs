use serde::Deserialize;

use crate::entities::server::ServerType;

#[derive(Deserialize)]
pub struct CreateServerRequest {
	pub name: String,
	pub server_type: ServerType,
	pub minecraft_version: String,
	pub loader_version: String,
}
