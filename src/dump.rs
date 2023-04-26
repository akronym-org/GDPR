use crate::cli::CliArgs;
use crate::connection;

pub struct DumpArgs {
    client: String,
    url: String,
}

impl From<CliArgs> for DumpArgs {
    fn from(connection_args: CliArgs) -> Self {
        Self {
            client: connection_args.client,
            url: connection_args.url,
        }
    }
}

pub fn get_client(client: &str) -> Box<dyn connection::SqlClient> {
    match client {
        "postgres" => Box::new(connection::PostgresClient),
        "mysql" => Box::new(connection::MysqlClient),
        // Add cases for "sqlite" and "mssql"
        _ => panic!("Invalid client name"),
    }
}

pub fn handle_dump(args: DumpArgs) {

    let client = get_client(&args.client);
    match client.read_data(&args.url) {
        Ok(results) => {
            if results.is_empty() {
                println!("No data found");
            } else {
                for result in results {
                    println!("{}", serde_json::to_string_pretty(&result).unwrap());
                }
            }
        }
        Err(err) => eprintln!("Error reading data: {:?}", err),
    }
}
