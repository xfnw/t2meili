use serde::Serialize;
use std::io;
use std::io::Read;

use lava_torrent::torrent::v1::Torrent;

#[derive(Serialize, Debug)]
struct Info {
    hash: String,
    name: String,
    length: i64,
    piece_length: i64,
    private: u8,
    creation_date: Option<i64>,
    comment: Option<String>,
    created_by: Option<String>,
    magnet: Option<String>,
}


fn main() {

    let torrent = Torrent::read_from_file("sample.torrent").unwrap();

    let output = Info {
        hash: torrent.info_hash(),
        name: torrent.name.to_owned(),
        length: torrent.length,
        piece_length: torrent.piece_length,
        private: if torrent.is_private() {1} else {0},
        creation_date: None,
        comment: None,
        created_by: None,
        magnet: torrent.magnet_link().ok(),
    };

    println!("{}", serde_json::to_string(&output).unwrap());
}
