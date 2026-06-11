use egui::{Color32, RichText, TextEdit, Ui};
use uuid::Uuid;

use crate::{
    errors::PuwumpError,
    models::{CardAdd, core::Model},
    ui::{core::PuwumpUi, task_handler::AddTaskHandler, util::text_field},
};

impl PuwumpUi {
    pub fn add_view<A: Model + CardAdd>(&mut self, ui: &mut Ui, task_handler: &mut AddTaskHandler<A>) {
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
            self.add_form(ui, form_width, height, task_handler);
            ui.add_space(margin);
            ui.separator();
            ui.add_space(margin);
            if let Some(id) = self.add_list(ui, list_width, available_height, inner_margin, task_handler) {
                A::delete(&self.db, id).unwrap();
                task_handler.data = A::get_all(&self.db).unwrap_or_default();
            }
        });
    }

    pub fn add_list<A: CardAdd + Model>(&self, ui: &mut Ui, list_width: f32, available_height: f32, inner_margin: i8, task_handler: &AddTaskHandler<A>) -> Option<Uuid> {
        let margin = self.sizes.margin / 3.0;
        let mut to_delete = None;
        ui.vertical(|ui| {
            ui.set_width(list_width);
            ui.set_min_height(available_height);
            egui::ScrollArea::vertical()
                .max_width(list_width - margin)
                .max_height(available_height)
                .show(ui, |ui| {
                    for elem in task_handler.data.iter() {
                        if self.exercise_card(ui, elem, list_width, margin, inner_margin) {
                            to_delete = Some(elem.key());
                        }
                        ui.add_space(margin * 0.5);
                    }
                    ui.add_space(margin);
                });
        });
        to_delete
    }

    /// Generates a field containing the title, description and a delete button for one exercise
    pub fn exercise_card<A: CardAdd>(&self, ui: &mut Ui, card: &A, list_width: f32, margin: f32, inner_margin: i8) -> bool {
        let mut deleted = false;
        egui::Frame::NONE
            .fill(self.theme.text_field)
            .corner_radius(self.sizes.corner_radius)
            .inner_margin(egui::Margin::same(inner_margin))
            .show(ui, |ui| {
                ui.set_width(list_width - margin * 2.0);
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new(card.title())
                            .color(self.theme.title)
                            .strong()
                            .size(20.0),
                    );
                    if self.delete_button(ui) {
                        deleted = true
                    }
                });
                if let Some(body) = card.body() {
                    ui.separator();
                    ui.add(
                        egui::Label::new(
                            RichText::new(body)
                                .color(self.theme.fg)
                                .weak()
                                .size(16.0),
                        )
                        .wrap(),
                    );
                }
            });
        deleted
    }

    /// Handles the input when the confirm button for the exercises is pressed
    pub fn on_add_confirm<A: CardAdd + Model>(&mut self, task_handler: &mut AddTaskHandler<A>) {
        if task_handler.track_empty() {
            task_handler.set_err("Fill out both fields");
            return;
        }
        match A::insert(&self.db, &task_handler.title_track, task_handler.body_track.as_deref()) {
            Ok(_) => {
                task_handler.status = Some(Ok(()));
                task_handler.reset();
                task_handler.data = A::get_all(&self.db).unwrap(); // TODO: handle
            }
            Err(PuwumpError::UniqueViolation) => {
                task_handler.set_err("Exercise already exists");
            }
            Err(e) => panic!("db is broken {e}"),
        }
    }

    /// Does not do anything (returns) if the exercise status is None
    /// Adds a green text "Exercise saved!" if the stuats is Ok()
    /// Adds a red text with the error message if the status is Err()
    pub fn exercise_status<A: CardAdd>(&self, ui: &mut Ui, task_handler: &AddTaskHandler<A>) {
        let Some(status) = &task_handler.status else {
            return;
        };

        let (text, color) = match status {
            Ok(_) => ("Successfully saved!", self.theme.green),
            Err(e) => (e.as_str(), self.theme.red),
        };

        ui.label(RichText::new(text).color(color));
    }

    /// Full add-exercise form
    fn add_form<A: CardAdd + Model>(&mut self, ui: &mut Ui, form_width: f32, height: f32, task_handler: &mut AddTaskHandler<A>) {
        ui.vertical(|ui| {
            ui.set_width(form_width);
            self.add_form_fields(ui, form_width, task_handler);
            ui.add_space(height * 0.02);
            if self.button(ui, form_width, height * 0.07, self.theme.green, "Confirm") {
                self.on_add_confirm(task_handler);
            }
            self.exercise_status(ui, task_handler);
        });
    }

    /// Creates the fields needed to create a new eercise  
    fn add_form_fields<A: CardAdd + Model>(&mut self, ui: &mut Ui, height: f32, task_handler: &mut AddTaskHandler<A>) {
        text_field(ui, &self.theme, &self.sizes, |ui| {
            ui.add(
                TextEdit::singleline(&mut task_handler.title_track)
                    .hint_text("Name")
                    .desired_width(f32::INFINITY)
                    .background_color(Color32::TRANSPARENT),
            );
        });
        if let Some(body) = task_handler.body_track.as_mut() {
            ui.add_space(height * 0.02);

            text_field(ui, &self.theme, &self.sizes, |ui| {
                ui.add(
                    TextEdit::multiline(body)
                        .hint_text("Instructions")
                        .desired_width(f32::INFINITY)
                        .desired_rows(6)
                        .background_color(Color32::TRANSPARENT),
                );
            });
        }
    }
}
