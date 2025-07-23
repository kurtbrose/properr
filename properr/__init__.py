from importlib import import_module

# Import the Rust extension module
_rust = import_module("_properr")

# Re-export functions from Rust
add = _rust.add

__all__ = ["add"]
