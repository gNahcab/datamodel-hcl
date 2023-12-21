use std::path::Path;
use calamine::{DataType, Range};
use crate::errors::ParsingError;
use crate::transform_parse::domain::sheet_info::SheetInfo;
use crate::xlsx_parse::data_sheet::{data_sheet, DataSheet};

pub fn import_some_ordered_df<P: AsRef<Path>>(data_path: P, sheet_numbers: &Vec<usize>, sheet_infos: &Vec<SheetInfo>) -> Result<Vec<DataSheet>, ParsingError> {
    // import only those sheets that are mentioned in transform-hcl
    let worksheets: Vec<(String, Range<DataType>)> = import_data::operations::load_excel_worksheets(data_path)?;
    let mut all_datasheets:Vec<DataSheet> = vec![];
    let mut nr_ : usize = 0;
    for (i, worksheet) in worksheets.iter().enumerate() {
        if !sheet_numbers.contains(&(i + 1)) {continue;}
        let datasheet = data_sheet(worksheet, sheet_infos.get(nr_).unwrap())?;
        nr_ += 1;
        all_datasheets.push(datasheet);
    }
    Ok(all_datasheets)
}

pub fn import_all_ordered_df<P: AsRef<Path>>(data_path: P, sheet_numbers: &Vec<usize>, sheet_infos: &Vec<SheetInfo>) -> Result<Vec<DataSheet>, ParsingError> {
    // import all sheets because transform_hcl tells us to import all sheets (sheets = "all")
    let worksheets = import_data::operations::load_excel_worksheets(data_path)?;
    if worksheets.iter().len() != sheet_numbers.iter().len() {
        return Err(ParsingError::ValidationError(format!("all worksheets should be processed but not all worksheets are described, found worksheets in xlsx: '{:?}', worksheets described: '{:?}'",worksheets.iter().len(), sheet_numbers)));
    }
    let mut all_datasheets:Vec<DataSheet> = vec![];
    for (nr_, worksheet) in worksheets.iter().enumerate() {
        let datasheet = data_sheet(worksheet, sheet_infos.get(nr_).unwrap())?;
        all_datasheets.push(datasheet);
    }
    Ok(all_datasheets)
}


