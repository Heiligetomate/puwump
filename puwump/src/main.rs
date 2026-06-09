pub mod db;
pub mod errors;
pub mod util;

use crate::{db::Db, errors::Result};

fn main() -> Result<()> {
    let db = Db::init()?;
    db.reset()?;
    Ok(())
}
