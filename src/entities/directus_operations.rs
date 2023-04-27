//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "directus_operations")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub name: Option<String>,
    pub key: String,
    pub r#type: String,
    pub position_x: i32,
    pub position_y: i32,
    pub options: Option<Json>,
    #[sea_orm(unique)]
    pub resolve: Option<Uuid>,
    #[sea_orm(unique)]
    pub reject: Option<Uuid>,
    pub flow: Uuid,
    pub date_created: Option<DateTimeWithTimeZone>,
    pub user_created: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::directus_flows::Entity",
        from = "Column::Flow",
        to = "super::directus_flows::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    DirectusFlows,
    #[sea_orm(
        belongs_to = "Entity",
        from = "Column::Reject",
        to = "Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    SelfRef2,
    #[sea_orm(
        belongs_to = "Entity",
        from = "Column::Resolve",
        to = "Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    SelfRef1,
    #[sea_orm(
        belongs_to = "super::directus_users::Entity",
        from = "Column::UserCreated",
        to = "super::directus_users::Column::Id",
        on_update = "NoAction",
        on_delete = "SetNull"
    )]
    DirectusUsers,
}

impl Related<super::directus_flows::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DirectusFlows.def()
    }
}

impl Related<super::directus_users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DirectusUsers.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
