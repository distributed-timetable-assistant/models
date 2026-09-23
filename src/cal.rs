use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use crate::cal::time_cells::basic::BasicTimeCell;
use crate::cal::time_cells::daily::DailyTimeCell;
use crate::cal::time_cells::holiday::HolidayTimeCell;
use crate::cal::time_cells::monthly::MonthlyTimeCell;
use crate::cal::time_cells::weekly::WeeklyTimeCell;
use crate::cal::time_cells::yearly::YearlyTimeCell;

pub mod time_cells;

#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug, Default, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "store", derive(reactive_stores::Store))]
pub struct Cal {
    pub avails: Vec<GeneralTimeCell>,
}

#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "store", derive(reactive_stores::Store))]
#[serde(tag = "type")]
pub enum GeneralTimeCell {
    Yearly(YearlyTimeCell<GeneralTime>),
    Monthly(MonthlyTimeCell<GeneralTime>),
    Weekly(WeeklyTimeCell<GeneralTime>),
    Daily(DailyTimeCell<GeneralTime>),
    Basic(BasicTimeCell<GeneralTime>),
    Holiday(HolidayTimeCell<GeneralTime>),
}

pub type GeneralTime = DateTime<FixedOffset>;

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use serde_yaml::Value;
    
    #[rstest]
    fn calendar_when_deserialize_and_serialize_should_be_equal(
        #[files("test/fixtures/cal/samples/*")]
        #[mode = str]
        contents: &str,
    ) {
        // Act
        let input_value: Value = serde_yaml::from_str(contents).unwrap();
        let cal: Cal = serde_yaml::from_value(input_value.clone()).unwrap();
        let final_value = serde_yaml::to_value(&cal).unwrap();

        // Assert
        assert_eq!(input_value, final_value);
    }
}
