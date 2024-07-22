use pyo3::prelude::*;
use jiff::Span;

#[pyclass(name = "Span")]
pub(crate) struct PySpan {
    pub inner: Span,
}

#[pymethods]
impl PySpan {
    #[new]
    fn new() -> PyResult<Self> {
        let span = Span::new();
        Ok(PySpan { inner: span })
    }

    #[setter]
    fn set_years(&mut self, years: i64) -> PyResult<()> {
        self.inner = self.inner.years(years);
        Ok(())
    }

    #[getter]
    fn get_years(&self) -> i16 {
        self.inner.get_years()
    }

    #[setter]
    fn set_months(&mut self, months: i64) -> PyResult<()> {
        self.inner = self.inner.months(months);
        Ok(())
    }

    #[getter]
    fn get_months(&self) -> i32 {
        self.inner.get_months()
    }

    #[setter]
    fn set_days(&mut self, days: i64) -> PyResult<()> {
        self.inner = self.inner.days(days);
        Ok(())
    }

    #[getter]
    fn get_days(&self) -> i32 {
        self.inner.get_days()
    }

    #[setter]
    fn set_hours(&mut self, hours: i64) -> PyResult<()> {
        self.inner = self.inner.hours(hours);
        Ok(())
    }

    #[getter]
    fn get_hours(&self) -> i32 {
        self.inner.get_hours()
    }

    #[setter]
    fn set_minutes(&mut self, minutes: i64) -> PyResult<()> {
        self.inner = self.inner.minutes(minutes);
        Ok(())
    }

    #[getter]
    fn get_minutes(&self) -> i64 {
        self.inner.get_minutes()
    }

    #[setter]
    fn set_seconds(&mut self, seconds: i64) -> PyResult<()> {
        self.inner = self.inner.seconds(seconds);
        Ok(())
    }

    #[getter]
    fn get_seconds(&self) -> i64 {
        self.inner.get_seconds()
    }

    #[setter]
    fn set_milliseconds(&mut self, milliseconds: i64) -> PyResult<()> {
        self.inner = self.inner.milliseconds(milliseconds);
        Ok(())
    }

    #[getter]
    fn get_milliseconds(&self) -> i64 {
        self.inner.get_milliseconds()
    }

    #[setter]
    fn set_microseconds(&mut self, microseconds: i64) -> PyResult<()> {
        self.inner = self.inner.microseconds(microseconds);
        Ok(())
    }

    #[getter]
    fn get_microseconds(&self) -> i64 {
        self.inner.get_microseconds()
    }

    #[setter]
    fn set_nanoseconds(&mut self, nanoseconds: i64) -> PyResult<()> {
        self.inner = self.inner.nanoseconds(nanoseconds);
        Ok(())
    }

    #[getter]
    fn get_nanoseconds(&self) -> i64 {
        self.inner.get_nanoseconds()
    }

    fn is_zero(&self) -> bool {
        self.inner.is_zero()
    }

    fn is_negative(&self) -> bool {
        self.inner.is_negative()
    }

    fn negate(&self) -> PyResult<PySpan> {
        Ok(PySpan { inner: -self.inner })
    }

    fn abs(&self) -> PyResult<PySpan> {
        Ok(PySpan { inner: self.inner.abs() })
    }

    fn __mul__(&self, other: i64) -> PyResult<PySpan> {
        Ok(PySpan { inner: self.inner * other })
    }

    fn __add__(&mut self, other: &mut PySpan) -> PyResult<PySpan> {
        let span = self.inner.checked_add(other.inner);
        match span {
            Ok(span) => Ok(PySpan { inner: span }),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string())),
        }
    }

    fn __sub__(&mut self, other: &mut PySpan) -> PyResult<PySpan> {
        let span = self.inner.checked_sub(other.inner);
        match span {
            Ok(span) => Ok(PySpan { inner: span }),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string())),
        }
    }

    fn __str__(&self) -> String {
        self.inner.to_string()
    }

    fn __repr__(&self) -> String {
        format!("Span({})", self.inner.to_string())
    }
}

pub(crate) fn init_module(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PySpan>()?;
    Ok(())
}
