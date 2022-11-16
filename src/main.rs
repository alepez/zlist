mod parser;
mod report;
mod zfs;

use report::AutoSnapReport;

use self::parser::AutoSnapList;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let s = zfs::list_snapshots()?;
    let list = AutoSnapList::from_str(&s)?;
    let report = AutoSnapReport::from(list);
    println!("{}", report);

    Ok(())
}
