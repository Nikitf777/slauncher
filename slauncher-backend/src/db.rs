use sea_orm::{ConnectionTrait, Database, DbErr, Schema};

use crate::entities::server;

pub async fn init() -> Result<sea_orm::DatabaseConnection, DbErr> {
	let conn_str = "sqlite://slauncher.db?mode=rwc";
	let db = Database::connect(conn_str).await?;

	// Create the servers table if it does not exist
	let backend = db.get_database_backend();
	let schema = Schema::new(backend);
	let stmt = schema.create_table_from_entity(server::Entity);
	let statement = backend.build(&stmt);
	match db.execute(statement).await {
		Ok(_) => {}
		Err(e) => {
			// SQLite returns "table already exists" on re-creation
			if !e.to_string().contains("already exists") {
				return Err(e);
			}
		}
	}

	Ok(db)
}
