use hcl::{Attribute, Block, Expression};
use crate::errors::ParseError;
use crate::transform_parse::domain::header_value::{HeaderMethods, HeaderValue};
use crate::transform_parse::domain::methods_domain::wrapper_trait::Wrapper;


pub struct WrapperCombineMethod (pub(crate) Block);
#[derive(Debug)]
struct TransientStructureCombineMethod {
    input: Option<Vec<HeaderValue>>,
    output: String,
    separator: Option<String>,
    prefix: Option<String>,
    suffix: Option<String>,
}

impl TransientStructureCombineMethod {
    fn new(output: String) -> TransientStructureCombineMethod {
        TransientStructureCombineMethod{
            input: None,
            output,
            separator: None,
            prefix: None,
            suffix: None,
        }
    }
    pub(crate) fn add_input(&mut self, input: Expression) -> Result<(), ParseError> {
        if self.input.is_some() {
            return Err(ParseError::ValidationError(format!("method: '{:?}' has multiple input-attributes", self)));
        }
        match input {
            Expression::Array(array) => {
                let str_vec:Vec<HeaderValue> = array.iter().map(|expr|expr.to_header_value().unwrap()).collect();

                if str_vec.len() != 2 {
                    return Err(ParseError::ValidationError(format!("error in combine-method '{:?}'. Input-attributes array doesn't have exactly two entries.", self)));
                }
                self.input = Option::from(str_vec);
            }
            _ => {
                return Err(ParseError::ValidationError(format!("combine-methods: '{:?}' input-attribute is not an array", self)));
            }
        }
        Ok(())
    }
    pub(crate) fn add_separator(&mut self, separator: String) -> Result<(), ParseError>{
        if self.separator.is_some() {
            return Err(ParseError::ValidationError(format!("method: '{:?}' has multiple separator-attributes", self)));
        }
        self.separator = Option::from(separator);
        Ok(())
    }
    pub(crate) fn add_prefix(&mut self, prefix: String) -> Result<(), ParseError>{
        if self.prefix.is_some() {
            return Err(ParseError::ValidationError(format!("method: '{:?}' has multiple prefix-attributes", self)));
        }
        self.prefix = Option::from(prefix);
        Ok(())
    }
    pub(crate) fn add_suffix(&mut self, suffix: String) -> Result<(), ParseError>{
        if self.suffix.is_some() {
            return Err(ParseError::ValidationError(format!("method: '{:?}' has multiple suffix-attributes", self)));
        }
        self.suffix = Option::from(suffix);
        Ok(())
    }

    pub(crate) fn is_consistent(&self) -> Result<(), ParseError> {
        if self.input.is_none() {
            return Err(ParseError::ValidationError(format!("combine-method: '{:?}' doesn't have an input-attribute provided", self)));
        }
        if self.separator.is_none() {
            return Err(ParseError::ValidationError(format!("combine-method: '{:?}' doesn't have a separator provided", self)));
        }
        // suffix, prefix are optional
        Ok(())
    }
}


impl WrapperCombineMethod {

    pub(crate) fn to_combine_method(&self) -> Result<CombineMethod, ParseError> {
        let mut transient_structure = TransientStructureCombineMethod::new(self.0.get_output()?);
        self.0.no_blocks()?;
        for attribute in self.0.attributes() {
            match attribute.key.as_str() {
                "input" => {
                    transient_structure.add_input(attribute.expr.to_owned())?;
                }
                "separator" => {
                    transient_structure.add_separator(attribute.expr.to_string())?;
                }
                "prefix" => {
                    transient_structure.add_prefix(attribute.expr.to_string())?;
                }
                "suffix" => {
                    transient_structure.add_suffix(attribute.expr.to_string())?;
                }
                _ => {
                    return Err(ParseError::ValidationError(format!("found this unknown attribute '{:?}' in method '{:?}'.",attribute, transient_structure.output)));
                }
            }

        }
        transient_structure.is_consistent()?;

        Ok(CombineMethod{
            input: transient_structure.input.unwrap(),
            output: transient_structure.output,
            separator: transient_structure.separator,
            prefix: transient_structure.prefix,
            suffix: transient_structure.suffix,
        })

    }
}
#[derive(Debug)]
pub struct CombineMethod{
    input: Vec<HeaderValue>,
    output: String,
    separator: Option<String>,
    prefix: Option<String>,
    suffix: Option<String>,
}
#[cfg(test)]
mod test {
    use hcl::block;
    use crate::transform_parse::domain::methods_domain::combine_method::WrapperCombineMethod;

    #[test]
    fn test_combine_method() {
        let block = block!(combine "new_ID"{
            input = [0, "lower"]//"{$0}{$lower}"
            separator = "_"
            prefix = "BIZ_"
            suffix = "_ZIP"});
        let result = WrapperCombineMethod(block.to_owned()).to_combine_method();
        println!("{:?}", result);
        assert!(result.is_ok());
    }
}