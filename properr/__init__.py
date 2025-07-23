from importlib import import_module

# Import the Rust extension module packaged inside this package
_rust = import_module("._properr", package=__name__)

# Re-export functions from Rust
add = _rust.add

__all__ = ["add"]
