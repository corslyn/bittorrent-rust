use serde_bencode::{de, ser};
use serde_bytes::ByteBuf;
use sha1::{Digest, Sha1};
use std::fs::File;
use std::io::Read;

use crate::tracker::TrackerRequest;

#[derive(Debug, Deserialize)]
pub struct Torrent {
    /// Tracker URL
    #[serde(default)]
    pub announce: String,

    pub info: Info,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Info {
    /// suggested name to save the file / directory as
    #[serde(default)]
    name: String,

    /// number of bytes in each piece
    #[serde(rename = "piece length")]
    piece_length: usize,

    pieces: ByteBuf,

    /// Size of the file in bytes, for single-file torrents
    #[serde(default)]
    pub length: usize,
}

impl Torrent {
    /// Reads and parses a torrent file from disk
    pub fn new(file_name: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut file = File::open(file_name)?;
        let mut contents = Vec::new();
        file.read_to_end(&mut contents)?;

        let torrent: Torrent = de::from_bytes(&contents)?;
        Ok(torrent)
    }

    /// Prints basic torrent info
    pub fn print_info(&self) {
        println!("Tracker URL: {}", self.announce);
        println!("Length: {} bytes", self.info.length);
        println!("Info hash: {}", hex::encode(self.info_hash()));
        println!("Piece length: {}", self.info.piece_length);
        println!("Piece Hashes:");

        for hash in self.pieces_hashes().iter() {
            println!("{hash}");
        }
    }
    pub fn info_hash(&self) -> Vec<u8> {
        let bencoded = ser::to_bytes(&self.info).unwrap();
        let mut hasher = Sha1::new();
        hasher.update(&bencoded);
        hasher.finalize().to_vec()
    }

    fn pieces_hashes(&self) -> Vec<String> {
        self.info
            .pieces
            .chunks(20) // SHA-1 hash length is 20 bytes
            .map(|chunk| hex::encode(chunk))
            .collect()
    }

    pub fn get_peers(&self) {
        let tracker_request = TrackerRequest::new(&self);
        let peers = tracker_request.request_peers();
        for peer in peers {
            println!("{}:{}", peer.ip, peer.port);
        }
    }
}
