use egui::{Color32, Ui};

use crate::ui::{core::PuwumpUi, util::text_field};

impl PuwumpUi {
    pub fn add_exercise_view(&mut self, ui: &mut Ui) {
        let width = ui.available_width();
        let height = ui.available_height();
        let margin = width * 0.05;
        let inner_width = width - margin * 2.0;
        let inner_margin = (width * 0.02) as i8;

        self.no_text_box_hover(ui);

        ui.style_mut()
            .text_styles
            .insert(egui::TextStyle::Body, egui::FontId::proportional(height * 0.03));

        ui.add_space(height * 0.05);

        ui.vertical_centered(|ui| {
            ui.set_width(inner_width);

            text_field(ui, inner_margin, |ui| {
                ui.add(
                    egui::TextEdit::singleline(&mut self.add_exercise.name)
                        .hint_text("Name")
                        .desired_width(f32::INFINITY)
                        .background_color(Color32::TRANSPARENT),
                );
            });

            ui.add_space(height * 0.02);
            text_field(ui, inner_margin, |ui| {
                ui.add(
                    egui::TextEdit::multiline(&mut self.add_exercise.instructions)
                        .hint_text("Instructions")
                        .desired_width(f32::INFINITY)
                        .desired_rows(6)
                        .background_color(Color32::TRANSPARENT),
                );
            });

            ui.add_space(height * 0.02);

            if self.button(ui, inner_width * 0.5, height * 0.07, Color32::from_rgb(184, 187, 38), "Confirm") {
                // TODO: save to db
            }
        });
    }
}
