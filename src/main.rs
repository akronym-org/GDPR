use crate::dump::{dump_entrypoint, DumpOptions};
use clap::Parser;
use cli::{Cli, Command};

use futures::executor::block_on;

pub mod cli;
pub mod config;
pub mod database;
pub mod dump;
pub mod entities;
pub mod manifest;
pub mod test;
pub mod utils;
pub mod wildcard;

fn main() {
    let args = Cli::parse();
    // println!("{:#?}", args);

    match args.command {
        Command::Dump(args) => {
            let mut options = DumpOptions::from(args);
            if let Err(err) = block_on(dump_entrypoint(&mut options)) {
                panic!("{}", err);
            }
        }
        Command::Replace(_replace) => {}
    }
}
