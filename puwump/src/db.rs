use std::{fs::remove_file, path::PathBuf};

use rusqlite::{Connection, params};
use uuid::Uuid;

use crate::{
    errors::{PuwumpError, Result},
    models::{Exercise, core::statement_to_model, plan::Plan},
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
        Ok(Self { path: full, con })
    }

    pub fn create(&self) -> Result<()> {
        self.con.execute_batch(SQL_INIT)?;

        Ok(())
    }

    pub fn delete(self) -> Result<()> {
        remove_file(&self.path).map_err(|_| PuwumpError::DbRemoval)?;

        Ok(())
    }

    pub fn reset(self) -> Result<Self> {
        self.delete()?;
        let db = Self::init()?;
        db.create()?;
        Ok(db)
    }

    pub fn new_exercise(&self, name: &str, instructions: &str) -> Result<()> {
        let id = Uuid::new_v4().to_string();
        self.con
            .execute("INSERT INTO exercise (id, name, instructions) VALUES (?1, ?2, ?3)", params![id, name, instructions])?;
        Ok(())
    }

    pub fn new_plan(&self, name: &str, description: &str, est_min: u16) -> Result<()> {
        let id = Uuid::new_v4().to_string();
        self.con
            .execute("INSERT INTO plan (id, name, description, est_mins)  VALUES (?1, ?2, ?3, ?4)", params![id, name, description, est_min])?;
        Ok(())
    }

    pub fn get_all_plans(&self) -> Result<Vec<Uuid>> {
        let stmt = self
            .con
            .prepare("SELECT id FROM plan")?;
        let ids = ids_from_statement(stmt)?;
        Ok(ids)
    }

    pub fn get_all_exercises(&self) -> Result<Vec<Uuid>> {
        let stmt = self
            .con
            .prepare("SELECT id FROM exercise")?;

        let ids = ids_from_statement(stmt)?;

        Ok(ids)
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

    pub fn insert_exercise(&self, plan_id: Uuid, exercise_id: Uuid, reps: u16, order_index: u16) -> Result<()> {
        self.con.execute(
            "INSERT INTO plan_exercise (plan_id, exercise_id, reps, order_index) VALUES (?1, ?2, ?3, ?4)",
            params![plan_id.to_string(), exercise_id.to_string(), reps, order_index],
        )?;
        Ok(())
    }
}
