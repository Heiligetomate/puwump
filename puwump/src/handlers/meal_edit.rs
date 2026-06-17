use uuid::Uuid;

use crate::{
    db::Db,
    errors::{PuwumpError, Result},
    handlers::EditHandler,
    models::{Ingredient, Meal, MealIngredientDetail},
    ui::ButtonTheme,
};

#[derive(Default)]
pub struct MealEditHandler {
    selectable: Vec<Ingredient>,
    selected: Option<Meal>,
    data: Vec<Meal>,
    sel_data: Option<Vec<MealIngredientDetail>>,
}

impl MealEditHandler {
    pub fn new(db: &Db) -> Result<Self> {
        let meals = db.get_all_meals()?;
        let ingredients = db.get_all_ingredients()?;
        Ok(Self {
            data: meals,
            selectable: ingredients,
            selected: None,
            sel_data: None,
        })
    }
}

impl EditHandler for MealEditHandler {
    type Model = Meal;
    type SelModel = MealIngredientDetail;
    type Selectable = Ingredient;

    fn get_selectable(&self) -> &Vec<Self::Selectable> {
        &self.selectable
    }

    fn sel_is_none(&self) -> bool {
        self.selected.is_none()
    }

    fn update_selectable(&mut self, db: &Db) -> Result<()> {
        let new_selectable = db.get_all_ingredients()?;
        self.selectable = new_selectable;

        Ok(())
    }

    fn insert_handler_model(&self, db: &Db, id: Uuid) -> Result<()> {
        let selected = self
            .get_selected()
            .ok_or(PuwumpError::SelectedDataNotFound)?
            .id;
        db.insert_meal_ingredient(selected, id, 1)
    }

    fn card_buttons() -> &'static [ButtonTheme] {
        const BUTTONS: [ButtonTheme; 3] = [ButtonTheme::delete(), ButtonTheme::plus(), ButtonTheme::minus()];
        &BUTTONS
    }

    fn handle_buttons(&mut self, results: Vec<(Uuid, Vec<bool>)>, db: &Db) -> Result<()> {
        for (id, clicked) in results {
            if clicked[0] {
                db.remove_meal_ingredient(id)?;
                self.updated_sel_data(db)?;
            } else if clicked[1] {
                db.incr_meal_ingredient(id, 1)?;
                self.updated_sel_data(db)?;
            } else if clicked[2] {
                db.decr_meal_ingredient(id, 1)?;
                self.updated_sel_data(db)?;
            }
        }

        Ok(())
    }

    fn update(&mut self, db: &Db) -> Result<()> {
        self.data = db.get_all_meals()?;

        Ok(())
    }

    fn get_data(&self) -> &Vec<Self::Model> {
        &self.data
    }

    fn update_sel(&mut self, db: &Db, id: Uuid) -> Result<()> {
        let new_meal = db.get_meal(id)?;
        self.selected = Some(new_meal);

        Ok(())
    }

    fn get_selected(&self) -> Option<&Self::Model> {
        self.selected.as_ref()
    }

    fn get_sel_data(&self) -> Result<&Vec<Self::SelModel>> {
        Ok(self
            .sel_data
            .as_ref()
            .ok_or(PuwumpError::SelectedDataNotFound)?)
    }

    fn updated_sel_data(&mut self, db: &Db) -> Result<()> {
        if let Some(sel) = &self.selected {
            let meals = db.get_meal_ingredients(sel.id)?;
            self.sel_data = Some(meals);
            return Ok(());
        }

        Err(PuwumpError::SelectedDataNotFound)
    }
}
