use std::collections::HashMap;
use crate::transform_parse::domain::organized_by::OrganizedBy;
use crate::transform_parse::domain::read_transform_hcl::RowOrResourceName;

#[derive(Debug)]
pub struct WorksheetInfo {
    organized_by: OrganizedBy,
    col_row_to_property:  HashMap<usize, String>,
    row_or_resource_name: RowOrResourceName,
    condition_to_code: HashMap<String, String>,

}
