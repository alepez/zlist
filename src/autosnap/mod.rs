mod report;

use chrono::prelude::*;
pub use report::AutoSnapReport;
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

#[derive(Debug, PartialEq, Eq)]
pub enum Period {
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

#[derive(Debug)]
pub struct AutoSnapInfo {
    pub name: String,
    pub timestamp: DateTime<Utc>,
    pub period: Period,
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

#[derive(Debug)]
pub struct AutoSnapList(Vec<AutoSnapInfo>);

impl FromStr for AutoSnapList {
    type Err = ParserError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Vec<AutoSnapInfo> = s
            .lines()
            .map(crate::parser::parse_line)
            .filter_map(|list_item| {
                let autosnap_info = AutoSnapInfo::from_str(&list_item.name);
                autosnap_info.ok()
            })
            .collect();
        Ok(Self(v))
    }
}

impl AutoSnapList {
    pub fn iter(&self) -> std::slice::Iter<'_, AutoSnapInfo> {
        self.0.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
