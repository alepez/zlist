use byte_unit::Byte;
use chrono::prelude::*;
use std::str::FromStr;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("AutoSnap failed to parse")]
    Fail,
    #[error("AutoSnap delimiter not found")]
    NotAutoSnapDelimiterFound,
    #[error("AutoSnap invalid period")]
    InvalidPeriod,
    #[error("AutoSnap invalid timestamp: {0}")]
    InvalidTimestamp(chrono::format::ParseError),
}

struct ListItem {
    name: String,
    used: Option<Byte>,
    available: Option<Byte>,
    refer: Option<Byte>,
}

fn parse_line(s: &str) -> ListItem {
    let z: Vec<&str> = s.split_ascii_whitespace().collect();
    let name = z[0].to_string();
    ListItem {
        name,
        used: Byte::from_str(z[1]).ok(),
        available: Byte::from_str(z[2]).ok(),
        refer: Byte::from_str(z[3]).ok(),
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Period {
    Hourly,
    Daily,
    Monthly,
    Yearly,
}

impl FromStr for Period {
    type Err = ParserError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "hourly" => Ok(Period::Hourly),
            "daily" => Ok(Period::Daily),
            "monthly" => Ok(Period::Monthly),
            "yearly" => Ok(Period::Yearly),
            _ => Err(ParserError::InvalidPeriod),
        }
    }
}

struct AutoSnapInfo {
    name: String,
    timestamp: DateTime<Utc>,
    period: Period,
}

impl FromStr for AutoSnapInfo {
    type Err = ParserError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, r) = s
            .split_once("@autosnap_")
            .ok_or(ParserError::NotAutoSnapDelimiterFound)?;

        let mut tokens = r.split('_');
        let date = tokens.next().ok_or(ParserError::Fail)?;
        let time = tokens.next().ok_or(ParserError::Fail)?;
        let period = tokens.next().ok_or(ParserError::Fail)?;

        let timestamp = format!("{} {}", date, time);

        let timestamp = Utc
            .datetime_from_str(&timestamp, "%Y-%m-%d %H:%M:%S")
            .map_err(ParserError::InvalidTimestamp)?;

        let period = Period::from_str(period).map_err(|_| ParserError::Fail)?;

        let result = AutoSnapInfo {
            name: name.to_string(),
            timestamp,
            period,
        };

        Ok(result)
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

    #[test]
    fn test_parse_autosnap_info() {
        let info = AutoSnapInfo::from_str(
            "radon_pool/data/root/home/alepez/workspace@autosnap_2022-11-13_13:03:01_daily",
        )
        .unwrap();

        assert_eq!(info.name, "radon_pool/data/root/home/alepez/workspace");
        assert_eq!(info.timestamp.to_string(), "2022-11-13 13:03:01 UTC");
        assert_eq!(info.period, Period::Daily);
    }
}
