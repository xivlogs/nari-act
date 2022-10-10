use pyo3::prelude::*;
use sha2::{Digest, Sha256};
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

/// Gets [to_hash, check] from a line based on algo
#[pyfunction]
#[pyo3(text_signature = "(line: str, index: int) -> bool")]
pub(crate) fn validate_checksum_internal(line: &str, index: i32) -> bool {
    let last = line.len();
    let hash = decode_hex(&line[&last - 16..]).unwrap();
    let mut hasher = Sha256::new();
    hasher.update(&line[..&last - 16]);
    hasher.update(&index.to_string());
    hasher.finalize()[..8] == hash
}
