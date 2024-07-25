use pyo3::prelude::*;
use jiff::{Zoned, Timestamp, tz::TimeZone};
use crate::pytimestamp::PyTimestamp;
use crate::pytimezone::PyTimeZone;
use crate::pyspan::PySpan;

#[pyclass(name = "Zoned")]
pub(crate) struct PyZoned {
    pub inner: Zoned,
}

#[pymethods]
impl PyZoned {
    #[new]
    fn new(timestamp: PyRef<'_, PyTimestamp>, time_zone: PyRef<'_, PyTimeZone>) -> PyResult<Self> {
        let zoned = Zoned::new(timestamp.inner, time_zone.inner.clone());
        Ok(PyZoned { inner: zoned })
    }

    #[staticmethod]
    fn from_string(str: String) -> PyResult<Self> {
        let zoned = str.parse();
        match zoned {
            Ok(z) => {Ok(PyZoned { inner: z })},
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
        }
    }

    /// Returns the current system time in this system’s time zone.
    #[staticmethod]
    fn now() -> PyResult<Self> {
        let zoned = Zoned::now();
        Ok(PyZoned { inner: zoned })
    }

    /// Returns the current system time in UTC.
    #[staticmethod]
    fn utc() -> PyResult<Self> {
        Ok(PyZoned { inner: Zoned::new(
            Timestamp::now(),
            TimeZone::UTC
        ) })
    }

    /// Return a new zoned datetime with precisely the same instant in a different time zone.
    fn with_time_zone(&self, time_zone: PyRef<'_, PyTimeZone>) -> PyResult<Self> {
        let zoned = self.inner.with_time_zone(time_zone.inner.clone());
        Ok(PyZoned { inner: zoned })
    }

    #[getter]
    fn get_time_zone(&self) -> PyResult<PyTimeZone> {
        Ok(PyTimeZone { inner: self.inner.time_zone().clone() })
    }

    #[getter]
    fn get_year(&self) -> i16 {
        self.inner.year()
    }

    #[getter]
    fn get_month(&self) -> i8 {
        self.inner.month()
    }

    #[getter]
    fn get_day(&self) -> i8 {
        self.inner.day()
    }

    #[getter]
    fn get_hour(&self) -> i8 {
        self.inner.hour()
    }

    #[getter]
    fn get_minute(&self) -> i8 {
        self.inner.minute()
    }

    #[getter]
    fn get_second(&self) -> i8 {
        self.inner.second()
    }

    #[getter]
    fn get_microsecond(&self) -> i16 {
        self.inner.microsecond()
    }

    #[getter]
    fn get_millisecond(&self) -> i16 {
        self.inner.millisecond()
    }

    #[getter]
    fn get_nanosecond(&self) -> i16 {
        self.inner.nanosecond()
    }

    /// Returns the weekday corresponding to this zoned datetime.
    #[getter]
    fn get_weekday(&self) -> i8 {
        self.inner.weekday() as i8
    }

    /// Returns the ordinal day of the year that this zoned datetime resides in.
    ///
    /// For leap years, this always returns a value in the range 1..=366.
    /// Otherwise, the value is in the range 1..=365.
    #[getter]
    fn get_day_of_year(&self) -> i16 {
        self.inner.day_of_year()
    }

    /// Returns the ordinal day of the year that this zoned datetime resides
    /// in, but ignores leap years.
    ///
    /// That is, the range of possible values returned by this routine is
    /// 1..=365, even if this date resides in a leap year. If this date is
    /// February 29, then this routine returns None.
    ///
    /// The value 365 always corresponds to the last day in the year,
    /// December 31, even for leap years.
    #[getter]
    fn get_day_of_year_no_leap(&self) -> i16 {
        match self.inner.day_of_year_no_leap() {
            Some(d) => d as i16,
            None => -1
        }
    }

    fn __add__(&self, other: PyRef<'_, PySpan>) -> PyResult<Self> {
        let zoned = self.inner.checked_add(other.inner);
        match zoned {
            Ok(z) => Ok(PyZoned { inner: z }),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
        }
    }

    fn __lt__(&self, other: PyRef<'_, PyZoned>) -> PyResult<bool> {
        Ok(self.inner < other.inner)
    }

    fn __gt__(&self, other: PyRef<'_, PyZoned>) -> PyResult<bool> {
        Ok(self.inner > other.inner)
    }

    fn __eq__(&self, other: PyRef<'_, PyZoned>) -> PyResult<bool> {
        Ok(self.inner == other.inner)
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(self.inner.to_string())
    }
}

pub(crate) fn init_module(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyZoned>()?;
    Ok(())
}
