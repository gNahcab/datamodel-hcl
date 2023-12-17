use std::collections::HashMap;
use crate::transform_parse::domain::sheet_info::SheetInfo;
#[derive(Debug)]
#[derive(PartialEq)]
pub enum TransformName {
    XLSX,
    CSV
}
#[derive(Debug)]
pub enum TransformType {
    XLSX(TransformXLSX),
    CSV(TransformCSV),

}

#[derive(Debug)]
pub struct TransformXLSX {
    pub(crate) worksheets: HashMap<usize, SheetInfo>
}
#[derive(Debug)]
pub struct TransformCSV {
    pub(crate) delimiter: char,
    pub(crate) sheet_info: SheetInfo,
}
