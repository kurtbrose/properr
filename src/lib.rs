use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::f64;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;

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
        UncertainValue {
            nominal,
            derivatives: d,
        }
    }

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

    fn div_internal(&self, other: &UncertainValue) -> UncertainValue {
        let mut out = HashMap::new();
        for (k, v) in &self.derivatives {
            out.insert(*k, v * other.nominal);
        }
        for (k, v) in &other.derivatives {
            *out.entry(*k).or_insert(0.0) -= v * self.nominal;
        }
        let denom_sq = other.nominal * other.nominal;
        for val in out.values_mut() {
            *val /= denom_sq;
        }
        UncertainValue {
            nominal: self.nominal / other.nominal,
            derivatives: out,
        }
    }

    fn sin_internal(&self) -> UncertainValue {
        let nominal = self.nominal.sin();
        let factor = self.nominal.cos();
        let mut out = HashMap::new();
        for (k, v) in &self.derivatives {
            out.insert(*k, v * factor);
        }
        UncertainValue {
            nominal,
            derivatives: out,
        }
    }

    fn cos_internal(&self) -> UncertainValue {
        let nominal = self.nominal.cos();
        let factor = -self.nominal.sin();
        let mut out = HashMap::new();
        for (k, v) in &self.derivatives {
            out.insert(*k, v * factor);
        }
        UncertainValue {
            nominal,
            derivatives: out,
        }
    }

    fn sqrt_internal(&self) -> UncertainValue {
        let nominal = self.nominal.sqrt();
        let factor = 0.5 / nominal;
        let mut out = HashMap::new();
        for (k, v) in &self.derivatives {
            out.insert(*k, v * factor);
        }
        UncertainValue {
            nominal,
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

impl std::ops::Mul for &UncertainValue {
    type Output = UncertainValue;

    fn mul(self, rhs: Self) -> Self::Output {
        self.mul(rhs)
    }
}

impl std::ops::Div for &UncertainValue {
    type Output = UncertainValue;

    fn div(self, rhs: Self) -> Self::Output {
        self.div(rhs)
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

    /// Multiply two uncertain values
    pub fn mul(&self, other: &UncertainValue) -> UncertainValue {
        self.mul_internal(other)
    }

    /// Divide two uncertain values
    pub fn div(&self, other: &UncertainValue) -> UncertainValue {
        self.div_internal(other)
    }

    /// Sine of an uncertain value
    pub fn sin(&self) -> UncertainValue {
        self.sin_internal()
    }

    /// Cosine of an uncertain value
    pub fn cos(&self) -> UncertainValue {
        self.cos_internal()
    }

    /// Square root of an uncertain value
    pub fn sqrt(&self) -> UncertainValue {
        self.sqrt_internal()
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

/// Compute the sine of an uncertain quantity
#[cfg_attr(feature = "python", pyfunction)]
pub fn sin(v: &UncertainValue) -> UncertainValue {
    v.sin()
}

/// Compute the cosine of an uncertain quantity
#[cfg_attr(feature = "python", pyfunction)]
pub fn cos(v: &UncertainValue) -> UncertainValue {
    v.cos()
}

/// Compute the square root of an uncertain quantity
#[cfg_attr(feature = "python", pyfunction)]
pub fn sqrt(v: &UncertainValue) -> UncertainValue {
    v.sqrt()
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

    fn __mul__(&self, other: &UncertainValue) -> UncertainValue {
        self.mul(other)
    }

    fn __truediv__(&self, other: &UncertainValue) -> UncertainValue {
        self.div(other)
    }

    #[pyo3(name = "sin")]
    fn py_sin(&self) -> UncertainValue {
        self.sin()
    }

    #[pyo3(name = "cos")]
    fn py_cos(&self) -> UncertainValue {
        self.cos()
    }

    #[pyo3(name = "sqrt")]
    fn py_sqrt(&self) -> UncertainValue {
        self.sqrt()
    }
}

#[cfg(feature = "python")]
#[pymodule]
fn _properr(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(uval, m)?)?;
    m.add_function(wrap_pyfunction!(nominal, m)?)?;
    m.add_function(wrap_pyfunction!(stddev, m)?)?;
    m.add_function(wrap_pyfunction!(sin, m)?)?;
    m.add_function(wrap_pyfunction!(cos, m)?)?;
    m.add_function(wrap_pyfunction!(sqrt, m)?)?;
    m.add_class::<UncertainValue>()?;
    Ok(())
}
