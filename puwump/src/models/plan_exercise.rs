use uuid::Uuid;

use crate::models::{CardAdd, Exercise, core::Model};

#[derive(Debug)]
pub struct PlanExerciseDetail {
    pub id: Uuid,
    pub exercise: Exercise,
    pub order_index: u16,
    pub reps: u16,
    pub title: String,
}

impl Model for PlanExerciseDetail {
    fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Self> {
        let pe_id: String = row.get(0)?;
        let ex_id: String = row.get(1)?;
        let ex_name = row.get(2)?;
        let reps = row.get(5)?;
        let title = format!("{}x {}", reps, ex_name);
        Ok(Self {
            id: Uuid::parse_str(&pe_id).map_err(|_| rusqlite::Error::InvalidQuery)?,
            exercise: Exercise {
                id: Uuid::parse_str(&ex_id).map_err(|_| rusqlite::Error::InvalidQuery)?,
                name: ex_name,
                instructions: row.get(3)?,
            },
            order_index: row.get(4)?,
            reps: reps,
            title,
        })
    }
}

impl CardAdd for PlanExerciseDetail {
    fn key(&self) -> Uuid {
        self.id
    }

    fn body(&self) -> Option<&str> {
        None
    }

    fn title(&self) -> &str {
        self.title.as_str()
    }
}
