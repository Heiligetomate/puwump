use uuid::Uuid;

use crate::models::{Exercise, core::Model};

#[derive(Debug)]
pub struct PlanExerciseDetail {
    pub exercise: Exercise,
    pub order_index: u16,
    pub reps: Option<u16>,
}

impl Model for PlanExerciseDetail {
    fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Self> {
        let id: String = row.get(0)?;
        Ok(Self {
            exercise: Exercise {
                id: Uuid::parse_str(&id).map_err(|_| rusqlite::Error::InvalidQuery)?,
                name: row.get(1)?,
                instructions: row.get(2)?,
            },
            order_index: row.get(3)?,
            reps: row.get(4)?,
        })
    }
}
