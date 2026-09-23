use crate::cal::time_cells::basic::BasicTimeCell;
use crate::cal::time_cells::common::TimeCell;
use chrono::NaiveTime;
use serde::{Deserialize, Serialize};

pub type DailyTimeCell<T> = TimeCell<T, DailyCell>;

#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "store", derive(reactive_stores::Store))]
#[serde(tag = "type")]
pub enum DailyCell {
    Basic(BasicTimeCell<NaiveTime>),
}
