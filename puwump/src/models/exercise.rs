use uuid::Uuid;

use crate::{
    db::Db,
    errors::Result,
    models::{
        CardAdd,
        card_compatible::{CardCrud, InputField},
        core::Model,
    },
};

#[derive(Debug)]
pub struct Exercise {
    pub id: Uuid,
    pub name: String,
    pub instructions: String,
}

impl Model for Exercise {
    fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Self> {
        let id: String = row.get(0)?;
        Ok(Self {
            id: Uuid::parse_str(&id).map_err(|_| rusqlite::Error::InvalidQuery)?,
            instructions: row.get(1)?,
            name: row.get(2)?,
        })
    }
}

impl CardAdd for Exercise {
    fn title(&self) -> &str {
        &self.name
    }

    fn body(&self) -> Option<&str> {
        Some(&self.instructions)
    }

    fn key(&self) -> Uuid {
        self.id
    }
}

impl CardCrud for Exercise {
    fn get_all(db: &Db) -> Result<Vec<Self>> {
        db.get_all_exercises()
    }

    fn insert(db: &Db, values: &[InputField]) -> Result<()> {
        db.insert_exercise(values[0].value.as_str(), values[1].value.as_str())
    }

    fn delete(db: &Db, id: Uuid) -> Result<()> {
        db.remove_exercise(id)
    }

    fn name() -> &'static str {
        "exercise"
    }
}
