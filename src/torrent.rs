use rand::distributions::Alphanumeric;
use rand::Rng;
use serde_bencode::{de, ser};
use serde_bytes::ByteBuf;
use sha1::{Digest, Sha1};
use std::fs::File;
use std::io::{Read, Write};
use std::net::{SocketAddrV4, TcpStream};

use crate::peer::Handshake;
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
            println!("{}", peer.address);
        }
    }

    pub fn handshake_with_peer(&self, peer_address: &SocketAddrV4) {
        // Step 1: Create the handshake message
        let handshake = Handshake::new(self);
        let handshake_bytes = {
            let mut bytes = Vec::new();
            bytes.push(handshake.length as u8); // Add length of protocol string
            bytes.extend_from_slice(handshake.string.as_bytes()); // Add protocol string
            bytes.extend_from_slice(&handshake.reserved); // Add reserved bytes
            bytes.extend_from_slice(&handshake.infohash); // Add info hash
            bytes.extend_from_slice(handshake.peer_id.as_bytes()); // Add peer ID
            bytes
        };

        // Step 2: Connect to the peer
        let mut stream = TcpStream::connect(peer_address).unwrap();

        // Step 3: Send the handshake message
        stream.write_all(&handshake_bytes).unwrap();

        println!("Sent handshake to peer: {}", peer_address);

        // Step 4: Receive the response handshake
        let mut response = [0u8; 68]; // Handshake is always 68 bytes
        stream.read_exact(&mut response).unwrap();

        // Step 5: Validate the response
        // Extract the info hash from the response and compare it
        let received_infohash = &response[28..48]; // Info hash starts at byte 28 and is 20 bytes
        if received_infohash != handshake.infohash.as_ref() {
            panic!("Info hash mismatch! Peer is not serving the same torrent.");
        }

        println!(
            "Handshake successful with peer: {} (Info hash validated)",
            peer_address
        );
        let peer_id = hex::encode(&response[48..68]);
        println!("Remote peer id: {}", peer_id)
    }

    pub fn generate_client_id() -> String {
        let prefix = "-RS1337-";
        let random_suffix: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(12)
            .map(char::from)
            .collect();
        let client_id = format!("{}{}", prefix, random_suffix);

        println!("Client ID: {}", client_id);
        client_id
    }
}
