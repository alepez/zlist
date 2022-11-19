use std::str::FromStr;

use crate::autosnap::{AutosnapList, AutosnapReport};
use crate::parser::List;

mod autosnap;
mod parser;
mod zfs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let s = zfs::list_snapshots()?;
    let list = List::from_str(&s)?;
    let list = AutosnapList::try_from(list)?;
    let report = AutosnapReport::from(list);
    println!("{}", report);

    Ok(())
}
