use crate::cal::time_cells::common::{Rule, Status};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "store", derive(reactive_stores::Store))]
pub struct BasicTimeCell<T> {
    pub start: T,
    pub end: T,
    pub rules: Vec<Rule>,
    pub status: Status,
    // Capacity divide: 0 allows scheduling at any time within the interval;
    // 1 allows only one course to occupy the interval;
    // values greater than 1 allow the interval to be divided among multiple courses.
    pub div_cap: u16,
}
