use std::collections::HashMap;
use std::num::ParseIntError;
use hcl::{Block, Body, Expression};
use crate::errors::ParseError;
use crate::transform_parse::domain::sheet_info::{SheetInfo, SheetInfoWrapper};

pub struct TransientStructureTransformHCL {
    transform: Option<String>,
    all_sheets: Option<bool>,
    sheets: Vec<usize>,
    worksheets: HashMap<usize, SheetInfo>,
}


impl TransientStructureTransformHCL {
    fn new() -> TransientStructureTransformHCL {
        return TransientStructureTransformHCL{
            transform: None,
            all_sheets: None,
            sheets: vec![],
            worksheets: Default::default(),
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
        self.transform = Option::from(transform_expression.to_string());
        Ok(())
    }
    pub(crate) fn add_sheet_info(&mut self, label: &str, body: &Body) -> Result<(), ParseError> {
        let result: Result<usize, ParseIntError> =label.to_string().parse::<usize>();
        let sheet_nr = match result {
            Ok(value) => {value}
            Err(_) => {
                return Err(ParseError::ValidationError(format!("couldn't parse '{:?}' to usize. This should be a valid number referring to a sheet.",label)));
            }
        };
        let sheet_info: SheetInfo = SheetInfoWrapper(body.to_owned()).to_sheet_info()?;
        if self.worksheets.get(&sheet_nr).is_some() {
            return Err(ParseError::ValidationError(format!("the same sheet number was provided multiple times: sheet-nr: {:?}", sheet_nr)));
        }
        self.worksheets.insert(sheet_nr, sheet_info);
        Ok(())
    }
    pub(crate) fn is_consistent(&self) -> Result<(), ParseError> {
        //check if worksheet-info matches with "sheets"-number(which sheets were described vs which sheets should be checked)
        if self.sheets.is_empty() && self.all_sheets.is_none() {
            return Err(ParseError::ValidationError(format!("'all_sheets'-attribute and 'sheets'-array aren't provided, one of them must be provided")));
        }
        if !self.sheets.is_empty() && self.all_sheets.is_some() {
            return Err(ParseError::ValidationError(format!("'all_sheets'-attribute and 'sheets'-array are provided, but only one of them should be provided")));
        }
        if !self.sheets.is_empty() {
            let worksheet_numbers: Vec<&usize> = self.worksheets.iter().map(|(number, _)|number).collect();

            let not_existing: Vec<&&usize> = worksheet_numbers.iter().filter(|number| !self.sheets.contains(number)).collect();
            if !not_existing.is_empty() {
                return Err(ParseError::ValidationError(format!("provided more described worksheets than sheet-numbers provided, described: {:?}, sheet-numbers: {:?}", worksheet_numbers, self.sheets)));
            }
            let not_existing: Vec<&usize> = self.sheets.iter().filter(|number| !worksheet_numbers.contains(number)).collect();
            if !not_existing.is_empty() {
                return Err(ParseError::ValidationError(format!("provided less described worksheets than sheet-numbers provided, described: {:?}, sheet-numbers: {:?}", worksheet_numbers, self.sheets)));
            }
        }
        if self.transform.is_none() {
            return Err(ParseError::ValidationError(format!("'transform'-attribute and value wasn't provided")));
        }

        //todo check if transform = "xlsx" matches with statements in worksheet-info (in case TransformHCL could  be used for Filemarker, SQL etc. as well, in such cases we could need some additional statements)
        Ok(())
    }

}


#[derive(Debug)]
pub struct TransformHCL {
    //TODO replace this with TransformType: either return CSV or XLSX - related worksheet
    worksheets: HashMap<usize, SheetInfo>,
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
                _ => {
                    return Err(ParseError::ValidationError(format!("the identifier of this block is not allowed '{}'", block.identifier)));
                }
            }
        }
        transient_transform_hcl.is_consistent()?;
        Ok(TransformHCL::new(transient_transform_hcl))

    }
}
impl TransformHCL {
    pub(crate) fn new(transient_structure: TransientStructureTransformHCL) -> Self {
        TransformHCL{ worksheets: transient_structure.worksheets}
    }
}
#[cfg(test)]
mod test {
    use crate::errors::ParseError;
    use crate::transform_parse::domain::transform_hcl::TransformHCL;

    #[test]
    fn test_read_simple_transform_hcl() {
        let body = hcl::body!(
            transform = "xlsx"
            sheets = [1]
            sheet "1" {
                structured_by = "row"
                resource = "Person" //TODO wie wenn Ressource nur in einer Spalte oder Zeile festgeschrieben ist und für jede Spalte bzw. Zeile  ändert?
            assignments  {
                id = "ID" // String = Header, wenn vorhanden
                not_lowered = 1
                hasName = 2
                hasIdentifier = 3
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
                        input = 1
                        replace = ["DICT", "DICTIONARY"]
                        condition {
                         behavior = "lazy"
                            target = "part"
                        }
                    }

                }
            }

                // replace DICT with DICTIONARY, once per line and don't look for words but parts of words(no whitespaces between)

        );
        let result: Result<TransformHCL, ParseError> = body.try_into();
        println!("{:?}", result);
        assert!(result.is_ok())

    }
}
