use uuid::Uuid;

use crate::{db::Db, errors::Result, models::core::Model};

// TODO: CardAdd should not handle the input fields
// Add a new trait just for the input fields
// The new trait should have a vec with the required info
// It also stores the input data for each input field
// The trait shold then be stored either in the TaskHandler with a new generic parameter
// Or the CardAdd could hold a type to the input field trait

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
