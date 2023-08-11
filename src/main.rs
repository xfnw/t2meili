use serde::Serialize;
use std::{env::args, error::Error};

use lava_torrent::{bencode::BencodeElem, torrent::v1::Torrent};

#[derive(Serialize, Debug)]
struct Info {
    hash: String,
    name: String,
    length: i64,
    piece_length: i64,
    private: u8,
    creation_date: Option<i64>,
    created_by: Option<String>,
    encoding: Option<String>,
    comment: Option<String>,
    magnet: Option<String>,
}

fn parse_torrent(filename: &str) -> Result<Info, Box<dyn Error>> {
    let torrent = Torrent::read_from_file(filename)?;

    let mut creation_date = None;
    let mut created_by = None;
    let mut encoding = None;
    let mut comment = None;

    // extra Some({"creation date": Integer(1618518382), "created by": String("Transmission/3.00 (bb6b5a062e)"), "comment": String("cans custom arch image"), "encoding": String("UTF-8")})
    if let Some(ref extra) = torrent.extra_fields {
        creation_date = match extra.get("creation date") {
            Some(h) => match h {
                BencodeElem::Integer(h) => Some(h).cloned(),
                _ => None,
            },
            None => None,
        };
        created_by = match extra.get("created by") {
            Some(h) => match h {
                BencodeElem::String(h) => Some(h).cloned(),
                _ => None,
            },
            None => None,
        };
    };

    let out = Info {
        hash: torrent.info_hash(),
        name: torrent.name.to_owned(),
        length: torrent.length,
        piece_length: torrent.piece_length,
        private: if torrent.is_private() { 1 } else { 0 },
        magnet: torrent.magnet_link().ok(),
        creation_date: creation_date,
        created_by: created_by,
        encoding: encoding,
        comment: comment,
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
