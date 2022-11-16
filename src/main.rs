mod zfs;

use std::collections::BTreeMap;
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let s = zfs::list_snapshots()?;
    let r = list_autosnap(s);
    print!("{}", r);
    Ok(())
}

fn occurrencies(s: String) -> BTreeMap<String, usize> {
    let mut count = BTreeMap::new();
    for line in s.lines() {
        let key = remove_autosnap_timestamp(line);
        count
            .entry(key.into())
            .and_modify(|curr| *curr += 1)
            .or_insert(1);
    }
    count
}

fn list_autosnap(s: String) -> String {
    let count = occurrencies(s);
    let mut buf = Vec::new();

    for (key, value) in count {
        writeln!(buf, "{} {}", key, value).unwrap();
    }

    String::from_utf8(buf).expect("Invalid string")
}

fn remove_autosnap_timestamp(s: &str) -> &str {
    if let Some((l, _)) = s.split_once('@') {
        l
    } else {
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_autosnap_timestamp() {
        assert_eq!(
            "radon_pool/sys/root",
            remove_autosnap_timestamp("radon_pool/sys/root@autosnap_2022-11-13_13:03:02_daily")
        );
    }
}
