use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// The type / mod-loader of a Minecraft server.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
#[serde(rename_all = "lowercase")]
pub enum ServerType {
    #[sea_orm(string_value = "vanilla")]
    Vanilla,
    #[sea_orm(string_value = "forge")]
    Forge,
    #[sea_orm(string_value = "neoforge")]
    NeoForge,
    #[sea_orm(string_value = "fabric")]
    Fabric,
    #[sea_orm(string_value = "quilt")]
    Quilt,
    #[sea_orm(string_value = "liteloader")]
    LiteLoader,
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "servers")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub server_type: ServerType,
    pub minecraft_version: String,
    pub loader_version: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
