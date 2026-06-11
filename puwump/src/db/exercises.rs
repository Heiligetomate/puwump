use rusqlite::params;
use uuid::Uuid;

use crate::{
    db::Db,
    errors::Result,
    models::{Exercise, core::statement_to_model},
    util::ids_from_statement,
};

impl Db {
    /// Create a new exercise
    /// The name is unique
    pub fn insert_exercise(&self, name: &str, instructions: &str) -> Result<()> {
        let id = Uuid::new_v4().to_string();
        self.con
            .execute("INSERT INTO exercise (id, name, instructions) VALUES (?1, ?2, ?3)", params![id, name, instructions])?;
        Ok(())
    }

    /// Removes an exercise with the given uuid
    pub fn remove_exercise(&self, uuid: Uuid) -> Result<()> {
        self.con
            .execute("DELETE FROM exercise WHERE id = ?1", params![uuid.to_string()])?;
        Ok(())
    }

    /// Takes the uuid and returns an Exercise object
    pub fn get_exercise(&self, uuid: Uuid) -> Result<Exercise> {
        let stmt = self
            .con
            .prepare("SELECT id, instructions, name FROM exercise WHERE id = ?1")?;

        let exercise = statement_to_model(stmt, params![uuid.to_string()])?;

        Ok(exercise)
    }

    /// Returns a Vec with all exercise uuids
    pub fn get_all_exercise_ids(&self) -> Result<Vec<Uuid>> {
        let stmt = self
            .con
            .prepare("SELECT id FROM exercise")?;

        let ids = ids_from_statement(stmt)?;

        Ok(ids)
    }

    /// Returns a Vec with all exercises as Exercise object
    pub fn get_all_exercises(&self) -> Result<Vec<Exercise>> {
        let mut exercises = Vec::new();
        for id in self.get_all_exercise_ids()? {
            exercises.push(self.get_exercise(id)?);
        }

        Ok(exercises)
    }
}
