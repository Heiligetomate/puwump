use egui::{Color32, RichText, Ui};

use crate::{
    errors::PuwumpError,
    ui::{core::PuwumpUi, util::text_field},
};

#[derive(Default)]
pub struct AddExerciseForm {
    pub name: String,
    pub instructions: String,
    pub status: Option<Result<(), String>>,
}

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
                match self
                    .db
                    .new_exercise(&self.add_exercise.name, &self.add_exercise.instructions)
                {
                    Ok(_) => self.add_exercise.status = Some(Ok(())),
                    Err(e) => match e {
                        PuwumpError::UniqueViolation => self.add_exercise.status = Some(Err(e.to_string())),
                        _ => panic!("db is broken"),
                    },
                }
            }

            if let Some(status) = &self.add_exercise.status {
                match status {
                    Ok(_) => ui.label(RichText::new("Exercise saved!").color(Color32::from_rgb(184, 187, 38))),
                    Err(e) => ui.label(RichText::new(e).color(Color32::from_rgb(204, 36, 29))),
                };
            }
        });
    }
}
