use std::net::{Ipv4Addr, SocketAddrV4};

use crate::{peer::Peer, torrent::Torrent};
use reqwest::{blocking::get, Url};
use serde_bencode::de;
use serde_bytes::ByteBuf;
use urlencoding::encode_binary;

#[derive(Debug)]
pub struct TrackerRequest {
    announce: String,

    /// The info hash of the torrent
    info_hash: String,

    /// Unique client id
    peer_id: String,

    /// Port the client is listening on
    port: u16,

    /// Total amount uploaded
    uploaded: usize,

    /// Total downloaded
    downloaded: usize,

    /// Number of bytes left to download
    left: usize,

    /// Compact representation
    compact: bool,
}

#[derive(Debug, Deserialize)]
pub struct TrackerResponse {
    #[serde(default)]
    interval: usize,

    // Peers as a byte string in compact format
    #[serde(rename = "peers")]
    peers_bin: ByteBuf,
}

impl TrackerRequest {
    pub fn new(torrent: &Torrent) -> TrackerRequest {
        TrackerRequest {
            announce: torrent.announce.clone(),
            info_hash: encode_binary(&torrent.info_hash()).to_string(),
            peer_id: Torrent::generate_client_id(),
            port: 6881,
            uploaded: 0,
            downloaded: 0,
            left: torrent.info.length,
            compact: true,
        }
    }

    pub fn request_peers(&self) -> Vec<Peer> {
        let mut tracker_url = Url::parse(&self.announce).unwrap();

        // Manually build the query string to avoid double encoding
        let query = format!(
            "info_hash={}&peer_id={}&port={}&uploaded={}&downloaded={}&left={}&compact={}",
            self.info_hash,
            self.peer_id,
            self.port,
            self.uploaded,
            self.downloaded,
            self.left,
            if self.compact { "1" } else { "0" }
        );

        tracker_url.set_query(Some(&query));

        let response = get(tracker_url).unwrap().bytes().unwrap();

        let response: TrackerResponse = de::from_bytes(&response).unwrap();
        response.decode_peers()
    }
}

impl TrackerResponse {
    pub fn decode_peers(&self) -> Vec<Peer> {
        self.peers_bin
            .chunks(6)
            .filter_map(|chunk| {
                if chunk.len() == 6 {
                    let ip = Ipv4Addr::new(chunk[0], chunk[1], chunk[2], chunk[3]);
                    let port = u16::from_be_bytes([chunk[4], chunk[5]]);
                    Some(Peer {
                        address: SocketAddrV4::new(ip, port),
                    })
                } else {
                    None
                }
            })
            .collect()
    }
}
