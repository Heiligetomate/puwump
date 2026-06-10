use egui::{Color32, RichText, Ui};

use crate::{
    errors::PuwumpError,
    models::Exercise,
    ui::{core::PuwumpUi, util::text_field},
};

#[derive(Default)]
pub struct AddExerciseForm {
    pub name: String,
    pub instructions: String,
    pub status: Option<Result<(), String>>,
}

impl AddExerciseForm {
    fn reset(&mut self) {
        self.name = String::new();
        self.instructions = String::new();
    }

    fn is_empty(&self) -> bool {
        if self.name.is_empty() || self.instructions.is_empty() {
            return true;
        }
        false
    }
    fn set_err(&mut self, message: &str) {
        self.status = Some(Err(message.to_owned()));
    }
}

impl PuwumpUi {
    pub fn add_exercise_view(&mut self, ui: &mut Ui) {
        let width = ui.available_width();
        let height = ui.available_height();
        let margin = width * 0.05;
        let inner_margin = (width * 0.02) as i8;
        let form_width = width * 0.4;
        let list_width = width * 0.55 - margin * 2.0;

        self.no_text_box_hover(ui);
        ui.style_mut()
            .text_styles
            .insert(egui::TextStyle::Body, egui::FontId::proportional(height * 0.03));

        ui.add_space(height * 0.05);
        ui.horizontal(|ui| {
            ui.add_space(margin);
            self.exercise_form(ui, form_width, height, inner_margin);
            ui.add_space(margin);
            ui.separator();
            ui.add_space(margin);
            self.exercise_list(ui, list_width, height, margin, inner_margin);
        });
    }

    fn exercise_form(&mut self, ui: &mut Ui, form_width: f32, height: f32, inner_margin: i8) {
        ui.vertical(|ui| {
            ui.set_width(form_width);

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

            if self.button(ui, form_width, height * 0.07, Color32::from_rgb(184, 187, 38), "Confirm") {
                self.on_exercise_confirm();
            }

            self.exercise_status(ui);
        });
    }

    fn on_exercise_confirm(&mut self) {
        if self.add_exercise.is_empty() {
            self.add_exercise
                .set_err("Fill out both fields");
            return;
        }
        match self
            .db
            .new_exercise(&self.add_exercise.name, &self.add_exercise.instructions)
        {
            Ok(_) => {
                self.add_exercise.status = Some(Ok(()));
                self.add_exercise.reset();
                self.exercises = self
                    .db
                    .get_all_exercises()
                    .unwrap_or_default();
            }
            Err(PuwumpError::UniqueViolation) => {
                self.add_exercise.status = Some(Err("Exercise name already exists".to_string()));
            }
            Err(_) => panic!("db is broken"),
        }
    }

    fn exercise_status(&self, ui: &mut Ui) {
        if let Some(status) = &self.add_exercise.status {
            match status {
                Ok(_) => {
                    ui.label(RichText::new("Exercise saved!").color(Color32::from_rgb(184, 187, 38)));
                }
                Err(e) => {
                    ui.label(RichText::new(e).color(Color32::from_rgb(204, 36, 29)));
                }
            };
        }
    }

    fn exercise_list(&self, ui: &mut Ui, list_width: f32, height: f32, margin: f32, inner_margin: i8) {
        ui.vertical(|ui| {
            ui.set_width(list_width);
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .max_width(list_width - margin)
                .min_scrolled_height(height)
                .show(ui, |ui| {
                    for exercise in &self.exercises {
                        self.exercise_card(ui, exercise, list_width, height, margin, inner_margin);
                        ui.add_space(height * 0.01);
                    }
                });
        });
    }

    fn exercise_card(&self, ui: &mut Ui, exercise: &Exercise, list_width: f32, height: f32, margin: f32, inner_margin: i8) {
        egui::Frame::NONE
            .fill(Color32::from_rgb(60, 56, 54))
            .corner_radius(8.0)
            .inner_margin(egui::Margin::same(inner_margin))
            .show(ui, |ui| {
                ui.set_width(list_width - margin * 2.0);
                ui.label(
                    RichText::new(&exercise.name)
                        .color(self.theme.title)
                        .strong()
                        .size(height * 0.025),
                );
                ui.separator();
                ui.add(
                    egui::Label::new(
                        RichText::new(&exercise.instructions)
                            .color(self.theme.fg)
                            .weak(),
                    )
                    .wrap(),
                );
            });
    }
}
