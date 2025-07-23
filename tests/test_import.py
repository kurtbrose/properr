import properr
import pytest


def test_uncertain_arithmetic():
    x = properr.uval(10.0, 1.0)
    y = properr.uval(10.0, 1.0)

    z = x - y
    z2 = x - x

    assert properr.nominal(z) == 0.0
    assert properr.stddev(z) == pytest.approx(2 ** 0.5)
    assert properr.stddev(z2) == 0.0

    # object methods mirror the module-level helpers
    assert z.nominal() == 0.0
    assert z.stddev() == pytest.approx(2 ** 0.5)
