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

macro_rules! parse_extra_fields {
    ($extra:expr, $(($v:expr,$n:expr)),*) => {
        $({
        $v = match $extra.get($n) {
            // TODO: find a way to replace BencodeElem::String with
            // a macro argument, so that the macro can also be used
            // for created_by, since it is an integer
            Some(BencodeElem::String(h)) => Some(h).cloned(),
            _ => None,
        };
        })*
    };
}

fn parse_torrent(filename: &str) -> Result<Info, Box<dyn Error>> {
    let torrent = Torrent::read_from_file(filename)?;

    let mut creation_date = None;
    let mut created_by = None;
    let mut encoding = None;
    let mut comment = None;

    if let Some(ref extra) = torrent.extra_fields {
        creation_date = match extra.get("creation date") {
            Some(BencodeElem::Integer(h)) => Some(h).copied(),
            _ => None,
        };

        parse_extra_fields!(
            extra,
            (created_by, "created by"),
            (encoding, "encoding"),
            (comment, "comment")
        );
    };

    let out = Info {
        hash: torrent.info_hash(),
        name: torrent.name.to_owned(),
        length: torrent.length,
        piece_length: torrent.piece_length,
        private: if torrent.is_private() { 1 } else { 0 },
        magnet: torrent.magnet_link().ok(),
        creation_date,
        created_by,
        encoding,
        comment,
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
