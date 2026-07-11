use sea_orm::{
	ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, Set,
};

use crate::entities::server;

// ---------------------------------------------------------------------------
// Queries
// ---------------------------------------------------------------------------

/// Find a single server by its name.
pub async fn find_by_name(
	db: &DatabaseConnection,
	name: &str,
) -> Result<Option<server::Model>, DbErr> {
	server::Entity::find()
		.filter(server::Column::Name.eq(name))
		.one(db)
		.await
}

/// Find a single server by its primary key.
pub async fn find_by_id(db: &DatabaseConnection, id: i32) -> Result<Option<server::Model>, DbErr> {
	server::Entity::find_by_id(id).one(db).await
}

/// Check whether a server with the given name already exists.
pub async fn exists_by_name(db: &DatabaseConnection, name: &str) -> Result<bool, DbErr> {
	find_by_name(db, name).await.map(|opt| opt.is_some())
}

// ---------------------------------------------------------------------------
// Mutations
// ---------------------------------------------------------------------------

/// Insert a new server and return the persisted model (with the auto-generated id
/// populated).
pub async fn create(
	db: &DatabaseConnection,
	name: &str,
	server_type: server::ServerType,
	minecraft_version: &str,
	server_version: &str,
) -> Result<server::Model, DbErr> {
	let model = server::ActiveModel {
		name: Set(name.to_owned()),
		server_type: Set(server_type),
		minecraft_version: Set(minecraft_version.to_owned()),
		server_version: Set(server_version.to_owned()),
		..Default::default()
	};
	model.insert(db).await
}
