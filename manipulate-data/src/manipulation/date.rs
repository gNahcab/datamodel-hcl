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
            epoch: Option::from(Epoch::CE),
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
            if date_pattern.first_date.as_ref().unwrap().month_word.unwrap() == true {
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
                let number: &u8 = &caps["month2"].to_owned().parse::<u8>().unwrap();
                Option::from(number.to_owned())
            }
        } else {
            None
        };
        // year2 is mandatory
        let year2 = &caps["year2"].parse::<usize>().unwrap();
        let year2 = Option::from(year2.to_owned());
        let date1 = Option::from(TransientDate::new(day1, month1, year1));
        let date2 = TransientDate::new(day2, month2, year2);
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

fn parse_month_to_number(name: &String) -> Result<u8, ParsingError>  {
    //todo: allow utf-8; at the moment only ASCII-Characters can be parsed
        let januars = ["January", "Jan", "Jän", "Janv",  "Januar", "Janvier", "Gennaio", "Genn",];
        let februarys = ["February", "Feb", "Februar", "Février", "Fevrier", "Févr", "Fevr", "Febbraio", "Febbr",];
        let marchs = ["March", "Mar", "März", "Mars", "Marzo", "Mar",];
        let aprils = ["April", "Apr", "Avril", "Aprile",];
        let mays = ["May", "Mai", "Maggio", "Magg",];
        let junes = ["June", "Juni", "Juin", "Giugno",];
        let julys = ["July", "Juli", "Juillet", "Juil", "Luglio",];
        let augusts = ["August","Aug", "Août", "Aout", "Agosto", "Ag",];
        let septembers = ["September", "Sept", "Septembre", "Settembre", "Sett",];
        let octobres = ["October", "Oct", "Octobre", "Ottobre", "Ott",];
        let novembers = ["November", "Nov", "Novembre", "Novembre",];
        let decembers = ["December", "Dec", "Dezember", "Dez", "Décembre", "Decembre", "Déc", "Dicembre",];

    if januars.contains(&&**name) {
        return Ok(1u8);
    }
    if februarys.contains(&&**name) {
        return Ok(2u8);
    }
    if marchs.contains(&&**name) {
        return Ok(3u8);
    }
    if aprils.contains(&&**name) {
        return Ok(4u8);
    }
    if mays.contains(&&**name) {
        return Ok(5u8);
    }
    if junes.contains(&&**name) {
        return Ok(6u8);
    }
    if julys.contains(&&**name) {
        return Ok(7u8);
    }
    if augusts.contains(&&**name) {
        return Ok(8u8);
    }
    if septembers.contains(&&**name) {
        return Ok(9u8);
    }
    if octobres.contains(&&**name) {
        return Ok(10u8);
    }
    if novembers.contains(&&**name) {
        return Ok(11u8);
    }
    if decembers.contains(&&**name) {
        return Ok(12u8);
    }
    return Err(ParsingError::ValidationError(format!("couldn't find a matching month for '{:?}'. Either it is not a month or missing.", name)));
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Epoch {
    BC,
    CE
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
            calendar_type: transient.calendar_type,
        }
    }
    pub(crate) fn to_string_date(&self) ->String {
        // calendar:epoch:yyyy-mm-dd:epoch:yyyy-mm-dd

        let calendar = format!("{:?}", &self.calendar_type).to_uppercase();
        let epoch1 = self.date1.epoch;

        let day1= Date::two_string(self.date1.day);
        let month1 = Date::two_string(self.date1.month);
        let year1 = Date::four_string(self.date1.year);

        let epoch2 = self.date2.epoch;
        let day2: String = Date::two_string(self.date2.day);
        let month2: String = Date::two_string(self.date2.month);
        let year2: String = Date::four_string(self.date2.year);
        let date = format!("{}:{:?}:{}:{}:{}:{:?}:{}:{}:{}", calendar, epoch1, year1, month1, day1, epoch2, year2, month2, day2);

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
    fn two_string(number: u8) -> String {
        let mut parsed = number.to_string();
        while parsed.len() < 2 {
            parsed.insert(0, '0');
        }
        return parsed
    }
    fn four_string(number: usize) -> String {
        let mut parsed = number.to_string();
        while parsed.len() < 4 {
            parsed.insert(0, '0');
        }
        return parsed
    }
}
