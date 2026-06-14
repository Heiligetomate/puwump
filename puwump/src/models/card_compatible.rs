use uuid::Uuid;

use crate::{db::Db, errors::Result, models::core::Model};

pub trait CardAdd: Model {
    fn title(&self) -> &str;
    fn body(&self) -> Option<&str>;
    fn key(&self) -> Uuid;
}

pub trait CardCrud: CardAdd + Model + Sized {
    fn get_all(db: &Db) -> Result<Vec<Self>>;
    fn insert(db: &Db, name: &str, body: Option<&str>) -> Result<()>;
    fn delete(db: &Db, id: Uuid) -> Result<()>;
    fn name() -> &'static str;
}
