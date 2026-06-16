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
        self.con
            .execute("INSERT INTO plan (id, name, description, est_mins)  VALUES (?1, ?2, ?3, ?4)", params![id.to_string(), name, description, est_min])?;

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
    pub fn insert_plan_exercise(&self, plan_id: Uuid, exercise_id: Uuid, reps: u16) -> Result<()> {
        self.con.execute(
            "INSERT INTO plan_exercise (id, plan_id, exercise_id, reps, order_index)
         VALUES (?1, ?2, ?3, ?4, COALESCE((SELECT MAX(order_index) + 1 FROM plan_exercise WHERE plan_id = ?2), 0))",
            params![Uuid::new_v4().to_string(), plan_id.to_string(), exercise_id.to_string(), reps],
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
    /// Ordered by name (alphabetic)
    pub fn get_all_plans(&self) -> Result<Vec<Plan>> {
        let mut stmt = self
            .con
            .prepare("SELECT * FROM plan ORDER BY name COLLATE NOCASE ASC")?;
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
            "SELECT pe.id, e.id, e.name, e.instructions, pe.order_index, pe.reps
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

    pub fn remove_plan_exercise(&self, id: Uuid) -> Result<()> {
        let (plan_id, order_index): (String, i16) = self
            .con
            .query_row("SELECT plan_id, order_index FROM plan_exercise WHERE id = ?1", params![id.to_string()], |row| Ok((row.get(0)?, row.get(1)?)))?;

        self.con
            .execute("DELETE FROM plan_exercise WHERE id = ?1", params![id.to_string()])?;

        self.con
            .execute("UPDATE plan_exercise SET order_index = order_index - 1 WHERE plan_id = ?1 AND order_index > ?2", params![plan_id, order_index])?;

        Ok(())
    }

    pub fn move_plan_exercise(&self, id: Uuid, diff: i8) -> Result<()> {
        let (plan_id, current): (String, i16) = self
            .con
            .query_row("SELECT plan_id, order_index FROM plan_exercise WHERE id = ?1", params![id.to_string()], |row| Ok((row.get(0)?, row.get(1)?)))?;

        let target = current + diff as i16;
        if target < 0 {
            return Ok(());
        }

        // Park row at -1
        self.con
            .execute("UPDATE plan_exercise SET order_index = -1 WHERE id = ?1", params![id.to_string()])?;
        // Move neighbour to old slot
        let moved = self
            .con
            .execute("UPDATE plan_exercise SET order_index = ?3 WHERE plan_id = ?1 AND order_index = ?2", params![plan_id, target, current])?;
        // If no exists, undo the park and return
        if moved == 0 {
            self.con
                .execute("UPDATE plan_exercise SET order_index = ?2 WHERE id = ?1", params![id.to_string(), current])?;
            return Ok(());
        }
        // Move row to target
        self.con
            .execute("UPDATE plan_exercise SET order_index = ?2 WHERE id = ?1", params![id.to_string(), target])?;

        Ok(())
    }

    pub fn incr_plan_exercise(&self, id: Uuid) -> Result<()> {
        self.con
            .execute("UPDATE plan_exercise SET reps = reps + 1 WHERE id = ?1", (id.to_string(),))?;

        Ok(())
    }

    pub fn decr_plan_exercise(&self, id: Uuid) -> Result<()> {
        self.con
            .execute("UPDATE plan_exercise SET reps = MAX(reps - 1, 1) WHERE id = ?1", (id.to_string(),))?;

        Ok(())
    }
}
