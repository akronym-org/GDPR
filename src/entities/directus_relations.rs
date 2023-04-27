//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "directus_relations")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub many_collection: String,
    pub many_field: String,
    pub one_collection: Option<String>,
    pub one_field: Option<String>,
    pub one_collection_field: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub one_allowed_collections: Option<String>,
    pub junction_field: Option<String>,
    pub sort_field: Option<String>,
    pub one_deselect_action: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
