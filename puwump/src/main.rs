pub mod db;
pub mod errors;
pub mod models;
pub mod util;

use crate::{db::Db, errors::Result};

fn main() -> Result<()> {
    let db = Db::init()?;
    // let db = db.reset()?;
    let exercises = db.get_all_exercises()?;
    let ex = db.get_exercise(*exercises.get(0).unwrap())?;
    println!("{:#?}", ex);
    println!("{:?}", db.get_all_plans()?);
    Ok(())
}
