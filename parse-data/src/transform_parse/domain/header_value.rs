use hcl::Expression;
use crate::errors::ParsingError;

#[derive(Debug, Clone, PartialEq)]
pub enum HeaderValue {
    Name(String),
    Number(u8)
}

impl HeaderValue {
    pub(crate) fn is_equal(&self, output: &String) -> bool {
        // return true if headerValue is equal to a string-value
        return match self {
            HeaderValue::Name(name) => {
                name == output
            }
            HeaderValue::Number(_) => {
                false
            }
        }
    }
}


pub trait HeaderMethods {
    fn to_header_value(&self) -> Result<HeaderValue, ParsingError>;
}

impl HeaderMethods for hcl::Expression {
    fn to_header_value(&self) -> Result<HeaderValue, ParsingError> {
        let header_value = match self {
            Expression::Number(number) => {
                HeaderValue::Number(number.as_u8()?)
            }
            Expression::String(string) => {
                HeaderValue::Name(string.to_owned())
            }
            _ => {
                return Err(ParsingError::ValidationError(format!("Only transform Number and String-Expressions to HeaderValue, cannot transform this: '{:?}'", self)))
            }
        };
        Ok(header_value)
    }
}
pub trait U8implementation {
    fn as_u8(&self) -> Result<u8, ParsingError>;
}

impl U8implementation for hcl::Number {
    fn as_u8(&self) -> Result<u8, ParsingError> {
        let result = self.as_f64();
        if result.is_none() {
            return Err(ParsingError::ValidationError(format!("couldn't parse this number '{}' to f64.", self)));
        }
        let u8result = result.unwrap().floor() as u8;
        Ok(u8result)
    }
}
