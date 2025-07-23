Here is the goal we are working towards:


# Modern Uncertainty Propagation Library: Design Overview

## Purpose

Create a modern, performant, and composable library for numerical uncertainty propagation that improves upon the classic Python `uncertainties` package. The library should:

* Support scalar and array values with uncertainties
* Propagate uncertainties through arbitrary arithmetic and common math functions
* Interoperate cleanly with `numpy`, `pint`, and optionally `sympy`

---

## Guiding Principles

* **Correctness First**: All uncertainty propagation must be mathematically sound (first-order Taylor).
* **Composability**: Users should be able to combine uncertain values freely, including within numpy arrays, expressions, etc.
* **Identity-aware**: Shared variables should preserve correlation.
* **Performance**: Vectorized operations should be fast; Rust is used for inner loops.
* **Minimal Boilerplate**: Python API should be as ergonomic as `ufloat(3.0, 0.1)`.

---

## Key Concepts

### 1. Value with Uncertainty

Each value is represented as a tuple:

```rust
struct UncertainValue {
    nominal: f64,
    derivatives: HashMap<VariableId, f64>,
}
```

* `nominal`: the central (mean) value
* `derivatives`: partial derivatives w\.r.t. underlying variables (used to propagate error)

### 2. Variable Identity

Each root uncertain value has a unique `VariableId`, so that when `x - x` occurs, the correlation is known and cancels uncertainty.

### 3. Arithmetic and Functions

All operations (add, mul, sin, etc.) follow autodiff-style rules:

* Compute new nominal value
* Use chain rule to compute new derivatives

### 4. Final Uncertainty

To compute the uncertainty (standard deviation) of any result:

```text
sigma^2 = sum_i (∂f/∂x_i)^2 * sigma_i^2
```

This is done lazily on demand.

---

## Python API (Sketch)

```python
from uncertainties_next import uval, nominal, stddev

x = uval(10.0, 1.0)       # 10.0 ± 1.0
y = uval(10.0, 1.0)       # independent of x
z = x - y                 # 0.0 ± sqrt(2)
z2 = x - x                # 0.0 ± 0.0

nominal(z)                # 0.0
stddev(z)                 # ~1.414
```

Optional extensions:

* `@uncertain_function` decorator for black-box propagation
* NumPy ufunc integration
* Pretty-printing for Jupyter

---

## Rust Core

* All core arithmetic and derivative propagation lives in Rust
* Exposed to Python via `maturin` / `PyO3`
* Uses `f64`, `FxHashMap`, etc.
* Optional feature: SIMD batch evaluation for arrays of values with same structure

---

## Stretch Goals

* Unit integration (`pint`, `ure`) — track and propagate units alongside uncertainties
* Symbolic wrapping: generate SymPy-style symbolic representations of uncertainty flows
* Serialization: JSON/binary format for uncertain values
* Higher-order uncertainty (include second derivatives or non-Gaussian modeling)

---

## Out of Scope (for now)

* Full autodiff engine
* Backprop / optimization tooling
* Monte Carlo uncertainty estimation
* GPU compute

---

## Open Questions

* Should arrays be first-class (like in NumPy) or layered on top?
* Is it worth integrating with `jax` or `torch` backends?
* Should variables be hashable Python objects or opaque IDs?

---

## Summary

This project aims to build a modern uncertainty propagation engine that:

* Tracks first-order error terms accurately
* Preserves correlations between expressions
* Offers Pythonic and high-performance Rust APIs
* Can serve as a foundation for scientific and engineering applications
