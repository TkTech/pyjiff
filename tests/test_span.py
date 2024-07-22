import pytest

from jiff import Span


def test_span_from_year():
    span = Span()
    span.years = 30
    assert span.years == 30


def test_span_from_month():
    span = Span()
    span.months = 30
    assert span.months == 30


def test_span_from_day():
    span = Span()
    span.days = 30
    assert span.days == 30


def test_span_from_hour():
    span = Span()
    span.hours = 30
    assert span.hours == 30


def test_span_from_minute():
    span = Span()
    span.minutes = 30
    assert span.minutes == 30


def test_span_from_second():
    span = Span()
    span.seconds = 30
    assert span.seconds == 30


def test_span_from_millisecond():
    span = Span()
    span.milliseconds = 30
    assert span.milliseconds == 30


def test_span_from_microsecond():
    span = Span()
    span.microseconds = 30
    assert span.microseconds == 30


def test_span_from_nanosecond():
    span = Span()
    span.nanoseconds = 30
    assert span.nanoseconds == 30


def test_span_negate():
    span = Span()
    span.years = 1
    negated = span.negate()
    assert negated.years == -1


def test_span_mul():
    span = Span()
    span.years = 1
    multiplied = span * 2
    assert multiplied.years == 2


def test_span_add():
    span = Span()
    span.years = 1
    span2 = Span()
    span2.years = 2
    with pytest.raises(ValueError):
        span + span2

    span3 = Span()
    span3.days = 2
    span4 = Span()
    span4.days = 2

    added = span3 + span4
    assert added.days == 4


def test_span_sub():
    span = Span()
    span.years = 1
    span2 = Span()
    span2.years = 2
    with pytest.raises(ValueError):
        span2 - span

    span3 = Span()
    span3.days = 2
    span4 = Span()
    span4.days = 1

    sub = span3 - span4
    assert sub.days == 1
