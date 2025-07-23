use pyo3::prelude::*;

#[pyfunction]
fn add(a: f64, b: f64) -> f64 {
    a + b
}

#[pymodule]
fn _properr(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(add, m)?)?;
    Ok(())
}
