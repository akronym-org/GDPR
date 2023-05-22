use crate::cli::{Dump, OutputFormat};
use crate::directus;
use crate::entities::directus_permissions;
use crate::manifest;
use crate::reversed_permissions;
use crate::utils;
use crate::graph::{GraphToString,build_graph};
use serde::{Deserialize, Serialize};
use sea_orm::{Database,DbErr,Select,Condition};
use sea_orm::{entity::*, query::*};
use petgraph_graphml::GraphMl;


/// 🏡 Handle logic for the `dump` command.
///
/// # Arguments
///
/// * `args` - A reference to user's `dump` specific options.
pub async fn dump_entrypoint(args: &mut DumpOptions) -> Result<(), DbErr> {
    let db = Database::connect(&args.url).await?;

    // FIXME: importing collections and fields should relate to args.resources
    // and only request necessary rows.
    let collections = directus::fetch_collections(&db).await?;
    let fields = directus::fetch_fields(&db, &collections).await?;
    let query = args.resources.to_query();

    // Output query as string? Uncomment!
    // ```rust
    // let builder = db.get_database_backend();
    // let sql_query = query.build(builder).to_string();
    // println!("query: {}", sql_query);
    // ```
    let permissions: Vec<directus_permissions::Model> = query.all(&db).await?;
    let graph = build_graph(permissions, &fields);

    match args.output {
        OutputFormat::Dot => graph.draw(),
        OutputFormat::GraphML => println!("{}", GraphMl::new(&graph)
            .pretty_print(true)
            .export_node_weights_display()),
        _ => ()
    }

    // FIXME: -- skip. implemented as graph
    // let organized_dump = reversed_permissions::Builder::request(&args.resources)
    //     .permission(&permissions)
    //     .options()
    //     .dedupe_roles()
    //     .dedupe_permissions()
    //     .build();

    // TODO:
    // Toggled off: We'll replace role ids with role names later
    // let roles: Vec<Option<directus_roles::Model>> =
    //     permissions.load_one(directus_roles::Entity, &db).await?;
    // println!("roles {:#?}", roles);

    // output_dump(&args.output, &organized_dump);

    Ok(())
}

#[derive(Debug)]
pub struct DumpOptions {
    pub url: String,
    pub output: OutputFormat,
    pub resources: Vec<Request>,
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
                .map(|req| RequestEntity::from(req.clone()).into())
                .collect(),
        };
    }
}

#[derive(Debug)]
pub struct RequestEntity {
    collection: MaybeWildcard,
    field: MaybeWildcard,
}

impl From<String> for RequestEntity {
    fn from(string: String) -> Self {
        let split = utils::split_one_point_strictly(&string);
        Self {
            collection: MaybeWildcard::from(split.0.unwrap_or("*").to_string()),
            field: MaybeWildcard::from(split.1.unwrap_or("*").to_string()),
        }
    }
}

trait ToQuery {
    fn to_query(&self) -> Select<directus_permissions::Entity>;
}

impl ToQuery for Vec<Request> {
    fn to_query(&self) -> Select<directus_permissions::Entity> {
        directus_permissions::Entity::find().filter(self.to_conditions())
    }
}

trait ToConditions {
    fn to_conditions(&self) -> Condition;
}

impl ToConditions for Vec<Request> {
    fn to_conditions(&self) -> Condition {
        if self.is_empty() {
            Condition::all()
        } else {
            self.iter()
                .map(|res| res.to_condition())
                .fold(Condition::any(), |acc, condition| acc.add(condition))
        }
    }
}

#[derive(Debug)]
enum MaybeWildcard {
    Specific(String),
    HasWildcard(String),
    All,
}

impl MaybeWildcard {
    fn as_str(&self) -> &str {
        match self {
            MaybeWildcard::HasWildcard(s) => s.as_str(),
            MaybeWildcard::Specific(s) => s.as_str(),
            MaybeWildcard::All => "*",
        }
    }
}

impl From<String> for MaybeWildcard {
    fn from(s: String) -> Self {
        if s == "*" {
            MaybeWildcard::All
        } else if s.contains('*') {
            MaybeWildcard::HasWildcard(s)
        } else {
            MaybeWildcard::Specific(s)
        }
    }
}

#[derive(Debug)]
pub enum Request {
    BothAll(RequestEntity),
    WildCollectionAllFields(RequestEntity),
    BothWild(RequestEntity),
    WildCollectionSpecificField(RequestEntity),
    SpecificCollectionAllFields(RequestEntity),
    SpecificCollectionWildField(RequestEntity),
    BothSpecific(RequestEntity),
    AllCollectionsWildField(RequestEntity),
    AllCollectionsSpecificField(RequestEntity),
}

impl From<RequestEntity> for Request {
    fn from(request_item: RequestEntity) -> Self {
        match &request_item {
            RequestEntity {
                collection: MaybeWildcard::All,
                field: MaybeWildcard::All,
            } => Request::BothAll(request_item),
            RequestEntity {
                collection: MaybeWildcard::HasWildcard(_),
                field: MaybeWildcard::All,
            } => Request::WildCollectionAllFields(request_item),
            RequestEntity {
                collection: MaybeWildcard::HasWildcard(_),
                field: MaybeWildcard::HasWildcard(_),
            } => Request::BothWild(request_item),
            RequestEntity {
                collection: MaybeWildcard::HasWildcard(_),
                field: MaybeWildcard::Specific(_),
            } => Request::WildCollectionSpecificField(request_item),
            RequestEntity {
                collection: MaybeWildcard::Specific(_),
                field: MaybeWildcard::All,
            } => Request::SpecificCollectionAllFields(request_item),
            RequestEntity {
                collection: MaybeWildcard::Specific(_),
                field: MaybeWildcard::HasWildcard(_),
            } => Request::SpecificCollectionWildField(request_item),
            RequestEntity {
                collection: MaybeWildcard::Specific(_),
                field: MaybeWildcard::Specific(_),
            } => Request::BothSpecific(request_item),
            RequestEntity {
                collection: MaybeWildcard::All,
                field: MaybeWildcard::HasWildcard(_),
            } => Request::AllCollectionsWildField(request_item),
            RequestEntity {
                collection: MaybeWildcard::All,
                field: MaybeWildcard::Specific(_),
            } => Request::AllCollectionsSpecificField(request_item),
        }
    }
}

impl Request {
    pub fn to_condition(&self) -> Condition {
        match &self {
            Request::BothAll(_) => Condition::all(),
            Request::WildCollectionAllFields(r) => {
                directus::collection_wildcard(r.collection.as_str())
            }
            Request::BothWild(r) => directus::collection_wildcard(r.collection.as_str())
                .add(directus::field_wildcard(r.field.as_str())),
            Request::WildCollectionSpecificField(_) => {
                todo!() // FIXME:
                        // use a subquery that first selects all
                        // `directus_fields` that match field.
            }
            Request::SpecificCollectionAllFields(r) => {
                directus::collection_specific(r.collection.as_str())
            }
            Request::SpecificCollectionWildField(r) => {
                directus::collection_specific(r.collection.as_str())
                    .add(directus::field_wildcard(r.field.as_str()))
            }
            Request::BothSpecific(r) => directus::collection_specific(r.collection.as_str())
                .add(directus::field_specific(r.field.as_str())),
            Request::AllCollectionsWildField(_) => {
                todo!() // FIXME:
                        // use a subquery that first selects all
                        // `directus_fields` that match field.
            }
            Request::AllCollectionsSpecificField(_) => {
                todo!() // FIXME:
                        // use a subquery that first selects all
                        // `directus_fields` that match field.
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
struct DataWithVersion {
    version: String,
    #[serde(flatten)]
    data: reversed_permissions::CollectionRules,
}

/// Displays the organized permissions dump in a human-readable format.
///
/// # Arguments
///
/// * `output` - A reference to the user's preferred output format.
/// * `permissions` - a reference to the raw permissions.
pub fn output_dump(output: &OutputFormat, data: &reversed_permissions::CollectionRules) {
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
        _ => "ignore for now".to_owned(),
    };

    println!("{:#}", show);
}
