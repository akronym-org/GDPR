use crate::entities::*;
use sea_orm::*;

/// Get all fields in `directus_fields` that match `collection`
pub async fn fetch_fields(
    db: &DatabaseConnection,
    collections: &[String],
) -> Result<Vec<String>, DbErr> {

    let mut condition = Condition::any();
    for col in collections {
        condition = condition.add(directus_fields::Column::Collection.eq(col));
    }

    let fields: Vec<String> = directus_fields::Entity::find()
        .select_only()
        .column(directus_fields::Column::Collection)
        .column(directus_fields::Column::Field)
        .filter(condition)
        .into_tuple()
        .all(db)
        .await?
        .into_iter()
        .map(|(field,)| field)
        .collect();
    return Ok(fields);
}

/// Get all collections in `directus_fields` that match `collection`
pub async fn fetch_collections(
    db: &DatabaseConnection,
) -> Result<Vec<String>, DbErr> {
    let collections: Vec<String> = directus_collections::Entity::find()
        .select_only()
        .column(directus_collections::Column::Collection)
        .into_tuple()
        .all(db)
        .await?
        .into_iter()
        .map(|(collection,)| collection)
        .collect();
    return Ok(collections);
}

/// Build condition for fields.
///
/// It's tricky 'cause the `fields` column are CSVs or could be a wildcard.
///
/// # Arguments
/// * `field` - a string reference of a field.
pub fn field_finder(field: &str) -> Condition {
    return Condition::any()
            // if `field` matches exactly
            .add(directus_permissions::Column::Fields.eq(field))
            // if `field` is in the middle of a csv of multiple fields
            .add(
                directus_permissions::Column::Fields
                    .like(("%,".to_owned() + field + ",%").as_str()),
            )
            // if `field` is at the start
            .add(directus_permissions::Column::Fields.like((field.to_owned() + ",%").as_str()))
            // if `field` is at the end
            .add(directus_permissions::Column::Fields.like(("%,".to_owned() + field).as_str()))
            // if it's about all fields of a table directus uses a wildcard
            .add(directus_permissions::Column::Fields.eq("*"));
}
