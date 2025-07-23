use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;
use once_cell::sync::Lazy;

#[cfg(feature = "python")]
use pyo3::prelude::*;
#[cfg(feature = "python")]
use pyo3::wrap_pyfunction;

/// Global counter for assigning unique variable ids
static NEXT_ID: AtomicU64 = AtomicU64::new(0);
/// Mapping of variable id to its associated standard deviation
static SIGMAS: Lazy<Mutex<HashMap<u64, f64>>> = Lazy::new(|| Mutex::new(HashMap::new()));

/// Representation of a value with uncertainty
#[cfg_attr(feature = "python", pyclass)]
#[derive(Clone)]
pub struct UncertainValue {
    nominal: f64,
    derivatives: HashMap<u64, f64>,
}

impl UncertainValue {
    /// Create a new uncertain value from a nominal value and standard deviation
    pub fn new(nominal: f64, sigma: f64) -> Self {
        let id = NEXT_ID.fetch_add(1, Ordering::SeqCst);
        SIGMAS.lock().unwrap().insert(id, sigma);
        let mut d = HashMap::new();
        d.insert(id, 1.0);
        UncertainValue { nominal, derivatives: d }
    }

    fn combine(left: &HashMap<u64, f64>, right: &HashMap<u64, f64>, right_sign: f64) -> HashMap<u64, f64> {
        let mut out = left.clone();
        for (k, v) in right {
            *out.entry(*k).or_insert(0.0) += right_sign * v;
        }
        out
    }

    /// Internal addition used by both Rust and Python implementations
    fn add_internal(&self, other: &UncertainValue) -> UncertainValue {
        UncertainValue {
            nominal: self.nominal + other.nominal,
            derivatives: Self::combine(&self.derivatives, &other.derivatives, 1.0),
        }
    }

    /// Internal subtraction used by both Rust and Python implementations
    fn sub_internal(&self, other: &UncertainValue) -> UncertainValue {
        UncertainValue {
            nominal: self.nominal - other.nominal,
            derivatives: Self::combine(&self.derivatives, &other.derivatives, -1.0),
        }
    }

    fn mul_internal(&self, other: &UncertainValue) -> UncertainValue {
        let mut out = HashMap::new();
        for (k, v) in &self.derivatives {
            out.insert(*k, v * other.nominal);
        }
        for (k, v) in &other.derivatives {
            *out.entry(*k).or_insert(0.0) += v * self.nominal;
        }
        UncertainValue {
            nominal: self.nominal * other.nominal,
            derivatives: out,
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

impl std::ops::Add for &UncertainValue {
    type Output = UncertainValue;

    fn add(self, rhs: Self) -> Self::Output {
        self.add(rhs)
    }
}

impl std::ops::Sub for &UncertainValue {
    type Output = UncertainValue;

    fn sub(self, rhs: Self) -> Self::Output {
        self.sub(rhs)
    }
}

impl UncertainValue {
    /// Nominal value of this quantity
    pub fn nominal(&self) -> f64 {
        self.nominal
    }

    /// Standard deviation of this quantity
    pub fn stddev(&self) -> f64 {
        self.stddev_internal()
    }

    /// Add two uncertain values
    pub fn add(&self, other: &UncertainValue) -> UncertainValue {
        self.add_internal(other)
    }

    /// Subtract two uncertain values
    pub fn sub(&self, other: &UncertainValue) -> UncertainValue {
        self.sub_internal(other)
    }

    fn __mul__(&self, other: &UncertainValue) -> UncertainValue {
        self.mul_internal(other)
    }
}

/// Create a new uncertain value from a nominal value and standard deviation
#[cfg_attr(feature = "python", pyfunction)]
pub fn uval(nominal: f64, sigma: f64) -> UncertainValue {
    UncertainValue::new(nominal, sigma)
}

/// Get the nominal value of an uncertain quantity
#[cfg_attr(feature = "python", pyfunction)]
pub fn nominal(v: &UncertainValue) -> f64 {
    v.nominal()
}

/// Compute the standard deviation of an uncertain quantity
#[cfg_attr(feature = "python", pyfunction)]
pub fn stddev(v: &UncertainValue) -> f64 {
    v.stddev()
}

#[cfg(feature = "python")]
#[pymethods]
impl UncertainValue {
    #[pyo3(name = "nominal")]
    fn py_nominal(&self) -> f64 {
        self.nominal()
    }

    #[pyo3(name = "stddev")]
    fn py_stddev(&self) -> f64 {
        self.stddev()
    }

    fn __add__(&self, other: &UncertainValue) -> UncertainValue {
        self.add(other)
    }

    fn __sub__(&self, other: &UncertainValue) -> UncertainValue {
        self.sub(other)
    }
}

#[cfg(feature = "python")]
#[pymodule]
fn _properr(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(uval, m)?)?;
    m.add_function(wrap_pyfunction!(nominal, m)?)?;
    m.add_function(wrap_pyfunction!(stddev, m)?)?;
    m.add_class::<UncertainValue>()?;
    Ok(())
}
