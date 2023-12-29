use arrow::compute::year;
use regex::Match;
use parse_data::transform_parse::domain::methods_domain::date_type::DateType;


pub(crate) struct TransientDateInfo {
    pub year: Option<usize>,
    pub month: Option<usize>,
    pub day: Option<usize>,
    pub optional_year: Option<usize>,
    pub optional_month: Option<usize>,
    pub optional_day: Option<usize>,
    pub designation: Option<DatingDesignation>,
    pub calendar_type: Option<DateType>,
}


impl TransientDateInfo {
    pub(crate) fn new() -> TransientDateInfo {
        TransientDateInfo {
            year: None,
            month: None,
            day: None,
            optional_year: None,
            optional_month: None,
            optional_day: None,
            designation: None,
            calendar_type: None,
        }
    }
    pub(crate) fn add_year(&mut self, year: &str) {
        self.year = Option::from(year.parse::<usize>().unwrap());
    }
     pub(crate) fn add_month(&mut self, month:&str) {
        self.month = Option::from(month.parse::<usize>().unwrap());
    }
     pub(crate) fn add_day(&mut self, day:&str) {
        self.day = Option::from(day.parse::<usize>().unwrap());
    }
    pub(crate) fn add_calendar_type(&mut self, calendar_type: DateType) {
        self.calendar_type = Option::from(calendar_type);
    }
    pub(crate) fn add_designation(&mut self, designation: &Option<Match>) {
        if designation.is_some() {
            self.designation = Option::from(DatingDesignation::Before);
        } else {
            self.designation =  Option::from(DatingDesignation::After)
        }
    }
    pub(crate) fn add_optional_year(&mut self, year: &Option<Match>) {
        if year.is_some() {
          self.optional_year = Option::from(year.unwrap().as_str().parse::<usize>().unwrap());
        }
    }
    pub(crate) fn add_optional_month(&mut self, month: &Option<Match>) {
        if month.is_some() {
            self.optional_month = Option::from(month.unwrap().as_str().parse::<usize>().unwrap());
        }
    }

    pub(crate) fn add_optional_day(&mut self, day: &Option<Match>) {
        if day.is_some() {
            self.optional_day = Option::from(day.unwrap().as_str().parse::<usize>().unwrap());
        }
    }

    pub(crate) fn prepare_optional(&mut self) {
        if self.optional_day.is_some() || self.optional_month.is_some() || self.optional_year.is_some() {
            if self.optional_year.is_none() {
                self.optional_year = self.year;
            }
            self.switch_optional_day_month();
        }

    }
    fn switch_optional_day_month(&mut self) {
        // in case there is something like mm1-yyyy1 - dd2 - mm2 - yyyy2, it won't recognize that mm1 is a month a not a date, so we need to switch this here
        if self.optional_day.is_some() {
            if self.optional_month.is_none() {
                self.optional_month = self.optional_day;
                self.optional_day = None
            }
        }
    }
}
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum DatingDesignation {
    Before,
    After
}


#[derive(Debug, PartialEq)]
pub struct Date{
    pub year: usize,
    pub month: Option<usize>,
    pub day: Option<usize>,
    pub designation: DatingDesignation,
    pub calendar_type: DateType,
}

#[derive(Debug, PartialEq)]
pub struct DatePeriod {
    pub(crate) date1: Date,
    pub(crate) date2: Option<Date>,
}
impl DatePeriod {
    pub(crate) fn new(transient: TransientDateInfo) -> DatePeriod {
        let mut date2 = None;
        if transient.optional_day.is_some() || transient.optional_month.is_some() || transient.optional_year.is_some() {
            date2 = Option::from((Date::new_date2(&transient)));
        }
        DatePeriod{
            date1: Date::new_date1(transient),
            date2,
        }
    }
    pub(crate) fn to_string_date(&self) ->String {
        todo!()
    }
}
impl Date {
    pub(crate) fn new_date1(transient_data: TransientDateInfo) -> Date {
        Date{
            year: transient_data.year.unwrap(),
            month: transient_data.month,
            day: transient_data.day,
            designation: DatingDesignation::Before,
            calendar_type: DateType::Gregorian,
        }
    }
    pub(crate) fn new_date2(transient_data: &TransientDateInfo) -> Date {
        Date{
            year: transient_data.optional_year.unwrap(),
            month:transient_data.optional_month,
            day: transient_data.optional_day,
            designation: transient_data.designation.as_ref().unwrap().clone(),
            calendar_type: transient_data.calendar_type.clone().unwrap(),
        }
    }
}
#[cfg(test)]
mod test {
    use parse_data::transform_parse::domain::methods_domain::date_type::DateType;
    use crate::manipulation::date_variants::DateWrapper;

    #[test]
    fn test_to_date_1() {
        let maybe_date: String = "-04-050-12-23-300".to_string();
        //todo 04-050-12-23-300 is read as day:04, year:050, day:12:month:23:year:300, but it is more likely: month:4 instead of day 4
        let result  = DateWrapper(DateType::Gregorian, maybe_date).to_date();
        println!("{:?}", result);
        assert!(result.is_ok());
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