use hcl::{Attribute, Block};
use crate::errors::ParseError;


#[derive(Debug)]
pub struct WrapperLowerUpperMethod(pub(crate) hcl::Block);


struct TransientStructureLowerUpperMethod{
    output: String,
    input: Option<String>,
}

impl TransientStructureLowerUpperMethod {
    fn new(name: String) -> TransientStructureLowerUpperMethod {
        TransientStructureLowerUpperMethod {
            output: name,
            input: None,
        }
    }
    pub(crate) fn add_input(&mut self, variable: String) -> Result<(), ParseError> {
        if self.input.is_some() {
            return Err(ParseError::ValidationError(format!("found more than one 'input'-declaration in method '{:?}'.",self.output)));
        }
        self.input = Option::from(variable);
        Ok(())
    }
    pub(crate) fn is_complete(&self) -> Result<(), ParseError> {
        if self.input.is_none() {
            return Err(ParseError::ValidationError(format!("found no 'input'-declaration in method '{:?}'.",self.output)));
        }
        Ok(())
    }
}
impl WrapperLowerUpperMethod {
    pub fn to_lower_method(&self) -> Result<LowerMethod, ParseError> {
        let transient_structure = get_transient_structure(&self)?;
        Ok(LowerMethod{ output: transient_structure.output, input: transient_structure.input.unwrap()})
    }
    pub fn to_upper_method(&self) -> Result<UpperMethod, ParseError> {
        let transient_structure = get_transient_structure(&self)?;
        Ok(UpperMethod{ output: transient_structure.output, input: transient_structure.input.unwrap()})
    }
    fn no_blocks(&self) -> Result<(), ParseError> {
        let blocks: Vec<&Block> = self.0.body.blocks().collect();
        if blocks.len() != 0 {
            return Err(ParseError::ValidationError(format!("found those blocks '{:?}' in method '{:?}', but blocks are not allowed.",blocks, self)));
        }
        Ok(())
    }
    fn get_name(&self) -> Result<String, ParseError> {
        if self.0.labels.len() == 0 {
            return Err(ParseError::ValidationError(format!("no label found for method: '{:?}'", self)));
        }
        if self.0.labels.len() > 1 {
            return Err(ParseError::ValidationError(format!("this method should have one label but has more than one: '{:?}'", self.0.labels)));
        }
        return Ok(self.0.labels.get(0).unwrap().as_str().to_string());
    }
}

fn get_transient_structure(wrapper: &WrapperLowerUpperMethod) -> Result<TransientStructureLowerUpperMethod, ParseError> {
    wrapper.no_blocks()?;
    let mut transient_structure: TransientStructureLowerUpperMethod = TransientStructureLowerUpperMethod::new( wrapper.get_name()?);
    let attributes: Vec<&Attribute> = wrapper.0.body.attributes().collect();
    for attribute in attributes {
        match attribute.key.as_str() {
            "input" => {
                transient_structure.add_input(attribute.expr.to_string())?;
            }
            _ => {
                return Err(ParseError::ValidationError(format!("found this unknown attribute '{:?}' in method '{:?}'.",attribute, transient_structure.output)));
            } }

    }
    transient_structure.is_complete()?;
    Ok(transient_structure)
}
#[derive(Debug)]
pub struct LowerMethod{
output: String,
    input: String,
}
#[derive(Debug)]
pub struct UpperMethod{
    output: String,
    input: String,
}

#[cfg(test)]
mod test {
    use crate::transform_parse::domain::methods_domain::lower_upper_method::WrapperLowerUpperMethod;

    #[test]
    fn test_lower_method() {
        let block = hcl::block!(lower "lower"{
            // lower the b-variable
            input = 1
       });
        let result = WrapperLowerUpperMethod(block).to_lower_method();
        assert!(result.is_ok());
    #[test]
    fn test_upper_method() {
        let block = hcl::block!(upper "upper"{
        input = 2
   });
        let result = WrapperLowerUpperMethod(block).to_upper_method();
        assert!(result.is_ok());
    }}
}