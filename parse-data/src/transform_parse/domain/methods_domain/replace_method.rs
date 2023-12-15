use std::collections::HashMap;
use hcl::{Attribute, Expression};
use hcl::ser::Block;
use crate::errors::ParseError;
use crate::transform_parse::domain::header_value::{HeaderMethods, HeaderValue};
use crate::transform_parse::domain::methods_domain::wrapper_trait::Wrapper;

pub struct WrapperReplaceMethod(pub(crate) hcl::Block);

#[derive(Debug)]
struct TransientStructureReplaceMethod {
    output: String,
   input: Option<HeaderValue>,
    replace: Option<Vec<String>>,
    behavior: Option<String>,
    target: Option<String>,
}

impl TransientStructureReplaceMethod {
    fn new(output: String) -> TransientStructureReplaceMethod {
        TransientStructureReplaceMethod{
            output,
            input: None,
            replace: None,
            behavior: None,
            target: None,
        }
    }
    fn add_input(&mut self, expression: Expression) -> Result<(), ParseError> {
        if self.input.is_some() {
            return Err(ParseError::ValidationError(format!("found multiple input-attributes  in method '{:?}'.",self.output)));
        }
        let header_value = expression.to_header_value()?;
        self.input = Option::from(header_value);
        Ok(())
    }
    fn add_replace(&mut self, expression: Expression) -> Result<(), ParseError> {
        if self.replace.is_some() {
            return Err(ParseError::ValidationError(format!("found multiple replace-attributes  in method '{:?}'.",self.output)));
        }
        match expression {
            Expression::Array(array) => {
                let vec_string: Vec<String> = array.iter().map(|expr|expr.to_string()).collect();
                if vec_string.len() != 2 {
                    return Err(ParseError::ValidationError(format!("error in replace-method '{:?}'. 'replace' array doesn't have exactly two entries.", self)));
                }
                self.replace = Option::from(vec_string);
            }
            _ => {
                return Err(ParseError::ValidationError(format!("found 'replace'-value that is not an array in method '{:?}', found: {:?}.",self.output, expression)));
            }
        }
        Ok(())
    }
    fn add_condition(&mut self, block: hcl::Block) -> Result<(), ParseError> {
        block.no_blocks()?;
        for attribute in block.attributes() {
            match attribute.key.as_str() {
                "behavior" => {
                    if self.behavior.is_some() {
                        return Err(ParseError::ValidationError(format!("found multiple behavior-attributes  in method '{:?}'.",self.output)));
                    }
                    self.behavior = Option::from(attribute.expr.to_string());
                }
                "target" => {
                    if self.target.is_some() {
                        return Err(ParseError::ValidationError(format!("found multiple target-attributes  in method '{:?}'.",self.output)));
                    }
                    self.target = Option::from(attribute.expr.to_string());
                }
                _ => {
                    return Err(ParseError::ValidationError(format!("found 'condition'-attribute that is unknown in method '{:?}', found: {:?}.",self.output, attribute)));
                }
            }
        }
        Ok(())
    }
    fn is_consistent(&self) -> Result<(), ParseError> {
        if self.input.is_none() {
            return Err(ParseError::ValidationError(format!("replace-method '{:?}' doesn't have an input-attribute provided", self)));
        }
        if self.replace.is_none() {
            return Err(ParseError::ValidationError(format!("replace-method '{:?}' doesn't have a replace-attribute provided", self)));
        }
        if self.behavior.is_none() {
            return Err(ParseError::ValidationError(format!("replace-method '{:?}' doesn't have a behavior-attribute in 'condition'-block provided", self)));
        }
        if self.target.is_none() {
            return Err(ParseError::ValidationError(format!("replace-method '{:?}' doesn't have a target-attribute in 'condition'-block provided", self)));
        }

        Ok(())
    }
}
impl WrapperReplaceMethod{
    pub(crate) fn to_replace_method(&self) -> Result<ReplaceMethod, ParseError> {
        let mut transient_structure = TransientStructureReplaceMethod::new(self.0.get_output()?);
        for attribute in self.0.attributes() {
            match attribute.key.as_str() {
                "input" => {
                    transient_structure.add_input(attribute.expr.to_owned())?;
                }
                "replace" => {
                    transient_structure.add_replace(attribute.expr.to_owned())?;
                }
                _ => {
                    return Err(ParseError::ValidationError(format!("found this unknown attribute '{:?}' in method '{:?}'.",attribute, transient_structure.output)));
                }
            }
        }
        for block in self.0.blocks() {
            match block.identifier.as_str() {
                "condition" => {
                    transient_structure.add_condition(block.to_owned())?;
                }
                _ => {
                    return Err(ParseError::ValidationError(format!("found this unknown block '{:?}' in method '{:?}'.",block, transient_structure.output)));
                }
            }
        }

        transient_structure.is_consistent()?;


        Ok(ReplaceMethod{
            output: transient_structure.output,
            input: transient_structure.input.unwrap(),
            replace: transient_structure.replace.unwrap(),
            behavior: transient_structure.behavior.unwrap(),
            target: transient_structure.target.unwrap(),
        })
    }
}
#[derive(Debug)]
pub struct ReplaceMethod {
    output: String,
    input: HeaderValue,
    replace: Vec<String>,
    behavior: String,
    target: String,
}

#[cfg(test)]
mod test {
    use hcl::block;
    use crate::transform_parse::domain::methods_domain::replace_method::WrapperReplaceMethod;

    #[test]
    fn test_replace_method() {
        let block = block!(replace "replacement"{
            input = 3
            replace = ["Dict", "Dictionary"]
            condition {
                behavior = "lazy"
                target = "part" // target = "word"
            }
        });

        let result = WrapperReplaceMethod(block.to_owned()).to_replace_method();
        println!("{:?}", result);
        assert!(result.is_ok());
    }
}
