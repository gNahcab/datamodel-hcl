use hcl::Expression;
use crate::errors::ParseError;

#[derive(Debug)]
pub enum HeaderValue {
    Name(String),
    Number(u8)
}


pub trait HeaderMethods {
    fn to_header_value(&self) -> Result<HeaderValue, ParseError>;
}

impl HeaderMethods for hcl::Expression {
    fn to_header_value(&self) -> Result<HeaderValue, ParseError> {
        let header_value = match self {
            Expression::Number(number) => {
                HeaderValue::Number(number.as_u8()?)
            }
            Expression::String(string) => {
                HeaderValue::Name(string.to_owned())
            }
            _ => {
                return Err(ParseError::ValidationError(format!("Only transform Number and String-Expressions to HeaderValue, cannot transform this: '{:?}'", self)))
            }
        };
        Ok(header_value)
    }
}
pub trait U8implementation {
    fn as_u8(&self) -> Result<u8, ParseError>;
}

impl U8implementation for hcl::Number {
    fn as_u8(&self) -> Result<u8, ParseError> {
        let result = self.as_f64();
        if result.is_none() {
            return Err(ParseError::ValidationError(format!("couldn't parse this number '{}' to f64.", self)));
        }
        let u8result = result.unwrap().floor() as u8;
        Ok(u8result)
    }
}
