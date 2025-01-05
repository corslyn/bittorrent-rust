#[macro_use]
extern crate serde_derive;

mod args;
mod torrent;
mod tracker;

use args::*;

use clap::Parser;
use torrent::Torrent;

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Info { input_file }) => {
            match Torrent::new(&input_file) {
                Ok(torrent) => torrent.print_info(),
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            };
        }
        Some(Commands::Peers { input_file }) => {
            match Torrent::new(&input_file) {
                Ok(torrent) => torrent.get_peers(),
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            };
        }

        _ => {}
    }
}
