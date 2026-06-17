use std::str::FromStr;

use uuid::Uuid;

use crate::{
    db::Db,
    errors::{PuwumpError, Result},
    models::{Ingredient, Meal, MealIngredientDetail, Model, statement_to_model},
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

    /// Removes the ingredient with the given id
    /// Every Recipe entry gets deleted automatically
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
    pub fn insert_meal_ingredient(&self, meal_id: Uuid, ingredient_id: Uuid, amount: u32) -> Result<()> {
        let id = Uuid::new_v4();
        self.con
            .execute(
                "INSERT INTO ingredient_in_meal (id, amount_gr, meal_id, ingredient_id) VALUES (?1, ?2, ?3, ?4)",
                (id.to_string(), amount, meal_id.to_string(), ingredient_id.to_string()),
            )
            .map_err(Self::map_sqlite_err)?;
        Ok(())
    }

    /// Insert a new meal
    /// Name is unique
    pub fn insert_meal(&self, name: &str, description: &str, calories: u32) -> Result<()> {
        let id = Uuid::new_v4();
        self.con.execute(
            "INSERT INTO meal (id, name, description, calories) VALUES (?1, ?2, ?3, ?4)",
            (id.to_string(), name, description, calories),
        )?;
        Ok(())
    }

    /// Remove a meal by its id
    pub fn remove_meal(&self, id: Uuid) -> Result<()> {
        self.con
            .execute("DELETE FROM meal WHERE id = ?1", (id.to_string(),))?;
        Ok(())
    }

    /// Get a meal by its id
    pub fn get_meal(&self, id: Uuid) -> Result<Meal> {
        let stmt = self
            .con
            .prepare("SELECT * FROM meal WHERE id = ?1")?;
        statement_to_model(stmt, (id.to_string(),))
    }

    /// Returns a Vec with all meal ids
    /// Ordered by name, case-insensitive
    pub fn get_all_meal_ids(&self) -> Result<Vec<Uuid>> {
        let mut stmt = self
            .con
            .prepare("SELECT id FROM meal ORDER BY name COLLATE NOCASE ASC")?;
        let raw_ids = stmt
            .query_map([], |row| row.get(0))?
            .collect::<rusqlite::Result<Vec<String>>>()
            .map_err(|_| PuwumpError::RowNotFound)?;
        let ids: Vec<Uuid> = raw_ids
            .iter()
            .filter_map(|i| Uuid::parse_str(i).ok())
            .collect();
        Ok(ids)
    }

    /// Returns a Vec with all meals as Meal objects
    /// Ordered by name, case-insensitive
    pub fn get_all_meals(&self) -> Result<Vec<Meal>> {
        let mut meals = Vec::new();
        for id in self.get_all_meal_ids()? {
            meals.push(self.get_meal(id)?);
        }
        Ok(meals)
    }

    /// Returns a Vec with all ingredients in a recipe  
    /// Returns an object containing the ingredient and the amount in grams
    pub fn get_meal_ingredients(&self, meal_id: Uuid) -> Result<Vec<MealIngredientDetail>> {
        let mut stmt = self.con.prepare(
            "SELECT im.id, i.id, i.name, im.amount_gr
     FROM ingredient_in_meal im
     JOIN ingredient i ON i.id = im.ingredient_id
     WHERE im.meal_id = ?1",
        )?;
        let ingredients = stmt
            .query_map((meal_id.to_string(),), <MealIngredientDetail as Model>::from_row)?
            .collect::<rusqlite::Result<Vec<MealIngredientDetail>>>()
            .map_err(|_| PuwumpError::RowNotFound)?;
        Ok(ingredients)
    }

    pub fn incr_meal_ingredient(&self, id: Uuid, incr: u16) -> Result<()> {
        self.con
            .execute("UPDATE ingredient_in_meal SET amount_gr = amount_gr + ?2 WHERE id = ?1", (id.to_string(), incr))?;
        Ok(())
    }

    pub fn decr_meal_ingredient(&self, id: Uuid, decr: u16) -> Result<()> {
        self.con
            .execute("UPDATE ingredient_in_meal SET amount_gr = MAX(amount_gr - ?2, 0) WHERE id = ?1", (id.to_string(), decr))?;
        Ok(())
    }

    pub fn remove_meal_ingredient(&self, id: Uuid) -> Result<()> {
        self.con
            .execute("DELETE FROM ingredient_in_meal WHERE id = ?1", (id.to_string(),))?;
        Ok(())
    }
}
