use chrono::{DateTime, Utc};
use pyo3::prelude::*;
use pyo3::types::PyDateTime;
#[pyclass(module = "rs_fsrs_python")]
#[derive(Debug, Clone)]
pub struct FSRS(fsrs::FSRS);
#[pymethods]
impl FSRS {
    #[new]
    pub fn new(parameters: Parameters) -> Self {
        Self(fsrs::FSRS::new(parameters.into()))
    }
    #[staticmethod]
    pub fn default() -> Self {
        Self(fsrs::FSRS::default())
    }
    pub fn repeat(&self, card: Card, now: Py<PyDateTime>) -> RecordLog {
        Python::with_gil(|py| RecordLog(self.0.repeat(card.0, now.extract(py).unwrap())))
    }
}
#[pyclass(module = "rs_fsrs_python")]
#[derive(Debug, Clone)]
pub struct Parameters {
    pub request_retention: f64,
    pub maximum_interval: i32,
    pub w: [f64; 19],
    pub decay: f64,
    pub factor: f64,
    pub enable_short_term: bool,
}

impl From<Parameters> for fsrs::Parameters {
    fn from(value: Parameters) -> Self {
        fsrs::Parameters {
            request_retention: value.request_retention,
            maximum_interval: value.maximum_interval,
            w: value.w,
            decay: value.decay,
            factor: value.factor,
            enable_short_term: value.enable_short_term,
        }
    }
}

#[pyclass(module = "rs_fsrs_python")]
#[derive(Debug, Clone)]
pub struct Card(fsrs::Card);

#[pyclass(eq, eq_int, module = "rs_fsrs_python")]
#[derive(Debug, Clone, PartialEq)]
pub enum Rating {
    Again,
    Hard,
    Good,
    Easy,
}

impl From<fsrs::Rating> for Rating {
    fn from(value: fsrs::Rating) -> Self {
        use crate::Rating::*;
        use fsrs::Rating as r;
        match value {
            r::Again => Again,
            r::Hard => Hard,
            r::Good => Good,
            r::Easy => Easy,
        }
    }
}
impl From<Rating> for fsrs::Rating {
    fn from(value: Rating) -> Self {
        use crate::Rating as r;
        use fsrs::Rating::*;
        match value {
            r::Again => Again,
            r::Hard => Hard,
            r::Good => Good,
            r::Easy => Easy,
        }
    }
}

#[pyclass(module = "rs_fsrs_python")]
#[derive(Debug, Clone)]
pub struct ReviewLog(fsrs::ReviewLog);
#[pymethods]
impl ReviewLog {
    fn __str__(&self) -> String {
        format!("{:?}", self.0)
    }
}

#[pymethods]
impl Card {
    #[new]
    pub fn new() -> Self {
        Self(fsrs::Card::new())
    }
    #[getter]
    pub fn difficulty(&self) -> f64 {
        self.0.difficulty
    }
    #[getter]
    pub fn stability(&self) -> f64 {
        self.0.stability
    }
    #[getter]
    pub fn elapsed_days(&self) -> i64 {
        self.0.elapsed_days
    }
    #[getter]
    pub fn due(&self) -> Py<PyDateTime> {
        Python::with_gil(|py| {
            PyDateTime::from_timestamp_bound(py, self.0.due.timestamp() as f64, Default::default())
                .expect("error")
                .into()
        })
    }
    #[getter]
    pub fn scheduled_days(&self) -> i64 {
        self.0.scheduled_days
    }
    #[getter]
    pub fn reps(&self) -> i32 {
        self.0.reps
    }
    #[getter]
    pub fn last_review(&self) -> Py<PyDateTime> {
        Python::with_gil(|py| {
            PyDateTime::from_timestamp_bound(py, self.0.last_review.timestamp() as f64, Default::default())
                .expect("error")
                .into()
        })
    }
    pub fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
}

#[pyclass(module = "rs_fsrs_python")]
#[derive(Debug, Clone)]
pub struct SchedulingInfo(fsrs::SchedulingInfo);

#[pymethods]
impl SchedulingInfo {
    #[getter]
    pub fn card(&self) -> Card {
        Card(self.0.card.clone())
    }
    #[getter]
    pub fn review_log(&self) -> ReviewLog {
        ReviewLog(self.0.review_log.clone())
    }
}
#[pyclass(module = "rs_fsrs_python")]
pub struct RecordLog(fsrs::RecordLog);
#[pymethods]
impl RecordLog {
    pub fn get(&self, r: Rating) -> SchedulingInfo {
        SchedulingInfo(self.0.get(&r.into()).unwrap().clone()).clone()
    }
}

#[pymodule]
fn rs_fsrs_python(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<FSRS>()?;
    m.add_class::<Card>()?;
    m.add_class::<Rating>()?;
    m.add_class::<SchedulingInfo>()?;
    m.add_class::<RecordLog>()?;
    Ok(())
}
