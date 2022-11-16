use crate::parser::{AutoSnapList, Period};
use std::{collections::BTreeMap, fmt::Display};

#[derive(Debug)]
struct AutoSnapReportItem {
    name: String,
    hourly_count: usize,
    daily_count: usize,
    monthly_count: usize,
    yearly_count: usize,
}

impl AutoSnapReportItem {
    fn new(name: String) -> Self {
        Self {
            name,
            hourly_count: 0,
            daily_count: 0,
            monthly_count: 0,
            yearly_count: 0,
        }
    }
}

pub struct AutoSnapReport(Vec<AutoSnapReportItem>);

impl From<AutoSnapList> for AutoSnapReport {
    fn from(list: AutoSnapList) -> Self {
        let mut count: BTreeMap<String, AutoSnapReportItem> = BTreeMap::new();

        for item in list.iter() {
            let key = &item.name;
            count
                .entry(key.into())
                .and_modify(|curr| match item.period {
                    Period::Hourly => curr.hourly_count += 1,
                    Period::Daily => curr.daily_count += 1,
                    Period::Monthly => curr.monthly_count += 1,
                    Period::Yearly => curr.yearly_count += 1,
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
            writeln!(
                f,
                "{:60} h: {:<3}, d: {:<3}, m: {:<3}, y: {:<3}",
                item.name,
                item.hourly_count,
                item.daily_count,
                item.monthly_count,
                item.yearly_count
            )?;
        }

        Ok(())
    }
}
