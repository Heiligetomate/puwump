use uuid::Uuid;

use crate::{
    errors::PuwumpError,
    models::{CardAdd, CardCrud, InputField, core::Model},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Plan {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub est_mins: u32,
}

impl Model for Plan {
    fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Self> {
        let id: String = row.get(0)?;
        Ok(Self {
            id: Uuid::parse_str(&id).map_err(|_| rusqlite::Error::InvalidQuery)?,
            name: row.get(1)?,
            description: row.get(2)?,
            est_mins: row.get(3)?,
        })
    }
}

impl CardAdd for Plan {
    fn key(&self) -> Uuid {
        self.id
    }

    fn body(&self) -> Option<&str> {
        Some(&self.description)
    }

    fn title(&self) -> &str {
        &self.name
    }
}

impl CardCrud for Plan {
    fn name() -> &'static str {
        "plan"
    }

    fn insert(db: &crate::db::Db, values: &[InputField]) -> crate::errors::Result<()> {
        db.insert_plan(
            values[0].value.as_str(),
            values[1].value.as_str(),
            values[2]
                .value
                .parse()
                .map_err(|_| PuwumpError::InputFieldIntParse("estimated minutes should be a number"))?,
        )?;
        Ok(())
    }

    fn delete(db: &crate::db::Db, id: Uuid) -> crate::errors::Result<()> {
        db.remove_plan(id)
    }

    fn get_all(db: &crate::db::Db) -> crate::errors::Result<Vec<Self>> {
        db.get_all_plans()
    }
}
