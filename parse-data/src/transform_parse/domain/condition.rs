use crate::errors::ParseError;

pub struct WrapperCondition(pub(crate) hcl::Expression);

impl WrapperCondition {
    pub(crate) fn to_condition(&self) -> Result<Condition, ParseError> {
        todo!()
    }
}

#[derive(Debug)]
pub struct Condition {

}