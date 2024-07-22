use pyo3::prelude::*;
use jiff::tz::TimeZone;

#[pyclass(name = "TimeZone")]
pub(crate) struct PyTimeZone {
    pub inner: TimeZone,
}

#[pymethods]
impl PyTimeZone {
    #[staticmethod]
    fn utc() -> PyResult<Self> {
        Ok(PyTimeZone { inner: TimeZone::UTC })
    }
}

pub(crate) fn init_module(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyTimeZone>()?;
    Ok(())
}
