use md5::{Digest, Md5};
use pyo3::prelude::*;
use sha2::Sha256;
use std::num::ParseIntError;


#[pyfunction]
#[pyo3(text_signature = "(time_str: str) -> int")]
pub(crate) fn date_from_cs_string(time_str: &str) -> i64 {
    let str_len = time_str.len();
    let date_time_str = &mut time_str[..&str_len - 7].to_owned();
    date_time_str.push_str(&time_str[&str_len - 6..]);
    let time = chrono::DateTime::parse_from_str(date_time_str.as_str(), "%Y-%m-%dT%H:%M:%S%.f%:z")
        .unwrap();
    time.timestamp_millis()
}

fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

/// Pads string to 4 length with 0 in front
#[pyfunction]
#[pyo3(text_signature = "(src_str: str) -> str")]
pub(crate) fn pad4(str: &str) -> String {
    format!("{:0>4}", str)
}

/// Pads string to 8 length with 0 in front
#[pyfunction]
#[pyo3(text_signature = "(src_str: str) -> str")]
pub(crate) fn pad8(str: &str) -> String {
    format!("{:0>8}", str)
}

/// Gets [to_hash, check] from a line based on algo
#[pyfunction]
#[pyo3(text_signature = "(line: str, index: int, algo: str) -> bool")]
pub(crate) fn validate_checksum_internal(line: &str, index: i32, alg: u32) -> bool {
    let (md5, sub) = match alg {
        0 => (true, 32),
        _ => (false, 16),
    };
    let last = line.len();
    let hash = decode_hex(&line[&last - &sub..]).unwrap();
    if md5 {
        let mut hasher = Md5::new();
        hasher.update(&line[..&last - &sub]);
        hasher.update(&index.to_string());
        hasher.finalize()[..] == hash
    } else {
        let mut hasher = Sha256::new();
        hasher.update(&line[..&last - &sub]);
        hasher.update(&index.to_string());
        hasher.finalize()[..8] == hash
    }
}

fn encode_hex(v: &Vec<u8>) -> String {
    v.iter().map(|u| {
        let a = get_char(u >> 4);
        let b = get_char(u & 0xf);
        a.to_string() + &b.to_string()
    }).collect::<String>()
}

fn get_char(u: u8) -> char {
    match u {
        0 => '0',
        1 => '1',
        2 => '2',
        3 => '3',
        4 => '4',
        5 => '5',
        6 => '6',
        7 => '7',
        8 => '8',
        9 => '9',
        10 => 'a',
        11 => 'b',
        12 => 'c',
        13 => 'd',
        14 => 'e',
        15 => 'f',
        _ => '0'
    }
}
