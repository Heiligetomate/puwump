use egui::Ui;

use crate::ui::core::{PuwumpUi, View};

impl PuwumpUi {
    pub fn home_view(&mut self, ui: &mut Ui) {
        println!("{} {}", self.sizes.height, ui.available_height());
        let button_height = self.calc_button_height(ui, 3);

        ui.add_space(self.sizes.margin);
        ui.vertical_centered(|ui| {
            if self.button_full_width(ui, button_height, self.theme.blue, "Add Exercise") {
                self.view = View::AddExercise;
                self.exercises = self
                    .db
                    .get_all_exercises()
                    .unwrap_or_default();
            }

            ui.add_space(self.sizes.spacing);
            if self.button_full_width(ui, button_height, self.theme.green, "Add Plan") {
                self.view = View::AddPlan;
            }

            ui.add_space(self.sizes.spacing);
            if self.button_full_width(ui, button_height, self.theme.red, "Workout") {
                self.view = View::Workout;
            }
        });
    }
}
