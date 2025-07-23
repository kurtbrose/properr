from importlib import import_module

# Import the Rust extension module packaged inside this package
_rust = import_module("._properr", package=__name__)

# Re-export functions from Rust
uval = _rust.uval
uvals = _rust.uvals
nominal = _rust.nominal
nominals = _rust.nominals
stddev = _rust.stddev
stddevs = _rust.stddevs
sin = _rust.sin
cos = _rust.cos
exp = _rust.exp
ln = _rust.ln
sqrt = _rust.sqrt
_from_parts = _rust.from_parts
UncertainValue = _rust.UncertainValue

__all__ = [
    "uval",
    "uvals",
    "nominal",
    "nominals",
    "stddev",
    "stddevs",
    "sin",
    "cos",
    "exp",
    "ln",
    "sqrt",
    "UncertainValue",
    "uncertain_function",
]


def uncertain_function(func, *, epsilon=1e-8):
    """Decorate a plain Python function for uncertainty propagation.

    The wrapped function is evaluated on the nominal values of any
    ``UncertainValue`` arguments and uses finite differences to
    approximate partial derivatives. These derivatives are combined with
    the derivatives of the inputs so that correlations are preserved.
    """

    def wrapper(*args):
        nominals = []
        deriv_maps = []
        for a in args:
            if isinstance(a, UncertainValue):
                nominals.append(a.nominal())
                deriv_maps.append(a._derivatives())
            else:
                nominals.append(a)
                deriv_maps.append(None)

        result_nom = func(*nominals)

        out: dict[int, float] = {}
        for i, dmap in enumerate(deriv_maps):
            if dmap is None:
                continue
            plus = nominals.copy()
            minus = nominals.copy()
            plus[i] += epsilon
            minus[i] -= epsilon
            f_plus = func(*plus)
            f_minus = func(*minus)
            partial = (f_plus - f_minus) / (2 * epsilon)
            for vid, d in dmap.items():
                out[vid] = out.get(vid, 0.0) + partial * d

        return _from_parts(result_nom, out)

    return wrapper
