use crate::parser;
use pyo3::prelude::*;

/// Get actor type from tuple
#[pyfunction]
#[pyo3(text_signature = "(name_id_pair: list[str]) -> (int, str)")]
pub(crate) fn parse_actor(inp: Vec<&str>) -> (u32, &str) {
    (
        parser::u32_from_param(inp.first().unwrap()),
        inp.last().unwrap(),
    )
}
