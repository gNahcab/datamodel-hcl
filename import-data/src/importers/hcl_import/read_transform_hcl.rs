use std::collections::HashMap;
use std::path::Path;
use crate::errors::DataImportError;

pub enum OrganizedBy {
    ROWOrganized,
    COLOrganized,
}
pub enum RowOrResourceName {
    RowNr,
    ResourceName,
}
pub struct TransformHCL {
    worksheet_alias: Vec<String> ,
    organized_by: OrganizedBy,
    worksheetnr_to_resource_row: HashMap<usize, RowOrResourceName>,
    // it is row or col, depends if worksheet is organized by row or col
    worksheet_to_row_col_to_code: HashMap<usize, HashMap<usize, String>>
}

pub fn read_transform_hcl<P: AsRef<Path>>(path: P) -> Result<TransformHCL, DataImportError> {

    return Ok((TransformHCL{
        worksheet_alias: vec![],
        organized_by: OrganizedBy::ROWOrganized,
        worksheetnr_to_resource_row: Default::default(),
        worksheet_to_row_col_to_code: Default::default(),
    }))
}

#[cfg(test)]
mod test {

    #[test]
    fn test_read_transform_hcl() {

    }
}
