use crate::errors::ParsingError;

#[derive(Debug, Clone)]
pub enum TargetType {
    Part,
    Whole,
}

impl TargetType {
    pub(crate) fn target_type(string: String) -> Result<TargetType, ParsingError>{
        match string.as_str() {
            "part" => Ok(TargetType::Part),
            "whole" => Ok(TargetType::Whole),
            _ => Err(ParsingError::ValidationError(format!("unknown value for 'target'-attribute: {:?}", string))),
        }

    }
}