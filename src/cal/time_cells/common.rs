use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "store", derive(reactive_stores::Store))]
pub struct TimeCell<T, C> 
where C: Cell
{
    pub id: Uuid,
    pub start: T,
    pub end: T,
    #[cfg_attr(feature = "store", store(key: Uuid = |time_cell| time_cell.id()))]
    pub time_cells: Vec<C>,
}

pub trait Cell {
    fn id(&self) -> Uuid;
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
