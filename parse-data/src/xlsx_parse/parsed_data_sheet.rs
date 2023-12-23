use std::collections::HashMap;
use std::path::Path;
use calamine::{DataType, Range};
use polars::export::num::ToPrimitive;
use crate::errors::ParsingError;
use crate::transform_parse::domain::header_value::HeaderValue;
use crate::transform_parse::domain::organized_by::OrganizedBy;
use crate::transform_parse::domain::sheet_info::SheetInfo;
use crate::transform_parse::domain::transform_type::TransformXLSX;
use crate::xlsx_parse::organize_structure::{import_all_ordered_df, import_some_ordered_df};
#[derive(Debug)]
pub struct ParsedDataSheet {
    pub tabular_data: Vec<Vec<String>>,
    pub height: usize,
    pub width: usize,
    pub headers: Option<Vec<String>>,
    pub assignments: HashMap<String, HeaderValue>,
}


impl ParsedDataSheet {
    pub fn new () -> ParsedDataSheet {
        ParsedDataSheet { tabular_data: vec![], height: 0, width: 0, headers: None, assignments: Default::default()}
    }
    pub fn copy(&self) -> ParsedDataSheet {
        ParsedDataSheet{
            tabular_data: self.tabular_data.clone(),
            height: self.height,
            width: self.width,
            headers: self.headers.clone(),
            assignments: self.assignments.clone(),
        }
    }
    pub fn add_height(&mut self, height: usize) {
        self.height = height;
    }
    pub fn add_width(&mut self, width: usize) {
        self.width = width;
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
            .filter(|number| number.to_usize().unwrap() > self.width).collect();
        if numbers_greater_than_width.len() != 0 {
            return Err(ParsingError::ValidationError(format!("Some column/row numbers in 'assignments' of sheet-nr '{:?}' are greater than the width of the spreadsheet: '{:?}'",sheet_info.sheet_nr, numbers_greater_than_width)));
        }
        Ok(())
    }
    fn do_headers_exist(&self, sheet_info: &SheetInfo) -> Result<(), ParsingError> {
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
    fn assignments_correct(&self, sheet_info: &SheetInfo)-> Result<(), ParsingError> {
        self.do_column_numbers_exist(sheet_info)?;
        if self.headers.is_some() {
            // if headers exist, check if strings can be assigned to headers as well
            self.do_headers_exist(sheet_info)?;
        }
        Ok(())
    }
    pub fn check_assignments_from_sheet_info(&self, sheet_info: &SheetInfo) -> Result<(), ParsingError> {
        self.assignments_correct(sheet_info)?;
        self.resource_row_correct(sheet_info)?;
        Ok(())
    }
    fn resource_row_correct(&self, sheet_info: &SheetInfo) -> Result<(), ParsingError> {
        if sheet_info.resource_row.is_some() {
            match sheet_info.resource_row.as_ref().unwrap() {
                HeaderValue::Name(name) => {
                    // self.headers should always be Some, never None here (checked previously)
                    if !self.headers.as_ref().unwrap().contains(name) {
                        return Err(ParsingError::ValidationError(format!("value of 'resource_row' '{:?}' in sheet-nr '{:?}' doesn't exist in associated spreadsheet",name, sheet_info.sheet_nr)))
                    }
                }
                HeaderValue::Number(number) => {
                   if self.width < *number as usize {
                       return Err(ParsingError::ValidationError(format!("number of 'resource_row' '{:?}' in sheet-nr '{:?}' doesn't exist in associated spreadsheet, because spreadsheet doesn't have that many columns/rows.",number, sheet_info.sheet_nr)))
                   }
                }
            }
        }
        Ok(())
    }
    pub fn check_transform_form_sheet_info(&self, sheet_info: &SheetInfo) -> Result<(), ParsingError> {
        // get all output-values of transform
        let output_values: Vec<&String> = sheet_info.transformations.as_ref().unwrap().output_values();
        let input_values : Vec<&HeaderValue> = sheet_info.transformations.as_ref().unwrap().input_values();
        // 1. special case: input == output (if input==output then input/output needs to value in assignments -> not allowed
        // 2. do all numbers mentioned in transform exist in spreadsheet?
        let numbers_too_great: Vec<&u8> = input_values.iter().flat_map(|header|match header {
            HeaderValue::Name(name) => {None}
            HeaderValue::Number(number) => {if *number as usize > self.width {
                Some(number)
            } else {
                None
            }}
        }).collect();
        if numbers_too_great.len() != 0 {
            return Err(ParsingError::ValidationError(format!("'transform' of sheet-nr '{:?}' has methods with input numbers that are greater than the number of columns/rows of the spreadsheet: '{:?}'", sheet_info.sheet_nr, numbers_too_great)));
        }
        // 3. do all headers mentioned in transform exist in spreadsheet, assignments or transform?
        let headers_not_existing: Vec<&u8> = input_values.iter().flat_map(|header|match header {
            HeaderValue::Name(name) => {None}
            HeaderValue::Number(number) => {if *number as usize > self.width {
                Some(number)
            } else {
                None
            }}
        }).collect();
        if headers_not_existing.len() != 0 {
            return Err(ParsingError::ValidationError(format!("'transform' of sheet-nr '{:?}' has methods with input headers that don't exist in headers nor as other output-values of another transform-methods: '  '{:?}'", sheet_info.sheet_nr, headers_not_existing)));
        }
        // 4. are there duplicates? Such that some Numbers are used in Transform but in Assignments
        // new header-values are assigned to that number, so the header-values should be used (when number was replaced by string, string should be used)
        // (this could also be checked earlier, because Spreadsheet is not needed for this)

        Ok(())
    }
}
pub fn xlsx_data_sheets<P: AsRef<Path>>(transform_xlsx: &TransformXLSX, data_path: P) -> Result<Vec<ParsedDataSheet>, ParsingError> {
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
pub fn data_sheet(worksheet: &(String, Range<DataType>), sheet_info: &SheetInfo) -> Result<ParsedDataSheet, ParsingError> {
    match sheet_info.structured_by {
        OrganizedBy::ROWOrganized => {
            datasheet_by_row(worksheet, sheet_info)
        }
        OrganizedBy::COLOrganized => {
            datasheet_by_col(worksheet, sheet_info)
        }
    }
}
fn datasheet_by_col(worksheet: &(String, Range<DataType>), sheet_info: &SheetInfo) -> Result<ParsedDataSheet, ParsingError> {
    // returns a datasheet that is reorganised by column, this is necessary because the importer imports the data by row
    let mut data_sheet: ParsedDataSheet = ParsedDataSheet::new();
    data_sheet.add_width(worksheet.1.height());
    data_sheet.add_height(worksheet.1.width());
    let mut start = 1;
    if sheet_info.headers_exist {
        let first_column: Vec<String> = worksheet.1.rows().map(|entry| entry.iter().take(1).map(|entry| entry.to_string()).collect()).collect();
        data_sheet.headers = Option::from(first_column);
        start = 2;
    }
    for i in start..worksheet.1.width() {
        let column: Vec<String> = worksheet.1.rows().map(|entry|entry.iter().take(i).map(|entry| entry.to_string()).collect()).collect();
        data_sheet.tabular_data.push(column);
    }
    Ok(data_sheet)
}

fn datasheet_by_row(worksheet: &(String, Range<DataType>), sheet_info: &SheetInfo) -> Result<ParsedDataSheet, ParsingError> {
    let mut data_sheet: ParsedDataSheet = ParsedDataSheet::new();
    data_sheet.add_width(worksheet.1.width());
    data_sheet.add_height(worksheet.1.height());
    let mut worksheet_iterator = worksheet.1.rows();
    if sheet_info.headers_exist == true {
        let first_row = worksheet_iterator.next();
        let first_row = match first_row {
            None => {
                return Err(ParsingError::XlsxParse(format!("couldn't take first row in worksheet: {:?}. Does worksheet have a first row?", worksheet.0)));
            }
            Some(row) => {row}
        };
        let row_vec: Vec<String> = first_row.iter().map(|entry|(entry.to_string())).collect();
        data_sheet.headers = Option::from(row_vec);
    }
    for row in worksheet_iterator {
        let row_vec: Vec<String> = row.iter().map(|entry|(entry.to_string())).collect();
        data_sheet.tabular_data.push(row_vec);
    }
    Ok(data_sheet)
}