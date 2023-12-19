use hcl::Expression;
use crate::errors::ParsingError;

pub trait To2String {
    fn to_string_2(&self) -> Result<String, ParsingError>;
}

impl To2String for hcl::Expression {
    fn to_string_2(&self) -> Result<String, ParsingError> {
        match self {
            Expression::String(value) => {Ok(value.to_owned())}
            _ => Err(ParsingError::ValidationError(format!("cannot parse this hcl::Expression '{:?}' to string, because it is not a string", self)))
        }
    }

}