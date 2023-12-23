use std::path::Path;
use parse_data::errors::ParsingError;
use parse_data::transform_parse::domain::transform_hcl::TransformHCL;
use parse_data::transform_parse::domain::transform_type::{TransformCSV, TransformXLSX};
use parse_data::xlsx_parse::parsed_data_sheet::{ParsedDataSheet, xlsx_data_sheets};
use crate::manipulation::xlsx_data_sheet::{check_consistency, manipulate_xlsx_data_sheets};

pub fn manipulate_xlsx_data<P: AsRef<Path>>(transform_xlsx: TransformXLSX, data_path: P) -> Result<Vec<ParsedDataSheet>, ParsingError> {
    let data_sheets = xlsx_data_sheets(&transform_xlsx, data_path)?;
    check_consistency(&data_sheets, &transform_xlsx)?;
    let manipulated_data_sheets: () = manipulate_xlsx_data_sheets(data_sheets, &transform_xlsx)?;
    let vec_temp: Vec<ParsedDataSheet> = vec![];
    Ok(vec_temp)
}

pub fn manipulate_csv_data<P: AsRef<Path>>(transform_csv: TransformCSV, data_path: P) -> Result<Vec<ParsedDataSheet>, ParsingError> {
    todo!()
}



pub(crate) fn manipulate_with_methods(data_sheets: Vec<ParsedDataSheet>, transform: &TransformHCL) -> Result<Vec<ParsedDataSheet>, ParsingError> {
    todo!()
}