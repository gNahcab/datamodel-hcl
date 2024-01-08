use std::collections::{HashMap, HashSet};
use std::path::Path;
use calamine::{DataType, Range};
use polars::export::arrow::types::Index;
use polars::export::num::ToPrimitive;
use crate::errors::ParsingError;
use crate::transform_parse::domain::assignment::Assignments;
use crate::transform_parse::domain::header_value::HeaderValue;
use crate::transform_parse::domain::organized_by::OrganizedBy;
use crate::transform_parse::domain::sheet_info::SheetInfo;
use crate::transform_parse::domain::transform_type::TransformXLSX;
use crate::xlsx_parse::organize_structure::{import_all_ordered_df, import_some_ordered_df};

#[derive(Debug)]
pub struct DataSheetWrapper(pub (String, Range<DataType>),pub SheetInfo);
impl DataSheetWrapper {
    pub fn to_data_sheet(&self) -> Result<DataSheet, ParsingError> {
        match self.1.structured_by {
            OrganizedBy::ROWOrganized => {
                println!("ROW");
                self.datasheet_by_row()
            }
            OrganizedBy::COLOrganized => {
                println!("COL");
                self.datasheet_by_col()
            }
        }
    }
    fn datasheet_by_row(&self) -> Result<DataSheet, ParsingError> {
        let mut transient_data_sheet: TransientDataSheet = TransientDataSheet::new(&self.1.resource, &self.1.assignments);
        transient_data_sheet.add_width(self.0.1.width());
        transient_data_sheet.add_height(self.0.1.height());
        let mut worksheet_iterator = self.0.1.rows();
        let mut start: usize = 0;
        if self.1.headers_exist == true {
            let first_row = worksheet_iterator.next();
            let first_row = match first_row {
                None => {
                    return Err(ParsingError::XlsxParse(format!("couldn't take first row in self.0: {:?}. Does self.0 have a first row?", self.0.0)));
                }
                Some(row) => {row}
            };
            let headers: Vec<String> = first_row.iter().map(|entry|(entry.to_string())).collect();

            transient_data_sheet.add_headers(headers);
            start = 1;
        }
        let mut data: Vec<Vec<String>> = Vec::with_capacity(self.0.1.width() - start);
        for _ in start..worksheet_iterator.len() {
            data.push(vec![]);
        }
        for row in worksheet_iterator {
            let row_vec: Vec<String> = row.iter().map(|entry|(entry.to_string())).collect();
            row_vec.iter().enumerate().for_each(|(i, value)|data[i].push(value.to_owned()));
        }
        transient_data_sheet.add_tabular_data(data);
        let data_sheet = DataSheet::new(transient_data_sheet);
        Ok(data_sheet)
    }
    fn datasheet_by_col(&self) -> Result<DataSheet, ParsingError> {
        // returns a datasheet that is reorganised by column, this is necessary because the importer imports the data by row
        let mut transient_data_sheet: TransientDataSheet= TransientDataSheet::new(&self.1.resource, &self.1.assignments);
        transient_data_sheet.add_width(self.0.1.height());
        transient_data_sheet.add_height(self.0.1.width());
        let mut worksheet_iterator = self.0.1.rows();
        let mut start = 0;
        if self.1.headers_exist {
             let headers: Vec<String> = self.0.1.cells().into_iter().filter(|(i, j, value)| j == &0usize).map(|(i,j, value)| value.to_string()).collect();
            transient_data_sheet.add_headers(headers);
            start = 1;
        }
        let mut data: Vec<Vec<String>> = vec![];
        for  row in worksheet_iterator {
            let row_vec: Vec<String> = row.iter().skip(start).map(|entry|entry.to_string()).collect();
            data.push(row_vec);
        }
        transient_data_sheet.add_tabular_data(data);

        let data_sheet = DataSheet::new(transient_data_sheet);
        Ok(data_sheet)
    }
}
pub struct TransientDataSheet {
    pub tabular_data: Vec<Vec<String>>,
    pub resource: String,
    pub height: usize,
    pub width: usize,
    pub headers: Option<Vec<String>>,
    pub assignments: HashMap<String, HeaderValue>,
}

impl TransientDataSheet {
    pub fn new(resource: &String, assignments: &Assignments) -> TransientDataSheet {
        TransientDataSheet { tabular_data: vec![], resource: resource.to_owned(), height: 0, width: 0, headers: None, assignments: assignments.assignments_to_header_value.to_owned() }
    }
    fn add_headers(&mut self, first_column: Vec<String>) -> Result<(), ParsingError> {
        //no duplicates allowed
        let mut uniq = HashSet::new();
        for name in first_column.iter() {
            //no name more than once used
            if uniq.insert(name) == false {
                return Err(ParsingError::ValidationError(format!("found duplicate in headers: '{:?}'. Every header must be unique.", name)));
            }
        }
        self.headers = Option::from(first_column);
        Ok(())
    }
    pub fn add_height(&mut self, height: usize) {
        self.height = height;
    }
    pub fn add_width(&mut self, width: usize) {
        self.width = width;
    }
    pub(crate) fn add_tabular_data(&mut self, data: Vec<Vec<String>>) {
        self.tabular_data = data;
    }
}
#[derive(Debug)]
pub struct DataSheet {
    pub tabular_data: Vec<Vec<String>>,
    pub resource: String,
    pub height: usize,
    pub width: usize,
    pub headers: Option<Vec<String>>,
    pub assignments: HashMap<String, HeaderValue>,
}

impl DataSheet {
}


impl DataSheet {
    pub fn new(transient_data_sheet: TransientDataSheet) -> DataSheet {
        DataSheet{
            tabular_data: transient_data_sheet.tabular_data,
            resource: transient_data_sheet.resource,
            height: transient_data_sheet.height,
            width: transient_data_sheet.width,
            headers: transient_data_sheet.headers,
            assignments: transient_data_sheet.assignments,
        }
    }
    pub fn copy(&self) -> DataSheet {
        DataSheet {
            tabular_data: self.tabular_data.clone(),
            resource: self.resource.clone(),
            height: self.height,
            width: self.width,
            headers: self.headers.clone(),
            assignments: self.assignments.clone(),
        }
    }

    fn do_column_numbers_exist(&self, sheet_info: &SheetInfo) -> Result<(), ParsingError> {
        // check if the column-number can be assigned to a column
        let headers: Vec<&HeaderValue> = sheet_info.assignments.assignments_to_header_value.iter().map(|(new_header, current)| current).collect();
        let numbers_greater_than_width: Vec<&u8>= headers
            .iter()
            .filter_map(|header_value| match header_value {
                HeaderValue::Number(number) => Some(number),
                _ => None,
            })
            .filter(|number| usize::from(number.to_owned().to_owned())  > self.width).collect();
        if numbers_greater_than_width.len() != 0 {
            return Err(ParsingError::ValidationError(format!("Some column/row numbers in 'assignments' of sheet-nr '{:?}' are greater than the width of the spreadsheet: '{:?}'",sheet_info.sheet_nr, numbers_greater_than_width)));
        }
        Ok(())
    }
    fn check_headers_in_assignments(&self, sheet_info:&SheetInfo) -> Result<(), ParsingError> {
        // check headers in assignments exist in spreadsheet
        let headers: Vec<&String> = sheet_info.assignments.assignments_to_header_value.iter().map(|(new_header, current)| current).flat_map(|current|match current {
            HeaderValue::Name(name) => {Some(name)}
            HeaderValue::Number(_) => {None}
        }).collect();
        let missing_headers: Vec<&&String> = headers.iter().filter(|header| !self.headers.as_ref().unwrap().contains(header)).collect();
        if missing_headers.len() != 0 {
            return Err(ParsingError::ValidationError(format!("Some column/row headers in 'assignments' of sheet-nr '{:?}' cannot be found in the spreadsheet: '{:?}'",sheet_info.sheet_nr, missing_headers)));
        }
        Ok(())
    }
    fn check_headers_numbers_not_match(&self, sheet_info: &SheetInfo) -> Result<(), ParsingError> {
        // check headers and numbers assigned don't match: otherwise a column would be assigned twice: once as a number and once as a Header-string
        if self.headers.is_some() {
            let assigned_headers_to_header_number: Vec<(&String, usize)>= sheet_info.assignments.assignments_to_header_value.iter().
                filter_map(|(name, header_value)| match header_value {
                    HeaderValue::Name(value) => {Some(value)}
                    HeaderValue::Number(_) => {None}
                }).
                map(|value|
                    (value, self.headers.as_ref().unwrap().
                        iter().
                        position(|header_string|header_string == value).
                        unwrap())
                ).
                collect();
            let assigned_numbers: Vec<&u8> = sheet_info.assignments.assignments_to_header_value.iter().
                filter_map(|(name, header_value)| match header_value {
                    HeaderValue::Name(_) => {None}
                    HeaderValue::Number(value) => {Some(value)}
                }).collect();
            let headers_used_as_numbers: Vec<&(&String, usize)> = assigned_headers_to_header_number.iter().filter(|(header_name, position)| assigned_numbers.contains(&&(position.to_owned() as u8))).collect();
            if headers_used_as_numbers.len() != 0 {
                return Err(ParsingError::ValidationError(format!("found string-headers that were used as numbers as well: '{:?}'. Either use the number or the headers, but not both.", headers_used_as_numbers)))
            }
        }
        Ok(())
    }
    fn do_headers_exist(&self, sheet_info: &SheetInfo) -> Result<(), ParsingError> {
        if self.headers.is_some() {
            // check headers in assignments
            self.check_headers_in_assignments(sheet_info)?;
            // check headers and numbers assigned don't match
            self.check_headers_numbers_not_match(sheet_info)?;

        } else {
           // check that no headers exist in assignments
            let headers_that_should_not_exist: Vec<&String> = sheet_info.assignments.assignments_to_header_value
                .iter()
                .flat_map(|(_, value)| match value {
                    HeaderValue::Name(header) => {Some(header)}
                    HeaderValue::Number(_) => {None}
                }).collect();
            if headers_that_should_not_exist.len() != 0 {
                return Err(ParsingError::ValidationError(format!("headers was set to 'false' but in the assignments were headers found, where only numbers should be: '{:?}'", headers_that_should_not_exist)));
            }
        }
        Ok(())
    }
    pub fn check_assignments_from_sheet_info(&self, sheet_info: &SheetInfo) -> Result<(), ParsingError> {
        self.do_column_numbers_exist(sheet_info)?;
        self.do_headers_exist(sheet_info)?;
        Ok(())
    }
    pub fn check_transform_from_sheet_info(&self, sheet_info: &SheetInfo) -> Result<(), ParsingError> {
        // get all output-values of transform
        let output_values: Vec<&String> = sheet_info.transformations.as_ref().unwrap().output_values();
        let input_values : Vec<&HeaderValue> = sheet_info.transformations.as_ref().unwrap().input_values();
        // 1. special case: input == output (if input==output then input/output needs to be equivalent to a value in assignments -> not allowed

        // 2. do all numbers mentioned in transform exist in spreadsheet?
        let numbers_too_great: Vec<&u8> = input_values.iter().flat_map(|header|match header {
            HeaderValue::Name(name) => {None}
            HeaderValue::Number(number) => {if *number as usize >= self.width {
                Some(number)
            } else {
                None
            }}
        }).collect();
        if numbers_too_great.len() != 0 {
            return Err(ParsingError::ValidationError(format!("'transform' of sheet-nr '{:?}' has methods with input numbers that are greater than the number of columns/rows of the spreadsheet: '{:?}'", sheet_info.sheet_nr, numbers_too_great)));
        }
        // 3. do all headers mentioned in transform exist in spreadsheet, assignments or transform?
        let assignments: Vec<&String> = sheet_info.assignments.assignments_to_header_value.iter().map(|(value, header)|value).collect();
        let headers_not_existing: Vec<&String> = input_values.iter().flat_map(|header|match header {
            HeaderValue::Name(name) => {
                if output_values.contains(&name) {
                    None
                }
                else if assignments.contains(&name) {
                    None
                }
                else if self.headers.is_some() && self.headers.as_ref().unwrap().contains(name) {
                    None
                }
                else {Some(name)}
            }
            HeaderValue::Number(number) => {None}
        }).collect();
        if headers_not_existing.len() != 0 {
            return Err(ParsingError::ValidationError(format!("'transform' of sheet-nr '{:?}' has methods with input headers that don't exist in headers of the spreadsheet nor in assignments nor as other output-values of another transform-methods: '  '{:?}'", sheet_info.sheet_nr, headers_not_existing)));
        }
        Ok(())
    }
    pub fn modified_assignments(&self) -> HashMap<String, usize> {
        // 1. we only need string to vec-number: replace HeaderValue with vec-number
        // 2 .add headers to assignments, if they don't exist already in assignments
        let mut new_assignments:HashMap<String, usize> = HashMap::new();
        // 1
        for (name, header) in self.assignments.iter() {
            match header {
                HeaderValue::Name(header_value) => {
                   let header_nr = self.headers.as_ref().unwrap().iter().enumerate().find(|(nr, value)| value == &header_value).unwrap().0;
                    new_assignments.insert(name.to_owned(), header_nr);
                }
                HeaderValue::Number(number) => {
                    new_assignments.insert(name.to_owned(), usize::from(number.to_owned()));
                }
            }
        }
        // 2
        if self.headers.is_some() {
            let numbers: Vec<usize> = new_assignments.iter().map(|(name, vec_nr)| vec_nr.to_owned()).collect();
            for (nr, header) in self.headers.as_ref().unwrap().iter().enumerate() {
                if numbers.contains(&&nr){
                   continue
                }
                new_assignments.insert(header.to_owned(), nr);
            }
        }
        return new_assignments
    }
}
pub fn xlsx_data_sheets<P: AsRef<Path>>(transform_xlsx: &TransformXLSX, data_path: P) -> Result<Vec<DataSheet>, ParsingError> {
    let data_frames = match transform_xlsx.all_sheets {
        true => {
            import_all_ordered_df(data_path, &transform_xlsx.sheet_numbers, &transform_xlsx.worksheets)?
        }
        false => {
            import_some_ordered_df(data_path, &transform_xlsx.sheet_numbers, &transform_xlsx.worksheets)?
        }
    };
    Ok(data_frames)
}