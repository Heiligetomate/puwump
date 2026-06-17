use egui::{Color32, RichText, TextEdit, Ui};
use uuid::Uuid;

use crate::{
    errors::PuwumpError,
    handlers::AddTaskHandler,
    models::{CardAdd, CardCrud, CardInputs, Model},
    ui::{core::PuwumpUi, theme::ButtonTheme, util::text_field},
};

impl PuwumpUi {
    /// Generates the full view
    pub fn add_view<A, I>(&mut self, ui: &mut Ui, task_handler: &mut AddTaskHandler<A, I>)
    where
        A: CardCrud,
        I: CardInputs,
    {
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
            for (id, clicked) in self.add_list(ui, list_width, available_height, inner_margin, &task_handler.data, &[ButtonTheme::delete()]) {
                if clicked[0] {
                    A::delete(&self.db, id).unwrap();
                    task_handler.data = A::get_all(&self.db).unwrap_or_default();
                }
            }
        });
    }

    /// Generates a full scrollable list for all add tasks
    /// Generates a delete button for each card if delete is set to true
    /// Returns an id if a task got deleted
    pub fn add_list<A: CardAdd + Model>(&self, ui: &mut Ui, list_width: f32, available_height: f32, inner_margin: i8, data: &Vec<A>, buttons: &[ButtonTheme]) -> Vec<(Uuid, Vec<bool>)> {
        let margin = self.sizes.margin / 3.0;
        let mut results = Vec::new();
        ui.vertical(|ui| {
            ui.set_width(list_width);
            ui.set_min_height(available_height);
            egui::ScrollArea::vertical()
                .max_width(list_width - margin)
                .max_height(available_height)
                .show(ui, |ui| {
                    for elem in data {
                        let clicked = self.add_card(ui, elem, list_width, margin, inner_margin, buttons);
                        results.push((elem.key(), clicked));
                        ui.add_space(margin * 0.5);
                    }
                    ui.add_space(margin);
                });
        });
        results
    }

    /// Generates a field containing the title and the body if existent
    /// Adds a delete button if delete is set to true
    pub fn add_card<A: CardAdd>(&self, ui: &mut Ui, card: &A, list_width: f32, margin: f32, inner_margin: i8, buttons: &[ButtonTheme]) -> Vec<bool> {
        let mut clicked = vec![false; buttons.len()];
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
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        for (i, theme) in buttons.iter().enumerate().rev() {
                            if self.card_button_labeled(ui, *theme) {
                                clicked[i] = true;
                            }
                        }
                    });
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
        clicked
    }

    /// Handles the input when the confirm button for the add task is pressed
    pub fn on_add_confirm<A: CardCrud, I: CardInputs>(&mut self, task_handler: &mut AddTaskHandler<A, I>) {
        if task_handler.input_fields.is_empty() {
            task_handler.set_err("Fill out all fields");
            return;
        }

        match A::insert(&self.db, task_handler.input_fields.get_fields()) {
            Ok(_) => {
                task_handler.status = Some(Ok(()));
                task_handler.input_fields.clear();
                task_handler.data = A::get_all(&self.db).unwrap(); // TODO: handle
            }
            Err(PuwumpError::UniqueViolation) => {
                let err = format!("{} already exists.", A::name());
                task_handler.set_err(err.as_str());
            }
            Err(PuwumpError::InputFieldIntParse(e)) => task_handler.set_err(e),
            Err(e) => panic!("db is broken {e}"),
        }
    }

    /// Does not do anything (returns) if the add task status is None
    /// Adds a green text "Successfully saved!" if the stuats is Ok()
    /// Adds a red text with the error message if the status is Err()
    pub fn add_status<A: CardCrud, I: CardInputs>(&self, ui: &mut Ui, task_handler: &AddTaskHandler<A, I>) {
        let Some(status) = &task_handler.status else {
            return;
        };

        let (text, color) = match status {
            Ok(_) => ("Successfully saved!", self.theme.green),
            Err(e) => (e.as_str(), self.theme.red),
        };

        ui.label(RichText::new(text).color(color));
    }

    /// Full add form
    fn add_form<A: CardCrud, I: CardInputs>(&mut self, ui: &mut Ui, form_width: f32, height: f32, task_handler: &mut AddTaskHandler<A, I>) {
        ui.vertical(|ui| {
            ui.set_width(form_width);
            self.add_form_fields(ui, form_width, task_handler);
            ui.add_space(height * 0.02);
            if self.button(ui, form_width, height * 0.07, self.theme.green, "Confirm") {
                self.on_add_confirm(task_handler);
            }
            self.add_status(ui, task_handler);
        });
    }

    /// Creates the fields needed to create a new eercise  
    fn add_form_fields<A: CardAdd, I: CardInputs>(&mut self, ui: &mut Ui, height: f32, task_handler: &mut AddTaskHandler<A, I>) {
        let fields = task_handler
            .input_fields
            .get_fields_mut();
        let last = fields.len().saturating_sub(1);
        for (i, input) in fields.into_iter().enumerate() {
            text_field(ui, &self.theme, &self.sizes, |ui| {
                let edit = if input.line_count > 1 {
                    TextEdit::multiline(&mut input.value).desired_rows(input.line_count as usize)
                } else {
                    TextEdit::singleline(&mut input.value)
                };
                ui.add(
                    edit.hint_text(input.hint_text.as_str())
                        .desired_width(f32::INFINITY)
                        .background_color(Color32::TRANSPARENT),
                );
            });
            if i < last {
                ui.add_space(height * 0.02);
            }
        }
    }
}
