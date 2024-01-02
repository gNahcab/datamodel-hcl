use std::num::ParseIntError;
use hcl::{Attribute, Block, body, Body, Expression, Identifier};
use crate::errors::ParsingError;
use crate::expression_trait::ExpressionTransform;
use crate::transform_parse::domain::organized_by::OrganizedBy;
use crate::transform_parse::domain::assignment::{Assignments, AssignmentsWrapper};
use crate::transform_parse::domain::header_value::{HeaderValue, U8implementation};
use crate::transform_parse::domain::transformations::{Transformations, TransformationsWrapper};

#[derive(Debug)]
pub struct SheetInfoWrapper (pub(crate) Block);

#[derive(Debug)]
struct TransientStructureSheetInfo {
    sheet_nr: usize,
    structured_by: Option<String>,
    headers_exist: Option<bool>,
    resource: Option<String>,
    assignments: Option<Assignments>,
    transformations: Option<Transformations>,
}


impl SheetInfoWrapper {
    pub fn to_sheet_info(&self) -> Result<SheetInfo, ParsingError> {
        let label = self.0.labels.get(0).unwrap().as_str();
        let result: Result<usize, ParseIntError> =label.to_string().parse::<usize>();
        let sheet_nr = match result {
            Ok(value) => {value}
            Err(_) => {
                return Err(ParsingError::ValidationError(format!("couldn't parse '{:?}' to usize. This should be a valid number referring to a sheet.", label)));
            }
        };
        let mut transient_structure = TransientStructureSheetInfo::new(sheet_nr);
        let attributes: Vec<&Attribute> = self.0.body.attributes().collect();
        let blocks: Vec<&Block> = self.0.body.blocks().collect();
        for attribute in attributes {
            match attribute.key.as_str() {
                "structured_by" => {
                    transient_structure.add_structured_by(attribute.expr.to_string_2()?)?;
                },
                "resource" => {
                    transient_structure.add_resource(attribute.expr.to_string_2()?)?;
                }
                "headers" => {
                    transient_structure.add_headers_exist(attribute.expr.to_bool()?)?
                }
                _ => {
                    return Err(ParsingError::ValidationError(format!("attribute with name '{:?}' not allowed in a sheet", attribute.key)));
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
        sheet_info.is_consistent()?;
        Ok(sheet_info)
        }
}

impl TransientStructureSheetInfo {
    fn new(sheet_nr: usize) -> TransientStructureSheetInfo {
        TransientStructureSheetInfo {
            sheet_nr,
            structured_by: None,
            headers_exist: None,
            resource: None,
            assignments: None,
            transformations: None,
        }
    }
    pub(crate) fn add_headers_exist(&mut self, headers_exist: bool) -> Result<(), ParsingError> {
        if self.headers_exist.is_some() {
            return Err(ParsingError::ValidationError(format!("only one declaration of 'headers_exist' allowed, second 'headers_exist' found in '{:?}'", self)));
        }
        self.headers_exist = Option::from(headers_exist);
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
        if self.headers_exist.is_none() {
            return Err(ParsingError::ValidationError(format!("Sheet should contain 'headers_exist'-attribute but it doesn't: '{:?}'", self)));
        }
        if self.resource.is_none() {
            return Err(ParsingError::ValidationError(format!("Sheet should contain 'resource'-attribute but it doesn't: '{:?}'", self)));
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
    pub sheet_nr: usize,
    pub structured_by: OrganizedBy,
    pub headers_exist: bool,
    pub resource: String,
    pub assignments: Assignments,
    pub transformations: Option<Transformations>,
}

impl SheetInfo {
    fn new(transient_structure: TransientStructureSheetInfo) -> Result<SheetInfo, ParsingError> {
        let structured_by: OrganizedBy = OrganizedBy::organized_by(transient_structure.structured_by.unwrap().to_string())?;
        Ok(SheetInfo{
            sheet_nr: transient_structure.sheet_nr,
            structured_by,
            headers_exist: transient_structure.headers_exist.unwrap(),
            resource:transient_structure.resource.unwrap(),
            assignments: transient_structure.assignments.unwrap(),
            transformations: transient_structure.transformations,
        })
    }

    fn is_consistent(&self) -> Result<(), ParsingError> {
        // check transformation and assignments go along
        if self.transformations.is_some() {
            self.transformations.as_ref().unwrap().is_consistent(self.sheet_nr)?;
            // 1. output-value in transform should never match with new header(output) in assignments
            let output_assignments: Vec<&String> = self.assignments.assignments_to_header_value.iter().map(|(output, input)|output).collect();
            let output_transforms: Vec<&String> = self.transformations.as_ref().unwrap().output_values();
            let identical_values: Vec<&&String> = output_assignments.iter().filter(|a_output|output_transforms.contains(a_output)).collect();
            if identical_values.len() != 0 {
                return Err(ParsingError::ValidationError(format!("found in sheet-nr '{:?}' identical values in output-values of transform and assignments: '{:?}'.", self.sheet_nr, identical_values)));
            }
            // 2. input-values in assignments shouldn't be reused in transform as input-values
            let input_assignments: Vec<&HeaderValue> = self.assignments.assignments_to_header_value.iter().map(|(output, input)|input).collect();
            let input_transforms:Vec<&HeaderValue> = self.transformations.as_ref().unwrap().input_values();
            let identical_values: Vec<&&HeaderValue> = input_assignments.iter().filter(|header|input_transforms.contains(header)).collect();
            if identical_values.len() != 0 {
                return Err(ParsingError::ValidationError(format!("the input_values used in assignments shouldn't be reused in transform-methods. If a number is assigned to a value in assignments, use the assigned value in transform-methods. The following input-values were reused: {:?}", identical_values)));
            }
        }
        Ok(())
    }
}

