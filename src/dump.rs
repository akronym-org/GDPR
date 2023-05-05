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
    roles: Vec<String>,
    permissions: Vec<HashMap<String, HashMap<String, HashMap<String, String>>>>,
    validation: Option<Vec<HashMap<String, HashMap<String, String>>>>,
}

#[derive(Serialize, Deserialize)]
pub struct DumpField {
    create: Vec<DumpOperation>,
    read: Vec<DumpOperation>,
    update: Vec<DumpOperation>,
    delete: Vec<DumpOperation>,
    share: Vec<DumpOperation>,
}

impl DumpField {
    pub fn iter_keys() -> impl Iterator<Item = &'static str> {
        return ["create", "read", "update", "delete", "share"].into_iter();
    }
}


type DumpAll = HashMap<String, DumpField>;

/// Organize the database permissions into GDPR's base format.
///
/// # Arguments
///
/// * `args` - A reference to the user's settings used by the dump command.
/// * `permissions` - A reference to a vector of `directus_permissions::Model` objects
///   containing the permissions to be organized.
pub fn organize_dump(args: &DumpOptions, input: &Vec<directus_permissions::Model>) {
    // println!("args {:#?}", args);
    // println!("permissions {:#?}", input);

    let mut output: DumpAll = HashMap::new();

    // TODO: unwraps to `id` by default.
    // It should loop over all fields in directus_fields of the collection.
    // let field_name = args.field.unwrap_or("id".to_owned());

    // loop over each action and collect all roles
    for key in DumpField::iter_keys() {
        let roles = input
            .iter()
            .filter(|permission| permission.action == key)
            .map(|permission| {
                permission.role.clone().unwrap_or("public".to_owned())
            })
            .collect::<Vec<String>>();

        // output
        //     .entry(field_name)
        //     .or_insert_with(HashMap::new)
        //     .entry(key)
        //     .or_insert_with(Vec::new)
        //     .push(roles);
        println!("action {}: {:#?}", key, roles);
    }
    // println!("output: {:#?}", output);
}

/// Displays the organized permissions dump in a human-readable format.
/// 
/// # Arguments
/// 
/// * `output` - A reference to the user's preferred output format.
/// * `permissions` - a reference to the raw permissions.
pub fn show_dump(output: &OutputFormat, permissions: &Vec<directus_permissions::Model>) {
    let show: String = match output {
        OutputFormat::Yaml => serde_yaml::to_string(permissions).unwrap(),
        OutputFormat::Json => serde_json::to_string_pretty(permissions).unwrap(),
        OutputFormat::Pretty => panic!("Pretty is not yet implemented. Choose either json or yaml."),
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
    //     .filter(condition.clone()).build(DbBackend::Postgres).to_string();
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
        .filter(condition)
        .all(&db)
        .await?;

    let _organized_dump = organize_dump(&args, &permissions);

    // Toggled off: We'll replace role ids with role names later
    // let roles: Vec<Option<directus_roles::Model>> =
    //     permissions.load_one(directus_roles::Entity, &db).await?;
    // println!("roles {:#?}", roles);

    show_dump(&args.output, &permissions);

    return Ok(());
}
