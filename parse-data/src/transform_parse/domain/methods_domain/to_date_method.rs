use hcl::{Attribute, Expression};
use crate::datamodel_parse::remove_useless_quotation_marks;
use crate::errors::ParseError;
use crate::transform_parse::domain::header_value::{HeaderMethods, HeaderValue, U8implementation};
use crate::transform_parse::domain::methods_domain::date_type::DateType;
use crate::transform_parse::domain::methods_domain::wrapper_trait::Wrapper;

pub struct WrapperToDateMethod(pub(crate) hcl::Block);

#[derive(Debug)]
struct TransientStructureToDateMethod{
    output: String,
    input: Option<HeaderValue>,
    date_type: Option<String>,
}

impl TransientStructureToDateMethod {
    fn new(output: String) -> TransientStructureToDateMethod {
        TransientStructureToDateMethod{
            output,
            input: None,
            date_type: None,
        }
    }
    fn add_input(&mut self, input: Expression) -> Result<(), ParseError> {
        if self.input.is_some() {
            return Err(ParseError::ValidationError(format!("error in to_date-method '{:?}'. 'input'-attribute multiple times provided", self)));
        }
        let input_header_value = input.to_header_value()?;
        self.input = Option::from(input_header_value);
        Ok(())
    }

    fn add_date_type(&mut self, date_type: String) -> Result<(), ParseError> {
        if self.date_type.is_some() {
            return Err(ParseError::ValidationError(format!("error in to_date-method '{:?}'. 'date_type'-attribute multiple times provided", self))); }
        self.date_type = Option::from(date_type);
        Ok(())
    }
    fn is_consistent(&self) -> Result<(), ParseError> {
        if self.input.is_none() {
            return Err(ParseError::ValidationError(format!("error in to_date-method '{:?}'. 'input'-attribute not provided", self)));
        }
        if self.date_type.is_none() {
            return Err(ParseError::ValidationError(format!("error in to_date-method '{:?}'. 'date_type'-attribute not provided", self)));
        }
        Ok(())
    }
}
impl WrapperToDateMethod {
    pub fn to_date_method(&self) -> Result<ToDateMethod, ParseError> {
        self.0.no_blocks()?;
        let mut transient_structure: TransientStructureToDateMethod = TransientStructureToDateMethod::new(self.0.get_output()?);
        for attribute in self.0.attributes() {
            match attribute.key.as_str() {
                "input" => {
                    transient_structure.add_input(attribute.expr.to_owned())?;
                }
                "date_type" => {
                    transient_structure.add_date_type(remove_useless_quotation_marks(attribute.expr.to_string()))?;
                }
                _ => {
                    return Err(ParseError::ValidationError(format!("found this unknown attribute '{:?}' in method '{:?}'.",attribute, transient_structure.output)));
                }
            }

        }
        transient_structure.is_consistent()?;
        return Ok(ToDateMethod::new(transient_structure)?)
    }
}
#[derive(Debug)]
pub struct ToDateMethod{
    output: String,
    input: HeaderValue,
    date_type: DateType,
}

impl ToDateMethod {
    fn new(transient_structure: TransientStructureToDateMethod) -> Result<ToDateMethod, ParseError> {
        let date_type: DateType = DateType::date_type(transient_structure.date_type.unwrap())?;
        Ok(ToDateMethod{
            output: transient_structure.output,
            input: transient_structure.input.unwrap(),
            date_type,
        })
    }
}
#[cfg(test)]
mod test {
    use hcl::block;
    use crate::transform_parse::domain::methods_domain::to_date_method::WrapperToDateMethod;

    #[test]
    fn test_replace_method() {
        let block = block!(to_date "to_date"{
            input = 3
            date_type = "Gregorian" // or "Julian"
        });

        let result = WrapperToDateMethod(block.to_owned()).to_date_method();
        println!("{:?}", result);
        assert!(result.is_ok());
    }
}
