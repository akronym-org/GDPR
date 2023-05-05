use clap::Parser;
use cli::{Cli, Command};
use crate::dump::{DumpOptions, dump_entrypoint};

use futures::executor::block_on;


pub mod cli;
pub mod dump;
pub mod entities;
pub mod utils;
pub mod test;

fn main() {
    let args = Cli::parse();
    // println!("{:#?}", args);

    match args.command {
        Command::Dump(args) => {
            let options = DumpOptions::from(args);
            if let Err(err) = block_on(dump_entrypoint(&options)) {
                panic!("{}", err);
            }
        }
        Command::Replace(_replace) => { }
    }
}
