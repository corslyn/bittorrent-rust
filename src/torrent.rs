use serde::Deserialize;
use serde_bencode::{de, ser};
use serde_bytes::ByteBuf;
use sha1::{Digest, Sha1};
use std::fs::File;
use std::io::Read;

#[derive(Debug, Deserialize)]
pub struct Torrent {
    /// Tracker URL
    #[serde(default)]
    announce: String,

    info: Info,
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
    length: usize,
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
        println!("Info hash: {}", self.info_hash());
        println!("Piece length: {}", self.info.piece_length);
        println!("Piece Hashes:");

        for hash in self.pieces_hashes().iter() {
            println!("{hash}");
        }
    }

    fn info_hash(&self) -> String {
        let bencoded = ser::to_bytes(&self.info).unwrap();
        let mut hasher = Sha1::new();
        hasher.update(&bencoded);
        let result = hasher.finalize();

        hex::encode(result)
    }

    fn pieces_hashes(&self) -> Vec<String> {
        self.info
            .pieces
            .chunks(20) // SHA-1 hash length is 20 bytes
            .map(|chunk| hex::encode(chunk))
            .collect()
    }
}
