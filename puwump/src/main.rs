pub mod db;
pub mod errors;
pub mod util;

use crate::{db::Db, errors::Result};

fn main() -> Result<()> {
    let db = Db::init()?;
    // let db = db.reset()?;
    db.new_exercise("meow", "meow around")?;
    db.new_plan("meow-plan", "very meowe", 161)?;
    let plans = db.get_all_plans()?;
    for plan in plans.iter() {
        db.remove_plan(*plan)?;
    }
    println!("{:?}", db.get_all_plans()?);
    Ok(())
}
