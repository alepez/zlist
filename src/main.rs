use std::str::FromStr;

use crate::autosnap::{AutoSnapList, AutoSnapReport};
use crate::parser::List;

mod autosnap;
mod parser;
mod zfs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let s = zfs::list_snapshots()?;
    let list = List::from_str(&s)?;
    let list = AutoSnapList::try_from(list)?;
    let report = AutoSnapReport::from(list);
    println!("{}", report);

    Ok(())
}
