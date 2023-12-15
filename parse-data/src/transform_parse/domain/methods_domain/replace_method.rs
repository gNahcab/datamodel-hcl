use crate::errors::ParseError;

pub struct WrapperReplaceMethod(pub(crate) hcl::Block);

impl WrapperReplaceMethod{
    pub(crate) fn to_replace_method(&self) -> Result<ReplaceMethod, ParseError> {
        todo!()
    }
}
#[derive(Debug)]
pub struct ReplaceMethod{

}
