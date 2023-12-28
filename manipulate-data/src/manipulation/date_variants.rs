use regex::Regex;
use parse_data::errors::ParsingError;
use parse_data::transform_parse::domain::methods_domain::date_type::DateType;
use crate::manipulation::date::{Date, DatingDesignation, TransientDate};

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
    pub fn to_date(&self) -> Result<Date, ParsingError> {
        // parse string here
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

        Ok(Date{
            year: 0,
            month: None,
            day: None,
            designation: DatingDesignation::Before,
            calendar_type: self.0.to_owned(),
        })
    }
    pub(crate) fn variant_1(&self) -> Result <Date, ()> {
        let re = Regex::new(r"^(?x)
            (?P<year>\d{3,4})  # the year
            -
            (?P<month>\d{1,2}) # the month
            -
            (?P<day>\d{1,2})   # the day
            $").unwrap();

        let caps = re.captures(self.1.as_str());
        if caps.is_some() {
            let caps = caps.unwrap();
            let mut transient_data = TransientDate::new();
            transient_data.add_year(&caps["year"]);
            transient_data.add_month(&caps["month"]);
            transient_data.add_day(&caps["day"]);
            transient_data.add_calendar_type(self.0.to_owned()) ;
            transient_data.add_designation(DatingDesignation::After);
            return Ok(Date::new(transient_data));
        }
        Err(())
}
    fn variant_2(&self) -> Result <Date, ()> {
        //0476_09_04
        todo!()
    }
    fn variant_3(&self) -> Result <Date, ()> {
        //30.4.2021
        todo!()
    }
    fn variant_4(&self) -> Result <Date, ()> {
        //5/11/2021
        todo!()
    }
    fn variant_5(&self) -> Result <Date, ()> {
        //Jan 26, 1993
        todo!()
    }
    fn variant_6(&self) -> Result <Date, ()> {
        //28.2.-1.12.1515
        todo!()
    }
    fn variant_7(&self) -> Result <Date, ()> {
        //25.-26.2.0800
        todo!()
    }
    fn variant_8(&self) -> Result <Date, ()> {
        //1.9.2022-3.1.2024
        todo!()
    }
    fn variant_9(&self) -> Result <Date, ()> {
        //1848
        todo!()
    }
    fn variant_10(&self) -> Result <Date, ()> {
        //1849/1850
        todo!()
    }
    fn variant_11(&self) -> Result <Date, ()> {
        //1849/50
        todo!()
    }
    fn variant_12(&self) -> Result <Date, ()> {
        //1849-50
        todo!()
    }
    fn variant_13(&self) -> Result <Date, ()> {
        //840-1
        todo!()
    }
    fn variant_14(&self) -> Result <Date, ()> {
        //1000-900 av. J.-C
        todo!()
    }
    fn variant_15(&self) -> Result <Date, ()> {
        //45 av. J.-C.
        todo!()
    }
    fn variant_16(&self) -> Result <Date, ()> {
        //-1000-900 -> BC:1000:BC:900? or BC:1000:CE:900? -> always take shorter period
        todo!()
    }
}