use hcl::{Attribute, Block, body, Body, Identifier};
use crate::errors::ParsingError;
use crate::to_2_string::To2String;
use crate::transform_parse::domain::organized_by::OrganizedBy;
use crate::transform_parse::domain::assignment::{Assignments, AssignmentsWrapper};
use crate::transform_parse::domain::transformations::{Transformations, TransformationsWrapper};

#[derive(Debug)]
pub struct SheetInfoWrapper (pub(crate) Body);
#[derive(Debug)]
struct TransientStructureSheetInfo {
    structured_by: Option<String>,
    resource: Option<String>,
    resource_row: Option<String>,
    assignments: Option<Assignments>,
    transformations: Option<Transformations>,
}


impl SheetInfoWrapper {
    pub fn to_sheet_info(&self) -> Result<SheetInfo, ParsingError> {
        let attributes: Vec<&Attribute> = self.0.attributes().collect();
        let blocks: Vec<&Block> = self.0.blocks().collect();
        let mut transient_structure = TransientStructureSheetInfo::new();
        for attribute in attributes {
            match attribute.key.as_str() {
                "structured_by" => {
                    transient_structure.add_structured_by(attribute.expr.to_string_2()?)?;
                },
                "resource" => {
                    transient_structure.add_resource(attribute.expr.to_string_2()?)?;
                }
                "resource_row" => {
                    transient_structure.add_resource_row(attribute.expr.to_string_2()?)?;
                }
                _ => {
                    return Err(ParsingError::ValidationError(format!("attribute with name '{:?}' not allowed in 'sheet'", attribute.key)));
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
                    return Err(ParsingError::ValidationError(format!("block with name '{:?}' not allowed in 'sheet'", block.identifier)));
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
            resource_row: None,
            assignments: None,
            transformations: None,
        }
    }
    pub(crate) fn add_resource_row(&mut self, resource_row_string: String) -> Result<(), ParsingError> {
        if self.resource_row.is_some() {
            return Err(ParsingError::ValidationError(format!("error in sheet: found duplicate for 'resource_row' in: '{:?}'", self)));
        }
        self.resource_row = Option::from(resource_row_string);
        Ok(())
    }

    pub(crate) fn add_resource(&mut self, resource_string: String) -> Result<(), ParsingError> {
        if self.resource.is_some() {
            return Err(ParsingError::ValidationError(format!("error in sheet: found duplicate for 'resource' in: '{:?}'", self)));
        }
        self.resource = Option::from(resource_string);
        Ok(())
    }
    pub(crate) fn add_structured_by(&mut self, structured_by_string: String) -> Result<(), ParsingError> {
        if self.structured_by.is_some() {
            return Err(ParsingError::ValidationError(format!("error in sheet: found duplicate for 'structured_by' in: '{:?}'", self)));
        }
        self.structured_by = Option::from(structured_by_string);
        Ok(())
    }

    pub(crate) fn add_assignments(&mut self, assignments: Assignments) -> Result<(), ParsingError> {
        if self.assignments.is_some() {
            return Err(ParsingError::ValidationError(format!("assignments is allowed only once, but found duplicate assignments: '{:?}'", self.assignments)));
        }
        self.assignments = Option::from(assignments);
        Ok(())
    }

    pub(crate) fn add_transformations(&mut self, transformations: Transformations) -> Result<(), ParsingError> {
        if self.transformations.is_some() {
            return Err(ParsingError::ValidationError(format!("transformations is allowed only once, but found duplicate transformations: '{:?}'", transformations)));
        }
        self.transformations = Option::from(transformations);
        Ok(())
    }

    pub(crate) fn is_complete(&self) -> Result<(), ParsingError> {
        if self.structured_by.is_none() {
            return Err(ParsingError::ValidationError(format!("Sheet should contain 'structured_by'-attribute but it doesn't: '{:?}'", self)));
        }
        if self.resource.is_none() && self.resource_row.is_none() {
            return Err(ParsingError::ValidationError(format!("Sheet should contain 'resource'-attribute or 'resource_row'-attribute but it doesn't: '{:?}'", self)));
        }
        if self.resource.is_some() && self.resource_row.is_some() {
            return Err(ParsingError::ValidationError(format!("Sheet should contain only 'resource'-attribute or 'resource_row'-attribute but it contains both: '{:?}'", self)));
        }
        if self.assignments.is_none() {
            return Err(ParsingError::ValidationError(format!("Sheet should contain 'assignments'-block but it doesn't: '{:?}'", self)));
        }
        //transformations can be None
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct SheetInfo {
    pub structured_by: OrganizedBy,
    pub(crate) resource: Option<String>,
    pub(crate) assignments: Assignments,
    pub(crate) transformations: Option<Transformations>,
}

impl SheetInfo {
    fn new(transient_structure: TransientStructureSheetInfo) -> Result<SheetInfo, ParsingError> {
        let structured_by: OrganizedBy = OrganizedBy::organized_by(transient_structure.structured_by.unwrap().to_string())?;
        Ok(SheetInfo{
            structured_by,
            resource:transient_structure.resource,
            assignments: transient_structure.assignments.unwrap(),
            transformations: transient_structure.transformations,
        })
    }
}

