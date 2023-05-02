use clap::Parser;
use std::str;
use std::fmt;

#[derive(Parser)]
#[command(
    author,
    version,
    about = "GDPR - Granular Directus Permissions Resolver",
    long_about = "Find out who has access to columns. Batch edit many roles."
)]
// One can use stringer to modify or inspect strings straight from the terminal"
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Parser)]
pub enum Command {
    Dump(Dump),
    Replace(Replace),
}

#[derive(Parser)]
pub struct Dump {
    #[clap(flatten)]
    pub global_args: GlobalArgs,

    #[clap(flatten)]
    pub dump_args: DumpArgs,
}

#[derive(Parser)]
pub struct Replace {
    #[clap(flatten)]
    args: GlobalArgs,
}

#[derive(Parser)]
pub enum DbClient {
    Postgres,
    MySql,
    MariaDb,
    Sqlite,
}

/// TODO: #low-priority
/// Find a way to use serde or clap to serialize/deserialize OutputFormat.
/// There is the strum crate, which does this. But it's another dependency
/// which we should avoid.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
    Pretty
}

const JSON_FORMAT: &str = "json";
const YAML_FORMAT: &str = "yaml";
const PRETTY_FORMAT: &str = "pretty";

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OutputFormat::Json => write!(f, "{}", JSON_FORMAT),
            OutputFormat::Yaml => write!(f, "{}", YAML_FORMAT),
            OutputFormat::Pretty => write!(f, "{}", PRETTY_FORMAT),
        }
    }
}

impl str::FromStr for OutputFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            JSON_FORMAT => Ok(OutputFormat::Json),
            YAML_FORMAT => Ok(OutputFormat::Yaml),
            PRETTY_FORMAT => Ok(OutputFormat::Pretty),
            _ => Err(format!(
                "Unknown output format. Choose `{}`, `{}` or `{}` : {}",
                JSON_FORMAT, YAML_FORMAT, PRETTY_FORMAT, s
            )),
        }
    }
}

#[derive(Parser)]
pub struct GlobalArgs {
    #[arg(short = 'u', long, default_value_t = String::from("postgres://dbuser:dbpass@localhost:54322/mydb"))]
    pub url: String,

    #[arg(short = 'o', long, default_value_t = OutputFormat::Json)]
    pub output: OutputFormat,
}

#[derive(Parser)]
pub struct DumpArgs {
    #[arg(short = 'f', long)]
    pub field: Option<String>,

    #[arg(short = 't', long)]
    pub table: Option<String>,
}

// pub trait FieldTableArgs {
//     fn field(&mut self) -> &mut Option<String>;
//     fn table(&mut self) -> &mut Option<String>;

//     // fn test_me_round(&self) {
//     //     println!("hahahahahal {:#?}", self.field());
//     // }
//     /// Read field and table arguments from CLI and unwrap table.field
//     /// or panic if table is defined twice.
//     fn unwrap_or_panic(&mut self) {
//         if let Some(unpacked_field) = self.field() {
//             let split_table_field = utils::split_one_point_strictly(unpacked_field);
//             match split_table_field {
//                 (t, Some(f)) => {
//                     if self.table().is_some() {
//                         panic!(concat!(
//                             "You cannot use --field with dot notation (e.g.: `--field ",
//                             "table_name.field_name`) together with option --table. ",
//                             "Either use field with a simple field_name `--field field_name` ",
//                             "or don't use --table"
//                         ));
//                     }
//                     *self.table() = Some(t.to_owned());
//                     *self.field() = Some(f.to_owned());
//                 }
//                 (f, None) => {
//                     *self.field() = Some(f.to_owned());
//                 }
//             }
//         }
//     }
// }

// Define a generic function that works with both structs
// pub fn dododo<T: FieldTableArgs>(arg: &mut T) {
//     arg.test_me_round();
// }
