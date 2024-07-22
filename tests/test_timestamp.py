from jiff import Timestamp


def test_timestamp_from_second():
    ts = Timestamp.from_second(0)
    assert str(ts) == "1970-01-01T00:00:00Z"


def test_timestamp_from_millisecond():
    ts = Timestamp.from_millisecond(0)
    assert str(ts) == "1970-01-01T00:00:00Z"


def test_timestamp_from_microsecond():
    ts = Timestamp.from_microsecond(0)
    assert str(ts) == "1970-01-01T00:00:00Z"


def test_timestamp_from_nanosecond():
    ts = Timestamp.from_nanosecond(0)
    assert str(ts) == "1970-01-01T00:00:00Z"
