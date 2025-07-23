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
