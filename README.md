# pyjiff

Python bindings for [Jiff][], a date-time library for Rust that
encourages you to jump into the pit of success. 

## Documentation

See https://tkte.ch/pyjiff/ for the most recent release documentation.

## Example

```python
from jiff import Zoned, Timestamp, Span

ts = Timestamp.from_string("2024-07-11T01:14:00Z")
span = Span()
span.months = 1
span.hours = 2
zoned = ts.intz("America/New_York") + span
assert str(zoned) == "2024-08-10T23:14:00-04:00[America/New_York]" 
```


[Jiff]: https://github.com/BurntSushi/jiff