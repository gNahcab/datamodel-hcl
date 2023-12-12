use crate::errors::ParseError;

pub struct WrapperFunctionBody(pub(crate) hcl::Expression);
#[derive(Debug)]
pub struct FunctionBody {

}

impl WrapperFunctionBody {
    pub fn to_function_body(&self) -> Result<FunctionBody, ParseError>{
        todo!()
    }
}
