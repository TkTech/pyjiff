use pyo3::prelude::*;
use jiff::Timestamp;
use crate::pyzoned::PyZoned;

#[pyclass(name = "Timestamp")]
pub(crate) struct PyTimestamp {
    pub inner: Timestamp,
}

#[pymethods]
impl PyTimestamp {
    #[new]
    fn new(seconds: i64, nanoseconds: i32) -> PyResult<Self> {
        let ts = Timestamp::new(seconds, nanoseconds)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        Ok(PyTimestamp { inner: ts })
    }

    #[staticmethod]
    fn now() -> PyResult<Self> {
        let ts = Timestamp::now();
        Ok(PyTimestamp { inner: ts })
    }

    #[staticmethod]
    fn from_string(str: String) -> PyResult<Self> {
        let ts = str.parse();
        match ts {
            Ok(t) => Ok(PyTimestamp { inner: t }),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
        }
    }

    #[staticmethod]
    fn from_microsecond(microseconds: i64) -> PyResult<Self> {
        let ts = Timestamp::from_microsecond(microseconds)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        Ok(PyTimestamp { inner: ts })
    }

    #[staticmethod]
    fn from_millisecond(milliseconds: i64) -> PyResult<Self> {
        let ts = Timestamp::from_millisecond(milliseconds)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        Ok(PyTimestamp { inner: ts })
    }

    #[staticmethod]
    fn from_nanosecond(nanoseconds: i128) -> PyResult<Self> {
        let ts = Timestamp::from_nanosecond(nanoseconds)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        Ok(PyTimestamp { inner: ts })
    }

    #[staticmethod]
    fn from_second(seconds: i64) -> PyResult<Self> {
        let ts = Timestamp::from_second(seconds)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        Ok(PyTimestamp { inner: ts })
    }

    fn subsec_microsecond(&self) -> i32 {
        self.inner.subsec_microsecond()
    }

    fn as_millisecond(&self) -> i64 {
        self.inner.as_millisecond()
    }

    fn as_nanosecond(&self) -> i128 {
        self.inner.as_nanosecond()
    }

    fn as_second(&self) -> i64 {
        self.inner.as_second()
    }

    fn is_zero(&self) -> bool {
        self.inner.is_zero()
    }

    fn intz(&self, time_zone_name: &str) -> PyResult<PyZoned> {
        let zoned = self.inner.intz(time_zone_name);
        match zoned {
            Ok(z) => Ok(PyZoned { inner: z }),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
        }
    }

    fn __str__(&self) -> String {
        self.inner.to_string()
    }
}

pub(crate) fn init_module(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyTimestamp>()?;
    Ok(())
}
