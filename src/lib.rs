mod actor;
mod lines_to_params;
mod parser;
mod utils;

use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::wrap_pymodule;

/// Parser sub module in nari-act-rust
#[pymodule]
fn parser_mod(py: Python, m: &PyModule) -> PyResult<()>{
    m.add_function(wrap_pyfunction!(parser::f32_from_param, m)?)?;
    m.add_function(wrap_pyfunction!(parser::join_params_pad, m)?)?;
    m.add_function(wrap_pyfunction!(parser::u8x4_from_param, m)?)?;
    m.add_function(wrap_pyfunction!(parser::u16_from_param, m)?)?;
    m.add_function(wrap_pyfunction!(parser::u16x2_from_param, m)?)?;
    m.add_function(wrap_pyfunction!(parser::u32_from_param, m)?)?;
    m.add_function(wrap_pyfunction!(parser::u64_from_param, m)?)?;

    Ok(())
}

/// Utils sub module in nari-act-rust
#[pymodule]
fn utils_mod(py: Python, m: &PyModule) -> PyResult<()>{
    m.add_function(wrap_pyfunction!(utils::date_from_cs_string, m)?)?;
    m.add_function(wrap_pyfunction!(utils::pad4, m)?)?;
    m.add_function(wrap_pyfunction!(utils::pad8, m)?)?;
    m.add_function(wrap_pyfunction!(utils::validate_checksum, m)?)?;

    Ok(())
}


/// Main module for rust extensions with nari-act
#[pymodule]
fn nari_act_rust(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(actor::parse_actor, m)?)?;
    m.add_function(wrap_pyfunction!(lines_to_params::ability_from_params, m)?)?;
    m.add_function(wrap_pyfunction!(lines_to_params::action_effect_from_params, m)?)?;
    m.add_function(wrap_pyfunction!(lines_to_params::status_effect_from_params, m)?)?;
    m.add_function(wrap_pyfunction!(lines_to_params::statuslist_from_params, m)?)?;

    m.add_wrapped(wrap_pymodule!(parser_mod))?;
    m.add_wrapped(wrap_pymodule!(utils_mod))?;

    // HACK: abuse python imports to make `from rustext.utils import validate_checksum` work
    // https://github.com/PyO3/pyo3/issues/759#issuecomment-977835119 For PyO3 v0.16.x <=
    // https://github.com/PyO3/pyo3/issues/2644#issuecomment-1259721976 For PyO3 v0.17.x >=
    let sys: &PyModule = py.import("sys").unwrap();
    let sys_modules: &PyDict = sys.getattr("modules")?.downcast()?;
    sys_modules.set_item("nari_act_rust.parser", m.getattr("parser")?)?;
    sys_modules.set_item("nari_act_rust.utils", m.getattr("utils")?)?;

    Ok(())
}
