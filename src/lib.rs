use pyo3::prelude::*;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;
use once_cell::sync::Lazy;

/// Global counter for assigning unique variable ids
static NEXT_ID: AtomicU64 = AtomicU64::new(0);
/// Mapping of variable id to its associated standard deviation
static SIGMAS: Lazy<Mutex<HashMap<u64, f64>>> = Lazy::new(|| Mutex::new(HashMap::new()));

/// Representation of a value with uncertainty
#[pyclass]
#[derive(Clone)]
struct UncertainValue {
    nominal: f64,
    derivatives: HashMap<u64, f64>,
}

impl UncertainValue {
    fn combine(
        left: &HashMap<u64, f64>,
        right: &HashMap<u64, f64>,
        right_sign: f64,
    ) -> HashMap<u64, f64> {
        let mut out = left.clone();
        for (k, v) in right {
            *out.entry(*k).or_insert(0.0) += right_sign * v;
        }
        out
    }

    fn add_internal(&self, other: &UncertainValue) -> UncertainValue {
        UncertainValue {
            nominal: self.nominal + other.nominal,
            derivatives: Self::combine(&self.derivatives, &other.derivatives, 1.0),
        }
    }

    fn sub_internal(&self, other: &UncertainValue) -> UncertainValue {
        UncertainValue {
            nominal: self.nominal - other.nominal,
            derivatives: Self::combine(&self.derivatives, &other.derivatives, -1.0),
        }
    }

    fn stddev_internal(&self) -> f64 {
        let sigmas = SIGMAS.lock().unwrap();
        let mut var: f64 = 0.0;
        for (id, deriv) in &self.derivatives {
            if let Some(sigma) = sigmas.get(id) {
                var += deriv * deriv * sigma * sigma;
            }
        }
        var.sqrt()
    }
}

#[pyfunction]
fn add(a: f64, b: f64) -> f64 {
    a + b
}

#[pyfunction]
fn uval(nominal: f64, sigma: f64) -> PyResult<UncertainValue> {
    let id = NEXT_ID.fetch_add(1, Ordering::SeqCst);
    SIGMAS.lock().unwrap().insert(id, sigma);
    let mut d = HashMap::new();
    d.insert(id, 1.0);
    Ok(UncertainValue { nominal, derivatives: d })
}

#[pyfunction]
fn nominal(v: &UncertainValue) -> f64 {
    v.nominal
}

#[pyfunction]
fn stddev(v: &UncertainValue) -> PyResult<f64> {
    Ok(v.stddev_internal())
}

#[pymethods]
impl UncertainValue {
    fn nominal(&self) -> f64 {
        self.nominal
    }

    fn stddev(&self) -> f64 {
        self.stddev_internal()
    }

    fn __add__(&self, other: &UncertainValue) -> UncertainValue {
        self.add_internal(other)
    }

    fn __sub__(&self, other: &UncertainValue) -> UncertainValue {
        self.sub_internal(other)
    }
}

#[pymodule]
fn _properr(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(add, m)?)?;
    m.add_function(wrap_pyfunction!(uval, m)?)?;
    m.add_function(wrap_pyfunction!(nominal, m)?)?;
    m.add_function(wrap_pyfunction!(stddev, m)?)?;
    m.add_class::<UncertainValue>()?;
    Ok(())
}
