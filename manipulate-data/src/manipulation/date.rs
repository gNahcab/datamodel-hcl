use regex::{Captures};
use parse_data::errors::ParsingError;
use parse_data::transform_parse::domain::methods_domain::date_pattern::DatePattern;
use parse_data::transform_parse::domain::methods_domain::date_type::DateType;

pub struct TransientDate{
    day: Option<u8>,
    month: Option<u8>,
    year: Option<usize>,
    epoch: Option<Epoch>,
}

impl TransientDate {
    fn new(day: Option<u8>, month: Option<u8>, year: Option<usize>) -> TransientDate {
        TransientDate{
            day,
            month,
            year,
            //todo: discern BC/CE
            epoch: Option::from(Epoch::After),
        }
    }
}

pub(crate) struct TransientDatePeriod {
    pub date1: Option<TransientDate>,
    pub date2: TransientDate,
    pub calendar_type: DateType,
}

impl TransientDatePeriod {
    pub(crate) fn new(caps: Captures, date_pattern: &DatePattern, date_type: &DateType) -> Result<TransientDatePeriod, ParsingError> {
        let day1:Option<u8> = if caps.name("day1").is_some() {
            let number: &u8 = &caps["day1"].to_owned().parse::<u8>().unwrap();
            Option::from(number.to_owned())
        } else {
            None
        };
        let month1 = if caps.name("month1").is_some() {
            if date_pattern.first_date.unwrap().month_word.unwrap() == true {
                let name = &caps["month1"].to_owned();
                Option::from(parse_month_to_number(name)?)
            } else {
                let number: &u8 = &caps["month1"].to_owned().parse::<u8>().unwrap();
                Option::from(number.to_owned())
            }
        } else {
            None
        };
        let year1 = if caps.name("year1").is_some() {
            let number: &usize = &caps["year1"].to_owned().parse::<usize>().unwrap();
            Option::from(number.to_owned())
        } else {
            None
        };
        let day2:Option<u8> = if caps.name("day2").is_some() {
            let number: &u8 = &caps["day2"].to_owned().parse::<u8>().unwrap();
            Option::from(number.to_owned())
        } else {
            None
        };
        let month2 = if caps.name("month2").is_some() {
            if date_pattern.date.month_word.unwrap() == true {
                let name = &caps["month2"].to_owned();
                Option::from(parse_month_to_number(name)?)
            } else {
                let number: &u8 = &caps["month1"].to_owned().parse::<u8>().unwrap();
                Option::from(number.to_owned())
            }
        } else {
            None
        };
        // year2 is obligatory
        let year2 = &caps["year2"].parse::<usize>().unwrap();
        let year2 = Option::from(year2.to_owned());
        let mut date1 = Option::from(TransientDate::new(day1, month1, year1));
        let mut date2 = TransientDate::new(day2, month2, year2);
        Ok(TransientDatePeriod {
            date1,
            date2,
            calendar_type: date_type.to_owned(),
        })
    }
    pub(crate) fn complete_dates(&mut self) -> Result<(), ParsingError> {
        if self.date2.day.is_none() {
            self.date2.day = Option::from(1u8);
        }
        if self.date2.month.is_none() {
            self.date2.month = Option::from(1u8);
        }
        if self.date1.is_some() {
            if self.date1.as_ref().unwrap().day.is_none() {
                self.date1.as_mut().unwrap().day = self.date2.day;
            }
            if self.date1.as_ref().unwrap().month.is_none() {
                self.date1.as_mut().unwrap().month = self.date2.month;
            }
            if self.date1.as_ref().unwrap().year.is_none() {
                self.date1.as_mut().unwrap().year = self.date2.year;
            }
        } else {
            self.date1 = Option::from(TransientDate{
                day: self.date2.day,
                month: self.date2.month,
                year: self.date2.year,
                epoch: self.date2.epoch,
            });
        }
        Ok(())
    }
}

fn parse_month_to_number(name: &String) -> Result<u8, ParsingError> {
    // parse Jan/Janu/Januar/January/Janv/Janvier etc. to 1  etc.pp.
    todo!()
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Epoch {
    Before,
    After
}


#[derive(Debug, PartialEq)]
pub struct Date{
    pub year: usize,
    pub month: u8,
    pub day: u8,
    pub epoch: Epoch,
}

#[derive(Debug, PartialEq)]
pub struct DatePeriod {
    pub(crate) date1: Date,
    pub(crate) date2: Date,
    pub calendar_type: DateType,
}
impl DatePeriod {
    pub(crate) fn new(transient: TransientDatePeriod) -> DatePeriod {
        let date2 = Date::new_date(transient.date2);
        let date1 = Date::new_date(transient.date1.unwrap());
        DatePeriod{
            date1,
            date2,
            calendar_type: DateType::Gregorian,
        }
    }
    pub(crate) fn to_string_date(&self) ->String {
        // calendar:epoch:yyyy-mm-dd:epoch:yyyy-mm-dd
        let calendar = &self.calendar_type;
        let mut date:String = "".to_string();
        if self.date1.is_some() {
            let epoch = self.date1.as_ref().unwrap().epoch;
            let day = self.date1.as_ref().unwrap().day;
            let month = self.date1.as_ref().unwrap().month;
            let year = self.date1.as_ref().unwrap().year;
            date = format!("{:?}:{:?}:{:?}:{:?}:{:?}:", calendar, epoch, year, month, day);
        }
        let epoch = self.date2.epoch;
        let day = self.date2.day;
        let month = self.date2.month;
        let year = self.date2.year;
        date = format!("{:?}{:?}:{:?}:{:?}:{:?}:{:?}",date, calendar, epoch, year, month, day);
        date
    }
}
impl Date {
    pub(crate) fn new_date(date: TransientDate) -> Date {
        Date{
            year: date.year.unwrap(),
            month: date.month.unwrap(),
            day: date.day.unwrap(),
            epoch: date.epoch.unwrap(),
        }
    }
}
#[cfg(test)]
mod test {
    use parse_data::transform_parse::domain::methods_domain::date_type::DateType;
    use crate::manipulation::date_variants::DateWrapper;

    #[test]
    fn test_to_date_1() {
        //todo 04-050-12-23-300 is read as day:04, year:050, day:12:month:23:year:300, but it is more likely: month:4 instead of day 4
    }
}
/*
all possible date formats: https://github.com/dasch-swiss/dsp-tools/blob/main/src/dsp_tools/commands/excel2xml/excel2xml_lib.py
- 0476-09-04 -> GREGORIAN:CE:0476-09-04:CE:0476-09-04
    - 0476_09_04 -> GREGORIAN:CE:0476-09-04:CE:0476-09-04
    - 30.4.2021 -> GREGORIAN:CE:2021-04-30:CE:2021-04-30
    - 5/11/2021 -> GREGORIAN:CE:2021-11-05:CE:2021-11-05
    - Jan 26, 1993 -> GREGORIAN:CE:1993-01-26:CE:1993-01-26
    - 28.2.-1.12.1515 -> GREGORIAN:CE:1515-02-28:CE:1515-12-01
    - 25.-26.2.0800 -> GREGORIAN:CE:0800-02-25:CE:0800-02-26
    - 1.9.2022-3.1.2024 -> GREGORIAN:CE:2022-09-01:CE:2024-01-03
    - 1848 -> GREGORIAN:CE:1848:CE:1848
    - 1849/1850 -> GREGORIAN:CE:1849:CE:1850
    - 1849/50 -> GREGORIAN:CE:1849:CE:1850
    - 1845-50 -> GREGORIAN:CE:1845:CE:1850
    - 840-50 -> GREGORIAN:CE:840:CE:850
    - 840-1 -> GREGORIAN:CE:840:CE:841
    - 1000-900 av. J.-C. -> GREGORIAN:BC:1000:BC:900
    - 45 av. J.-C. -> GREGORIAN:BC:45:BC:45
 */