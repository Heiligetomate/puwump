use rusqlite::params;
use uuid::Uuid;

// TODO: (for all deletes basically) "dependencies" have to be deleted

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
    /// Inserts a new plan
    /// Plan names are not unique
    pub fn insert_plan(&self, name: &str, description: &str, est_min: u16) -> Result<Uuid> {
        let id = Uuid::new_v4();
        self.con.execute(
            "INSERT INTO plan (id, name, description, est_mins)  VALUES (?1, ?2, ?3, ?4)",
            params![id.to_string(), name, description, est_min],
        )?;

        Ok(id)
    }

    /// Removes a plan with the given Uuid
    pub fn remove_plan(&self, uuid: Uuid) -> Result<()> {
        self.con
            .execute("DELETE FROM plan WHERE id = ?1", params![uuid.to_string()])?;
        Ok(())
    }

    /// Get a plan with its id
    pub fn get_plan(&self, uuid: Uuid) -> Result<Plan> {
        let stmt = self
            .con
            .prepare("SELECT id, name, description, est_mins FROM plan WHERE id = ?1")?;
        let plan = statement_to_model(stmt, params![uuid.to_string()])?;

        Ok(plan)
    }

    /// Insert a new exercise into a plan
    pub fn insert_plan_exercise(&self, plan_id: Uuid, exercise_id: Uuid, reps: u16, order_index: u16) -> Result<()> {
        self.con.execute(
            "INSERT INTO plan_exercise (plan_id, exercise_id, reps, order_index) VALUES (?1, ?2, ?3, ?4)",
            params![plan_id.to_string(), exercise_id.to_string(), reps, order_index],
        )?;
        Ok(())
    }

    /// Get all Plan Uuids
    pub fn get_all_plan_ids(&self) -> Result<Vec<Uuid>> {
        let stmt = self
            .con
            .prepare("SELECT id FROM plan")?;
        let ids = ids_from_statement(stmt)?;
        Ok(ids)
    }

    /// Get all plans as a vec of Plan objects
    pub fn get_all_plans(&self) -> Result<Vec<Plan>> {
        let mut stmt = self.con.prepare("SELECT * FROM plan")?;
        let exercises = stmt
            .query_map(params![], <Plan as Model>::from_row)?
            .collect::<rusqlite::Result<Vec<Plan>>>()
            .map_err(|_| PuwumpError::RowNotFound)?;
        Ok(exercises)
    }

    // Get all Exercises of a plan with the given uuid
    // Returns the a Vec of PlanExerciseDetail containing the Plan and some extra values
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

    pub fn remove_plan_exercise(&self, plan_id: Uuid, exercise_id: Uuid) -> Result<()> {
        self.con
            .execute("DELETE FROM plan_exercise WHERE plan_id = ?1 AND exercise_id = ?2", (plan_id.to_string(), exercise_id.to_string()))?;
        Ok(())
    }
}
