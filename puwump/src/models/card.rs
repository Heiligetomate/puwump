use uuid::Uuid;

use crate::{
    db::Db,
    errors::Result,
    models::{InputField, core::Model},
};

pub trait CardAdd: Model {
    fn title(&self) -> &str;
    fn body(&self) -> Option<&str>;
    fn key(&self) -> Uuid;
}

pub trait CardCrud: CardAdd + Model + Sized {
    fn get_all(db: &Db) -> Result<Vec<Self>>;
    fn insert(db: &Db, values: &[InputField]) -> Result<()>;
    fn delete(db: &Db, id: Uuid) -> Result<()>;
    fn name() -> &'static str;
}
