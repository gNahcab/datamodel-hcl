use hcl::{Attribute, Block, BlockLabel, Body, from_reader, Identifier, Number, to_vec, value};
use polars::export::arrow::compute::filter::filter;
use regex::Regex;
use crate::errors::ParsingError;
use crate::transform_parse::domain::methods_domain::date_bricks::{DateBricks, DateInfo, DateName, WrapperDateBricks};
use crate::transform_parse::domain::methods_domain::wrapper_trait::Wrapper;

pub struct WrapperDatePattern(pub(crate) hcl::Block);

#[derive(Debug, Clone)]
pub struct DatePattern {
    pub nr: usize,
    pub first_date: Option<DateBricks>,
    pub date: DateBricks,
}

impl DatePattern {
    fn new(transient: TransientDatePattern) -> DatePattern {
        DatePattern{nr: transient.nr, first_date: transient.first_date, date: transient.date.unwrap()}
    }
    fn is_consistent(&self) -> Result<(), ParsingError> {
        if self.date.year.is_none() {
            return Err(ParsingError::ValidationError(format!("the 'year' of 'date' doesn't exist in pattern-nr '{:?}', this is forbidden.", self.nr)));
        }
        if self.date.day.is_some() && self.date.year.is_some() && self.date.month.is_none() {
            return Err(ParsingError::ValidationError(format!("the 'day' and 'year' of 'date' exist but not 'month' in pattern-nr '{:?}', this is forbidden: either day, month and year exist or month and year or year alone.", self.nr)));
        }
        if self.first_date.is_some() {
             if self.first_date.as_ref().unwrap().year.is_some() && self.first_date.as_ref().unwrap().month.is_none() {
                return Err(ParsingError::ValidationError(format!("the 'day' and 'year' of 'first' exist but not 'month' in pattern-nr '{:?}', this is forbidden: either day, month and year exist or month and year or year or month or day alone.", self.nr)));
            }
            // if day exists: does day exist in date?
            if self.first_date.as_ref().unwrap().day.is_some() && self.date.day.is_none() {
                return Err(ParsingError::ValidationError(format!("'day' exists in 'first' but not in 'date' in pattern-nr '{:?}', this is forbidden.", self.nr)));
            }
            // if month exists: does month exist in date?
            if self.first_date.as_ref().unwrap().month.is_some() && self.date.month.is_none() {
                return Err(ParsingError::ValidationError(format!("'month' exists in 'first' but not in 'date' in pattern-nr '{:?}', this is forbidden.", self.nr)));
            }
        }
        Ok(())
    }
    pub fn to_regex(&self) -> Result<Regex, ParsingError> {
        let regex_pattern= self.build_regex_pattern()?;
        let regex = Regex::new(regex_pattern.as_str())?;
        Ok(regex)
    }
    fn build_regex_pattern(&self) -> Result<String,ParsingError> {
        let mut regex_str: String = format!(r"^(?x)");
        let non_word_or_number = r"\W{1,2}";
        if self.first_date.is_some() {
            let nr_day = &self.first_date.as_ref().unwrap().day;
            let nr_month = &self.first_date.as_ref().unwrap().month;
            let nr_year = &self.first_date.as_ref().unwrap().year;
            let mut dates: Vec<&DateInfo> = [nr_day, nr_month, nr_year].to_vec().iter().filter_map(|maybe_nr|
                match maybe_nr {
                    None => {None}
                    Some(date_info) => {Some(date_info)}
                }).collect();
            dates.sort_by(|date_1, date_2| date_1.nr.cmp(&date_2.nr));
            for date_info in dates {
                match date_info.name {
                    DateName::Day => {
                        let day = r"(?P<day1>\d{1,2})" ;
                        regex_str = format!("{}{}{}", regex_str, day, non_word_or_number);
                    }
                    DateName::Month => {
                        let month =
                        if self.first_date.as_ref().unwrap().month_word.unwrap() == true {
                            r"(?P<month1>[A-Za-z]*)"
                        } else {
                            r"(?P<month1>\d{1,2})"
                        };
                        regex_str = format!("{}{}{}", regex_str, month, non_word_or_number);
                    }
                    DateName::Year => {
                        let year = r"(?P<year1>\d{3,4})";
                        regex_str = format!("{}{}{}", regex_str, year, non_word_or_number);
                    }
                }
            }
            //remove last '\W{1,2} and replace with \W{1,2,3}'
            let separator = r"\W{3,4}";
            regex_str = format!("{}{}",&regex_str[0..regex_str.len() - non_word_or_number.to_string().len()], separator);
        }
        let nr_day = &self.date.day;
        let nr_month = &self.date.month;
        let nr_year = &self.date.year;
        let mut dates: Vec<&DateInfo> = [nr_day, nr_month, nr_year].to_vec().iter().filter_map(|maybe_nr|
            match maybe_nr {
                None => {None}
                Some(date_info) => {Some(date_info)}
            }).collect();
        dates.sort_by(|date_1, date_2| date_1.nr.cmp(&date_2.nr));
        for date_info in dates {
            match date_info.name {
                DateName::Day => {
                    let day = r"(?P<day2>\d{1,2})";
                    regex_str = format!(r"{}{}{}", regex_str, day, non_word_or_number);
                }
                DateName::Month => {
                    let month =
                        if self.date.month_word.unwrap() == true {
                            r"(?P<month2>[A-Za-z]*)"
                        } else {
                            r"(?P<month2>\d{1,2})"
                        };
                    regex_str = format!(r"{}{}{}", regex_str, month, non_word_or_number);
                }
                DateName::Year => {
                    let year = r"(?P<year2>\d{3,4})";
                    regex_str = format!(r"{}{}{}", regex_str, year, non_word_or_number);
                }
            }
        }
        //remove last '\W{1,2}'
        regex_str = format!("{}", &regex_str[0..regex_str.len() - non_word_or_number.to_string().len()]);
        Ok(regex_str)
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
        date_pattern.is_consistent()?;
        Ok(date_pattern)
    }
}
