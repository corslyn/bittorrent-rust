use std::net::SocketAddrV4;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Outputs torrent file info (tracker url & length)
    Info {
        /// The torrent file
        input_file: String,
    },

    /// Outputs peers (ip:port)
    Peers {
        /// The torrent file
        input_file: String,
    },

    Handshake {
        /// The torrent file
        input_file: String,

        /// The peer to perform the handshake to (127.0.0.1:1234)
        peer: SocketAddrV4,
    },
}
