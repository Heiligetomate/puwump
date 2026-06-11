use egui::{Align, Button, Color32, Layout, RichText, Ui};
use uuid::Uuid;

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
        self.name.is_empty() || self.instructions.is_empty()
    }

    fn set_err(&mut self, message: &str) {
        self.status = Some(Err(message.to_owned()));
    }
}

impl PuwumpUi {
    pub fn add_exercise_view(&mut self, ui: &mut Ui) {
        let width = self.sizes.width;
        let height = self.sizes.height;
        let margin = self.sizes.margin;
        let inner_margin = (width * 0.01) as i8;
        let form_width = width * 0.4;
        let list_width = width * 0.55 - margin * 2.0;

        self.no_text_box_hover(ui);
        ui.style_mut()
            .text_styles
            .insert(egui::TextStyle::Body, egui::FontId::proportional(height * 0.03));

        ui.add_space(height * 0.05);
        let available_height = ui.available_height();

        ui.horizontal(|ui| {
            ui.add_space(margin);
            self.exercise_form(ui, form_width, height);
            ui.add_space(margin);
            ui.separator();
            ui.add_space(margin);
            if let Some(id) = self.exercise_list(ui, list_width, available_height, inner_margin) {
                self.db.remove_exercise(id).unwrap();
                self.exercises = self
                    .db
                    .get_all_exercises()
                    .unwrap_or_default();
            }
        });
    }

    /// Full add-exercise form
    fn exercise_form(&mut self, ui: &mut Ui, form_width: f32, height: f32) {
        ui.vertical(|ui| {
            ui.set_width(form_width);
            self.exercise_form_fields(ui, form_width);
            ui.add_space(height * 0.02);
            if self.button(ui, form_width, height * 0.07, self.theme.green, "Confirm") {
                self.on_exercise_confirm();
            }
            self.exercise_status(ui);
        });
    }

    /// Creates the fields needed to create a new eercise  
    fn exercise_form_fields(&mut self, ui: &mut Ui, height: f32) {
        text_field(ui, &self.theme, &self.sizes, |ui| {
            ui.add(
                egui::TextEdit::singleline(&mut self.add_exercise.name)
                    .hint_text("Name")
                    .desired_width(f32::INFINITY)
                    .background_color(Color32::TRANSPARENT),
            );
        });
        ui.add_space(height * 0.02);
        text_field(ui, &self.theme, &self.sizes, |ui| {
            ui.add(
                egui::TextEdit::multiline(&mut self.add_exercise.instructions)
                    .hint_text("Instructions")
                    .desired_width(f32::INFINITY)
                    .desired_rows(6)
                    .background_color(Color32::TRANSPARENT),
            );
        });
    }

    /// Handles the input when the confirm button for the exercises is pressed
    fn on_exercise_confirm(&mut self) {
        if self.add_exercise.is_empty() {
            self.add_exercise
                .set_err("Fill out both fields");
            return;
        }
        match self
            .db
            .insert_exercise(&self.add_exercise.name, &self.add_exercise.instructions)
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
                self.add_exercise
                    .set_err("Exercise already exists");
            }
            Err(_) => panic!("db is broken"),
        }
    }

    /// Does not do anything (returns) if the exercise status is None
    /// Adds a green text "Exercise saved!" if the stuats is Ok()
    /// Adds a red text with the error message if the status is Err()
    fn exercise_status(&self, ui: &mut Ui) {
        let Some(status) = &self.add_exercise.status else { return };
        let (text, color) = match status {
            Ok(_) => ("Exercise saved!", self.theme.green),
            Err(e) => (e.as_str(), self.theme.red),
        };
        ui.label(RichText::new(text).color(color));
    }

    /// lists all currently available exercises
    fn exercise_list(&self, ui: &mut Ui, list_width: f32, available_height: f32, inner_margin: i8) -> Option<Uuid> {
        let margin = self.sizes.margin / 3.0;
        let mut to_delete = None;
        ui.vertical(|ui| {
            ui.set_width(list_width);
            ui.set_min_height(available_height);
            egui::ScrollArea::vertical()
                .max_width(list_width - margin)
                .max_height(available_height)
                .show(ui, |ui| {
                    for exercise in &self.exercises {
                        if self.exercise_card(ui, exercise, list_width, margin, inner_margin) {
                            to_delete = Some(exercise.id);
                        }
                        ui.add_space(margin * 0.5);
                    }
                    ui.add_space(margin);
                });
        });
        to_delete
    }

    /// Generates a field containing the title, description and a delete button for one exercise
    fn exercise_card(&self, ui: &mut Ui, exercise: &Exercise, list_width: f32, margin: f32, inner_margin: i8) -> bool {
        let mut deleted = false;
        egui::Frame::NONE
            .fill(self.theme.text_field)
            .corner_radius(self.sizes.corner_radius)
            .inner_margin(egui::Margin::same(inner_margin))
            .show(ui, |ui| {
                ui.set_width(list_width - margin * 2.0);
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new(&exercise.name)
                            .color(self.theme.title)
                            .strong()
                            .size(20.0),
                    );
                    if self.delete_button(ui) {
                        deleted = true
                    }
                });
                ui.separator();
                ui.add(
                    egui::Label::new(
                        RichText::new(&exercise.instructions)
                            .color(self.theme.fg)
                            .weak()
                            .size(16.0),
                    )
                    .wrap(),
                );
            });
        deleted
    }

    /// Creates a small red round delete button with a X
    fn delete_button(&self, ui: &mut Ui) -> bool {
        let mut clicked = false;
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            if ui
                .add(
                    Button::new(
                        RichText::new("X")
                            .color(self.theme.white)
                            .size(12.0),
                    )
                    .fill(self.theme.red)
                    .corner_radius(self.sizes.corner_radius),
                )
                .clicked()
            {
                clicked = true;
            }
        });
        clicked
    }
}
