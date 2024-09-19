use pyo3::prelude::*;
use pyo3::types::PyDateTime;
#[pyclass(module = "rs_fsrs_python")]
#[derive(Debug, Clone)]
pub struct FSRS(fsrs::FSRS);
#[pymethods]
impl FSRS {
    #[new]
    pub fn new() -> Self {
        Self(fsrs::FSRS::default())
    }
    pub fn schedule(&self, card: Card, now: Py<PyDateTime>) -> ScheduledCards {
        Python::with_gil(|py| ScheduledCards(self.0.schedule(card.0, now.extract(py).unwrap())))
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
    pub fn log(&self) -> ReviewLog {
        ReviewLog(self.0.log.clone().unwrap())
    }
}

#[pyclass(module = "rs_fsrs_python")]
pub struct ScheduledCards(fsrs::ScheduledCards);
#[pymethods]
impl ScheduledCards {
    pub fn select_card(&self, r: Rating) -> Card {
        Card(self.0.select_card(r.into()))
    }
}

#[pymodule]
fn rs_fsrs_python(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<FSRS>()?;
    m.add_class::<Card>()?;
    m.add_class::<Rating>()?;
    m.add_class::<ScheduledCards>()?;
    Ok(())
}
