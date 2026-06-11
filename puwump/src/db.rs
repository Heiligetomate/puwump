use core::ffi;
use std::{fs::remove_file, path::PathBuf};

use rusqlite::{Connection, ErrorCode, params};
use uuid::Uuid;

use crate::{
    errors::{PuwumpError, Result},
    models::{
        Exercise, PlanExerciseDetail,
        core::{Model, statement_to_model},
        plan::Plan,
    },
    util::{create_dirs_to_path, get_full_db_path, ids_from_statement},
};

pub const DB_LOCATION: &str = "~/.local/share/puwump/puwump.db";
pub const SQL_INIT: &str = include_str!("../../init.sql");

pub struct Db {
    path: PathBuf,
    con: Connection,
}

impl Db {
    pub fn init() -> Result<Self> {
        let full: PathBuf = get_full_db_path()?;
        create_dirs_to_path(&full)?;
        let con = Connection::open(&full)?;
        con.pragma_update(None, "foreign_keys", "on")?;

        Ok(Self { path: full, con })
    }

    pub fn create(self) -> Result<Self> {
        self.con.execute_batch(SQL_INIT)?;

        Ok(self)
    }

    pub fn delete(self) -> Result<()> {
        remove_file(&self.path).map_err(|_| PuwumpError::DbRemoval)?;

        Ok(())
    }

    pub fn reset(self) -> Result<Self> {
        self.delete()?;
        let db = Self::init()?;
        db.create()
    }

    pub fn new_exercise(&self, name: &str, instructions: &str) -> Result<()> {
        let id = Uuid::new_v4().to_string();
        self.con
            .execute("INSERT INTO exercise (id, name, instructions) VALUES (?1, ?2, ?3)", params![id, name, instructions])?;
        Ok(())
    }

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

    pub fn get_all_exercise_ids(&self) -> Result<Vec<Uuid>> {
        let stmt = self
            .con
            .prepare("SELECT id FROM exercise")?;

        let ids = ids_from_statement(stmt)?;

        Ok(ids)
    }

    pub fn get_all_exercises(&self) -> Result<Vec<Exercise>> {
        let mut exercises = Vec::new();
        for id in self.get_all_exercise_ids()? {
            exercises.push(self.get_exercise(id)?);
        }

        Ok(exercises)
    }

    pub fn remove_exercise(&self, uuid: Uuid) -> Result<()> {
        self.con
            .execute("DELETE FROM exercise WHERE id = ?1", params![uuid.to_string()])?;
        Ok(())
    }

    pub fn remove_plan(&self, uuid: Uuid) -> Result<()> {
        self.con
            .execute("DELETE FROM plan WHERE id = ?1", params![uuid.to_string()])?;
        Ok(())
    }

    pub fn get_exercise(&self, uuid: Uuid) -> Result<Exercise> {
        let stmt = self
            .con
            .prepare("SELECT id, instructions, name FROM exercise WHERE id = ?1")?;

        let exercise = statement_to_model(stmt, params![uuid.to_string()])?;

        Ok(exercise)
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

    pub fn insert_exercise(&self, plan_id: Uuid, exercise_id: Uuid, reps: u16, order_index: u16) -> Result<()> {
        self.con.execute(
            "INSERT INTO plan_exercise (plan_id, exercise_id, reps, order_index) VALUES (?1, ?2, ?3, ?4)",
            params![plan_id.to_string(), exercise_id.to_string(), reps, order_index],
        )?;
        Ok(())
    }

    pub fn insert_ingredient(&self, name: &str) -> Result<()> {
        self.con
            .execute("INSERT INTO ingredient (name) VALUES (?1)", params![name.to_owned()])?;
        Ok(())
    }

    pub fn insert_meal(&self, name: &str, description: &str, calories: u32) -> Result<()> {
        self.con.execute(
            "INSERT INTO meal (name, description, calories) VALUES (?1, ?2, ?3)",
            params![name.to_owned(), description.to_owned(), calories],
        )?;
        Ok(())
    }

    pub fn insert_meal_ingredient(&self, meal_name: &str, ingredient_name: &str, amount: u32) -> Result<()> {
        self.con
            .execute(
                "INSERT INTO ingredient_in_meal (amount_gr, meal_name, ingredient_name) VALUES (?1, ?2, ?3)",
                params![amount, meal_name, ingredient_name],
            )
            .map_err(Self::map_sqlite_err)?;
        Ok(())
    }

    fn map_sqlite_err(e: rusqlite::Error) -> PuwumpError {
        match e {
            rusqlite::Error::SqliteFailure(e, _) => match e.extended_code {
                1555 | 2067 => PuwumpError::UniqueViolation,
                787 => PuwumpError::ForeignKeyViolation,
                _ => PuwumpError::Rusqlite(e.to_string()),
            },
            _ => PuwumpError::Rusqlite(e.to_string()),
        }
    }
}
