use crate::{entities::server::Model as ServerModel, server_types::Server};
use std::collections::HashMap;

pub struct ServerRunner {
	running_servers: HashMap<ServerModel, Box<dyn Server>>,
}
