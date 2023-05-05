use crate::cli::{Dump, DumpArgs, GlobalArgs, OutputFormat};
use crate::entities::*;
use crate::utils;
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

#[derive(Serialize, Deserialize)]
pub struct DumpOperation {
    pub roles: Vec<String>,
    pub permissions: String,
    pub validation: String,
}

#[derive(Serialize, Deserialize)]
pub struct DumpField {
    pub create: Vec<DumpOperation>,
    pub read: Vec<DumpOperation>,
    pub update: Vec<DumpOperation>,
    pub delete: Vec<DumpOperation>,
    pub share: Vec<DumpOperation>,
}

type DumpAll = HashMap<String, DumpField>;

/// Organize the permissions data into a structured format.
///
/// This function takes a reference to `DumpOptions` and a reference to a vector of
/// `directus_permissions::Model` objects, and processes them to create a structured
/// organization of the permissions dump.
///
/// # Arguments
///
/// * `args` - A reference to the CLI options.
/// * `permissions` - A reference to a vector of `directus_permissions::Model` objects
///   containing the permissions to be organized.
pub fn organize_dump(args: &DumpOptions, permissions: &Vec<directus_permissions::Model>) {
    println!("args {:#?}", args);
    println!("permissions {:#?}", permissions);

    // 1st: get all fields that are required
}

/// Displays the organized permissions dump in a human-readable format.
/// 
/// # Arguments
/// 
/// * `args` - A reference to the CLI options.
/// * `permissions` - a reference to the raw permissions.
pub fn show_dump(args: &DumpOptions, permissions: &Vec<directus_permissions::Model>) {
    let show: String;
    if args.output == OutputFormat::Yaml {
        show = serde_yaml::to_string(permissions).unwrap();
    } else if args.output == OutputFormat::Json {
        show = serde_json::to_string_pretty(permissions).unwrap();
    } else if args.output == OutputFormat::Pretty {
        panic!("Pretty is not yet implemented. Choose either json or yaml.")
    } else {
        panic!("Choose either json or yaml.")
    }

    println!("{:#}", show);
}

pub async fn dump_entrypoint(args: &DumpOptions) -> Result<(), DbErr> {
    let db = Database::connect(&args.url).await?;

    // Build the db query where clause
    let mut condition = Condition::all();
    if let Some(ref table) = args.table {
        condition = condition.add(directus_permissions::Column::Collection.eq(table));
    }
    if let Some(ref field) = args.field {
        condition = condition
            .add(
                Condition::any()
                    // if `field` matches exactly
                    .add(directus_permissions::Column::Fields.eq(field))
                    // if `field` is in the middle of a csv of multiple fields
                    .add(directus_permissions::Column::Fields.like(("%,".to_owned() + field + ",%").as_str()))
                    // if `field` is at the start
                    .add(directus_permissions::Column::Fields.like((field.to_owned() + ",%").as_str()))
                    // if `field` is at the end
                    .add(directus_permissions::Column::Fields.like(("%,".to_owned() + field).as_str()))
                    // if it's about all fields of a table directus uses a wildcard
                    .add(directus_permissions::Column::Fields.eq("*"))
            )
    }

    // If you build the query as string you get this:
    // ```rust
    // let query = directus_permissions::Entity::find()
    //     .filter(condition).build(DbBackend::Postgres).to_string();
    // println!("query: {}", query);
    // ```
    // SELECT * FROM "directus_permissions"
    // WHERE "directus_permissions"."collection" = 'table_name'
    // AND ("directus_permissions"."fields" = 'field_name'
    //   OR "directus_permissions"."fields" LIKE '%,field_name,%'
    //   OR "directus_permissions"."fields" LIKE 'field_name,%'
    //   OR "directus_permissions"."fields" LIKE '%,field_name'
    //   OR "directus_permissions"."fields" = '*')
    let permissions: Vec<directus_permissions::Model> = directus_permissions::Entity::find()
        .filter(condition.clone())
        .all(&db)
        .await?;

    let organized_dump = organize_dump(args, &permissions);

    // Toggled off: We'll replace role ids with role names later
    // let roles: Vec<Option<directus_roles::Model>> =
    //     permissions.load_one(directus_roles::Entity, &db).await?;
    // println!("roles {:#?}", roles);

    show_dump(&args, &permissions);

    return Ok(());
}
