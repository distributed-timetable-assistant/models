use crate::cal::time_cells::basic::BasicTimeCell;
use crate::cal::time_cells::common::TimeCell;
use crate::cal::time_cells::daily::DailyTimeCell;
use chrono::{NaiveTime, Weekday};
use serde::{Deserialize, Serialize};

pub type WeeklyTimeCell<T> = TimeCell<T, WeeklyCell>;

#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "store", derive(reactive_stores::Store))]
#[serde(tag = "type")]
pub enum WeeklyCell {
    Daily(DailyTimeCell<WeekTime>),
    Basic(BasicTimeCell<WeekTime>),
}

#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "store", derive(reactive_stores::Store))]
pub struct WeekTime {
    pub day: Weekday,
    pub time: NaiveTime,
}
