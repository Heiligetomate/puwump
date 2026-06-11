use std::str::FromStr;

use uuid::Uuid;

use crate::{
    db::Db,
    errors::{PuwumpError, Result},
    models::{
        Ingredient, Meal, MealIngredientDetail,
        core::{Model, statement_to_model},
    },
};

impl Db {
    /// Insert a new ingredient
    /// The name is unique
    /// Returns the Id
    pub fn insert_ingredient(&self, name: &str) -> Result<Uuid> {
        let id = Uuid::new_v4();
        self.con
            .execute("INSERT INTO ingredient (name, id) VALUES (?1, ?2)", (name, id.to_string()))?;
        Ok(id)
    }

    /// Get an ingredient by its name
    /// Just useful for mapping and checking if the ingredient exists
    pub fn get_ingredient(&self, id: Uuid) -> Result<Ingredient> {
        let stmt = self
            .con
            .prepare("SELECT name FROM ingredient WHERE id = ?1")?;
        statement_to_model(stmt, (id.to_string(),))
    }

    pub fn remove_ingredient(&self, id: Uuid) -> Result<()> {
        self.con
            .execute("DELETE FROM ingredient WHERE id = ?1", (id.to_string(),))?;

        Ok(())
    }

    /// Returns a Vec with all ingredient names
    /// Ordered by name, case-insensitive
    pub fn get_all_ingredient_names(&self) -> Result<Vec<String>> {
        let mut stmt = self
            .con
            .prepare("SELECT name FROM ingredient ORDER BY name COLLATE NOCASE ASC")?;
        let names = stmt
            .query_map([], |row| row.get(0))?
            .collect::<rusqlite::Result<Vec<String>>>()
            .map_err(|_| PuwumpError::RowNotFound)?;
        Ok(names)
    }

    /// Returns a Vec with all ingredient ids
    /// Ordered by name, case-insensitive
    pub fn get_all_ingredient_ids(&self) -> Result<Vec<Uuid>> {
        let mut stmt = self
            .con
            .prepare("SELECT id FROM ingredient ORDER BY name COLLATE NOCASE ASC")?;
        let raw_ids = stmt
            .query_map([], |row| row.get(0))?
            .collect::<rusqlite::Result<Vec<String>>>()
            .map_err(|e| PuwumpError::Rusqlite(e.to_string()))?;
        let ids = raw_ids
            .iter()
            .map(|s| Ok(Uuid::from_str(s).map_err(|_| PuwumpError::UuidParse)?))
            .collect::<crate::errors::Result<Vec<Uuid>>>();

        Ok(ids?)
    }

    /// Returns a Vec with all ingredients as Ingredient objects
    /// Ordered by name, case-insensitive
    pub fn get_all_ingredients(&self) -> Result<Vec<Ingredient>> {
        let mut stmt = self.con.prepare(
            "SELECT id, name
         FROM ingredient
         ORDER BY name COLLATE NOCASE ASC",
        )?;

        let ingredients = stmt
            .query_map([], |row| {
                let id: String = row.get(0)?;
                let name: String = row.get(1)?;

                Ok(Ingredient {
                    id: Uuid::parse_str(&id).map_err(|_| rusqlite::Error::InvalidColumnType(0, "id".to_string(), rusqlite::types::Type::Text))?,
                    name,
                })
            })?
            .collect::<rusqlite::Result<Vec<_>>>()?;

        Ok(ingredients)
    }

    /// Add an ingredient to a meal
    /// Amount is amount in grams
    pub fn insert_meal_ingredient(&self, meal_name: &str, ingredient_name: &str, amount: u32) -> Result<()> {
        self.con
            .execute(
                "INSERT INTO ingredient_in_meal (amount_gr, meal_name, ingredient_name) VALUES (?1, ?2, ?3)",
                (amount, meal_name, ingredient_name),
            )
            .map_err(Self::map_sqlite_err)?;
        Ok(())
    }

    /// Insert a new meal
    /// Name is unique
    pub fn insert_meal(&self, name: &str, description: &str, calories: u32) -> Result<()> {
        self.con
            .execute("INSERT INTO meal (name, description, calories) VALUES (?1, ?2, ?3)", (name, description, calories))?;
        Ok(())
    }

    /// Remove a meal by its name
    pub fn remove_meal(&self, name: &str) -> Result<()> {
        self.con
            .execute("DELETE FROM meal WHERE name = ?1", (name,))?;
        Ok(())
    }

    /// Get a meal by its name
    pub fn get_meal(&self, name: &str) -> Result<Meal> {
        let stmt = self
            .con
            .prepare("SELECT * FROM meal WHERE name = ?1")?;
        statement_to_model(stmt, (name,))
    }

    /// Returns a Vec with all meal names
    /// Ordered by name, case-insensitive
    pub fn get_all_meal_names(&self) -> Result<Vec<String>> {
        let mut stmt = self
            .con
            .prepare("SELECT name FROM meal ORDER BY name COLLATE NOCASE ASC")?;
        let names = stmt
            .query_map([], |row| row.get(0))?
            .collect::<rusqlite::Result<Vec<String>>>()
            .map_err(|_| PuwumpError::RowNotFound)?;
        Ok(names)
    }

    /// Returns a Vec with all meals as Meal objects
    /// Ordered by name, case-insensitive
    pub fn get_all_meals(&self) -> Result<Vec<Meal>> {
        let mut meals = Vec::new();
        for name in self.get_all_meal_names()? {
            meals.push(self.get_meal(&name)?);
        }
        Ok(meals)
    }

    /// Returns a Vec with all ingredients in a recipe  
    /// Returns an object containing the ingredient and the amount in grams
    pub fn get_meal_ingredients(&self, meal_name: &str) -> Result<Vec<MealIngredientDetail>> {
        let mut stmt = self.con.prepare(
            "SELECT i.name, im.amount_gr
        FROM ingredient_in_meal im
        JOIN ingredient i ON i.name = im.ingredient_name
        WHERE im.meal_name = ?1",
        )?;
        let ingredients = stmt
            .query_map((meal_name,), <MealIngredientDetail as Model>::from_row)?
            .collect::<rusqlite::Result<Vec<MealIngredientDetail>>>()
            .map_err(|_| PuwumpError::RowNotFound)?;
        Ok(ingredients)
    }
}
