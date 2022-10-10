mod parser;
mod utils;
mod number_parser;

use pyo3::prelude::*;
use pyo3::types::PyDict;


/// Main module for rust extensions with nari-act
#[pymodule]
fn nari_act_rust(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parser::ability_from_params, m)?)?;
    m.add_function(wrap_pyfunction!(parser::statuslist_from_params, m)?)?;

    let utils_mod = PyModule::new(py, "utils")?;
    utils_mod.add_function(wrap_pyfunction!(utils::date_from_cs_string, m)?)?;
    utils_mod.add_function(wrap_pyfunction!(utils::validate_checksum_internal, m)?)?;
    m.add_submodule(utils_mod)?;

    // HACK: abuse python imports to make `from rustext.utils import validate_checksum` work
    // https://github.com/PyO3/pyo3/issues/759#issuecomment-977835119 For PyO3 v0.16.x <=
    // https://github.com/PyO3/pyo3/issues/2644#issuecomment-1259721976 For PyO3 v0.17.x >=
    let sys: &PyModule = py.import("sys").unwrap();
    let sys_modules: &PyDict = sys.getattr("modules")?.downcast()?;
    sys_modules.set_item("nari_act_rust.utils", m.getattr("utils")?)?;

    Ok(())
}
