use hcl::Expression;
use crate::errors::ParsingError;

pub trait ExpressionTransform {
    fn to_string_2(&self) -> Result<String, ParsingError>;
    fn to_bool(&self) -> Result<bool, ParsingError>;
}

impl ExpressionTransform for hcl::Expression {
    fn to_string_2(&self) -> Result<String, ParsingError> {
        match self {
            Expression::String(value) => {Ok(value.to_owned())}
            _ => Err(ParsingError::ValidationError(format!("cannot parse this hcl::Expression '{:?}' to string, because it is not a string", self)))
        }
    }

    fn to_bool(&self) -> Result<bool, ParsingError> {
        match self {
            Expression::Bool(value) => {Ok(value.to_owned())}
            _ => Err(ParsingError::ValidationError(format!("cannot parse this hcl::Expression '{:?}' to bool, because it is not a bool. Did you write a bool-value within quotation marks? Everything within quotation marks will be read as string-value.", self)))
        }
    }
}