use std::net::SocketAddrV4;

use serde_bytes::ByteBuf;
use urlencoding::encode_binary;

use crate::torrent::Torrent;

#[derive(Debug, Deserialize)]
pub struct Peer {
    pub address: SocketAddrV4,
}

#[derive(Debug, Serialize)]
pub struct Handshake {
    length: usize,     // 19
    string: String,    // the string "BitTorrent protocol"
    reserved: ByteBuf, // 8 null bytes
    infohash: String,
    peer_id: String,
}

impl Handshake {
    pub fn new(torrent: &Torrent) -> Handshake {
        Handshake {
            length: 19,
            string: "BitTorrent protocol".to_string(),
            reserved: ByteBuf::from(vec![0; 8]),
            infohash: encode_binary(&torrent.info_hash()).to_string(),
            peer_id: Torrent::generate_client_id(),
        }
    }
}
