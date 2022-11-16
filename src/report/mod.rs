use crate::parser::{AutoSnapList, Period};
use chrono::prelude::*;
use std::{collections::BTreeMap, fmt::Display};

#[derive(Debug, Default)]
struct AutoSnapReportPeriodItem {
    count: usize,
    last: Option<DateTime<Utc>>,
}

#[derive(Debug)]
struct AutoSnapReportItem {
    name: String,
    hourly: AutoSnapReportPeriodItem,
    daily: AutoSnapReportPeriodItem,
    monthly: AutoSnapReportPeriodItem,
    yearly: AutoSnapReportPeriodItem,
}

impl AutoSnapReportItem {
    fn new(name: String) -> Self {
        Self {
            name,
            hourly: Default::default(),
            daily: Default::default(),
            monthly: Default::default(),
            yearly: Default::default(),
        }
    }
}

pub struct AutoSnapReport(Vec<AutoSnapReportItem>);

fn update_last_timestamp(last: &mut Option<DateTime<Utc>>, timestamp: DateTime<Utc>) {
    if let Some(last_time) = last {
        if timestamp > *last_time {
            *last = Some(timestamp);
        }
    } else {
        *last = Some(timestamp);
    }
}

fn update_period_item(curr: &mut AutoSnapReportPeriodItem, timestamp: DateTime<Utc>) {
    update_last_timestamp(&mut curr.last, timestamp);
    curr.count += 1
}

impl From<AutoSnapList> for AutoSnapReport {
    fn from(list: AutoSnapList) -> Self {
        let mut count: BTreeMap<String, AutoSnapReportItem> = BTreeMap::new();

        for item in list.iter() {
            let key = &item.name;
            count
                .entry(key.into())
                .and_modify(|curr| {
                    match item.period {
                        Period::Hourly => update_period_item(&mut curr.hourly, item.timestamp),
                        Period::Daily => update_period_item(&mut curr.daily, item.timestamp),
                        Period::Monthly => update_period_item(&mut curr.monthly, item.timestamp),
                        Period::Yearly => update_period_item(&mut curr.yearly, item.timestamp),
                    };
                })
                .or_insert_with(|| AutoSnapReportItem::new(key.to_string()));
        }

        let v = count.into_values().collect();

        Self(v)
    }
}

impl Display for AutoSnapReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for item in self.0.iter() {
            let times = [
                item.hourly.last,
                item.daily.last,
                item.monthly.last,
                item.yearly.last,
            ];

            let last_time = times.iter().filter_map(|x| x.as_ref()).max();
            let last_time = last_time.map(|x| x.to_string()).unwrap_or_default();

            writeln!(
                f,
                "{:60} h: {:<3}, d: {:<3}, m: {:<3}, y: {:<3} {}",
                item.name,
                item.hourly.count,
                item.daily.count,
                item.monthly.count,
                item.yearly.count,
                last_time
            )?;
        }

        Ok(())
    }
}
