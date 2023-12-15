use hcl::{Attribute, Block, Expression};
use crate::errors::ParseError;
use crate::transform_parse::domain::header_value::{HeaderValue, U8implementation};
use crate::transform_parse::domain::methods_domain::wrapper_trait::Wrapper;


#[derive(Debug)]
pub struct WrapperLowerUpperMethod(pub(crate) hcl::Block);

#[derive(Debug)]
struct TransientStructureLowerUpperMethod{
    output: String,
    input: Option<HeaderValue>,
}

impl TransientStructureLowerUpperMethod {
    fn new(name: String) -> TransientStructureLowerUpperMethod {
        TransientStructureLowerUpperMethod {
            output: name,
            input: None,
        }
    }
    pub(crate) fn add_input(&mut self, input: Expression) -> Result<(), ParseError> {
        if self.input.is_some() {
            return Err(ParseError::ValidationError(format!("found more than one 'input'-declaration in method '{:?}'.",self.output)));
        }
        let input_header_value = match input {
            Expression::Number(value) => {
                HeaderValue::Number(value.as_u8()?)
            }
            Expression::String(value) => {
                HeaderValue::Name(value)
            }
            _ => {
                return Err(ParseError::ValidationError(format!("error in lower-upper-method '{:?}'. 'input'-expression can only be of type 'String' or 'Number' but found this: '{:?}'", self, input)));
            }
        };
        self.input = Option::from(input_header_value);
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
}

fn get_transient_structure(wrapper: &WrapperLowerUpperMethod) -> Result<TransientStructureLowerUpperMethod, ParseError> {
    wrapper.0.no_blocks()?;
    let mut transient_structure: TransientStructureLowerUpperMethod = TransientStructureLowerUpperMethod::new( wrapper.0.get_output()?);
    for attribute in wrapper.0.attributes() {
        match attribute.key.as_str() {
            "input" => {
                transient_structure.add_input(attribute.expr.to_owned())?;
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
    input: HeaderValue,
}
#[derive(Debug)]
pub struct UpperMethod{
    output: String,
    input: HeaderValue,
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