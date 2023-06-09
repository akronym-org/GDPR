//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "directus_presets")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub bookmark: Option<String>,
    pub user: Option<Uuid>,
    pub role: Option<Uuid>,
    pub collection: Option<String>,
    pub search: Option<String>,
    pub layout: Option<String>,
    pub layout_query: Option<Json>,
    pub layout_options: Option<Json>,
    pub refresh_interval: Option<i32>,
    pub filter: Option<Json>,
    pub icon: Option<String>,
    pub color: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::directus_roles::Entity",
        from = "Column::Role",
        to = "super::directus_roles::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    DirectusRoles,
    #[sea_orm(
        belongs_to = "super::directus_users::Entity",
        from = "Column::User",
        to = "super::directus_users::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    DirectusUsers,
}

impl Related<super::directus_roles::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DirectusRoles.def()
    }
}

impl Related<super::directus_users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DirectusUsers.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
