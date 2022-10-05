use md5::{Digest, Md5};
use pyo3::prelude::*;
use sha2::Sha256;
use std::num::ParseIntError;
use std::fmt;

#[pyfunction]
#[pyo3(text_signature = "(time_str: str) -> int")]
pub(crate) fn date_from_act_timestamp(time_str: &str) -> i64 {
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

pub(crate) fn parse_float(inp: &str) -> f32 {
    f32::from_bits(parse_int(inp))
}

pub(crate) fn parse_int(inp: &str) -> u32 {
    inp.parse::<u32>().unwrap_or(0)
}

/// Gets [to_hash, check] from a line based on algo
#[pyfunction]
#[pyo3(text_signature = "(line: str, index: int, algo: str) -> bool")]
pub(crate) fn validate_checksum(line: &str, index: i32, alg: &str) -> bool {
    let (md5, sub) = match alg {
        "md5" => (true, 32),
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

fn encode_hex<T: AsRef<[u8]>>(data: T) -> String {
    let mut s = String::with_capacity(data.as_ref().len() * 2);

    // Writing to a string never errors, so we can unwrap here.
    data.write_hex_upper(&mut s).unwrap();
    s
}

trait ToHex {
    /// Writes the hex string representing `self` into `w`. Lower case letters
    /// are used (e.g. `f9b4ca`).
    fn write_hex<W: fmt::Write>(&self, w: &mut W) -> fmt::Result;

    /// Writes the hex string representing `self` into `w`. Upper case letters
    /// are used (e.g. `F9B4CA`).
    fn write_hex_upper<W: fmt::Write>(&self, w: &mut W) -> fmt::Result;
}


impl<T: AsRef<[u8]>> ToHex for T {
    fn write_hex<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        static CHARS: &'static [u8] = b"0123456789abcdef";

        for &byte in self.as_ref().iter() {
            w.write_char(CHARS[(byte >>  4) as usize].into())?;
            w.write_char(CHARS[(byte & 0xf) as usize].into())?;
        }

        Ok(())
    }

    fn write_hex_upper<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        static CHARS: &'static [u8] = b"0123456789ABCDEF";

        for &byte in self.as_ref().iter() {
            w.write_char(CHARS[(byte >>  4) as usize].into())?;
            w.write_char(CHARS[(byte & 0xf) as usize].into())?;
        }

        Ok(())
    }
}
