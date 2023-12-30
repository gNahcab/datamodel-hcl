use std::env::consts::FAMILY;
use hcl::{Block, Number};
use crate::errors::ParsingError;
use crate::expression_trait::ExpressionTransform;
use crate::transform_parse::domain::header_value::U8implementation;

pub struct WrapperDateBricks(pub(crate) hcl::Body);
#[derive(Debug, Clone)]
pub enum DateName{
Day,
    Month,
    Year,
}
#[derive(Debug, Clone)]
pub struct DateInfo {
   pub nr: u8,
    pub name: DateName,
}

impl DateInfo {
    fn new(nr: u8, date_name: DateName) -> DateInfo {
        DateInfo{ nr, name: date_name}
    }
}

#[derive(Debug, Clone)]
pub struct DateBricks {
    pub month_word: Option<bool>,
    pub day: Option<DateInfo>,
    pub month: Option<DateInfo>,
    pub year: Option<DateInfo>,
}

impl DateBricks {
    fn new() -> DateBricks {
        DateBricks{
            month_word: None,
            day: None,
            month: None,
            year: None,
        }
    }

    fn number(&self, number: Number) -> Result<u8, ParsingError> {
        let result =  number.as_u8();
        let nr = match result {
            Ok(nr) => {nr}
            Err(_) => {
                return Err(ParsingError::ValidationError(format!("cannot parse this to u8 {:?} in 'to_date'-pattern-number.", number)));
            }
        };
        if nr < 1 || nr > 3 {
            return Err(ParsingError::ValidationError(format!("number {:?} in 'to_date'-pattern should be between 1 and 3.", number)));
        }
        Ok(nr)
    }
    fn add_day(&mut self, day: Number) -> Result<(), ParsingError> {
        if self.day.is_some() {
            return Err(ParsingError::ValidationError(format!("found a duplicate for day in 'to-date'-pattern")));
        }
        let nr = self.number(day)?;
        self.day = Option::from(DateInfo::new(nr, DateName::Day));
        Ok(())
    }
    fn add_month(&mut self, month: Number) -> Result<(), ParsingError> {
        if self.month.is_some() {
            return Err(ParsingError::ValidationError(format!("found a duplicate for month in 'to-date'-pattern")));
        }
        let nr = self.number(month)?;
        self.day = Option::from(DateInfo::new(nr, DateName::Month));
        Ok(())
    }
    fn add_month_word(&mut self, month_word: bool) -> Result<(), ParsingError> {
        if self.month_word.is_some() {
            return Err(ParsingError::ValidationError(format!("found a duplicate for month_word in 'to-date'-pattern")));
        }
        self.month_word = Option::from(month_word);
        Ok(())
    }
    fn add_year(&mut self, year: Number) -> Result<(), ParsingError> {
        if self.year.is_some() {
            return Err(ParsingError::ValidationError(format!("found a duplicate for year in 'to-date'-pattern")));
        }
        let nr = self.number(year)?;
        self.day = Option::from(DateInfo::new(nr, DateName::Year));
        Ok(())
    }
    fn check_month_word(&mut self) {
        if self.month_word.is_none() {
            self.month_word = Option::from(false);
        }
    }
    fn has_values(&self) -> Result<(), ParsingError> {
        if self.day.is_none() && self.month.is_none() && self.year.is_none() {
            return Err(ParsingError::ValidationError(format!("found an empty to-date-pattern")));
        }
        Ok(())
    }
}
impl WrapperDateBricks {
    pub(crate) fn to_date_bricks(&self) -> Result<DateBricks, ParsingError> {
        let blocks = self.0.blocks().collect::<Vec<&Block>>();
        if blocks.len() != 0 {
            return Err(ParsingError::ValidationError(format!("found blocks '{:?}' in 'to_date'-pattern, but no blocks are allowed.", blocks)))
        }
        let mut date_bricks = DateBricks::new();
        let attributes = self.0.attributes();
        for attribute in attributes {
            match attribute.key.as_str() {
                "day" => {
                    date_bricks.add_day(attribute.expr.to_number()?)?;
                },
                "month" => {
                    date_bricks.add_month(attribute.expr.to_number()?)?;
                },
                "month_word" => {
                    date_bricks.add_month_word(attribute.expr.to_bool()?)?;
                }
                "year" => {
                    date_bricks.add_year(attribute.expr.to_number()?)?;
                },
                _ => return Err(ParsingError::ValidationError(format!("found unknown attribute '{:?}' in 'to_date'pattern, only day, month and year are allowed.", attribute.key)))
            }
        }
        date_bricks.check_month_word();
        date_bricks.has_values()?;
        Ok(date_bricks)
    }
}
