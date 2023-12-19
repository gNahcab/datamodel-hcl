use crate::errors::ParsingError;

#[derive(Debug, Clone)]
pub enum BehaviorType {
    Lazy,
    Greedy
}
impl BehaviorType {
    pub(crate) fn behavior_type(string: String) -> Result<BehaviorType, ParsingError>{
        match string.as_str() {
            "greedy" => Ok(BehaviorType::Greedy),
            "lazy" => Ok(BehaviorType::Lazy),
            _ => Err(ParsingError::ValidationError(format!("unknown value for 'behavior'-attribute: {:?}", string))),
        }

    }
}
