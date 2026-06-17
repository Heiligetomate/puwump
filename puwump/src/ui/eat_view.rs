use egui::{Label, Margin, RichText, Ui};

use crate::{handlers::EatHandler, models::CardAdd, ui::core::PuwumpUi};

impl PuwumpUi {
    pub fn eat_view(&mut self, ui: &mut Ui) {
        let mut handler = std::mem::take(&mut self.eat_hndl);
        self.eat_view_inner(ui, &mut handler);
        self.eat_hndl = handler;
    }

    fn eat_view_inner(&mut self, ui: &mut Ui, handler: &mut EatHandler) {
        let width = self.sizes.width;
        let height = self.sizes.height;
        let margin = self.sizes.margin;

        ui.add_space(height * 0.05);

        ui.vertical_centered(|ui| {
            ui.set_width(width * 0.6);

            self.eat_meal_picker(ui, width * 0.6, handler);
            ui.add_space(margin);

            let Some(meal) = handler.selected.as_ref() else {
                ui.label(RichText::new("Pick a meal to see its details").color(self.theme.fg));
                return;
            };

            self.eat_meal_card(ui, width * 0.6, height, meal, handler);
            ui.add_space(margin * 1.5);

            if self.button(ui, width * 0.6, height * 0.1, self.theme.green, "Mampf") {
                // TODO: make mampf
            }
        });
    }

    fn eat_meal_picker(&mut self, ui: &mut Ui, width: f32, handler: &mut EatHandler) {
        handler.update_meals(&self.db).unwrap();

        let selected_key = handler
            .selected
            .as_ref()
            .map(|m| m.key());

        if let Some(id) = self.styled_dropdown(ui, "eat_meal_selector", width, &handler.meals, selected_key, "select a meal") {
            handler
                .select_meal(&self.db, id)
                .unwrap();
        }
    }

    fn eat_meal_card(&self, ui: &mut Ui, width: f32, height: f32, meal: &crate::models::Meal, handler: &EatHandler) {
        let margin = self.sizes.margin;

        egui::Frame::NONE
            .fill(self.theme.text_field)
            .corner_radius(self.sizes.corner_radius)
            .inner_margin(Margin::same((margin * 1.5) as i8))
            .show(ui, |ui| {
                ui.set_width(width - margin * 2.0);
                ui.vertical_centered(|ui| {
                    self.label(ui, meal.title(), self.theme.title, height * 0.045, true);
                    ui.add_space(margin * 0.3);
                    self.label(ui, &format!("{} kcal", meal.calories), self.theme.green, 16.0, false);

                    if let Some(body) = meal.body() {
                        self.section_divider(ui, margin);
                        ui.add(
                            Label::new(
                                RichText::new(body)
                                    .color(self.theme.fg)
                                    .weak()
                                    .size(15.0),
                            )
                            .wrap(),
                        );
                    }

                    if !handler.ingredients.is_empty() {
                        self.section_divider(ui, margin);
                        self.label(ui, "Ingredients", self.theme.fg, 13.0, false);
                        ui.add_space(margin * 0.3);

                        for entry in &handler.ingredients {
                            let text = format!("{} - {}g", entry.ingredient.title(), entry.amount_gr);
                            self.label(ui, &text, self.theme.fg, 15.0, false);
                        }
                    }
                });
            });
    }

    fn section_divider(&self, ui: &mut Ui, margin: f32) {
        ui.add_space(margin * 0.5);
        ui.separator();
        ui.add_space(margin * 0.5);
    }

    fn label(&self, ui: &mut Ui, text: &str, color: egui::Color32, size: f32, strong: bool) {
        let mut label = RichText::new(text)
            .color(color)
            .size(size);
        if strong {
            label = label.strong();
        } else {
            label = label.weak();
        }
        ui.label(label);
    }
}
