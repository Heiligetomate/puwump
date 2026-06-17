use uuid::Uuid;

use crate::{db::Db, errors::Result, models::CardAdd, ui::ButtonTheme};

pub trait EditHandler {
    type Model: CardAdd + PartialEq;
    type SelModel: CardAdd;
    type Selectable: CardAdd;

    fn get_selectable(&self) -> &Vec<Self::Selectable>;
    fn update_selectable(&mut self, db: &Db) -> Result<()>;

    fn get_selected(&self) -> Option<&Self::Model>;
    fn get_sel_data(&self) -> Result<&Vec<Self::SelModel>>;
    fn get_data(&self) -> &Vec<Self::Model>;
    fn update(&mut self, db: &Db) -> Result<()>;
    fn update_sel(&mut self, db: &Db, id: Uuid) -> Result<()>;
    fn updated_sel_data(&mut self, db: &Db) -> Result<()>;
    fn insert_handler_model(&self, db: &Db, id: Uuid) -> Result<()>;
    fn sel_is_none(&self) -> bool;

    fn card_buttons() -> &'static [ButtonTheme];
    fn handle_buttons(&mut self, results: Vec<(Uuid, Vec<bool>)>, db: &Db) -> Result<()>;
}
