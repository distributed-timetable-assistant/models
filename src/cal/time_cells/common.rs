use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "store", derive(reactive_stores::Store))]
pub struct TimeCell<T, C> {
    pub start: T,
    pub end: T,
    pub time_cells: Vec<C>,
}

#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum Status {
    Available,
    Unavailable,
    Preferred,
    Undesired,
}

#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "store", derive(reactive_stores::Store))]
#[serde(tag = "type")]
pub enum Rule {}

#[derive(Copy, Serialize, Deserialize)]
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "store", derive(reactive_stores::Store))]
pub struct MonthDay(u8);

impl MonthDay {
    pub fn new(value: u8) -> Option<Self> {
        (1..=31).contains(&value).then_some(Self(value))
    }

    pub fn get(self) -> u8 {
        self.0
    }
}
