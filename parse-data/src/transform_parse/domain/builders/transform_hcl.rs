use std::collections::HashMap;
use hcl::Expression;
use crate::errors::ParseError;
use crate::transform_parse::domain::builders::{TransformBuilderTrait, WorksheetBuilderTrait};
use crate::transform_parse::domain::read_transform_hcl::{OrganizedBy, RowOrResourceName, TransformHCL, WorksheetInfo};

pub struct TransformHCLBuilder {
    all_sheets: bool,
    sheets: Vec<String>,
    sheets_info: Vec<WorksheetInfoBuilder>,
}

pub struct WorksheetInfoBuilder {
    organized_by: OrganizedBy,
    col_row_to_property:  HashMap<usize, String>,
    row_or_resource_name: RowOrResourceName,
    condition_to_code: HashMap<String, String>,
}

impl WorksheetBuilderTrait for WorksheetInfoBuilder {
    type OutputType = WorksheetInfo;
    fn new() -> Self {
        todo!()
    }

    fn build(self) -> Result<WorksheetInfo, ParseError> {
        todo!()
    }
}

impl TransformBuilderTrait for TransformHCLBuilder {
    type OutputType = TransformHCL;

    fn new() -> Self {
        TransformHCLBuilder{
            all_sheets: false,
            sheets: vec![],
            sheets_info: vec![],
        }
    }

    fn build(self) -> Result<TransformHCL, ParseError> {
        // build if sheets and all_sheets match
        // if sheets and sheet_info match
        todo!()
    }

    fn add_sheets(&mut self, sheets: Vec<String>) {
        self.sheets = sheets;
    }
}