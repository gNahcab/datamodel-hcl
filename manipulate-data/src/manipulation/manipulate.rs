use std::path::Path;
use parse_data::errors::ParsingError;
use parse_data::transform_parse::domain::transform_type::{TransformCSV, TransformXLSX};
use parse_data::xlsx_parse::data_sheet::xlsx_data_sheets;
use crate::manipulation::manipulated_data_sheet::ManipulatedDataSheet;
use crate::manipulation::xlsx_data_sheet::{check_consistency, manipulate_xlsx_data_sheets};

pub fn process_xlsx_data<P: AsRef<Path>>(transform_xlsx: TransformXLSX, data_path: P) -> Result<Vec<ManipulatedDataSheet>, ParsingError> {
    let data_sheets = xlsx_data_sheets(&transform_xlsx, data_path)?;
    check_consistency(&data_sheets, &transform_xlsx)?;
    let manipulated_data_sheets = manipulate_xlsx_data_sheets(data_sheets, &transform_xlsx)?;
    Ok(manipulated_data_sheets)
}

pub fn process_csv_data<P: AsRef<Path>>(transform_csv: TransformCSV, data_path: P) -> Result<Vec<ManipulatedDataSheet>, ParsingError> {
    todo!()
}


