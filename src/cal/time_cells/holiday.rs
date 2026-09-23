use crate::cal::time_cells::common::Status;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "store", derive(reactive_stores::Store))]
pub struct HolidayTimeCell<T> {
    start: T,
    end: T,
    country: String,
    status: Status,
}
