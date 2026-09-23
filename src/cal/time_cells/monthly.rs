use crate::cal::time_cells::basic::BasicTimeCell;
use crate::cal::time_cells::common::MonthDay;
use crate::cal::time_cells::common::TimeCell;
use crate::cal::time_cells::daily::DailyTimeCell;
use crate::cal::time_cells::weekly::WeeklyTimeCell;
use chrono::NaiveTime;
use serde::{Deserialize, Serialize};

pub type MonthlyTimeCell<T> = TimeCell<T, MonthlyCell>;

#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "store", derive(reactive_stores::Store))]
#[serde(tag = "type")]
pub enum MonthlyCell {
    Weekly(WeeklyTimeCell<MonthTime>),
    Daily(DailyTimeCell<MonthTime>),
    Basic(BasicTimeCell<MonthTime>),
}

#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "store", derive(reactive_stores::Store))]
pub struct MonthTime {
    day: MonthDay,
    time: NaiveTime,
}
