import properr
import pytest


def test_uncertain_arithmetic():
    x = properr.uval(10.0, 1.0)
    y = properr.uval(10.0, 1.0)

    z = x - y
    z2 = x - x

    assert properr.nominal(z) == 0.0
    assert properr.stddev(z) == pytest.approx(2**0.5)
    assert properr.stddev(z2) == 0.0

    # object methods mirror the module-level helpers
    assert z.nominal() == 0.0
    assert z.stddev() == pytest.approx(2**0.5)


def test_uncertain_multiplication():
    x = properr.uval(10.0, 1.0)
    y = properr.uval(10.0, 1.0)

    z = x * y
    z2 = x * x

    assert properr.nominal(z) == 100.0
    assert properr.stddev(z) == pytest.approx((200) ** 0.5)
    assert properr.stddev(z2) == pytest.approx(20.0)


def test_uncertain_division():
    x = properr.uval(10.0, 1.0)
    y = properr.uval(2.0, 0.2)

    z = x / y
    z2 = x / x

    assert properr.nominal(z) == 5.0
    assert properr.stddev(z) == pytest.approx(0.5**0.5)
    assert properr.nominal(z2) == 1.0
    assert properr.stddev(z2) == 0.0


def test_uncertain_sine():
    x = properr.uval(0.0, 1.0)
    y = properr.sin(x)

    assert properr.nominal(y) == 0.0
    assert properr.stddev(y) == 1.0


def test_uncertain_cosine():
    x = properr.uval(0.0, 1.0)
    y = properr.cos(x)

    assert properr.nominal(y) == 1.0
    assert properr.stddev(y) == 0.0


def test_uncertain_sqrt():
    x = properr.uval(4.0, 0.5)
    y = properr.sqrt(x)

    assert properr.nominal(y) == 2.0
    assert properr.stddev(y) == pytest.approx(0.125)


def test_uncertain_exp():
    x = properr.uval(1.0, 0.1)
    y = properr.exp(x)

    assert properr.nominal(y) == pytest.approx(2.718281828459045)
    # derivative is e^x -> variance = (e^1 * 0.1)^2 = (0.2718281828)^2
    assert properr.stddev(y) == pytest.approx((2.718281828459045 * 0.1))
