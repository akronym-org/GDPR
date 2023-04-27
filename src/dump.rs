use crate::cli::CliArgs;
use sea_orm::*;
use crate::entities::directus_permissions;
use crate::entities::{prelude::*, *};

pub struct DumpArgs {
    url: String,
}

impl From<CliArgs> for DumpArgs {
    fn from(cli_args: CliArgs) -> Self {
        Self {
            url: cli_args.url,
        }
    }
}

pub async fn handle_dump(args: DumpArgs) -> Result<Vec<directus_permissions::Model>, DbErr> {
    let db = Database::connect(args.url).await?;
    let permissions: Vec<directus_permissions::Model> = directus_permissions::Entity::find().all(&db).await?;
    println!("{:#?}", permissions);
    return Ok(permissions);
}
