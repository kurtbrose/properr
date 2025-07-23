import math
import properr
import pytest

@properr.uncertain_function
def f(x, y):
    return math.sin(x) + y * y


def test_decorator_nominal_and_stddev():
    x = properr.uval(0.0, 1.0)
    y = properr.uval(3.0, 0.5)
    z = f(x, y)

    assert properr.nominal(z) == pytest.approx(math.sin(0.0) + 9.0)
    expected_sigma = (1.0 ** 2 * 1.0 ** 2 + (2 * 3.0) ** 2 * 0.5 ** 2) ** 0.5
    assert properr.stddev(z) == pytest.approx(expected_sigma)
