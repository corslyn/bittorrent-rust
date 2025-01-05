use serde::Deserialize;
use serde_bencode::de;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Deserialize)]
pub struct Torrent {
    /// Tracker URL
    #[serde(default)]
    announce: String,

    info: Info,
}

#[derive(Debug, Deserialize)]
pub struct Info {
    /// suggested name to save the file / directory as
    #[serde(default)]
    name: String,

    /// number of bytes in each piece
    #[serde(rename = "piece length")]
    piece_length: usize,

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
    }
}
