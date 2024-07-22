mod pyzoned;
mod pyspan;
mod pytimestamp;
mod pytimezone;

use pyo3::prelude::*;

#[pymodule]
fn _lowlevel(_py: Python, m: &PyModule) -> PyResult<()> {
    pytimestamp::init_module(_py, m)?;
    pyspan::init_module(_py, m)?;
    pytimezone::init_module(_py, m)?;
    pyzoned::init_module(_py, m)?;
    Ok(())
}