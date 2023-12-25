use std::collections::HashMap;
use std::num::ParseIntError;
use polars::export::arrow::types::Index;
use polars::export::num::ToPrimitive;
use parse_data::errors::ParsingError;
use parse_data::transform_parse::domain::header_value::HeaderValue;
use parse_data::transform_parse::domain::methods_domain::combine_method::CombineMethod;
use parse_data::transform_parse::domain::methods_domain::lower_upper_method::{LowerMethod, UpperMethod};
use parse_data::transform_parse::domain::methods_domain::method::Method;
use parse_data::transform_parse::domain::methods_domain::replace_method::ReplaceMethod;
use parse_data::transform_parse::domain::methods_domain::to_date_method::ToDateMethod;
use parse_data::transform_parse::domain::transformations::Transformations;
use parse_data::xlsx_parse::data_sheet::DataSheet;

pub struct ManipulatedDataSheetWrapper (pub(crate) DataSheet, pub(crate) Transformations);

impl ManipulatedDataSheetWrapper {
    pub fn to_manipulated_data_sheet(&self) -> Result<ManipulatedDataSheet, ParsingError> {
        let mut transient_data_sheet: TransientDataSheet = TransientDataSheet::new(self.0.copy());

        let mut any_success: bool = true;
        let mut all_methods: Vec<Method> = self.1.methods();
        let mut counter = 0;
        while any_success  {
            any_success = false;
            let maybe_method = all_methods.get(counter);
            let method = match maybe_method {
                None => {break}
                Some(method) => {method}
            };
            match method {
                Method::CombineMethod(method) => {
                    if transient_data_sheet.add_combine(method)? {
                        any_success = true;
                        all_methods.remove(counter);
                        counter += 1;
                    }
                }
                Method::ReplaceMethod(method) => {
                    if transient_data_sheet.add_replace(method)? {
                        any_success = true;
                        all_methods.remove(counter);
                        counter += 1;
                    }
                }
                Method::ToDateMethod(method) => {
                    if transient_data_sheet.add_to_date(method)? {
                        any_success = true;
                        all_methods.remove(counter);
                        counter += 1;
                    }
                }
                Method::LowerMethod(method) => {
                    if transient_data_sheet.add_lower(method)? {
                        any_success = true;
                        all_methods.remove(counter);
                        counter += 1;
                    }
                }
                Method::UpperMethod(method) => {
                    if transient_data_sheet.add_upper(method)? {
                        any_success = true;
                        all_methods.remove(counter);
                        counter += 1;
                    }
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
        // 2 doesn't contain input, check directly in headers
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
    pub fn add_replace(&self, replace: &ReplaceMethod) -> Result<bool, ParsingError> {
        todo!()
    }
    pub fn add_combine(&self, combine: &CombineMethod) -> Result<bool, ParsingError> {
        todo!()
    }
    pub fn add_to_date(&self, to_date: &ToDateMethod) -> Result<bool, ParsingError> {
        todo!()
    }
}

pub struct ManipulatedDataSheet {

}

impl ManipulatedDataSheet {
    fn new(transient_manipulated_data_sheet: TransientDataSheet) -> ManipulatedDataSheet {
        ManipulatedDataSheet{}
    }
}