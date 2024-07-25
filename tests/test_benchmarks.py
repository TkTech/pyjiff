import pendulum
from jiff import Zoned


def test_pendulum(benchmark):
    benchmark(pendulum.now)


def test_jiff(benchmark):
    benchmark(Zoned.now)
