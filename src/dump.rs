use crate::cli::{Dump, DumpUserArgs, GlobalArgs, OutputFormat};
use crate::config;
use crate::database;
use crate::entities::*;
use crate::manifest;
use crate::utils::{self, split_one_point_strictly};
use crate::wildcard;
use indexmap::IndexMap;
use sea_orm::sea_query::Table;
use sea_orm::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::default::Default;

/// Handle logic for the `dump` command.
///
/// # Arguments
///
/// * `args` - A reference to user's `dump` specific options.
pub async fn dump_entrypoint(args: &mut DumpOptions) -> Result<(), DbErr> {
    println!("fields: {:#?}", args.resources);

    let conditions = args.to_conditions();

    let db = Database::connect(&args.url).await?;

    // Building the query as string (uncomment to see the query)
    // ```
    // let query = directus_permissions::Entity::find()
    //     .filter(conditions.clone()).build(DbBackend::Postgres).to_string();
    // println!("query: {}", query);
    // ```
    let permissions: Vec<directus_permissions::Model> = directus_permissions::Entity::find()
        .filter(conditions)
        .all(&db)
        .await?;

    println!("hahaha {:#?}", permissions);
    // let organized_dump = organize_fields(&args, &permissions);

    // Toggled off: We'll replace role ids with role names later
    // let roles: Vec<Option<directus_roles::Model>> =
    //     permissions.load_one(directus_roles::Entity, &db).await?;
    // println!("roles {:#?}", roles);

    // output_dump(&args.output, &organized_dump);

    return Ok(());
}

#[derive(Debug)]
pub struct DumpOptions {
    url: String,
    output: OutputFormat,
    resources: Vec<FieldRequester>,
}

impl DumpOptions {
    pub fn to_conditions(&self) -> Condition {
       if self.resources.is_empty() {
            Condition::all()
        } else {
            self.resources
                .iter()
                .map(|res| res.to_condition())
                .fold(Condition::any(), |acc, condition| acc.add(condition))
        }
    }
}

impl From<Dump> for DumpOptions {
    fn from(dump: Dump) -> Self {
        return DumpOptions {
            url: dump.global_args.url,
            output: dump.global_args.output,
            resources: dump
                .dump_args
                .resource
                .unwrap_or_default()
                .iter()
                .map(|req| Resource::from(req.clone()).into())
                .collect(),
        };
    }
}

#[derive(Debug)]
struct Resource {
    collection: Option<MaybeWildcard>,
    field: Option<MaybeWildcard>,
}

impl Default for Resource {
    fn default() -> Self {
        Self {
            collection: Some(MaybeWildcard::HasWildcard("*".to_string())),
            field: None,
        }
    }
}

impl From<String> for Resource {
    fn from(string: String) -> Self {
        let split = split_one_point_strictly(&string);
        return Self {
            collection: Some(MaybeWildcard::from(split.0.to_string())),
            field: split.1.map(|s| MaybeWildcard::from(s.to_string())),
        };
    }
}

#[derive(Debug)]
enum MaybeWildcard {
    Specific(String),
    HasWildcard(String),
}

impl MaybeWildcard {
    fn as_str(&self) -> &str {
        match self {
            MaybeWildcard::HasWildcard(s) => s.as_str(),
            MaybeWildcard::Specific(s) => s.as_str(),
        }
    }
}

impl From<String> for MaybeWildcard {
    fn from(s: String) -> Self {
        if s.contains('*') {
            MaybeWildcard::HasWildcard(s)
        } else {
            MaybeWildcard::Specific(s)
        }
    }
}

#[derive(Debug)]
enum FieldRequester {
    BothNone(Resource),
    WildCollectionNoField(Resource),
    BothWild(Resource),
    WildCollectionSpecificField(Resource),
    SpecificCollectionNoField(Resource),
    SpecificCollectionWildField(Resource),
    BothSpecific(Resource),
    NoCollectionWildField(Resource),
    NoCollectionSpecificField(Resource),
}

impl From<Resource> for FieldRequester {
    fn from(resource: Resource) -> Self {
        match &resource {
            Resource {
                collection: None,
                field: None,
            } => FieldRequester::BothNone(resource),
            Resource {
                collection: Some(MaybeWildcard::HasWildcard(_)),
                field: None,
            } => FieldRequester::WildCollectionNoField(resource),
            Resource {
                collection: Some(MaybeWildcard::HasWildcard(_)),
                field: Some(MaybeWildcard::HasWildcard(_)),
            } => FieldRequester::BothWild(resource),
            Resource {
                collection: Some(MaybeWildcard::HasWildcard(_)),
                field: Some(MaybeWildcard::Specific(_)),
            } => FieldRequester::WildCollectionSpecificField(resource),
            Resource {
                collection: Some(MaybeWildcard::Specific(_)),
                field: None,
            } => FieldRequester::SpecificCollectionNoField(resource),
            Resource {
                collection: Some(MaybeWildcard::Specific(_)),
                field: Some(MaybeWildcard::HasWildcard(_)),
            } => FieldRequester::SpecificCollectionWildField(resource),
            Resource {
                collection: Some(MaybeWildcard::Specific(_)),
                field: Some(MaybeWildcard::Specific(_)),
            } => FieldRequester::BothSpecific(resource),
            Resource {
                collection: None,
                field: Some(MaybeWildcard::HasWildcard(_)),
            } => FieldRequester::NoCollectionWildField(resource),
            Resource {
                collection: None,
                field: Some(MaybeWildcard::Specific(_)),
            } => FieldRequester::NoCollectionSpecificField(resource),
        }
    }
}

impl FieldRequester {
    pub fn to_condition(&self) -> Condition {
        match &self {
            FieldRequester::BothNone(_) => Condition::all(),
            FieldRequester::WildCollectionNoField(r) => {
                database::collection_wildcard(r.collection.as_ref().unwrap().as_str())
            }
            FieldRequester::BothWild(r) => {
                database::collection_wildcard(r.collection.as_ref().unwrap().as_str())
                    .add(database::field_wildcard(r.field.as_ref().unwrap().as_str()))
            }
            FieldRequester::WildCollectionSpecificField(r) => {
                database::collection_wildcard(r.collection.as_ref().unwrap().as_str())
                    .add(database::field_specific(r.field.as_ref().unwrap().as_str()))
            }
            FieldRequester::SpecificCollectionNoField(r) => {
                database::collection_specific(r.collection.as_ref().unwrap().as_str())
            }
            FieldRequester::SpecificCollectionWildField(r) => {
                database::collection_specific(r.collection.as_ref().unwrap().as_str())
                    .add(database::field_wildcard(r.field.as_ref().unwrap().as_str()))
            }
            FieldRequester::BothSpecific(r) => {
                database::collection_specific(r.collection.as_ref().unwrap().as_str())
                    .add(database::field_specific(r.field.as_ref().unwrap().as_str()))
            }
            FieldRequester::NoCollectionWildField(r) => {
                database::field_wildcard(r.field.as_ref().unwrap().as_str())
            }
            FieldRequester::NoCollectionSpecificField(r) => {
                database::field_specific(r.field.as_ref().unwrap().as_str())
            }
        }
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
// pub fn deduplicate(input: Vec<Rule>) -> Vec<Rule> {
//     todo!()
// }

/// Organize the database permissions into GDPR's base format.
///
/// # Arguments
///
/// * `args` - A reference to the user's settings used by the dump command.
/// * `permissions` - A reference to a vector of `directus_permissions::Model` objects
///   containing the permissions to be organized.
// pub fn organize_fields(args: &DumpOptions, input: &Vec<directus_permissions::Model>) -> DumpAll {
//     let mut dump_all = HashMap::new();
//     for field_name in &args.resources {
//         let mut field = Field::new();

//         // loop over each action
//         for key in Field::iter_keys() {
//             let role_permission = input
//                 .iter()
//                 .filter(|permission| permission.action == key)
//                 .map(|permission| {
//                     let mut map = IndexMap::new();
//                     map.insert(
//                         "roles".to_owned(),
//                         JsonValue::String(permission.role.clone().unwrap_or("public".to_owned())),
//                     );
//                     map.insert(
//                         "permissions".to_owned(),
//                         permission.permissions.clone().unwrap_or(JsonValue::Null),
//                     );
//                     map.insert(
//                         "validation".to_owned(),
//                         permission.validation.clone().unwrap_or(JsonValue::Null),
//                     );
//                     map
//                 })
//                 .collect::<Vec<_>>();

//             for permission in role_permission {
//                 let dump_operation = Rule {
//                     roles: vec![permission["roles"].as_str().unwrap().to_owned()],
//                     permissions: permission["permissions"].clone(),
//                     validation: permission["validation"].clone(),
//                 };

//                 match key {
//                     "create" => field.create.push(dump_operation),
//                     "read" => field.read.push(dump_operation),
//                     "update" => field.update.push(dump_operation),
//                     "delete" => field.delete.push(dump_operation),
//                     "share" => field.share.push(dump_operation),
//                     _ => (),
//                 }
//             }
//         }

//         dump_all.insert(field_name.to_owned(), field);
//     }

//     return dump_all;
// }

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

/// Validate fields or panic
pub fn validate_from_vec_or_panic(
    to_be_checked: &[String],
    required: &[String],
    required_name: Option<&str>,
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
                required_name.unwrap_or("[unknown]"),
                check
            );
        }
    }
}
