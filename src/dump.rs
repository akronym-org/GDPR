use crate::cli::{Dump, DumpArgs, GlobalArgs};
use crate::entities::directus_permissions;
use crate::entities::{prelude::*, *};
use crate::utils;
use sea_orm::*;

#[derive(Debug)]
pub struct DumpOptions {
    url: String,
    output: String,
    table: Option<String>,
    field: Option<String>,
}

// impl FieldTableArgs for DumpOptions {
//     fn field(&mut self) -> &mut Option<String> {
//         &mut self.field
//     }
//     fn table(&mut self) -> &mut Option<String> {
//         &mut self.table
//     }
// }

// impl FieldTableArgs for MyStruct {
//     fn field_name(&self) -> &str {
//         &self.field_name
//     }
// }


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
                            "You cannot use --field with dot notation (e.g.: `--field ",
                            "table_name.field_name`) together with option --table. ",
                            "Either use field with a simple field_name `--field field_name` ",
                            "or don't use --table"
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

pub async fn handle_dump(args: DumpOptions) -> Result<Vec<directus_permissions::Model>, DbErr> {
    // args.test_me_round();
    let db = Database::connect(args.url).await?;
    let permissions: Vec<directus_permissions::Model> = directus_permissions::Entity::find()
        .filter(
            Condition::all()
                .add(directus_permissions::Column::Collection.eq(args.table))
                .add(directus_permissions::Column::Fields.is_in(args.field)),
        )
        .all(&db)
        .await?;
    let show = serde_json::to_string_pretty(&permissions).unwrap();
    println!("{:#}", show);
    return Ok(permissions);
}

// pub fn condition_builder(options: DumpOptions) -> Condition {}
