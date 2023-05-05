use crate::cli::{Dump, DumpArgs, GlobalArgs, OutputFormat};
use crate::entities::*;
use crate::manifest;
use crate::utils;
use indexmap::IndexMap;
use sea_orm::*;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::collections::HashMap;

#[derive(Debug)]
pub struct DumpOptions {
    url: String,
    output: OutputFormat,
    table: Option<String>,
    field: Option<String>,
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

        return DumpOptions {
            url,
            output,
            table,
            field,
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
pub fn organize_field(args: &DumpOptions, input: &Vec<directus_permissions::Model>) -> DumpAll {
    // TODO: panics if no --field option was specified.
    // Eventually this should loop over all fields in directus_fields of the collection.
    let field_name = args
        .field
        .clone()
        .expect("You must specify a `--field` option as of now.");

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

    let mut dump_all = HashMap::new();
    dump_all.insert(field_name, field);

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

/// Handle logic for the `dump` command.
///
/// # Arguments
///
/// * `args` - A reference to the `dump` specific arguments and user options.
pub async fn dump_entrypoint(args: &DumpOptions) -> Result<(), DbErr> {
    let db = Database::connect(&args.url).await?;

    // Build the db query's where clause
    let mut condition = Condition::all();
    if let Some(ref table) = args.table {
        condition = condition.add(directus_permissions::Column::Collection.eq(table));
    }
    if let Some(ref field) = args.field {
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
        )
    }

    // Building the query as string ...
    // ```rust
    // let query = directus_permissions::Entity::find()
    //     .filter(condition.clone()).build(DbBackend::Postgres).to_string();
    // println!("query: {}", query);
    // ```
    //
    // Returns this:
    // SELECT * FROM "directus_permissions"
    // WHERE "directus_permissions"."collection" = 'table_name'
    // AND ("directus_permissions"."fields" = 'field_name'
    //   OR "directus_permissions"."fields" LIKE '%,field_name,%'
    //   OR "directus_permissions"."fields" LIKE 'field_name,%'
    //   OR "directus_permissions"."fields" LIKE '%,field_name'
    //   OR "directus_permissions"."fields" = '*')
    let permissions: Vec<directus_permissions::Model> = directus_permissions::Entity::find()
        .filter(condition)
        .all(&db)
        .await?;

    let organized_dump = organize_field(&args, &permissions);

    // Toggled off: We'll replace role ids with role names later
    // let roles: Vec<Option<directus_roles::Model>> =
    //     permissions.load_one(directus_roles::Entity, &db).await?;
    // println!("roles {:#?}", roles);

    output_dump(&args.output, &organized_dump);

    return Ok(());
}
