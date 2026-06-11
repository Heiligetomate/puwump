use rusqlite::params;

use crate::{db::Db, errors::Result};

impl Db {
    pub fn insert_meal(&self, name: &str, description: &str, calories: u32) -> Result<()> {
        self.con.execute(
            "INSERT INTO meal (name, description, calories) VALUES (?1, ?2, ?3)",
            params![name.to_owned(), description.to_owned(), calories],
        )?;
        Ok(())
    }

    pub fn insert_ingredient(&self, name: &str) -> Result<()> {
        self.con
            .execute("INSERT INTO ingredient (name) VALUES (?1)", params![name.to_owned()])?;
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
}
