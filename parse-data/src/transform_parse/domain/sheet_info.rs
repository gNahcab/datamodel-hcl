use hcl::{Attribute, body, Body};
use crate::errors::ParseError;
use crate::transform_parse::domain::read_transform_hcl::OrganizedBy;
use crate::datamodel_parse::remove_useless_quotation_marks;

#[derive(Debug)]
pub struct SheetInfoWrapper (pub(crate) Body);
#[derive(Debug)]
struct TransientStructureSheetInfo {
    structured_by: Option<String>,
    resource: Option<String>,
}

impl SheetInfoWrapper {
    pub fn to_sheet_info(&self) -> Result<SheetInfo, ParseError> {
        let attributes: Vec<&Attribute> = self.0.attributes().collect();
        let mut transient_structure = TransientStructureSheetInfo::new();
        for attribute in attributes {
            match attribute.key.as_str() {
                "structured_by" => {
                    if transient_structure.structured_by.is_some() {
                        return Err(ParseError::ValidationError(format!("error in sheet: found duplicate for 'structured_by' in: '{:?}'", self)));
                    }
                    transient_structure.structured_by = Option::from(remove_useless_quotation_marks(attribute.expr.to_string()));
                },
                "resource" => {
                    if transient_structure.resource.is_some() {
                        return Err(ParseError::ValidationError(format!("error in sheet: found duplicate for 'resource' in: '{:?}'", self)));
                    }
                    transient_structure.resource = Option::from(attribute.expr.to_string());
                }
                _ => {
                    return Err(ParseError::ValidationError(format!("keyword '{:?}' not allowed in 'sheet'", attribute.key)));
                }
            }
        }
        transient_structure.is_complete()?;
        let sheet_info = SheetInfo::new(transient_structure)?;
        Ok(sheet_info)
        }
}

impl TransientStructureSheetInfo {
    fn new() -> TransientStructureSheetInfo {
        TransientStructureSheetInfo {
            structured_by: None,
            resource: None,
        }
    }
    pub(crate) fn is_complete(&self) -> Result<(), ParseError> {
        if self.structured_by.is_none() {
            return Err(ParseError::ValidationError(format!("Sheet should contain 'structured_by' but it doesn't: '{:?}'", self)));
        }
        Ok(())
    }
}

pub struct SheetInfo {
    structured_by: OrganizedBy,
    resource: Option<String>,
}

impl SheetInfo {
    fn new(transient_structure: TransientStructureSheetInfo) -> Result<SheetInfo, ParseError> {
        let structured_by: OrganizedBy = OrganizedBy::from_str(transient_structure.structured_by.unwrap().to_string())?;
        Ok(SheetInfo{
            structured_by,
            resource:transient_structure.resource
        })
    }
}

