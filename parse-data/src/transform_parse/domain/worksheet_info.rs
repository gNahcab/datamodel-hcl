use std::collections::HashMap;
use crate::transform_parse::domain::organized_by::OrganizedBy;

#[derive(Debug)]
pub enum RowOrResourceName {
    RowNr,
    ResourceName,
}

#[derive(Debug)]
pub struct WorksheetInfo {
    organized_by: OrganizedBy,
    col_row_to_property:  HashMap<usize, String>,
    row_or_resource_name: RowOrResourceName,
    condition_to_code: HashMap<String, String>,

}
pub struct TransientStructureWorksheetInfo {
    pub(crate) label : usize,
    pub(crate) structured_by : Option<OrganizedBy>,
    pub(crate) resource: Option<String>,
    pub(crate) resource_row: Option<usize>,
    pub(crate) name_to_assignment: HashMap<String, String>,
}

impl TransientStructureWorksheetInfo {
    pub(crate) fn add_structured_by(&mut self, structured_by: Option<OrganizedBy>) {
        self.structured_by = structured_by;
    }
    pub(crate) fn add_resource(&mut self, resource: Option<String>) {
        self.resource = resource;
    }
    pub(crate) fn add_to_assignments(&mut self, name_to_assignments: HashMap<String, String>) {
        self.name_to_assignment.extend(name_to_assignments);
    }
}

