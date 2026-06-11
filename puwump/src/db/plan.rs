use rusqlite::params;
use uuid::Uuid;

use crate::{
    db::Db,
    errors::{PuwumpError, Result},
    models::{
        Plan, PlanExerciseDetail,
        core::{Model, statement_to_model},
    },
    util::ids_from_statement,
};

impl Db {
    pub fn new_plan(&self, name: &str, description: &str, est_min: u16) -> Result<Uuid> {
        let id = Uuid::new_v4();
        self.con.execute(
            "INSERT INTO plan (id, name, description, est_mins)  VALUES (?1, ?2, ?3, ?4)",
            params![id.to_string(), name, description, est_min],
        )?;

        Ok(id)
    }

    pub fn get_all_plans(&self) -> Result<Vec<Uuid>> {
        let stmt = self
            .con
            .prepare("SELECT id FROM plan")?;
        let ids = ids_from_statement(stmt)?;
        Ok(ids)
    }

    pub fn remove_plan(&self, uuid: Uuid) -> Result<()> {
        self.con
            .execute("DELETE FROM plan WHERE id = ?1", params![uuid.to_string()])?;
        Ok(())
    }

    pub fn get_plan(&self, uuid: Uuid) -> Result<Plan> {
        let stmt = self
            .con
            .prepare("SELECT id, name, description, est_mins FROM plan WHERE id = ?1")?;
        let plan = statement_to_model(stmt, params![uuid.to_string()])?;

        Ok(plan)
    }

    pub fn get_plan_exercises(&self, uuid: Uuid) -> Result<Vec<PlanExerciseDetail>> {
        let mut stmt = self.con.prepare(
            "SELECT e.id, e.name, e.instructions, pe.order_index, pe.reps
        FROM plan_exercise pe
        JOIN exercise e ON e.id = pe.exercise_id
        WHERE pe.plan_id = ?1
        ORDER BY pe.order_index ASC",
        )?;
        let exercises = stmt
            .query_map(params![uuid.to_string()], <PlanExerciseDetail as Model>::from_row)?
            .collect::<rusqlite::Result<Vec<PlanExerciseDetail>>>()
            .map_err(|_| PuwumpError::RowNotFound)?;
        Ok(exercises)
    }
}
