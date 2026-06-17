use egui::Color32;

pub struct Theme {
    pub fg: Color32,
    pub title: Color32,
    pub red: Color32,
    pub green: Color32,
    pub blue: Color32,
    pub text_field: Color32,
    pub header_bg: Color32,
    pub white: Color32,
}

impl Default for Theme {
    fn default() -> Self {
        let red = Color32::from_rgb(204, 36, 29);
        let green = Color32::from_rgb(184, 187, 38);
        let blue = Color32::from_rgb(69, 133, 136);
        let text_field = Color32::from_rgb(60, 56, 54);
        let _bg = Color32::from_rgb(40, 40, 40);
        let fg = Color32::from_rgb(235, 219, 178);
        let title = Color32::from_rgb(250, 189, 47);
        let white = Color32::WHITE;
        let header_bg = Color32::from_rgb(50, 48, 47);
        Self {
            fg,
            title,
            blue,
            green,
            red,
            text_field,
            white,
            header_bg,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ButtonTheme {
    pub color: Color32,
    pub symbol: char,
}

impl ButtonTheme {
    pub const fn delete() -> Self {
        Self {
            color: Color32::from_rgb(204, 36, 29),
            symbol: 'X',
        }
    }

    pub const fn add() -> Self {
        Self {
            color: Color32::from_rgb(184, 187, 38),
            symbol: 'O',
        }
    }

    pub const fn minus() -> Self {
        Self {
            color: Color32::from_rgb(204, 36, 29),
            symbol: '-',
        }
    }

    pub const fn plus() -> Self {
        Self {
            color: Color32::from_rgb(184, 187, 38),
            symbol: '+',
        }
    }

    pub const fn move_up() -> Self {
        Self {
            color: Color32::from_rgb(69, 133, 136),
            symbol: '^',
        }
    }

    pub const fn move_down() -> Self {
        Self {
            color: Color32::from_rgb(69, 133, 136),
            symbol: 'v',
        }
    }
}
