use std::path::Path;
use calamine::{DataType, Range};
use hcl::Number;
use polars::export::num::ToPrimitive;
use crate::errors::ParsingError;
use crate::transform_parse::domain::header_value::HeaderValue;
use crate::transform_parse::domain::organized_by::OrganizedBy;
use crate::transform_parse::domain::sheet_info::SheetInfo;
use crate::transform_parse::domain::transform_type::TransformXLSX;
use crate::xlsx_parse::organize_structure::{import_all_ordered_df, import_some_ordered_df};
#[derive(Debug)]
pub struct DataSheet {
    pub tabular_data: Vec<Vec<String>>,
    pub height: usize,
    pub width: usize,
    pub headers: Option<Vec<String>>,
}

impl DataSheet {
}


impl DataSheet {
    pub fn new () -> DataSheet {
        DataSheet{ tabular_data: vec![], height: 0, width: 0, headers: None }
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
        let headers: Vec<&HeaderValue> = sheet_info.assignments.assignments_to_header_value.iter().map(|(new_header, current)| current).collect();
        let missing_headers: Vec<&String> = headers.iter().filter_map(|header_value|match header_value {
            HeaderValue::Name(name) => Some(name),
            _ => None
        }).filter(|name|self.headers.as_ref().unwrap().contains(name)).collect();
        if missing_headers.len() != 0 {
            return Err(ParsingError::ValidationError(format!("Some column/row headers in 'assignments' of sheet-nr '{:?}' cannot be found in the spreadsheet: '{:?}'",sheet_info.sheet_nr, missing_headers)));
        }
        Ok(())
    }
    pub fn add_assignment(&self, sheet_info: &SheetInfo) -> Result<(), ParsingError> {
        self.do_column_numbers_exist(sheet_info)?;
        if self.headers.is_some() {
            // if headers exist, check if strings can be assigned to headers as well
            self.do_headers_exist(sheet_info)?;
        }
        Ok(())
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
pub fn data_sheet(worksheet: &(String, Range<DataType>), sheet_info: &SheetInfo) -> Result<DataSheet, ParsingError> {
    match sheet_info.structured_by {
        OrganizedBy::ROWOrganized => {
            datasheet_by_row(worksheet, sheet_info)
        }
        OrganizedBy::COLOrganized => {
            datasheet_by_col(worksheet, sheet_info)
        }
    }
}
fn datasheet_by_col(worksheet: &(String, Range<DataType>), sheet_info: &SheetInfo) -> Result<DataSheet, ParsingError> {
    // returns a datasheet that is reorganised by column, this is necessary because the importer imports the data by row
    let mut data_sheet: DataSheet = DataSheet::new();
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

fn datasheet_by_row(worksheet: &(String, Range<DataType>), sheet_info: &SheetInfo) -> Result<DataSheet, ParsingError> {
    let mut data_sheet: DataSheet = DataSheet::new();
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