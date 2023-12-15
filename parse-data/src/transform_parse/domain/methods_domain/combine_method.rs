use hcl::{Attribute, Block};
use crate::errors::ParseError;
use crate::transform_parse::domain::methods_domain::wrapper_trait::Wrapper;


pub struct WrapperCombineMethod (pub(crate) Block);
#[derive(Debug)]
struct TransientStructureCombineMethod {
    input: Vec<String>,
    output: String,
    separator: Option<String>,
    prefix: Option<String>,
    suffix: Option<String>,
}

impl TransientStructureCombineMethod {
    fn new(output: String) -> TransientStructureCombineMethod {
        TransientStructureCombineMethod{
            input: vec![],
            output,
            separator: None,
            prefix: None,
            suffix: None,
        }
    }
    pub(crate) fn add_input(&mut self, input: String) {
        self.input.push(input);
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
    pub(crate) fn add_output(&mut self, output: String) {
        self.output = output;
    }

    pub(crate) fn is_consistent(&self) -> Result<(), ParseError> {
        todo!()
    }
}


impl WrapperCombineMethod {

    pub(crate) fn to_combine_method(&self) -> Result<CombineMethod, ParseError> {
        let mut transient_structure = TransientStructureCombineMethod::new(self.0.get_output()?);
        self.0.no_blocks()?;
        let attributes: Vec<&Attribute> = self.0.body.attributes().collect();
        for attribute in attributes {
            match attribute.key.as_str() {
                "input" => {
                    transient_structure.add_input(attribute.expr.to_string());
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
        println!("labels: {:?}", self.0.labels);
        println!("identifier: {:?}",  self.0.identifier);

        Ok(CombineMethod{
            input_variables: vec![],
            output_variable: "".to_string(),
            middle: None,
            prefix: None,
            suffix: None,
        })

    }
}
impl TransientStructureCombineMethod {
}

impl WrapperCombineMethod {

}
#[derive(Debug)]
pub struct CombineMethod{
    input_variables: Vec<String>,
    output_variable: String,
    middle: Option<String>,
    prefix: Option<String>,
    suffix: Option<String>,
}
#[cfg(test)]
mod test {
    use hcl::block;
    use crate::transform_parse::domain::methods_domain::combine_method::WrapperCombineMethod;

    #[test]
    fn test_read_simple_transform_hcl() {
        let block = block!(combine "new_ID"{
            input = [0, "lower"]//"{$0}{$lower}"
            separator = "_"
            prefix = "BIZ_"
            suffix = "_ZIP"});
        let result = WrapperCombineMethod(block.to_owned()).to_combine_method();
        assert!(result.is_ok())
    }
}