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
