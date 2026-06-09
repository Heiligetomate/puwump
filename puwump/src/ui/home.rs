use egui::{Color32, Ui};

use crate::ui::core::{PuwumpUi, View};

impl PuwumpUi {
    pub fn home_view(&mut self, ui: &mut Ui) {
        let red = Color32::from_rgb(204, 36, 29);
        let green = Color32::from_rgb(184, 187, 38);
        let blue = Color32::from_rgb(69, 133, 136);

        let width = ui.available_width();
        let height = ui.available_height();
        let margin = width * 0.05;
        let spacing = height * 0.02;
        let button_width = width - margin * 2.0;
        let button_height = (height - margin * 2.0 - spacing * 2.0) / 3.0;

        ui.add_space(margin);
        ui.vertical_centered(|ui| {
            if self.button(ui, button_width, button_height, blue, "Add Exercise") {
                self.view = View::AddExercise;
            }
            ui.add_space(spacing);
            if self.button(ui, button_width, button_height, green, "Add Plan") {
                self.view = View::AddPlan;
            }
            ui.add_space(spacing);
            if self.button(ui, button_width, button_height, red, "Workout") {
                self.view = View::Workout;
            }
        });
    }
}
