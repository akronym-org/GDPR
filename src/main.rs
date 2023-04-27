use clap::Parser;
use cli::{Cli, Command};
// use dump::{handle_dump, DumpArgs};
use futures::executor::block_on;


pub mod cli;
pub mod dump;
pub mod entities;
pub mod test;

fn main() {
    let args = Cli::parse();
    // println!("{:#?}", args);

    match args.command {
        Command::Dump(dump_args) => {
            let dump_args: dump::DumpArgs = dump_args.args.into();
            if let Err(err) = block_on(dump::handle_dump(dump_args)) {
                panic!("{}", err);
            }
        }
        Command::Replace(_replace) => {
            // skip
        }
    }
}
