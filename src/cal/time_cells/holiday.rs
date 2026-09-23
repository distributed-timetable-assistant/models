use crate::cal::time_cells::common::{Cell, Status};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "store", derive(reactive_stores::Store))]
pub struct HolidayTimeCell<T> {
    pub id: Uuid,
    pub start: T,
    pub end: T,
    pub country: String,
    pub status: Status,
}

impl<T> Cell for HolidayTimeCell<T> {
    fn id(&self) -> Uuid {
        self.id
    }
}