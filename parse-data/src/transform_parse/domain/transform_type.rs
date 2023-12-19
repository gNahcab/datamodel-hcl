use std::collections::HashMap;
use crate::transform_parse::domain::organized_by::OrganizedBy;
use crate::transform_parse::domain::sheet_info::SheetInfo;

#[derive(Debug)]
pub enum TransformType {
    XLSX(TransformXLSX),
    CSV(TransformCSV),

}

#[derive(Debug)]
pub struct TransformXLSX {
    pub all_sheets: bool,
    pub sheet_numbers: Vec<usize>,
    pub organized_bys: Vec<OrganizedBy>,
    pub worksheets: Vec<SheetInfo>
}
#[derive(Debug)]
pub struct TransformCSV {
    pub(crate) delimiter: char,
    pub organized_by: OrganizedBy,
    pub(crate) sheet_info: SheetInfo,
}
