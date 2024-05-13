use iregexp::check;
use pyo3::prelude::*;

// TODO: docs
#[pyfunction]
#[pyo3(name = "check")]
fn py_check(pattern: &str) -> bool {
    check(pattern)
}

#[pymodule]
fn i_regexp(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(py_check, m)?)?;
    Ok(())
}
