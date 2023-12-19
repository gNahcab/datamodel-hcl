use crate::errors::ParsingError;

#[derive(Debug, Clone)]
pub enum DateType {
   Gregorian,
    Julian,
}
impl DateType {
    pub(crate) fn date_type(string: String) -> Result<DateType, ParsingError>{
        match string.as_str() {
            "Gregorian" => Ok(DateType::Gregorian),
            "Julian" => Ok(DateType::Julian),
            _ => Err(ParsingError::ValidationError(format!("unknown value for 'date'-attribute: {:?}", string))),
        }

    }
}
