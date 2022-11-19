use byte_unit::Byte;
use std::str::FromStr;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParserError {}

pub struct ListItem {
    name: String,
    #[allow(unused)]
    used: Option<Byte>,
    #[allow(unused)]
    available: Option<Byte>,
    #[allow(unused)]
    refer: Option<Byte>,
}

impl ListItem {
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl FromStr for ListItem {
    type Err = ParserError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let z: Vec<&str> = s.split_ascii_whitespace().collect();
        let name = z[0].to_string();
        let x = ListItem {
            name,
            used: Byte::from_str(z[1]).ok(),
            available: Byte::from_str(z[2]).ok(),
            refer: Byte::from_str(z[3]).ok(),
        };
        Ok(x)
    }
}

pub struct List(Vec<ListItem>);

impl List {
    pub fn iter(&self) -> std::slice::Iter<'_, ListItem> {
        self.0.iter()
    }
}

impl FromStr for List {
    type Err = ParserError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Vec<ListItem> = s
            .lines()
            .filter_map(|x| ListItem::from_str(x).ok())
            .collect();
        Ok(Self(v))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_list_line() {
        let info = ListItem::from_str("radon_pool/data/root/home/alepez/workspace@autosnap_2022-11-13_13:03:01_daily                        51.8M      -     56.9G  -");
        let info = info.unwrap();

        assert_eq!(
            info.name,
            "radon_pool/data/root/home/alepez/workspace@autosnap_2022-11-13_13:03:01_daily"
        );
        assert_eq!(info.used.unwrap(), Byte::from_str("51.8 MB").unwrap());
        assert!(info.available.is_none());
        assert_eq!(info.refer.unwrap(), Byte::from_str("56.9 GB").unwrap());
    }
}
