use crate::entities::*;
use sea_orm::*;

/// Get all fields in `directus_fields` that match `collection`
pub async fn fetch_fields(
    db: &DatabaseConnection,
    collection: &str,
) -> Result<Vec<String>, DbErr> {
    let fields: Vec<String> = directus_fields::Entity::find()
        .select_only()
        .column(directus_fields::Column::Field)
        .filter(directus_fields::Column::Collection.eq(collection))
        .into_tuple()
        .all(db)
        .await?
        .into_iter()
        .map(|(field,)| field)
        .collect();
    return Ok(fields);
}
