use crate::cal::time_cells::basic::BasicTimeCell;
use crate::cal::time_cells::common::{MonthDay, TimeCell};
use crate::cal::time_cells::daily::DailyTimeCell;
use crate::cal::time_cells::monthly::MonthlyTimeCell;
use crate::cal::time_cells::weekly::WeeklyTimeCell;
use chrono::{Month, NaiveTime};
use serde::{Deserialize, Serialize};

pub type YearlyTimeCell<T> = TimeCell<T, YearlyCell>;

#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "store", derive(reactive_stores::Store))]
#[serde(tag = "type")]
pub enum YearlyCell {
    Monthly(MonthlyTimeCell<YearTime>),
    Weekly(WeeklyTimeCell<YearTime>),
    Daily(DailyTimeCell<YearTime>),
    Basic(BasicTimeCell<YearTime>),
}

#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "store", derive(reactive_stores::Store))]
pub struct YearTime {
    month: Month,
    day: MonthDay,
    time: NaiveTime,
}
