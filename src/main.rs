mod args;
use args::*;

use clap::Parser;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Decode { input }) => {
            todo!("Decode beencoded string");
        }
        _ => {}
    }
}
