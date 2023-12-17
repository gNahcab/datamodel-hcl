use std::any::Any;
use std::collections::HashMap;
use crate::errors::ParseError;
use crate::transform_parse::domain::assignment::Assignments;
use crate::transform_parse::domain::organized_by::OrganizedBy;
use crate::transform_parse::domain::sheet_info::SheetInfo;
use crate::transform_parse::domain::transform_hcl::{TransformHCL, TransientStructureTransformHCL};
use crate::transform_parse::domain::transform_type::{TransformCSV, TransformName, TransformType, TransformXLSX};

pub struct TransformHCLBuilder {
    transform: TransformName,
    all_sheets: Option<bool>,
    sheets: Vec<usize>,
    worksheets: HashMap<usize, SheetInfo>,
}

impl TransformHCLBuilder {
    pub fn new(transient_transform_hcl: TransientStructureTransformHCL) -> TransformHCLBuilder {
        TransformHCLBuilder{
            transform: TransformName::CSV,
            all_sheets: None,
            sheets: vec![],
            worksheets: Default::default(),
        }
    }
    pub fn is_consistent(&self) {
        todo!()
        // check sheets
        // check worksheets
        // check csv-transform or xlsx-transform is correct
    }
    pub fn build(&self) -> Result<TransformHCL, ParseError> {
        match self.transform {
            TransformName::XLSX => {
                Ok(TransformHCL { transform_type: TransformType::CSV(TransformCSV { delimiter: ';', sheet_info: SheetInfo {
                    structured_by: OrganizedBy::ROWOrganized,
                    resource: None,
                    assignments: Assignments { assignments_to_header_value: Default::default() },
                    transformations: None,
                } }), transform_name: TransformName::XLSX })
            }
            TransformName::CSV => {
                Ok(TransformHCL { transform_type: TransformType::XLSX(TransformXLSX { worksheets: Default::default() }), transform_name: TransformName::CSV })
            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::any::Any;
    use crate::errors::ParseError;
    use crate::transform_parse::domain::transform_hcl::TransformHCL;
    use crate::transform_parse::domain::transform_type::{TransformName, TransformType, TransformXLSX};

    #[test]
    fn test_builder() {
        todo!()
        }
}
