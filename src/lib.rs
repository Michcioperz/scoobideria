use lalrpop_util::lalrpop_mod;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3::exceptions::PyValueError;

lalrpop_mod!(pub grammar);

#[pyfunction]
fn answer(query: String) -> PyResult<String> {
    grammar::QueryParser::new()
        .parse(&query)
        .map_err(|e| PyValueError::new_err(format!("{}", e)))
}

#[pymodule]
fn scoobideria(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(answer, m)?)?;
    Ok(())
}
