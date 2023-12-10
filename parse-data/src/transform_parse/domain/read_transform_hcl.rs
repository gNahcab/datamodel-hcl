use std::any::Any;
use std::collections::HashMap;
use hcl::{Attribute, Block, BlockLabel, Body, Expression, Identifier, to_vec};
use hcl::format::Format;
use crate::errors::ParseError;
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


struct TransientStructureAssignments {
    label: Option<usize>,
    name_to_assignment: HashMap<String, String>
}
struct TransientStructureTransformHCL {
    all_sheets: Option<bool>,
    sheets: Vec<usize>,
    worksheets: HashMap<usize, TransientStructureWorksheetInfo>,
    methods: Vec<String>,
}
impl TransientStructureTransformHCL {
    fn new() -> TransientStructureTransformHCL {
        return TransientStructureTransformHCL{
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
    pub(crate) fn add_sheet_info(&self, labels: &Vec<BlockLabel>, body: &Body) -> Result<(), ParseError> {
        if labels.len() == 0 {
            return Err(ParseError::ValidationError(format!("sheet number- label is missing for 'sheet' : '{:?}'", body)));
        }
        if labels.len() > 1 {
            return Err(ParseError::ValidationError(format!("sheet should only have one label, cannot parse 'sheet' : '{:?}'", labels)));
        }
        let label = labels.get(0).unwrap().as_str();
        let sheet_info: SheetInfo = SheetInfoWrapper(body.to_owned()).to_sheet_info()?;
        self._add_sheet_info_to_worksheet_info(label, sheet_info)?;
        Ok(())
    }
    pub(crate) fn add_assignment(&self, labels: &Vec<BlockLabel>, body: &Body) -> Result<(), ParseError> {
        if labels.len() == 0 {
            return Err(ParseError::ValidationError(format!("assignments number- label is missing for 'assigments' : '{:?}'", body)));
        }
        if labels.len() > 1 {
            return Err(ParseError::ValidationError(format!("assignments should only have one label, cannot parse 'assignments' : '{:?}'", labels)));
        }
        let label = labels.get(0).unwrap().as_str();
        //let assignments &Assignments = as_assignments()?;
        //self._add_assignment_to_worksheet_info(label, assignments)?;
        Ok(())
    }
    pub(crate) fn add_methods(&self, labels: &Vec<BlockLabel>, body: &Body) {
        todo!()
    }
    fn _add_sheet_info_to_worksheet_info(&self, label: &str, sheet_info: SheetInfo) -> Result<(), ParseError> {
        todo!()
    }
    fn _add_assignment_to_worksheet_info(&self, label: &str, assignments: &TransientStructureAssignments) -> Result<(), ParseError> {
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
                _ => {
                    return Err(ParseError::ValidationError(format!("attribute '{}' with value '{}' not allowed", attribute.expr, attribute.key)));
                }
            }
        }

        let blocks: Vec<&Block> = body.blocks().collect();
        for block in blocks {
            match block.identifier.as_str() {
                "sheet" => {
                    transient_transform_hcl.add_sheet_info(&block.labels, &block.body)?;
                }
                "assignments" => {
                   transient_transform_hcl.add_assignment(&block.labels, &block.body);
                }
                "methods" => {
                    transient_transform_hcl.add_methods(&block.labels, &block.body);
                }
                _ => {
                    return Err(ParseError::ValidationError(format!("the identifier of this block is not allowed '{}'", block.identifier)));
                }
            }
        }
        Ok(TransformHCL{ worksheets: vec![] })
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
            sheets = [1,2]
            sheet "1" {
                structured_by = "row"
                resource = "Person"
            }
            assignments "1"  {
                id = 0
                label = "methods.add(£0,£1)"
                hasName = 2
                hasIdentifier = "methods.replace.a($3)"
                hasChildren = 4
                hasExternalLink = 5
            }

            methods {
                // lower the b-variable
                add = "£a_£lower(£b)"
                // replace DICT with DICTIONARY, once per line and don't look for words but parts of words(no whitespaces between)
                replace "a" {
                    repl = ["DICT", "DICTIONARY"]
                    }
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
