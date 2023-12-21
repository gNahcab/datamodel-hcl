use std::path::Path;
use parse_data::errors::ParsingError;
use parse_data::transform_parse::domain::transform_hcl::TransformHCL;
use parse_data::transform_parse::domain::transform_type::{TransformCSV, TransformType, TransformXLSX};
use parse_data::xlsx_parse::data_sheet::{DataSheet, xlsx_data_sheets};
use crate::manipulation::assignments::add_assignments_xlsx;

pub fn manipulate_xlsx_data<P: AsRef<Path>>(transform_xlsx: TransformXLSX, data_path: P) -> Result<Vec<DataSheet>, ParsingError> {
    let data_sheets = xlsx_data_sheets(&transform_xlsx, data_path)?;
    let data_sheets = add_assignments_xlsx(data_sheets, &transform_xlsx);
    let vec_temp: Vec<DataSheet> = vec![];
    Ok(vec_temp)
}

pub fn manipulate_csv_data<P: AsRef<Path>>(transform_csv: TransformCSV, data_path: P) -> Result<Vec<DataSheet>, ParsingError> {
    todo!()
}



pub(crate) fn manipulate_with_methods(data_sheets: Vec<DataSheet>, transform: &TransformHCL) -> Result<Vec<DataSheet>, ParsingError> {
    todo!()
}