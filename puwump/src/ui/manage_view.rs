use egui::{RichText, Ui};
use uuid::Uuid;

use crate::{
    handlers::EditHandler,
    models::CardAdd,
    ui::{core::PuwumpUi, theme::ButtonTheme},
};

impl PuwumpUi {
    pub fn edit_view<H: EditHandler>(&mut self, ui: &mut Ui, handler: &mut H) {
        let width = self.sizes.width;
        let height = self.sizes.height;
        let margin = self.sizes.margin;

        let inner_margin = (width * 0.01) as i8;
        let left_width = width * 0.4;
        let list_width = width * 0.55 - margin * 2.0;

        ui.add_space(height * 0.05);

        let available_height = ui.available_height();

        ui.horizontal(|ui| {
            ui.add_space(margin);

            ui.vertical(|ui| {
                ui.set_width(left_width);
                let drop_down_height = height * 0.2;
                let list_height = height - drop_down_height - margin;

                let orig = ui.spacing().interact_size.y;
                self.plan_drop_down(ui, left_width, handler);
                ui.spacing_mut().interact_size.y = orig;

                ui.add_space(margin);
                let sel_data = { if let Ok(exs) = handler.get_sel_data() { exs } else { return } };
                let results = self.add_list(ui, left_width, list_height, inner_margin, sel_data, H::card_buttons());

                handler
                    .handle_buttons(results, &self.db)
                    .unwrap();
            });

            ui.add_space(margin);
            ui.separator();
            ui.add_space(margin);
            for (id, clicked) in self.add_list(ui, list_width, available_height, inner_margin, handler.get_selectable(), &[ButtonTheme::add()]) {
                if clicked[0] {
                    if handler.sel_is_none() {
                        return;
                    }
                    handler
                        .insert_handler_model(&self.db, id)
                        .unwrap();

                    handler
                        .updated_sel_data(&self.db)
                        .unwrap();
                }
            }
        });
    }

    pub fn plan_drop_down<H: EditHandler>(&mut self, ui: &mut Ui, width: f32, handler: &mut H) {
        let selected_text = handler
            .get_selected()
            .map(|p| p.title())
            .unwrap_or("select");
        ui.spacing_mut().interact_size.y = 40.0;

        self.set_dropdown_rounding(ui);

        egui::ComboBox::from_id_salt("selector")
            .selected_text(
                RichText::new(selected_text)
                    .color(self.theme.fg)
                    .size(16.0),
            )
            .width(width)
            .show_ui(ui, |ui| {
                ui.style_mut()
                    .visuals
                    .widgets
                    .inactive
                    .bg_fill = self.theme.text_field;
                ui.style_mut()
                    .visuals
                    .widgets
                    .hovered
                    .bg_fill = self.theme.header_bg;

                handler.update(&self.db).unwrap();

                let data = handler.get_data();
                let before = handler.get_selected().map(|p| p.key());
                let mut new_selected = handler.get_selected().map(|p| p.key());

                for entry in data.iter() {
                    let is_selected = new_selected == Some(entry.key());
                    if ui
                        .selectable_label(
                            is_selected,
                            RichText::new(entry.title())
                                .color(self.theme.fg)
                                .size(20.0),
                        )
                        .clicked()
                    {
                        new_selected = Some(entry.key());
                    }
                }

                if new_selected != before {
                    if let Some(id) = new_selected {
                        handler
                            .update_sel(&self.db, id)
                            .unwrap();
                    }
                }
                let after: Option<Uuid> = handler
                    .get_selected()
                    .as_ref()
                    .map(|p| p.key());

                if before != after {
                    handler
                        .updated_sel_data(&self.db)
                        .unwrap();
                }
            });
        self.reset_dropdown_rounding(ui);
    }

    /// This function sets the correct rounding / styling for drop downs
    /// The reset_dropdown_rounding function should be called after the drop down was created
    pub fn set_dropdown_rounding(&self, ui: &mut Ui) {
        let rad = self.sizes.corner_radius as u8;
        let corner_radius = egui::CornerRadius::same(rad);

        ui.visuals_mut()
            .widgets
            .inactive
            .corner_radius = corner_radius;
        ui.visuals_mut()
            .widgets
            .active
            .corner_radius = corner_radius;
        ui.visuals_mut()
            .widgets
            .hovered
            .corner_radius = corner_radius;
        ui.visuals_mut()
            .widgets
            .open
            .corner_radius = corner_radius;

        ui.ctx().global_style_mut(|style| {
            style.visuals.menu_corner_radius = corner_radius;
        });
    }

    /// This function should be called after a dropdown was created
    /// and the set_dropdown_rounding was called
    /// This function should prevent conflict between ui settings
    pub fn reset_dropdown_rounding(&self, ui: &mut Ui) {
        let default = egui::CornerRadius::default();

        ui.visuals_mut()
            .widgets
            .inactive
            .corner_radius = default;
        ui.visuals_mut()
            .widgets
            .active
            .corner_radius = default;
        ui.visuals_mut()
            .widgets
            .hovered
            .corner_radius = default;
        ui.visuals_mut()
            .widgets
            .open
            .corner_radius = default;
        ui.ctx().global_style_mut(|style| {
            style.visuals.menu_corner_radius = default;
        });
    }
}
