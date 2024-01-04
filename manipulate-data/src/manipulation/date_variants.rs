use regex::Regex;
use parse_data::errors::ParsingError;
use parse_data::transform_parse::domain::methods_domain::date_pattern::DatePattern;
use parse_data::transform_parse::domain::methods_domain::date_type::DateType;
use parse_data::transform_parse::domain::methods_domain::to_date_method::ToDateMethod;
use crate::manipulation::date::{Date, DatePeriod, Epoch, TransientDatePeriod};

pub struct DateWrapper (pub(crate) ToDateMethod, pub(crate) String);

impl DateWrapper {
    /// five cases:
    /// case 1: year-month-day || day-month-year || month-day-year
    /// case 2: year-month || month-year
    /// case 4: year
    /// case 3: case 1 or case 2 with month written as word (e.g. Jan 1991)
    /// case 5: symbols, words used to convey date is BC or CE -> not implemented
    pub fn to_date(&self) -> Result<DatePeriod, ParsingError> {
        for i in 0..self.0.date_patterns.len() {
            let date_pattern = self.0.date_patterns.get(i).unwrap();
            let date_period: Option<DatePeriod> = self.date_period(date_pattern, &self.0.date_type)?;
            if date_period.is_some() {
                return Ok(date_period.unwrap())
            }
        }
        return Err(ParsingError::ValidationError(format!("cannot parse value '{:?}' to a date with existing patterns.", self.1)))
    }
    fn date_period(&self, date_pattern: &DatePattern, date_type: &DateType) -> Result<Option<DatePeriod>, ParsingError> {
        let regex = date_pattern.to_regex()?;
        let caps = regex.captures(self.1.as_str());
        if caps.is_none() {
            return Ok(None)
        }
        let mut transient = TransientDatePeriod::new(caps.unwrap(),date_pattern,date_type)?;
        transient.complete_dates()?;
        Ok(Some(DatePeriod::new(transient)))
    }
}