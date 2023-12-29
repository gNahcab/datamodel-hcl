use std::num::ParseIntError;
use hcl::{Attribute, Block, BlockLabel, Body, from_reader, Identifier, Number};
use polars::export::chrono::Date;
use crate::errors::ParsingError;
use crate::expression_trait::ExpressionTransform;
use crate::transform_parse::domain::header_value::U8implementation;
use crate::transform_parse::domain::methods_domain::date_bricks::{DateBricks, WrapperDateBricks};
use crate::transform_parse::domain::methods_domain::wrapper_trait::Wrapper;

pub struct WrapperDatePattern(pub(crate) hcl::Block);

#[derive(Debug)]
pub struct DatePattern {
    nr: usize,
    first_date: Option<DateBricks>,
    date: DateBricks,
}

impl DatePattern {
    fn new(transient: TransientDatePattern) -> DatePattern {
        DatePattern{nr: transient.nr, first_date: transient.first_date, date: transient.date.unwrap()}
    }
}

pub struct TransientDatePattern {
    nr: usize,
    first_date: Option<DateBricks>,
    date: Option<DateBricks>,
}

impl TransientDatePattern {
    fn new() -> TransientDatePattern{
        TransientDatePattern {
            nr: 0,
            first_date: None,
            date: None
        }
    }
    pub(crate) fn add_nr(&mut self, labels: &Vec<BlockLabel>) -> Result<(), ParsingError> {
        if labels.len() == 0 {
            return Err(ParsingError::ValidationError(format!("a 'to_date'-pattern should have one label, but found a pattern that has zero.")))
        }
        if labels.len() > 1 {
            return Err(ParsingError::ValidationError(format!("a pattern should only have one label, but has multiple '{:?}'", labels)))
        }
        let label = labels.get(0).unwrap().as_str();
        let nr_ = label.parse::<usize>();
        let nr_ = match nr_ {
            Ok(number) => {number}
            Err(_) => {
                return Err(ParsingError::ValidationError(format!("couldn't parse label of to-date-pattern to usize, is this a number: '{:?}'?",label)));
            }
        };
        self.nr = nr_;
        Ok(())
    }
    pub(crate) fn add_first(&mut self, first: &Body) -> Result<(), ParsingError> {
        let date = WrapperDateBricks(first.to_owned()).to_date_bricks()?;
        self.first_date = Option::from(date);
        Ok(())
    }
    pub(crate) fn add_date(&mut self, date: &Body) -> Result<(), ParsingError> {
        let date = WrapperDateBricks(date.to_owned()).to_date_bricks()?;
        self.date = Option::from(date);
        Ok(())
    }
}
impl WrapperDatePattern {
    pub(crate) fn to_pattern(&self) -> Result<DatePattern, ParsingError> {
        let mut transient_pattern = TransientDatePattern::new();
        transient_pattern.add_nr(&self.0.labels.to_owned())?;
        self.0.no_attributes()?;
        for block in self.0.blocks() {
            match block.identifier.as_str() {
                "first" => transient_pattern.add_first(&block.body.to_owned())?,
                "date" => transient_pattern.add_date(&block.body.to_owned())?,
                _ => return Err(ParsingError::ValidationError(format!("unknown block in 'to_date'-pattern: '{:?}'", block.identifier)))
            }
        }
        let date_pattern: DatePattern = DatePattern::new(transient_pattern);
        Ok(date_pattern)
    }
}
