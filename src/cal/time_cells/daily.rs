use crate::cal::time_cells::basic::BasicTimeCell;
use crate::cal::time_cells::common::{Cell, TimeCell};
use chrono::NaiveTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type DailyTimeCell<T> = TimeCell<T, DailyCell>;

#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "store", derive(reactive_stores::Store))]
#[serde(tag = "type")]
pub enum DailyCell {
    Basic(BasicTimeCell<NaiveTime>),
}

impl Cell for DailyCell {
    fn id(&self) -> Uuid {
        match self { DailyCell::Basic(basic) => basic.id }
    }
}