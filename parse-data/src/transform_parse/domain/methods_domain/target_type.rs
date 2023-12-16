use crate::errors::ParseError;

#[derive(Debug)]
pub enum TargetType {
    Part,
    Whole,
}

impl TargetType {
    pub(crate) fn target_type(string: String) -> Result<TargetType, ParseError>{
        match string.as_str() {
            "part" => Ok(TargetType::Part),
            "whole" => Ok(TargetType::Whole),
            _ => Err(ParseError::ValidationError(format!("unknown value for 'target'-attribute: {:?}", string))),
        }

    }
}