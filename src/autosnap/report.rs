use super::{AutosnapList, Period};
use chrono::prelude::*;
use prettytable::{format, row, Table};
use std::{collections::BTreeMap, fmt::Display};

#[derive(Debug, Default)]
struct AutosnapReportPeriodItem {
    count: usize,
    last: Option<DateTime<Utc>>,
}

#[derive(Debug)]
struct AutosnapReportItem {
    name: String,
    hourly: AutosnapReportPeriodItem,
    daily: AutosnapReportPeriodItem,
    monthly: AutosnapReportPeriodItem,
    yearly: AutosnapReportPeriodItem,
}

impl AutosnapReportItem {
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

pub struct AutosnapReport(Vec<AutosnapReportItem>);

fn update_last_timestamp(last: &mut Option<DateTime<Utc>>, timestamp: DateTime<Utc>) {
    if let Some(last_time) = last {
        if timestamp > *last_time {
            *last = Some(timestamp);
        }
    } else {
        *last = Some(timestamp);
    }
}

fn update_period_item(curr: &mut AutosnapReportPeriodItem, timestamp: DateTime<Utc>) {
    update_last_timestamp(&mut curr.last, timestamp);
    curr.count += 1
}

impl From<AutosnapList> for AutosnapReport {
    fn from(list: AutosnapList) -> Self {
        let mut count: BTreeMap<String, AutosnapReportItem> = BTreeMap::new();

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
                .or_insert_with(|| AutosnapReportItem::new(key.to_string()));
        }

        let v = count.into_values().collect();

        Self(v)
    }
}

impl Display for AutosnapReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut table = Table::new();
        let format = format::FormatBuilder::new()
            .separators(
                &[format::LinePosition::Title],
                format::LineSeparator::new('-', '+', '+', '+'),
            )
            .column_separator('|')
            .padding(1, 1)
            .build();
        table.set_format(format);
        table.set_titles(row!["name", "H", "D", "M", "Y", "last"]);

        for item in self.0.iter() {
            let times = [
                item.hourly.last,
                item.daily.last,
                item.monthly.last,
                item.yearly.last,
            ];

            let last_time = times.iter().filter_map(|x| x.as_ref()).max();
            let time_diff = last_time
                .and_then(|t| {
                    let now = Utc::now();
                    let diff = now.signed_duration_since(*t);
                    diff.to_std().ok()
                })
                .map(|diff| {
                    let f = timeago::Formatter::new();
                    f.convert(diff)
                })
                .unwrap_or_default();

            table.add_row(row![
                item.name,
                item.hourly.count,
                item.daily.count,
                item.monthly.count,
                item.yearly.count,
                time_diff,
            ]);
        }

        writeln!(f, "{}", table)
    }
}
