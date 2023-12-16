use std::collections::HashMap;
use crate::transform_parse::domain::sheet_info::SheetInfo;

enum TransformType {
    XLSX(TransformXLSX),
    CSV(TransformCSV),

}

struct TransformXLSX {
    worksheets: HashMap<usize, SheetInfo>
}

struct TransformCSV {
    delimiter: char,
    sheet_info: SheetInfo,
}