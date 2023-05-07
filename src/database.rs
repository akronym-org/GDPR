use crate::entities::*;
use sea_orm::*;

/// Get all fields in `directus_fields` that match `table`
pub async fn fetch_valid_fields_or_panic(
    db: &DatabaseConnection,
    table: String,
    field_names: Vec<String>,
) -> Result<Vec<String>, DbErr> {
    let fields: Vec<String> = directus_fields::Entity::find()
        .select_only()
        .column(directus_fields::Column::Field)
        .filter(directus_fields::Column::Collection.eq(&table))
        .into_tuple()
        .all(db)
        .await?
        .into_iter()
        .map(|(field,)| field)
        .collect();

    for requested_field in field_names {
        if !fields.contains(&requested_field) {
            panic!(
                concat!(
                    "You requested permissions for a field, that is unknown to Directus ",
                    "because it can't be found in directus_fields. \n
                    Collection: {}\n
                    Field: {}"
                ),
                table, requested_field
            );
        }
    }

    Ok(fields)
}
