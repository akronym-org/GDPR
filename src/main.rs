use clap::Parser;
use gdpr::cli::{Cli, Command};
use gdpr::dump;

fn main() {
    let args = Cli::parse();
    // println!("{:#?}", args);

    match args.command {
        Command::Dump(dump) => {
            let dump: dump::DumpArgs = dump.args.into();
            dump::handle_dump(dump)
        }
        Command::Replace(_replace) => {
            // skip
        }
    }
}
