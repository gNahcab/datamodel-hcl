use std::path::Path;
use calamine::{DataType, Range};
use crate::errors::ParsingError;
use crate::transform_parse::domain::organized_by::OrganizedBy;
use crate::transform_parse::domain::sheet_info::SheetInfo;
use crate::transform_parse::domain::transform_type::TransformXLSX;
use crate::xlsx_parse::organize_structure::{import_all_ordered_df, import_some_ordered_df};

pub struct DataSheet {
    tabular_data: Vec<Vec<String>>,
    pub headers: Option<Vec<String>>,
}

impl DataSheet {
    fn new () -> DataSheet {
        DataSheet{ tabular_data: vec![], headers: None }
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