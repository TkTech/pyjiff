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
