use std::collections::HashMap;
use hcl::{Attribute, Block, body, Body, Identifier};
use crate::errors::ParseError;
use crate::transform_parse::domain::organized_by::OrganizedBy;
use crate::datamodel_parse::remove_useless_quotation_marks;
use crate::transform_parse::domain::assignment::{Assignments, AssignmentsWrapper};
use crate::transform_parse::domain::transformations::{Transformations, TransformationsWrapper};

#[derive(Debug)]
pub struct SheetInfoWrapper (pub(crate) Body);
#[derive(Debug)]
struct TransientStructureSheetInfo {
    structured_by: Option<String>,
    resource: Option<String>,
    assignments: Option<Assignments>,
    transformations: Option<Transformations>,
}


impl SheetInfoWrapper {
    pub fn to_sheet_info(&self) -> Result<SheetInfo, ParseError> {
        let attributes: Vec<&Attribute> = self.0.attributes().collect();
        let blocks: Vec<&Block> = self.0.blocks().collect();
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
                    transient_structure.resource = Option::from(remove_useless_quotation_marks(attribute.expr.to_string()));
                }
                _ => {
                    return Err(ParseError::ValidationError(format!("attribute with name '{:?}' not allowed in 'sheet'", attribute.key)));
                }
            }
        }
        for block in blocks {
            match block.identifier.as_str() {
                 "assignments"=> {
                     let assignments: Assignments = AssignmentsWrapper(block.to_owned()).to_assignments()?;
                     transient_structure.add_assignments(assignments)?;
                 }
                "transformations"=> {
                    let transformations: Transformations = TransformationsWrapper(block.to_owned()).to_transformations()?;
                    transient_structure.add_transformations(transformations)?;
                }
                _ => {
                    return Err(ParseError::ValidationError(format!("block with name '{:?}' not allowed in 'sheet'",block.identifier)));
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
            assignments: None,
            transformations: None,
        }
    }

    pub(crate) fn add_assignments(&mut self, assignments: Assignments) -> Result<(), ParseError> {
        if self.assignments.is_some() {
            return Err(ParseError::ValidationError(format!("assignments is allowed only once, but found duplicate assignments: '{:?}'", self.assignments)));
        }
        self.assignments = Option::from(assignments);
        Ok(())
    }

    pub(crate) fn add_transformations(&mut self, transformations: Transformations) -> Result<(), ParseError> {
        if self.transformations.is_some() {
            return Err(ParseError::ValidationError(format!("transformations is allowed only once, but found duplicate transformations: '{:?}'", transformations)));
        }
        self.transformations = Option::from(transformations);
        Ok(())
    }

    pub(crate) fn is_complete(&self) -> Result<(), ParseError> {
        if self.structured_by.is_none() {
            return Err(ParseError::ValidationError(format!("Sheet should contain 'structured_by' but it doesn't: '{:?}'", self)));
        }
        if self.assignments.is_none() {
            return Err(ParseError::ValidationError(format!("Sheet should contain 'assignments' but it doesn't: '{:?}'", self)));
        }
        //transformations can be None
        Ok(())
    }
}

#[derive(Debug)]
pub struct SheetInfo {
    pub(crate) structured_by: OrganizedBy,
    pub(crate) resource: Option<String>,
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

