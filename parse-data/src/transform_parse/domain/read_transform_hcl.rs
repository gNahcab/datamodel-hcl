use std::any::Any;
use std::collections::HashMap;
use std::num::ParseIntError;
use hcl::{Block, BlockLabel, Body, Expression};
use hcl::format::Format;
use crate::errors::ParseError;
use crate::transform_parse::domain::assignment::{Assignments, AssignmentsWrapper};
use crate::transform_parse::domain::builders::transform_hcl;
use crate::transform_parse::domain::method::{Method, WrapperMethod};
use crate::transform_parse::domain::organized_by::OrganizedBy;
use crate::transform_parse::domain::sheet_info::{SheetInfo, SheetInfoWrapper};
use crate::transform_parse::domain::worksheet_info::WorksheetInfo;


#[derive(Debug)]
pub enum RowOrResourceName {
    RowNr,
    ResourceName,
}
#[derive(Debug)]
pub struct TransformHCL {
    worksheets: Vec<WorksheetInfo>,
}

impl TransformHCL {
    pub(crate) fn new(worksheets: Vec<WorksheetInfo>) -> Self {
        TransformHCL{
            worksheets
        }
    }
}

struct TransientStructureWorksheetInfo {
    label : usize,
    structured_by : Option<OrganizedBy>,
    resource: Option<String>,
    resource_row: Option<usize>,
    name_to_assignment: HashMap<String, String>,
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


struct TransientStructureAssignments {
    label: Option<usize>,
    name_to_assignment: HashMap<String, String>
}
struct TransientStructureTransformHCL {
    transform: Option<String>,
    all_sheets: Option<bool>,
    sheets: Vec<usize>,
    worksheets: HashMap<usize, TransientStructureWorksheetInfo>,
    methods: Vec<String>,
}


impl TransientStructureTransformHCL {
    fn new() -> TransientStructureTransformHCL {
        return TransientStructureTransformHCL{
            transform: None,
            all_sheets: None,
            sheets: vec![],
            worksheets: Default::default(),
            methods: vec![],
        }
    }
    pub(crate) fn add_sheets(&mut self, sheet_expression: &Expression) -> Result<(), ParseError> {
        if self.all_sheets.is_some() {
            return Err(ParseError::ValidationError(format!("only one declaration of 'sheets' allowed, second 'sheets' found with expression '{}'", sheet_expression)));
        }
        match sheet_expression {
            Expression::String(value) => {
                match value.as_str() { "all" => {
                    self.all_sheets = Option::from(true);
                }
                    _ => {
                        return Err(ParseError::ValidationError(format!("expression of 'sheets' is not allowed: '{}'", sheet_expression)));
                    }
                }
            }
            Expression::Array(vector) => {
                let number_vector : Vec<f64> = vec![];
                for expr in vector {
                    match expr {
                        Expression::Number(_) => {
                            let number_str = expr.to_string();
                            let number_usize: usize = number_str.parse()?;
                            self.sheets.push(number_usize);
                        }
                        _ => {
                            return Err(ParseError::ValidationError(format!("in 'sheets' array '{:?}' only numbers are allowed, but found: '{}'", vector, expr)));
                        }
                    }

                }
            }
            _ => {
                return Err(ParseError::ValidationError(format!("the type of expression of 'sheets' is not valid: '{:?}', only String or Array allowed", sheet_expression)));
            }
        }
        Ok(())
    }

    pub(crate) fn add_transform(&mut self, transform_expression: &Expression) -> Result<(), ParseError> {
        if self.all_sheets.is_some() {
            return Err(ParseError::ValidationError(format!("only one declaration of 'transform' allowed, second 'transform' found with expression '{}'", transform_expression)));
        }
        match transform_expression {
            Expression::String(value) => {
                match value.as_str() {
                    "xlsx" => {
                        self.transform = Option::from(value.to_string());
                    }
                    _ => {
                        return Err(ParseError::ValidationError(format!("expression of 'transform' is not allowed: '{}'", transform_expression)));
                    }
                }
            }
            _ => {
                return Err(ParseError::ValidationError(format!("the type of expression of '' is not valid: '{:?}', only String allowed", transform_expression)));
            }
        }
        Ok(())
    }
    pub(crate) fn add_sheet_info(&mut self, label: &str, body: &Body) -> Result<(), ParseError> {
        let sheet_info: SheetInfo = SheetInfoWrapper(body.to_owned()).to_sheet_info()?;
        self._add_sheet_info_to_worksheet_info(label, sheet_info)?;
        Ok(())
    }
    pub(crate) fn add_assignment(&mut self, label: &str, body: &Body) -> Result<(), ParseError> {
        let assignments: Assignments = AssignmentsWrapper(body.to_owned()).to_assignments()?;
        self._add_assignment_to_worksheet_info(label, assignments)?;
        Ok(())
    }
    pub(crate) fn add_method(&self, block: &Block) -> Result<(), ParseError>{
        let method: Method = WrapperMethod(block.to_owned()).to_method()?;
        self._add_methods_to_worksheet_info(method)?;
        Ok(())
    }
    fn _add_sheet_info_to_worksheet_info(&mut self, label: &str, sheet_info: SheetInfo) -> Result<(), ParseError> {
        let result:Result<usize, ParseIntError> = label.parse();
        let label = match result {
            Ok(value) => {value}
            Err(_) => {
                return Err(ParseError::ValidationError(format!("cannot parse label '{}' of 'sheet'", label)));
            }
        };
        match self.worksheets.get_mut(&label) {
            None => {
                self.worksheets.insert(label, TransientStructureWorksheetInfo{
                    label,
                    structured_by: Option::from(sheet_info.structured_by),
                    resource: sheet_info.resource,
                    resource_row: None,
                    name_to_assignment: Default::default(),
                });
            }
            Some(transient_structure) => {
                if transient_structure.structured_by.is_some() {
                    return Err(ParseError::ValidationError(format!("sheet '{:?}' contains two 'structured_by' but only one allowed", label)));
                }
                if transient_structure.resource.is_some() {
                    return Err(ParseError::ValidationError(format!("sheet '{:?}' contains two 'resource' but only one allowed", label)));
                }
                transient_structure.add_structured_by(Option::from(sheet_info.structured_by));
                transient_structure.add_resource(sheet_info.resource);
            }
        }
        Ok(())
    }
    fn _add_assignment_to_worksheet_info(&mut self, label: &str, assignments: Assignments) -> Result<(), ParseError> {
        let result:Result<usize, ParseIntError> = label.parse();
        let label = match result {
            Ok(value) => { value }
            Err(value) => {
                return Err(ParseError::ValidationError(format!("cannot parse label '{}' of 'sheet'", label)));
            }
        };
            match self.worksheets.get_mut(&label) {
                None => {
                    self.worksheets.insert(label, TransientStructureWorksheetInfo{
                        label: label,
                        structured_by: None,
                        resource: None,
                        resource_row: None,
                        name_to_assignment: assignments.name_to_assignments,
                    });
                }
                Some(transient_structure) => {
                    transient_structure.add_to_assignments(assignments.name_to_assignments);
                }
            }
            Ok(())
    }
    fn _add_methods_to_worksheet_info(&self, method: Method) -> Result<(), ParseError> {
        todo!()
    }
    pub(crate) fn is_complete(&self) -> Result<(), ParseError> {
        todo!()
    }
    pub(crate) fn as_worksheets(&self) -> Vec<WorksheetInfo> {
        todo!()
    }
}



impl TryFrom<hcl::Body> for TransformHCL {
    type Error = ParseError;
    fn try_from(body: hcl::Body) -> Result<Self, Self::Error> {
        let mut transient_transform_hcl = TransientStructureTransformHCL::new();

        let attributes: Vec<&hcl::Attribute> =  body.attributes().collect();
        for attribute in attributes {
            match attribute.key.as_str() {
                "sheets" => {
                transient_transform_hcl.add_sheets(attribute.expr())?;
                }
                "transform" => {
                    transient_transform_hcl.add_transform(attribute.expr())?;
                }
                _ => {
                    return Err(ParseError::ValidationError(format!("attribute '{}' with value '{}' not allowed", attribute.expr, attribute.key)));
                }
            }
        }
        let blocks: Vec<&Block> = body.blocks().collect();
        for block in blocks {
            match block.identifier.as_str() {
                "sheet" => {
                    if block.labels.len() == 0 {
                        return Err(ParseError::ValidationError(format!("assignments number- label is missing for 'assigments' : '{:?}'", body)));
                    }
                    if block.labels.len() > 1 {
                        return Err(ParseError::ValidationError(format!("assignments should only have one label, cannot parse 'assignments' : '{:?}'", body)));
                    }
                    let label = block.labels.get(0).unwrap().as_str();
                    transient_transform_hcl.add_sheet_info(label, &block.body)?;
                }
                "assignments" => {
                    if block.labels.len() == 0 {
                        return Err(ParseError::ValidationError(format!("assignments number- label is missing for 'assigments' : '{:?}'", body)));
                    }
                    if block.labels.len() > 1 {
                        return Err(ParseError::ValidationError(format!("assignments should only have one label, cannot parse 'assignments' : '{:?}'", body)));
                    }
                    let label = block.labels.get(0).unwrap().as_str();
                    transient_transform_hcl.add_assignment(label, &block.body)?;
                }
                "method" => {
                    transient_transform_hcl.add_method(block)?;
                }
                _ => {
                    return Err(ParseError::ValidationError(format!("the identifier of this block is not allowed '{}'", block.identifier)));
                }
            }
        }
        transient_transform_hcl.is_complete()?;
        Ok(TransformHCL::new(transient_transform_hcl.as_worksheets()))
    }
    }

#[cfg(test)]
mod test {
    use import_data::errors::DataImportError::PolarsError;
    use crate::errors::ParseError;
    use crate::transform_parse::domain::read_transform_hcl::TransformHCL;

    #[test]
    fn test_read_simple_transform_hcl() {
        let body = hcl::body!(
            transform = "xlsx"
            sheets = [1,2]
            sheet "1" {
                structured_by = "row"
                resource = "Person"
            }
            assignments "1"  {
                id = 0
                label = "new.a(£0,£1)"
                hasName = 2
                hasIdentifier = "replace.a($3)"
                hasChildren = 4
                hasExternalLink = 5
            }

            method "new" "a"{
                // lower the b-variable
                    function = "${a}_$lower({$b})"
                }
                // replace DICT with DICTIONARY, once per line and don't look for words but parts of words(no whitespaces between)
            method "replace" "a" {
                    function = ["DICT", "DICTIONARY"]
                }

        );
        let result: Result<TransformHCL, ParseError> = body.try_into();
        println!("{:?}", result);
        assert!(result.is_ok())
    }
    #[test]
    fn test_read_transform_complex() {
        /*
        let transform_hcl = hcl::block!(
            transform "xlsx" {
               sheets = 1,3
                sheet "1" {

                }
                sheet "3" {

                }
            }
        );
*/

    }
}
