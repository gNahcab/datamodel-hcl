use std::collections::HashMap;
use crate::errors::ParseError;
use crate::transform_parse::domain::builders::Builder;
use crate::transform_parse::domain::read_transform_hcl::{OrganizedBy, RowOrResourceName, TransformHCL};

pub struct TransformHCLBuilder {
    sheets: Vec<String>,
    sheets_info: Vec<WorksheetInfoBuilder>,

}

pub struct WorksheetInfoBuilder {
    organized_by: OrganizedBy,
    col_row_to_property:  HashMap<usize, String>,
    row_or_resource_name: RowOrResourceName,
    condition_to_code: HashMap<String, String>,
}

impl Builder for TransformHCLBuilder {
    type OutputType = TransformHCL;

    fn new() -> Self {
        TransformHCLBuilder{
            sheets: vec![],
            sheets_info: vec![],
        }
    }

    fn build(self) -> Result<TransformHCL, ParseError> {
        todo!()
    }

    fn add_sheets(&mut self, sheets: String) {
        //todo: turn into vec somehow
        println!("{:?}", sheets);
    }
}