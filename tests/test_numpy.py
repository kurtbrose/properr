import numpy as np
import properr
import pytest


def test_numpy_add():
    x = properr.uval(1.0, 0.1)
    y = properr.uval(2.0, 0.2)
    z = np.add(x, y)
    assert isinstance(z, properr.UncertainValue)
    assert z.nominal() == 3.0
    assert z.stddev() == pytest.approx((0.1**2 + 0.2**2) ** 0.5)


def test_numpy_sin():
    x = properr.uval(0.0, 1.0)
    y = np.sin(x)
    assert isinstance(y, properr.UncertainValue)
    assert y.nominal() == 0.0
    assert y.stddev() == pytest.approx(1.0)
