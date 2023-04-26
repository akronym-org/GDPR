use clap::Parser;

#[derive(Parser, Debug)]
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

#[derive(Parser, Debug)]
pub enum Command {
    Dump(Dump),
    Replace(Replace),
}

#[derive(Parser, Debug)]
pub struct Dump {
    #[clap(flatten)]
    pub args: CliArgs,
}

#[derive(Parser, Debug)]
pub struct Replace {
    #[clap(flatten)]
    args: CliArgs,
}

#[derive(Parser, Debug)]
pub enum DbClient {
    Postgres,
    MySql,
    MariaDb,
    Sqlite,
}

#[derive(Parser, Debug)]
pub struct CliArgs {
    #[arg(short = 'c', long, default_value_t = String::from("postgres"))]
    pub client: String,

    #[arg(short = 'u', long, default_value_t = String::from("postgres://dbuser:dbpass@localhost:54322/mydb"))]
    pub url: String,
}
