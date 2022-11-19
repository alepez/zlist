mod report;

use crate::parser::{List, ListItem};
use chrono::prelude::*;
pub use report::AutosnapReport;
use std::str::FromStr;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AutosnapError {
    #[error("Autosnap failed to parse")]
    Fail,
    #[error("Autosnap delimiter not found")]
    DelimiterNotFound,
    #[error("Autosnap invalid period")]
    InvalidPeriod,
    #[error("Autosnap invalid timestamp: {0}")]
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
    type Err = AutosnapError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "hourly" => Ok(Period::Hourly),
            "daily" => Ok(Period::Daily),
            "monthly" => Ok(Period::Monthly),
            "yearly" => Ok(Period::Yearly),
            _ => Err(AutosnapError::InvalidPeriod),
        }
    }
}

#[derive(Debug)]
pub struct AutosnapInfo {
    name: String,
    timestamp: DateTime<Utc>,
    period: Period,
}

impl TryFrom<List> for AutosnapList {
    type Error = AutosnapError;

    fn try_from(list: List) -> Result<Self, Self::Error> {
        let v = list
            .iter()
            .filter_map(|list_item| {
                let autosnap_info = AutosnapInfo::from_str(list_item.name());
                autosnap_info.ok()
            })
            .collect();
        Ok(Self(v))
    }
}

impl TryFrom<ListItem> for AutosnapInfo {
    type Error = AutosnapError;

    fn try_from(value: ListItem) -> Result<Self, Self::Error> {
        AutosnapInfo::from_str(value.name())
    }
}

#[derive(Debug)]
pub struct AutosnapList(Vec<AutosnapInfo>);

impl AutosnapList {
    pub fn iter(&self) -> std::slice::Iter<'_, AutosnapInfo> {
        self.0.iter()
    }
}

impl FromStr for AutosnapInfo {
    type Err = AutosnapError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, r) = s
            .split_once("@autosnap_")
            .ok_or(AutosnapError::DelimiterNotFound)?;

        let mut tokens = r.split('_');
        let date = tokens.next().ok_or(AutosnapError::Fail)?;
        let time = tokens.next().ok_or(AutosnapError::Fail)?;
        let period = tokens.next().ok_or(AutosnapError::Fail)?;

        let timestamp = format!("{} {}", date, time);

        let timestamp = Utc
            .datetime_from_str(&timestamp, "%Y-%m-%d %H:%M:%S")
            .map_err(AutosnapError::InvalidTimestamp)?;

        let period = Period::from_str(period).map_err(|_| AutosnapError::Fail)?;

        let result = AutosnapInfo {
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
    fn test_parse_autosnap_info() {
        let info = AutosnapInfo::from_str(
            "radon_pool/data/root/home/alepez/workspace@autosnap_2022-11-13_13:03:01_daily",
        )
        .unwrap();

        assert_eq!(info.name, "radon_pool/data/root/home/alepez/workspace");
        assert_eq!(info.timestamp.to_string(), "2022-11-13 13:03:01 UTC");
        assert_eq!(info.period, Period::Daily);
    }
}
