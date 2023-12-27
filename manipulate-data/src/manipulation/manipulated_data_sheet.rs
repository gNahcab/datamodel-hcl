use std::collections::HashMap;
use std::ffi::c_void;
use regex::{Regex, RegexBuilder};
use polars::export::arrow::types::Index;
use polars::export::num::ToPrimitive;
use parse_data::errors::ParsingError;
use parse_data::transform_parse::domain::header_value::HeaderValue;
use parse_data::transform_parse::domain::methods_domain::behavior_type::BehaviorType;
use parse_data::transform_parse::domain::methods_domain::combine_method::CombineMethod;
use parse_data::transform_parse::domain::methods_domain::lower_upper_method::{LowerMethod, UpperMethod};
use parse_data::transform_parse::domain::methods_domain::method::Method;
use parse_data::transform_parse::domain::methods_domain::replace_method::ReplaceMethod;
use parse_data::transform_parse::domain::methods_domain::target_type::TargetType;
use parse_data::transform_parse::domain::methods_domain::to_date_method::ToDateMethod;
use parse_data::transform_parse::domain::transformations::Transformations;
use parse_data::xlsx_parse::data_sheet::DataSheet;

pub struct ManipulatedDataSheetWrapper (pub(crate) DataSheet, pub(crate) Transformations);

impl ManipulatedDataSheetWrapper {
    pub fn to_manipulated_data_sheet(&self) -> Result<ManipulatedDataSheet, ParsingError> {
        let mut transient_data_sheet: TransientDataSheet = TransientDataSheet::new(self.0.copy());

        let mut any_success: bool = true;
        let mut all_methods: Vec<Method> = self.1.methods();

        'outer: while any_success  {
            any_success != any_success;
            for counter in 0.. all_methods.len() {
                let method = match all_methods.get(counter) {
                    None => {
                        // all_methods is empty
                        break 'outer
                    }
                    Some(method) => {method}
                };
                match method {
                    Method::CombineMethod(method) => {
                        if transient_data_sheet.add_combine(method)? {
                            any_success = true;
                            all_methods.remove(counter);
                        }
                    }
                    Method::ReplaceMethod(method) => {
                        if transient_data_sheet.add_replace(method)? {
                            any_success = true;
                            all_methods.remove(counter);
                        }
                    }
                    Method::ToDateMethod(method) => {
                        if transient_data_sheet.add_to_date(method)? {
                            any_success = true;
                            all_methods.remove(counter);
                        }
                    }
                    Method::LowerMethod(method) => {
                        if transient_data_sheet.add_lower(method)? {
                            println!("lower: {:?}", &method);
                            any_success = true;
                            all_methods.remove(counter);
                        }
                    }
                    Method::UpperMethod(method) => {
                        if transient_data_sheet.add_upper(method)? {
                            println!("upper: {:?}", &method);
                            any_success = true;
                            all_methods.remove(counter);
                        }
                    }
                    _ => break
                }
            }
        }
        if !all_methods.is_empty() {
            return Err(ParsingError::ValidationError(format!("couldn't perform some transformations because input couldn't be assigned to assignments or headers or other transform-outputs: '{:?}'", all_methods)));
        }
        let manipulated_data_sheet: ManipulatedDataSheet = ManipulatedDataSheet::new(transient_data_sheet);
        Ok(manipulated_data_sheet)
}
}
struct TransientDataSheet {
    pub tabular_data: Vec<Vec<String>>,
    pub width: usize,
    pub height: usize,
    pub headers: Vec<String>,
    pub assignments: HashMap<String, HeaderValue>,
    pub new_assignments: HashMap<String, HeaderValue>
}


impl TransientDataSheet {
    fn new(data_sheet: DataSheet) -> TransientDataSheet {
        TransientDataSheet {
            tabular_data: data_sheet.tabular_data,
            width: data_sheet.width,
            height: data_sheet.height,
            headers: data_sheet.headers,
            assignments: data_sheet.assignments,
            new_assignments: Default::default(),
        }
    }
    fn get_nr_from_headers(&self, name: &String) -> isize {
        for (i, header) in self.headers.iter().enumerate() {
            if header != name {continue}
            return i.to_isize().unwrap();
        }
        -1
    }
    fn get_tabular_number(&self, input: &HeaderValue) -> isize {
        // check if assignments contains input
        // 1.contains input:
        // if ass. contains input, check if headers contains assigned input
        // return row/col-nr
        // 2 doesn't contain input, check headers directly
        // return row/col-nr
        let name = match input {
            HeaderValue::Name(name) => {name}
            HeaderValue::Number(number) => { return number.to_isize().unwrap()}
        };
        let maybe = self.assignments.get(name.as_str());
        let assigned_value = match maybe {
            None => {
             return self.get_nr_from_headers(&name);

            }
            Some(assigned) => {assigned}
        };
        let assigned_header = match assigned_value {
            HeaderValue::Name(header) => {header}
            HeaderValue::Number(number) => {return number.to_isize().unwrap();}
        };
        return self.get_nr_from_headers(&assigned_header);

    }
    pub fn add_lower(&mut self, lower: &LowerMethod) -> Result<bool, ParsingError> {
        // copy vec, lower values and add at the end
        let nr = self.get_tabular_number(&lower.input);
        if nr == -1 {
            //doesn't exist or exists later
            return Ok(false)
        }
        let old_vec = self.tabular_data.get(nr.to_usize().unwrap()).unwrap();
        let new_vec: Vec<String> = old_vec.iter().map(|entry|entry.to_owned().to_lowercase()).collect();
        self.headers.push(lower.output.to_owned());
        self.tabular_data.push(new_vec);
        Ok(true)
    }

    pub fn add_upper(&mut self, upper: &UpperMethod) -> Result<bool, ParsingError> {
        // copy vec, upper values and add at the end
        let nr = self.get_tabular_number(&upper.input);
        if nr == -1 {
            //doesn't exist or exists later
            return Ok(false)
        }
        let old_vec = self.tabular_data.get(nr.to_usize().unwrap()).unwrap();
        let new_vec: Vec<String> = old_vec.iter().map(|entry|entry.to_owned().to_uppercase()).collect();
        self.headers.push(upper.output.to_owned());
        self.tabular_data.push(new_vec);
        Ok(true)
    }
    pub (crate) fn perform_replace(replace: &ReplaceMethod, old_vec: &Vec<String>) -> Vec<String> {
        match replace.target {
            TargetType::Part => {
                match replace.behavior {
                    BehaviorType::Lazy => {
                        let new_vec: Vec<String> = old_vec.iter()
                            .map(|value| value.replacen(&replace.old, &replace.new, 1))
                            .collect();
                        return new_vec;
                    }
                    BehaviorType::Greedy => {
                        let new_vec: Vec<String> = old_vec.iter()
                            .map(|value|
                                    value.replace(&replace.old, &replace.new)
                                )
                            .collect();
                        return new_vec;
                    }
                }
            }
            TargetType::Whole => {
                match replace.behavior {
                    BehaviorType::Lazy => {
                        let re = Regex::new(format!("(\\b{}\\b)", replace.old).as_str()).unwrap();
                        let new_vec: Vec<String> = old_vec.iter()
                            .map(|value| re.replace(value, &replace.new).to_string())
                            .collect();
                        return new_vec;
                    }
                    BehaviorType::Greedy => {
                        let re = Regex::new(format!("(\\b{}\\b)", replace.old).as_str()).unwrap();
                        let new_vec: Vec<String> = old_vec.iter()
                            .map(|value| re.replace_all(value, &replace.new).to_string())
                            .collect();
                        return new_vec;
                    }
                }
            }
        }
    }
    pub fn add_replace(&mut self, replace: &ReplaceMethod) -> Result<bool, ParsingError> {
        let nr = self.get_tabular_number(&replace.input);
        if nr == -1 {
            //doesn't exist or exists later
            return Ok(false)
        }
        let old_vec = self.tabular_data.get(nr.to_usize().unwrap()).unwrap();
        let new_vec = TransientDataSheet::perform_replace(replace, old_vec);
        self.headers.push(replace.output.to_owned());
        self.tabular_data.push(new_vec);
        Ok(true)
    }
    pub fn add_combine(&mut self, combine: &CombineMethod) -> Result<bool, ParsingError> {
        let mut inputs: Vec<usize> = vec![];
        for input in &combine.input {
            let nr = self.get_tabular_number(input);
            let nr = match nr {
                -1 => {return Ok(false)}
                _ => {nr.to_usize().unwrap()}
            };
            inputs.push(nr)
        }
        let old_vecs: Vec<&Vec<String>> = inputs.iter().map(|nr|self.tabular_data.get(nr.to_owned()).unwrap()).collect();
        let new_vec: Vec<String> = TransientDataSheet::perform_combine(old_vecs, &combine);
        self.headers.push(combine.output.to_owned());
        self.tabular_data.push(new_vec);
        Ok(true)
    }
    fn perform_combine(data: Vec<&Vec<String>>, combine_method: &&CombineMethod) -> Vec<String>{
        let first_vec = data.get(0).unwrap();
        let second_vec = data.get(1).unwrap();
        let mut new_vec:Vec<String> = vec![];
        for (i, value) in first_vec.iter().enumerate() {
            let mut new_value = value.to_owned();
            if combine_method.prefix.is_some() {
                new_value = format!("{}{}",combine_method.prefix.as_ref().unwrap(),new_value);
            }
            if combine_method.separator.is_some() {
                new_value = format!("{}{}", new_value, combine_method.separator.as_ref().unwrap());
            }
            new_value = format!("{}{}", new_value, second_vec.get(i).unwrap());
            if combine_method.suffix.is_some() {
                new_value = format!("{}{}", new_value, combine_method.suffix.as_ref().unwrap());
            }
            new_vec.push(new_value);
        }
        new_vec
    }
    pub fn add_to_date(&mut self, to_date: &ToDateMethod) -> Result<bool, ParsingError> {
        let nr = self.get_tabular_number(&to_date.input);
        if nr == -1 {
            //doesn't exist or exists later
            return Ok(false)
        }
        let old_vec = self.tabular_data.get(nr.to_usize().unwrap()).unwrap();
        let new_vec = TransientDataSheet::perform_to_date(to_date, old_vec)?;
        self.headers.push(to_date.output.to_owned());
        self.tabular_data.push(new_vec);
        Ok(true)
    } fn perform_to_date(to_date_method: &ToDateMethod, data: &Vec<String>) -> Result<Vec<String>, ParsingError> {
        let mut new_dates:Vec<String> = vec![];
        for value in data.iter() {
            let new_date = transform_to_date(value)?;
            new_dates.push(new_date);
        }
        Ok(new_dates)
    }
}

fn transform_to_date(maybe_date: &String) -> Result<String, ParsingError> {
    todo!()
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
}


pub struct ManipulatedDataSheet {

}

impl ManipulatedDataSheet {
    fn new(transient_manipulated_data_sheet: TransientDataSheet) -> ManipulatedDataSheet {
        ManipulatedDataSheet{}
    }
}
#[cfg(test)]
mod test {
    use regex::Regex;
    use parse_data::transform_parse::domain::header_value::HeaderValue;
    use parse_data::transform_parse::domain::methods_domain::behavior_type::BehaviorType;
    use parse_data::transform_parse::domain::methods_domain::combine_method::CombineMethod;
    use parse_data::transform_parse::domain::methods_domain::replace_method::ReplaceMethod;
    use parse_data::transform_parse::domain::methods_domain::target_type::TargetType;
    use crate::manipulation::manipulated_data_sheet::{ManipulatedDataSheet, transform_to_date, TransientDataSheet};

    #[test]
    fn test_to_date() {
        let value = &"".to_string();
        let result = transform_to_date(value);
    }
    #[test]
    fn test_combine() {
        let mut data :Vec<&Vec<String>>= vec![];
        let vec_1: Vec<String> = ["A".to_string(), "B".to_string(), "C".to_string()].to_vec();
        let vec_2: Vec<String> = ["a".to_string(), "b".to_string(), "c".to_string()].to_vec();
        data.push(&vec_1);
        data.push(&vec_2);
        let combine_method = CombineMethod{
            input: [HeaderValue::Name("vec1".to_string()), HeaderValue::Name("vec2".to_string())].to_vec(),
            output: "combineValue".to_string(),
            separator: Option::from("_".to_string()),
            prefix: Option::from("$_".to_string()),
            suffix: Option::from("_£".to_string()),
        };
        let result = TransientDataSheet::perform_combine(data,&&combine_method);
        println!("{:?}", result);
    }
    #[test]
    fn test_replace_lazy_part() {
        let replace_method: ReplaceMethod = ReplaceMethod{
            output: "hasNewValue".to_string(),
            input: HeaderValue::Name("hasValue".to_string()),
            old: "Dictionary".to_string(),
            new: "Dict".to_string(),
            behavior: BehaviorType::Lazy,
            target: TargetType::Part,
        };
        let mut old_vec: Vec<String> = vec![];
        old_vec.push("0000Dictionary1111Dictionary".to_string());
        old_vec.push("0000Dictionary1111Dictionary".to_string());
        old_vec.push("0000Dictionary1111Dictionary".to_string());
        let result = TransientDataSheet::perform_replace(&replace_method,&old_vec);
    }
    #[test]
    fn test_replace_greedy_part() {
        let replace_method: ReplaceMethod = ReplaceMethod{
            output: "hasNewValue".to_string(),
            input: HeaderValue::Name("hasValue".to_string()),
            old: "Dictionary".to_string(),
            new: "Dict".to_string(),
            behavior: BehaviorType::Greedy,
            target: TargetType::Part,
        };
        let mut old_vec: Vec<String> = vec![];
        old_vec.push("0000Dictionary1111Dictionary".to_string());
        old_vec.push("0000Dictionary1111Dictionary".to_string());
        old_vec.push("0000Dictionary1111Dictionary".to_string());
        let result = TransientDataSheet::perform_replace(&replace_method,&old_vec);

    }
    #[test]
    fn test_replace_lazy_whole() {
        let replace_method: ReplaceMethod = ReplaceMethod{
            output: "hasNewValue".to_string(),
            input: HeaderValue::Name("hasValue".to_string()),
            old: "Dictionary".to_string(),
            new: "Dict".to_string(),
            behavior: BehaviorType::Greedy,
            target: TargetType::Whole,
        };
        let mut old_vec: Vec<String> = vec![];
        old_vec.push("0000Dictionary1111Dictionary".to_string());
        old_vec.push("0000Dictionary1111Dictionary".to_string());
        old_vec.push("0000Dictionary1111Dictionary".to_string());
        let result = TransientDataSheet::perform_replace(&replace_method,&old_vec);
    }
    fn test_replace_greedy_whole() {
        todo!()
    }
}
