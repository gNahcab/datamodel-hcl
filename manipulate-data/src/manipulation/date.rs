use regex::Regex;
use parse_data::errors::ParsingError;
use parse_data::transform_parse::domain::methods_domain::date_type::DateType;
use parse_data::xlsx_parse::data_sheet::DataSheet;


pub(crate) struct TransientDate {
    pub year: Option<usize>,
    pub month: Option<usize>,
    pub day: Option<usize>,
    pub designation: Option<DatingDesignation>,
    pub calendar_type: Option<DateType>,
}
impl TransientDate {
    pub(crate) fn new() -> TransientDate {
        TransientDate{
            year: None,
            month: None,
            day: None,
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
    pub(crate) fn add_designation(&mut self, designation: DatingDesignation) {
        self.designation = Option::from(designation);
    }
}
#[derive(Debug, PartialEq)]
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

impl Date {
    pub(crate) fn to_string_date(&self) ->String {
        todo!()
    }
    pub(crate) fn new(transient: TransientDate) -> Date {
        Date{
            year: transient.year.unwrap(),
            month: transient.month,
            day: transient.day,
            designation: transient.designation.unwrap(),
            calendar_type: transient.calendar_type.unwrap(),
        }
    }
}
#[cfg(test)]
mod test {
    use parse_data::transform_parse::domain::methods_domain::date_type::DateType;
    use crate::manipulation::date::{Date, DatingDesignation};
    use crate::manipulation::date_variants::DateWrapper;

    #[test]
    fn test_to_date_1() {
        let maybe_date: String = "0476-09-04".to_string();
        let result  = DateWrapper(DateType::Gregorian, maybe_date).to_date();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Date{
            year: 476,
            month: Some(9),
            day: Some(4),
            designation: DatingDesignation::After,
            calendar_type: DateType::Gregorian,
        });
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