use egui::Ui;

use crate::{
    handlers::EditHandler,
    ui::core::{PuwumpUi, View},
};

impl PuwumpUi {
    pub fn home_view(&mut self, ui: &mut Ui) {
        let button_height = self.calc_button_height(ui, 8);

        let spc = self.spacing(ui);

        ui.add_space(self.sizes.margin);
        ui.vertical_centered(|ui| {
            if self.button_full_width(ui, button_height, self.theme.blue, "Add Exercise") {
                self.view = View::AddExercise;
                self.exercise_hndl.data = self
                    .db
                    .get_all_exercises()
                    .unwrap_or_default();
            }

            ui.add_space(spc);
            if self.button_full_width(ui, button_height, self.theme.blue, "Add Meal") {
                self.view = View::AddMeal;
                self.meal_hndl.data = self
                    .db
                    .get_all_meals()
                    .unwrap_or_default()
            }

            ui.add_space(spc);
            if self.button_full_width(ui, button_height, self.theme.blue, "Add Ingredient") {
                self.view = View::AddIngredient;
                self.ingredient_hdnl.data = self
                    .db
                    .get_all_ingredients()
                    .unwrap_or_default()
            }

            ui.add_space(spc);
            if self.button_full_width(ui, button_height, self.theme.blue, "Add Plan") {
                self.view = View::AddPlan
            }

            ui.add_space(spc);
            if self.button_full_width(ui, button_height, self.theme.green, "Edit Plan") {
                self.exercise_hndl.data = self
                    .db
                    .get_all_exercises()
                    .unwrap_or_default();
                self.view = View::EditPlan;
            }

            ui.add_space(spc);
            if self.button_full_width(ui, button_height, self.theme.green, "Edit Meal") {
                self.edit_meal_hndl
                    .update_selectable(&self.db)
                    .unwrap();
                self.view = View::EditMeal;
            }

            ui.add_space(spc);
            if self.button_full_width(ui, button_height, self.theme.red, "Workout") {
                self.workout_hndl
                    .update_plans(&self.db)
                    .unwrap();
                self.view = View::Workout;
            }

            ui.add_space(spc);
            if self.button_full_width(ui, button_height, self.theme.red, "Eat Meal") {
                self.eat_hndl
                    .update_meals(&self.db)
                    .unwrap();
                self.view = View::EatMeal;
            }
        });
    }
}
