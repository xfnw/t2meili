use serde::Serialize;
use std::{env::args, error::Error};

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

fn parse_torrent(filename: &str) -> Result<Info, Box<dyn Error>> {
    let torrent = Torrent::read_from_file(filename)?;

    let out = Info {
        hash: torrent.info_hash(),
        name: torrent.name.to_owned(),
        length: torrent.length,
        piece_length: torrent.piece_length,
        private: if torrent.is_private() { 1 } else { 0 },
        creation_date: None,
        comment: None,
        created_by: None,
        magnet: torrent.magnet_link().ok(),
    };

    Ok(out)
}

fn main() {
    for arg in args().skip(1) {
        match parse_torrent(&arg) {
            Ok(output) => {
                println!(
                    "{}",
                    serde_json::to_string(&output).expect("could not serialize")
                );
            }
            Err(e) => {
                eprintln!("warning: skipping {}: {}", &arg, e);
                continue;
            }
        }
    }
}
