use crate::config;
use crate::entities::*;
use sea_orm::*;

#[derive(Clone, Debug)]
pub struct Field {
    pub collection: String,
    pub field: String,
}
/// Get all fields in `directus_fields` that match `collection`
pub async fn fetch_fields(
    db: &DatabaseConnection,
    collections: &[String],
) -> Result<Vec<Field>, DbErr> {
    let mut condition = Condition::any();
    for col in collections {
        condition = condition.add(directus_fields::Column::Collection.eq(col));
    }

    let fields: Vec<Field> = directus_fields::Entity::find()
        .select_only()
        .column(directus_fields::Column::Collection)
        .column(directus_fields::Column::Field)
        .filter(condition)
        .into_tuple()
        .all(db)
        .await?
        .into_iter()
        .map(|(collection, field)| Field { collection, field })
        .collect();

    Ok(fields)
}

/// Get all collections in `directus_fields` that match `collection`
/// Mixes in directus system collections from config
pub async fn fetch_collections(db: &DatabaseConnection) -> Result<Vec<String>, DbErr> {
    let system_collections: Vec<String> = config::get_directus_system_collections();
    let mut collections: Vec<String> = directus_collections::Entity::find()
        .select_only()
        .column(directus_collections::Column::Collection)
        .into_tuple()
        .all(db)
        .await?
        .into_iter()
        .map(|(collection,)| collection)
        .collect();
    collections.extend(system_collections);

    Ok(collections)
}

/// Build condition for a specific field
///
/// It's tricky 'cause the `fields` column are CSVs or could be a wildcard.
///
/// # Arguments
/// * `field` - a string reference of a field.
pub fn field_specific(field: &str) -> Condition {
    return Condition::any()
        // if `field` matches exactly
        .add(directus_permissions::Column::Fields.eq(field))
        // if `field` is in the middle of a csv
        .add(directus_permissions::Column::Fields.like(("%,".to_owned() + field + ",%").as_str()))
        // if `field` is at the start
        .add(directus_permissions::Column::Fields.like((field.to_owned() + ",%").as_str()))
        // if `field` is at the end
        .add(directus_permissions::Column::Fields.like(("%,".to_owned() + field).as_str()))
        // if it's about all fields of a table directus uses a wildcard
        .add(directus_permissions::Column::Fields.eq("*"));
}

/// FIXME: This is more difficult.
/// Consider this example `value1,wildcard,unwild,value4`
/// If we search for wild*, we don't want to match unwild
/// Try if a subquery is good enough?
///     1. Use LIKE `%,wild%`
///        Use LIKE `%
///     2. then check if result contains only one comma before/after wild
///
/// Build condition for a wildcard field
///
/// It's tricky 'cause the `fields` column are CSVs or could be a wildcard.
///
/// # Arguments
/// * `field` - a string reference of a field with a wildcard character
pub fn field_wildcard(field: &str) -> Condition {
    let binding = field.replace("*", "%");
    return Condition::any()
        // if there's only one value in `field` column
        // this is so convoluted, because we can't check with .eq(&binding)
        // because we're using wildcards.
        .add(
            Condition::all()
                .add(directus_permissions::Column::Fields.not_like(","))
                .add(directus_permissions::Column::Fields.like(&binding)),
        )
        // if `field` is in the middle of a csv
        .add(
            directus_permissions::Column::Fields.like(("%,".to_owned() + &binding + ",%").as_str()),
        )
        // if `field` is at the start of csv
        // FIXME: this is not good, it only works if wildcard is at the end of binding
        .add(directus_permissions::Column::Fields.starts_with(&binding))
        // if `field` is at the end of csv
        // FIXME: this is not good, it only works if wildcard is at the start of binding
        .add(directus_permissions::Column::Fields.ends_with(&binding))
        // if it's about all fields of a table directus uses a wildcard
        .add(directus_permissions::Column::Fields.eq("*"));
}

pub fn collection_wildcard(collection: &str) -> Condition {
    Condition::all()
        .add(directus_permissions::Column::Collection.like(collection.replace("*", "%").as_ref()))
}

pub fn collection_specific(collection: &str) -> Condition {
    Condition::all().add(directus_permissions::Column::Collection.eq(collection))
}
