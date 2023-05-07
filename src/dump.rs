use crate::cli::{Dump, DumpArgs, GlobalArgs, OutputFormat};
use crate::database;
use crate::entities::*;
use crate::manifest;
use crate::utils;
use crate::wildcard;
use indexmap::IndexMap;
use sea_orm::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug)]
pub struct DumpOptions {
    url: String,
    output: OutputFormat,
    table: Option<String>,
    field: Vec<String>,
}

// TODO: generalize this for other commands in cli.rs
// with default trait impls
impl From<Dump> for DumpOptions {
    fn from(dump: Dump) -> Self {
        let Dump {
            global_args: GlobalArgs { url, output },
            dump_args:
                DumpArgs {
                    mut table,
                    mut field,
                },
        } = dump;

        // unpack field argument if it's referencing a table e.g. `table.field`
        if let Some(unpacked_field) = field.as_ref() {
            let split_table_field = utils::split_one_point_strictly(unpacked_field);
            match split_table_field {
                (t, Some(f)) => {
                    if table.is_some() {
                        panic!(concat!(
                            "You cannot use `--field` with dot notation (e.g.: table_name.field_name) ",
                            "in combination with `--table`. Either use --field with a simple field name ",
                            "e.g.: `--field field_name` or don't use --table."
                        ));
                    }
                    table = Some(t.to_owned());
                    field = Some(f.to_owned());
                }
                (f, None) => {
                    field = Some(f.to_owned());
                }
            }
        }

        // Collect `field` into a vector and split if it's a string with commas.
        // If there is only a single wildcard (*) somewhere, discard and add empty vector.
        let fields = match field {
            Some(f) => {
                let split_fields: Vec<String> = f.split(',').map(|s| s.to_owned()).collect();
                if split_fields.contains(&"*".to_string()) {
                    vec![]
                } else {
                    split_fields
                }
            }
            None => vec![],
        };

        return DumpOptions {
            url,
            output,
            table,
            field: fields,
        };
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Rule {
    roles: Vec<String>,
    permissions: JsonValue,
    validation: JsonValue,
}
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Field {
    create: Vec<Rule>,
    read: Vec<Rule>,
    update: Vec<Rule>,
    delete: Vec<Rule>,
    share: Vec<Rule>,
}

impl Field {
    fn new() -> Self {
        Self {
            create: Vec::new(),
            read: Vec::new(),
            update: Vec::new(),
            delete: Vec::new(),
            share: Vec::new(),
        }
    }

    pub fn iter_keys() -> impl Iterator<Item = &'static str> {
        return ["create", "read", "update", "delete", "share"].into_iter();
    }
}

type DumpAll = HashMap<String, Field>;

#[derive(Serialize, Deserialize)]
struct DataWithVersion {
    version: String,
    #[serde(flatten)]
    data: DumpAll,
}

/// Deduplicate permissions and validations
///
/// # Arguments
///
/// * `input` - An input JsonValue
pub fn deduplicate(input: Vec<Rule>) -> Vec<Rule> {
    todo!()
}

/// Organize the database permissions into GDPR's base format.
///
/// # Arguments
///
/// * `args` - A reference to the user's settings used by the dump command.
/// * `permissions` - A reference to a vector of `directus_permissions::Model` objects
///   containing the permissions to be organized.
pub fn organize_fields(args: &DumpOptions, input: &Vec<directus_permissions::Model>) -> DumpAll {
    let mut dump_all = HashMap::new();
    for field_name in &args.field {
        let mut field = Field::new();

        // loop over each action
        for key in Field::iter_keys() {
            let role_permission = input
                .iter()
                .filter(|permission| permission.action == key)
                .map(|permission| {
                    let mut map = IndexMap::new();
                    map.insert(
                        "roles".to_owned(),
                        JsonValue::String(permission.role.clone().unwrap_or("public".to_owned())),
                    );
                    map.insert(
                        "permissions".to_owned(),
                        permission.permissions.clone().unwrap_or(JsonValue::Null),
                    );
                    map.insert(
                        "validation".to_owned(),
                        permission.validation.clone().unwrap_or(JsonValue::Null),
                    );
                    map
                })
                .collect::<Vec<_>>();

            for permission in role_permission {
                let dump_operation = Rule {
                    roles: vec![permission["roles"].as_str().unwrap().to_owned()],
                    permissions: permission["permissions"].clone(),
                    validation: permission["validation"].clone(),
                };

                match key {
                    "create" => field.create.push(dump_operation),
                    "read" => field.read.push(dump_operation),
                    "update" => field.update.push(dump_operation),
                    "delete" => field.delete.push(dump_operation),
                    "share" => field.share.push(dump_operation),
                    _ => (),
                }
            }
        }

        dump_all.insert(field_name.to_owned(), field);
    }

    return dump_all;
}

/// Displays the organized permissions dump in a human-readable format.
///
/// # Arguments
///
/// * `output` - A reference to the user's preferred output format.
/// * `permissions` - a reference to the raw permissions.
pub fn output_dump(output: &OutputFormat, data: &DumpAll) {
    let data_with_version = DataWithVersion {
        version: manifest::get_version(),
        // TODO: #low-priority
        // avoid using clone()
        data: data.clone(),
    };

    let show: String = match output {
        OutputFormat::Yaml => serde_yaml::to_string(&data_with_version).unwrap(),
        OutputFormat::Json => serde_json::to_string_pretty(&data_with_version).unwrap(),
        OutputFormat::Pretty => {
            panic!("Pretty is not yet implemented. Choose either json or yaml.")
        }
    };

    println!("{:#}", show);
}

/// Build a series of WHERE conditions for all fields
///
/// # Arguments
///
/// * `field` - a reference to the vector of strings with all field names or wildcard
fn build_field_condition(fields: &Vec<String>) -> Condition {
    // println!("fields {:#?}", fields);
    if fields.contains(&String::from("*")) {
        // do something
        return Condition::any();
    }

    let mut condition = Condition::all();

    for field in fields {
        condition = condition.add(
            Condition::any()
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
                .add(directus_permissions::Column::Fields.eq("*")),
        );
    }
    return condition;
}

/// Validate fields or panic
pub fn validate_fields_or_panic(
    to_be_checked: &Vec<String>,
    required: &Vec<String>,
    collection: &str,
) {
    for check in to_be_checked {
        if !required.contains(&check) {
            panic!(
                concat!(
                    "You requested permissions for a field, that is unknown to Directus ",
                    "because it can't be found in directus_fields.\n",
                    "Collection: {}\n",
                    "Field: {} <-- does not exist!"
                ),
                collection, check
            );
        }
    }
}

/// Handle logic for the `dump` command.
///
/// # Arguments
///
/// * `args` - A reference to the `dump` specific arguments and user options.
pub async fn dump_entrypoint(args: &mut DumpOptions) -> Result<(), DbErr> {
    let db = Database::connect(&args.url).await?;

    // Get all fields that have to be searched (e.g. if there is a wildcard)
    let db_fields = database::fetch_fields(&db, args.table.as_ref().unwrap()).await?;

    args.field = match args.field.is_empty() {
        true => db_fields,
        false => {
            let fields_with_wildcard = args
                .field
                .iter()
                .flat_map(|field| wildcard::find_with(&field, &db_fields))
                .collect::<Vec<String>>();

            validate_fields_or_panic(
                &fields_with_wildcard,
                &db_fields,
                args.table.as_ref().unwrap(),
            );

            fields_with_wildcard
        }
    };

    // Build the db query's where clause
    let mut condition = Condition::all();
    if let Some(ref table) = args.table {
        condition = condition.add(directus_permissions::Column::Collection.eq(table));
    }

    condition = condition.add(build_field_condition(&args.field));

    // Building the query as string (uncomment to see the query)
    // ```
    // let query = directus_permissions::Entity::find()
    //     .filter(condition.clone()).build(DbBackend::Postgres).to_string();
    // println!("query: {}", query);
    // ```
    let permissions: Vec<directus_permissions::Model> = directus_permissions::Entity::find()
        .filter(condition)
        .all(&db)
        .await?;

    let organized_dump = organize_fields(&args, &permissions);

    // Toggled off: We'll replace role ids with role names later
    // let roles: Vec<Option<directus_roles::Model>> =
    //     permissions.load_one(directus_roles::Entity, &db).await?;
    // println!("roles {:#?}", roles);

    output_dump(&args.output, &organized_dump);

    return Ok(());
}
