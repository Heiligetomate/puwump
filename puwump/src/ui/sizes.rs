use eframe::CreationContext;
use egui::Ui;

pub struct SizeSheet {
    pub height: f32,
    pub width: f32,
    pub corner_radius: f32,
    pub margin: f32,
    pub spacing: f32,
}

const MARGIN_MULT: f32 = 0.02;

impl SizeSheet {
    pub fn new(cc: &CreationContext) -> Self {
        let width = cc.egui_ctx.viewport_rect().width();
        let height = cc.egui_ctx.viewport_rect().height();
        let margin = Self::margin_from_width(width);
        let corner_radius = 16.0;
        let spacing = height * 0.02;
        Self {
            corner_radius,
            margin,
            height,
            width,
            spacing,
        }
    }

    pub fn update(&mut self, ui: &Ui) {
        let width = ui.available_width();
        let height = ui.available_height();
        self.width = width;
        self.height = height;
        self.margin = Self::margin_from_width(width);
    }

    fn margin_from_width(width: f32) -> f32 {
        width * MARGIN_MULT
    }
}
