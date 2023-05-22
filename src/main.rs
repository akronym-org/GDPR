use crate::dump::{dump_entrypoint, DumpOptions};
use anyhow;
use clap::Parser;
use cli::{Cli, Command};

use futures::executor::block_on;

pub mod cli;
pub mod config;
pub mod directus;
pub mod dump;
pub mod entities;
pub mod graph;
pub mod manifest;
pub mod reversed_permissions;
pub mod utils;
pub mod wildcard;

fn main() -> anyhow::Result<()> {
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
    Ok(())
}
