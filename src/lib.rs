use chrono::{Timelike, TimeZone, Utc};
use pyo3::create_exception;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyDateAccess, PyDateTime, PyTimeAccess};
use ulid::Ulid;
use uuid::Uuid;

create_exception!(_ulid_rs_py, DecodeError, PyValueError, "Ulid decode error");
create_exception!(_ulid_rs_py, InvalidUuidError, PyValueError, "Invalid uuid error");

#[pyclass]
struct PyUlid(Ulid);

impl PyUlid {
    fn new(ulid: Ulid) -> Self {
        PyUlid(ulid)
    }
}

#[pymethods]
impl PyUlid {
    pub fn __repr__(&self) -> PyResult<String> {
        Ok(format!("<ULID('{}')>", self.0.to_string()))
    }

    pub fn bytes(&self) -> PyResult<[u8; 16]> {
        Ok(self.0.0.to_be_bytes())
    }

    pub fn timestamp(&self) -> PyResult<u64> {
        Ok(self.0.timestamp_ms())
    }

    pub fn randomness(&self) -> PyResult<u128> {
        Ok(self.0.random())
    }

    pub fn str(&self) -> PyResult<String> {
        Ok(self.0.to_string())
    }

    pub fn increment(&self) -> PyResult<Option<PyUlid>> {
        match self.0.increment() {
            None => Ok(None),
            Some(resp) => { Ok(Option::from(self::PyUlid::new(resp))) }
        }
    }
}

#[pyfunction]
fn gen_ulid(_py: Python) -> PyResult<String> {
    _py.allow_threads(|| {
        let ulid = Ulid::new();
        Ok(ulid.to_string())
    })
}

#[pyfunction]
#[pyo3(signature = (value))]
fn from_string(_py: Python, value: &str) -> PyResult<PyUlid> {
    match _py.allow_threads(|| Ulid::from_string(&value)) {
        Ok(ulid_result) => Ok(PyUlid::new(ulid_result)),
        Err(err) => Err(DecodeError::new_err(err.to_string()))
    }
}

#[pyfunction]
#[pyo3(signature = (value))]
fn from_uuid(_py: Python, value: &str) -> PyResult<PyUlid> {
    match _py.allow_threads(|| Uuid::parse_str(&value)) {
        Ok(instance_uuid) => {
            Ok(PyUlid::new(ulid::Ulid::from(instance_uuid)))
        }
        Err(err) => Err(InvalidUuidError::new_err(err.to_string()))
    }
}

#[pyfunction]
#[pyo3(signature = (value))]
fn from_timestamp(_py: Python, value: &PyDateTime) -> PyResult<PyUlid> {
    let year = value.get_year();
    let month = value.get_month() as u32;
    let day = value.get_day() as u32;
    let hour = value.get_hour() as u32;
    let minute = value.get_minute() as u32;
    let second = value.get_second() as u32;
    let microsecond = value.get_microsecond() as u32;
    _py.allow_threads(|| {
        let dt = Utc.with_ymd_and_hms(year, month, day, hour, minute, second).unwrap()
            .with_nanosecond(microsecond)
            .unwrap();
        let system_time = dt.into();
        let ulid = Ulid::from_datetime(system_time);
        Ok(PyUlid::new(ulid))
    })
}

#[pyfunction]
#[pyo3(signature = (timestamp, randomness))]
fn from_parts(_py: Python, timestamp: u64, randomness: u128) -> PyResult<PyUlid> {
    _py.allow_threads(||  {
        let ulid = Ulid::from_parts(timestamp, randomness);
        Ok(PyUlid::new(ulid))
    })
}

#[pymodule]
fn _ulid_rs_py(_py: Python, module: &PyModule) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(gen_ulid, module)?)?;
    module.add_function(wrap_pyfunction!(from_string, module)?)?;
    module.add_function(wrap_pyfunction!(from_uuid, module)?)?;
    module.add_function(wrap_pyfunction!(from_timestamp, module)?)?;
    module.add_function(wrap_pyfunction!(from_parts, module)?)?;
    module.add_class::<PyUlid>()?;
    module.add("DecodeError", _py.get_type::<DecodeError>())?;
    module.add("InvalidUuidError", _py.get_type::<DecodeError>())?;

    #[cfg(not(PyPy))]
    pyo3::prepare_freethreaded_python();

    Ok(())
}
