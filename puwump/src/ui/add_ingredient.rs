use egui::Ui;

use crate::ui::core::PuwumpUi;

impl PuwumpUi {
    pub fn add_ingredient_view(&mut self, ui: &mut Ui) {
        let mut handler = std::mem::take(&mut self.ingredient_hdnl);
        self.add_view(ui, &mut handler);
        self.ingredient_hdnl = handler;
    }
}
