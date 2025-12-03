use chrono::{DateTime, Datelike, TimeZone, Timelike, Utc};
use pyo3::create_exception;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyDateAccess, PyDateTime, PyTimeAccess};
use std::time::{Duration, SystemTime};
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
        // https://github.com/PyO3/pyo3/issues/4996
        Ok(self.0.to_bytes())
    }

    pub fn timestamp(&self) -> PyResult<f64> {
        let datetime: DateTime<Utc> = self.0.datetime().into();
        Ok(datetime.timestamp_micros() as f64 / 1_000_000.0)
    }

    pub fn datetime<'p>(&self, _py: Python<'p>) -> PyResult<Bound<'p, PyDateTime>> {
        let datetime: DateTime<Utc> = self.0.datetime().into();
        PyDateTime::new(
            _py,
            datetime.year(),
            datetime.month() as u8,
            datetime.day() as u8,
            datetime.hour() as u8,
            datetime.minute() as u8,
            datetime.second() as u8,
            datetime.timestamp_subsec_micros(),
            None,
        )
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
            Some(resp) => Ok(Option::from(PyUlid::new(resp))),
        }
    }
}

#[pyfunction]
fn new(_py: Python) -> PyResult<PyUlid> {
    _py.detach(|| {
        let ulid = Ulid::new();
        Ok(PyUlid::new(ulid))
    })
}

#[pyfunction]
#[pyo3(signature = (value))]
fn from_string(_py: Python, value: &str) -> PyResult<PyUlid> {
    match _py.detach(|| Ulid::from_string(&value)) {
        Ok(ulid_result) => Ok(PyUlid::new(ulid_result)),
        Err(err) => Err(DecodeError::new_err(err.to_string())),
    }
}

#[pyfunction]
#[pyo3(signature = (value))]
fn from_uuid(_py: Python, value: Uuid) -> PyResult<PyUlid> {
    _py.detach(|| Ok(PyUlid::new(Ulid::from(value))))
}

#[pyfunction]
#[pyo3(signature = (value))]
fn from_datetime(_py: Python, value: &Bound<PyDateTime>) -> PyResult<PyUlid> {
    let year = value.get_year();
    let month = value.get_month() as u32;
    let day = value.get_day() as u32;
    let hour = value.get_hour() as u32;
    let minute = value.get_minute() as u32;
    let second = value.get_second() as u32;
    let microsecond = value.get_microsecond();
    // ULID timestamps have millisecond precision only.
    // Truncate Python microseconds to milliseconds before converting to nanoseconds.
    let nanos = (microsecond / 1000) * 1_000_000;

    _py.detach(|| {
        let dt = Utc
            .with_ymd_and_hms(year, month, day, hour, minute, second)
            .unwrap()
            .with_nanosecond(nanos)
            .unwrap();
        let system_time = dt.into();
        let ulid = Ulid::from_datetime(system_time);
        Ok(PyUlid::new(ulid))
    })
}

#[pyfunction]
#[pyo3(signature = (value))]
fn from_timestamp(_py: Python, value: f64) -> PyResult<PyUlid> {
    _py.detach(|| {
        let system_time = SystemTime::UNIX_EPOCH + Duration::from_secs(value as u64);
        let ulid = Ulid::from_datetime(system_time.into());
        Ok(PyUlid::new(ulid))
    })
}

#[pyfunction]
#[pyo3(signature = (timestamp, randomness))]
fn from_parts(_py: Python, timestamp: f64, randomness: u128) -> PyResult<PyUlid> {
    _py.detach(|| {
        let ulid = Ulid::from_parts(timestamp as u64 * 1000, randomness);
        Ok(PyUlid::new(ulid))
    })
}

#[pymodule]
fn _ulid_rs_py(_py: Python, module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(new, module)?)?;
    module.add_function(wrap_pyfunction!(from_string, module)?)?;
    module.add_function(wrap_pyfunction!(from_uuid, module)?)?;
    module.add_function(wrap_pyfunction!(from_timestamp, module)?)?;
    module.add_function(wrap_pyfunction!(from_datetime, module)?)?;
    module.add_function(wrap_pyfunction!(from_parts, module)?)?;
    module.add_class::<PyUlid>()?;
    module.add("DecodeError", _py.get_type::<DecodeError>())?;
    module.add("InvalidUuidError", _py.get_type::<InvalidUuidError>())?;

    #[cfg(not(PyPy))]
    pyo3::Python::initialize();

    Ok(())
}
