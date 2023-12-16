use crate::errors::ParseError;

#[derive(Debug)]
pub enum DateType {
   Gregorian,
    Julian,
}
impl DateType {
    pub(crate) fn date_type(string: String) -> Result<DateType, ParseError>{
        match string.as_str() {
            "Gregorian" => Ok(DateType::Gregorian),
            "Julian" => Ok(DateType::Julian),
            _ => Err(ParseError::ValidationError(format!("unknown value for 'date'-attribute: {:?}", string))),
        }

    }
}
