from jiff import Zoned, Timestamp, Span

def test_from_string():
    zoned = Zoned.from_string("2024-06-19 15:22[America/New_York]")
    assert str(zoned) == "2024-06-19T15:22:00-04:00[America/New_York]"


def test_intz():
    ts = Timestamp.from_string("2024-07-11T01:14:00Z")
    span = Span()
    span.months = 1
    span.hours = 2
    zoned = ts.intz("America/New_York") + span
    assert str(zoned) == "2024-08-10T23:14:00-04:00[America/New_York]" 