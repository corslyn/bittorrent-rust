use std::net::SocketAddrV4;

use serde_bytes::ByteBuf;
use urlencoding::encode_binary;

use crate::torrent::Torrent;

#[derive(Debug, Deserialize)]
pub struct Peer {
    pub address: SocketAddrV4,
}

#[derive(Debug)]
pub struct Handshake {
    pub length: usize,     // 19
    pub string: String,    // the string "BitTorrent protocol"
    pub reserved: ByteBuf, // 8 null bytes
    pub infohash: ByteBuf, // sha1 hash (20 bytes)
    pub peer_id: String,
}

impl Handshake {
    pub fn new(torrent: &Torrent) -> Handshake {
        Handshake {
            length: 19,
            string: "BitTorrent protocol".to_string(),
            reserved: ByteBuf::from(vec![0; 8]),
            infohash: ByteBuf::from(torrent.info_hash()),
            peer_id: Torrent::generate_client_id(),
        }
    }
}
