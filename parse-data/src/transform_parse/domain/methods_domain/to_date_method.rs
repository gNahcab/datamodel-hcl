use crate::errors::ParseError;

pub struct WrapperToDateMethod(pub(crate) hcl::Block);
impl WrapperToDateMethod {
    pub fn to_date_method(&self) -> Result<ToDateMethod, ParseError> {
        todo!()
    }
}
#[derive(Debug)]
pub struct ToDateMethod{

}
