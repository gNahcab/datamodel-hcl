use std::any::Any;
use std::collections::HashMap;
use std::num::ParseIntError;
use std::string::ToString;
use hcl::{Block, Body, Expression};
use crate::errors::ParsingError;
use crate::expression_trait::ExpressionTransform;
use crate::transform_parse::domain::builders::transform_hcl::TransformHCLBuilder;
use crate::transform_parse::domain::organized_by::OrganizedBy;
use crate::transform_parse::domain::sheet_info::{SheetInfo, SheetInfoWrapper};
use crate::transform_parse::domain::transform_type::TransformType;

pub struct TransientStructureTransformHCL {
    pub(crate) transform: Option<String>,
    pub(crate) all_sheets: Option<bool>,
    pub(crate) sheets: Vec<usize>,
    pub(crate) organized_bys: Vec<OrganizedBy>,
    pub(crate) worksheets: Vec<SheetInfo>,
}


impl TransientStructureTransformHCL {
    fn new() -> TransientStructureTransformHCL {
        return TransientStructureTransformHCL{
            transform: None,
            all_sheets: None,
            sheets: vec![],
            organized_bys: vec![],
            worksheets: vec![],
        }
    }
    pub(crate) fn add_sheets(&mut self, sheet_expression: &Expression) -> Result<(), ParsingError> {
        if self.all_sheets.is_some() {
            return Err(ParsingError::ValidationError(format!("only one declaration of 'sheets' allowed, second 'sheets' found with expression '{}'", sheet_expression)));
        }
        match sheet_expression {
            Expression::String(value) => {
                match value.as_str() { "all" => {
                    self.all_sheets = Option::from(true);
                }
                    _ => {
                        return Err(ParsingError::ValidationError(format!("expression of 'sheets' is not allowed: '{}'", sheet_expression)));
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
                            return Err(ParsingError::ValidationError(format!("in 'sheets' array '{:?}' only numbers are allowed, but found: '{}'", vector, expr)));
                        }
                    }

                }
            }
            _ => {
                return Err(ParsingError::ValidationError(format!("the type of expression of 'sheets' is not valid: '{:?}', only String or Array allowed", sheet_expression)));
            }
        }
        Ok(())
    }

    pub(crate) fn add_transform(&mut self, transform_expression: &Expression) -> Result<(), ParsingError> {
        if self.all_sheets.is_some() {
            return Err(ParsingError::ValidationError(format!("only one declaration of 'transform' allowed, second 'transform' found with expression '{}'", transform_expression)));
        }
        self.transform = Option::from(transform_expression.to_string_2()?);
        Ok(())
    }
    pub(crate) fn add_sheet_info(&mut self, block: &Block) -> Result<(), ParsingError> {
        let sheet_info: SheetInfo = SheetInfoWrapper(block.to_owned()).to_sheet_info()?;
        self.organized_bys.push(sheet_info.structured_by);
        self.worksheets.push( sheet_info);
        Ok(())
    }
    pub(crate) fn is_complete(&self) -> Result<(), ParsingError> {
        //check if worksheet-info matches with "sheets"-number(which sheets were described vs which sheets should be checked)
        if self.sheets.is_empty() && self.all_sheets.is_none() {
            return Err(ParsingError::ValidationError(format!("'all_sheets'-attribute and 'sheets'-array aren't provided, one of them must be provided")));
        }
        if !self.sheets.is_empty() && self.all_sheets.is_some() {
            return Err(ParsingError::ValidationError(format!("'all_sheets'-attribute and 'sheets'-array are provided, but only one of them should be provided")));
        }
        if self.transform.is_none() {
            return Err(ParsingError::ValidationError(format!("'transform'-attribute and value weren't provided")));
        }

        //todo check if transform = "xlsx" matches with statements in worksheet-info (in case TransformHCL could  be used for Filemarker, SQL etc. as well, in such cases we could need some additional statements)
        Ok(())
    }

}


#[derive(Debug)]
pub struct TransformHCL {
    pub transform_type: TransformType,
}

impl TryFrom<hcl::Body> for TransformHCL {
    type Error = ParsingError;
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
                    return Err(ParsingError::ValidationError(format!("attribute '{}' with value '{}' not allowed", attribute.expr, attribute.key)));
                }
            }
        }

        let blocks: Vec<&Block> = body.blocks().collect();
        for block in blocks {
            match block.identifier.as_str() {
                "sheet" => {
                    if block.labels.len() == 0 {
                        return Err(ParsingError::ValidationError(format!("assignments number- label is missing for 'assigments' : '{:?}'", body)));
                    }
                    if block.labels.len() > 1 {
                        return Err(ParsingError::ValidationError(format!("assignments should only have one label, cannot parse 'assignments' : '{:?}'", body)));
                    }
                    transient_transform_hcl.add_sheet_info(&block)?;
                }
                _ => {
                    return Err(ParsingError::ValidationError(format!("the identifier of this block is not allowed '{}'", block.identifier)));
                }
            }
        }
        transient_transform_hcl.is_complete()?;
        let mut transform_hcl_builder : TransformHCLBuilder = TransformHCLBuilder::new(transient_transform_hcl);
        transform_hcl_builder.build()
    }
}

#[cfg(test)]
mod test {
    use crate::errors::ParsingError;
    use crate::transform_parse::domain::transform_hcl::TransformHCL;

    #[test]
    fn test_read_simple_transform_hcl() {
        let body = hcl::body!(
            transform = "xlsx"
            sheets = [1,2]
            sheet "2" {
                structured_by = "column"
                resource_row = 22
                headers = false
            assignments  {
                id = 5 // String = Header, wenn vorhanden
                not_lowered = 1
                hasName = 2
                hasChildren = 4
                hasExternalLink = 5
                }

                transformations {
                    lower "lower" {
                        input = "id"
                    }
                     combine "label"{
                            input = [0, 6]
                            separator = "_"
                            prefix = "BIZ_"
                            suffix = "_ZIP"
                    }
                 replace "hasIdentifier" {
                        input = 3
                        old = "DICT"
                        new = "Dictionary"
                        condition {
                         behavior = "lazy"
                            target = "part"
                        }
                    }
                to_date "hasDate" {
                  input = 6
                  calendar_type= "Gregorian"
                  pattern "1" {
                    // e.g. would catch 1.12 - 23.12.1991
                    // e.g. would catch 1 Dez - 23 Dezember 1991
                    first {
                                day = 1
                                month = 2
                            }
                    date {
                                day = 1
                                month = 2
                                year = 3
                            }
                  }
    }

                }
            }
            sheet "1" {
                structured_by = "row"
                headers = true
                resource = "Person" //TODO wie wenn Ressource nur in einer Spalte oder Zeile festgeschrieben ist und für jede Spalte bzw. Zeile  ändert?
            assignments  {
                id = "ID" // String = Header, wenn vorhanden
                not_lowered = 1
                hasName = 2
                hasChildren = 4
                hasExternalLink = 5
                }

                transformations {
                    lower "lower" {
                        input = "not_lower"
                    }
                     combine "label"{
                            input = [0, "lower"]//"{$0}{$lower}"
                            separator = "_"
                            prefix = "BIZ_"
                            suffix = "_ZIP"
                    }
                 replace "hasIdentifier" {
                        input = 3
                        old = "DICT"
                        new = "Dictionary"
                        condition {
                         behavior = "lazy"
                            target = "part"
                        }
                    }

                }
            }

                // replace DICT with DICTIONARY, once per line and don't look for words but parts of words(no whitespaces between)

        );
        let result: Result<TransformHCL, ParsingError> = body.try_into();
        println!("{:?}", result);
        assert!(result.is_ok())

    }
}
