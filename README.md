# properr

A modern error propagation library.

This repository contains both a Python package and a Rust core powered by [PyO3](https://pyo3.rs/) and [maturin](https://github.com/PyO3/maturin).

The directory layout is:

```
Cargo.toml        # Rust crate definition
pyproject.toml    # Python packaging configuration using maturin
src/              # Rust source code
properr/          # Python package
```

Run `maturin develop` to build and install the package in development mode.

## Rust crate

The crate can be used directly in Rust projects without pulling in the Python
dependencies. The Python bindings are gated behind the `python` cargo feature:

```toml
[dependencies]
properr = { path = ".." }

# To enable the Python bindings when building the extension module
# properr = { version = "0.1", features = ["python"] }
```

Example usage of the Rust API:

```rust
use properr::UncertainValue;

let x = UncertainValue::new(10.0, 1.0);
let y = UncertainValue::new(5.0, 0.5);
let z = &x - &y;

assert_eq!(z.nominal(), 5.0);
```

### Python bindings

When built with the `python` feature (e.g. via `maturin develop`), the same
functionality is available from Python:

```python
import properr

x = properr.uval(10.0, 1.0)
y = properr.uval(5.0, 0.5)
z = x - y

assert z.nominal() == 5.0
```

## Current Functionality

- Scalar uncertain values (`uval`) with automatic correlation tracking
- Addition, subtraction, multiplication, and division of `UncertainValue` instances
- Propagation through the sine function
- Propagation through the cosine function
- Propagation through the square root function
- Propagation through the exponential function
- Propagation through the natural logarithm function
- Shared Rust and Python APIs for high performance
- Creation of arrays of `UncertainValue` instances with `uvals`
- NumPy ufunc integration for arithmetic and math functions
- Decorator for black-box Python functions with `@uncertain_function`
