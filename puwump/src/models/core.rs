use std::fmt::Debug;

use rusqlite::{Params, Statement};

use crate::errors::{PuwumpError, Result};

pub trait Model: Debug {
    fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Self>
    where
        Self: Sized;
}

pub fn statement_to_model<M: Model>(mut stmt: Statement, params: impl Params) -> Result<M> {
    let plan = stmt
        .query_map(params, M::from_row)?
        .collect::<rusqlite::Result<Vec<M>>>()
        .map_err(|_| PuwumpError::RowNotFound)?
        .into_iter()
        .next()
        .ok_or(PuwumpError::RowNotFound)?;
    Ok(plan)
}
