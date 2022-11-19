use byte_unit::Byte;

pub struct ListItem {
    pub name: String, // FIXME
    #[allow(unused)]
    used: Option<Byte>,
    #[allow(unused)]
    available: Option<Byte>,
    #[allow(unused)]
    refer: Option<Byte>,
}

pub fn parse_line(s: &str) -> ListItem {
    let z: Vec<&str> = s.split_ascii_whitespace().collect();
    let name = z[0].to_string();
    ListItem {
        name,
        used: Byte::from_str(z[1]).ok(),
        available: Byte::from_str(z[2]).ok(),
        refer: Byte::from_str(z[3]).ok(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_list_line() {
        let info = parse_line("radon_pool/data/root/home/alepez/workspace@autosnap_2022-11-13_13:03:01_daily                        51.8M      -     56.9G  -");

        assert_eq!(
            info.name,
            "radon_pool/data/root/home/alepez/workspace@autosnap_2022-11-13_13:03:01_daily"
        );
        assert_eq!(info.used.unwrap(), Byte::from_str("51.8 MB").unwrap());
        assert!(info.available.is_none());
        assert_eq!(info.refer.unwrap(), Byte::from_str("56.9 GB").unwrap());
    }
}
