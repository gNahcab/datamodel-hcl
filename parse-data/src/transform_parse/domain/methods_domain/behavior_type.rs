use crate::errors::ParseError;

#[derive(Debug)]
pub enum BehaviorType {
    Lazy,
    Greedy
}
impl BehaviorType {
    pub(crate) fn behavior_type(string: String) -> Result<BehaviorType, ParseError>{
        match string.as_str() {
            "greedy" => Ok(BehaviorType::Greedy),
            "lazy" => Ok(BehaviorType::Lazy),
            _ => Err(ParseError::ValidationError(format!("unknown value for 'behavior'-attribute: {:?}", string))),
        }

    }
}
