use regex::Regex;
use parse_data::errors::ParsingError;
use parse_data::transform_parse::domain::methods_domain::date_type::DateType;
use crate::manipulation::date::{Date, DatePeriod, DatingDesignation, TransientDateInfo};

pub struct DateWrapper (pub(crate) DateType, pub(crate) String);

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
impl DateWrapper {
    pub fn to_date(&self) -> Result<DatePeriod, ParsingError> {
        //return as date-struct
        let date_1 = self.variant_1();
        if date_1.is_ok() {
            return Ok(date_1.unwrap());
        }
        let date_2 = self.variant_2();
        if date_2.is_ok() {
            return Ok(date_2.unwrap());
        }
        let date_3 = self.variant_3();
        if date_3.is_ok() {
            return Ok(date_3.unwrap());
        }
        Err(ParsingError::ValidationError(format!("cannot parse this '{:?}' to a date", &self.1)))
    }
    fn find_match_1(&self, re: Regex) -> Result<DatePeriod, ()>{
        let caps = re.captures(self.1.as_str());
        if caps.is_some() {
            let caps = caps.unwrap();
            let mut transient_data = TransientDateInfo::new();
            transient_data.add_year(&caps["year2"]);
            transient_data.add_month(&caps["month2"]);
            transient_data.add_day(&caps["day2"]);
            transient_data.add_optional_year(&caps.name("year1"));
            transient_data.add_optional_month(&caps.name("month1"));
            transient_data.add_optional_day(&caps.name("day1"));
            transient_data.add_calendar_type(self.0.to_owned()) ;
            transient_data.add_designation(&caps.name("bc"));
            transient_data.prepare_optional();
            return Ok(DatePeriod::new(transient_data));
        }
        Err(())
    }

    /// five cases:
    /// case 1: year-month-day || day-month-year
    /// case 2: year-month || month-year
    /// case 3: case 1 or case 2 with month written as word (e.g. Jan 1991)
    /// case 3.2 month-day-year
    /// case 4: year
    /// case 5: symbols, words used to convey date is BC or CE

    pub(crate) fn variant_1(&self) -> Result <DatePeriod, ()> {
        // year at start, match yyy{y}-m{m}-d{d}
        let re = Regex::new(r"^(?x)
            (?P<bc>-)?
            (?P<year1>\d{3,4})?  # the optional year 1
            \W?
            (?P<month1>\d{1,2})? # the optional month 1
            \W?
            (?P<day1>\d{1,2})?   # the optional day 1
            \W?
            (?P<year2>\d{3,4})  # the year
            \W
            (?P<month2>\d{1,2}) # the month
            \W
            (?P<day2>\d{1,2})   # the day
            $").unwrap();
        self.find_match_1(re)

}
    fn variant_2(&self) -> Result <DatePeriod, ()> {
        // year at end, match d{d}-m{m}-yyy{y}
        let re = Regex::new(r"^(?x)
            (?P<bc>-)?
            (?P<day1>\d{1,2})?  # the day 1
            \W?
            (?P<month1>\d{1,2})? # the month 1
            \W?
            (?P<year1>\d{3,4})?   # the year 1
            \W?
            (?P<day2>\d{1,2})  # the day 2
            \W
            (?P<month2>\d{1,2}) # the month 2
            \W
            (?P<year2>\d{3,4})   # the day 2
            $").unwrap();
        self.find_match_1(re)
    }
    fn variant_5(&self) -> Result <DatePeriod, ()> {
        //1848
        //1849/50
        //1849-50
        //840-1
        //-1000-900 -> BC:1000:BC:900? or BC:1000:CE:900? -> always take shorter period
        let re = Regex::new(r"^(?x)
            (?P<bc>-)?
            (?P<year1>\d{3,4})?   # the year 1
            \W?
            (?P<year2>\d{3,4})   # the year 2
            $").unwrap();
        self.find_match_1(re)
    }
    fn variant_3(&self) -> Result <DatePeriod, ()> {
        //Jan 26, 1993
        //January 26, 1993
        todo!()
    }
    fn variant_4(&self) -> Result <DatePeriod, ()> {
        //26 Jan 1993
        //26 Januar 1993
        todo!()
    }
    fn variant_6(&self) -> Result <Date, ()> {
        //1000-900 av. J.-C
        //45 av. J.-C.
        todo!()
    }
}