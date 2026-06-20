use egui::Ui;

use crate::{
    handlers::EditHandler,
    ui::core::{PuwumpUi, View},
};

impl PuwumpUi {
    pub fn home_view(&mut self, ui: &mut Ui) {
        let button_height = self.calc_button_height(ui, 4);
        let spc = self.spacing(ui);

        ui.add_space(self.sizes.margin);

        ui.vertical_centered(|ui| {
            ui.spacing_mut().item_spacing.x = spc;

            let button_width = (ui.available_width() - 2.0 * self.sizes.margin - spc) / 2.0;

            ui.horizontal(|ui| {
                ui.add_space(self.sizes.margin);

                if self.button(ui, button_width, button_height, self.theme.blue, "Add Exercise") {
                    self.view = View::AddExercise;
                    self.exercise_hndl.data = self
                        .db
                        .get_all_exercises()
                        .unwrap_or_default();
                }
                if self.button(ui, button_width, button_height, self.theme.blue, "Add Ingredient") {
                    self.view = View::AddIngredient;
                    self.ingredient_hdnl.data = self
                        .db
                        .get_all_ingredients()
                        .unwrap_or_default();
                }
            });

            ui.add_space(spc);

            ui.horizontal(|ui| {
                ui.add_space(self.sizes.margin);
                if self.button(ui, button_width, button_height, self.theme.blue, "Add Plan") {
                    self.view = View::AddPlan;
                    self.edit_plan_hndl.data = self
                        .db
                        .get_all_plans()
                        .unwrap_or_default();
                }

                if self.button(ui, button_width, button_height, self.theme.blue, "Add Meal") {
                    self.view = View::AddMeal;
                    self.meal_hndl.data = self
                        .db
                        .get_all_meals()
                        .unwrap_or_default();
                }
            });

            ui.add_space(spc);

            ui.horizontal(|ui| {
                ui.add_space(self.sizes.margin);

                if self.button(ui, button_width, button_height, self.theme.green, "Edit Plan") {
                    self.exercise_hndl
                        .refresh_data(&self.db)
                        .ok();
                    self.view = View::EditPlan;
                }

                if self.button(ui, button_width, button_height, self.theme.green, "Edit Meal") {
                    self.edit_meal_hndl
                        .update_selectable(&self.db)
                        .unwrap();
                    self.view = View::EditMeal;
                }
            });

            ui.add_space(spc);

            ui.horizontal(|ui| {
                ui.add_space(self.sizes.margin);

                if self.button(ui, button_width, button_height, self.theme.red, "Workout") {
                    self.view = View::Workout;
                }

                if self.button(ui, button_width, button_height, self.theme.red, "Eat") {
                    self.view = View::EatMeal;
                }
            });
        });
    }
}
