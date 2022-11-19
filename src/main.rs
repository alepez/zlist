use std::str::FromStr;

use crate::autosnap::{AutoSnapList, AutoSnapReport};

mod autosnap;
mod parser;
mod zfs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let s = zfs::list_snapshots()?;
    let list = AutoSnapList::from_str(&s)?;
    let report = AutoSnapReport::from(list);
    println!("{}", report);

    Ok(())
}
