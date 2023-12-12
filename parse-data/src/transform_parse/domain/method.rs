use hcl::{Attribute, Block, BlockLabel, Expression, Identifier};

use crate::datamodel_parse::remove_useless_quotation_marks;
use crate::errors::ParseError;
use crate::transform_parse::domain::condition::{Condition, WrapperCondition};
use crate::transform_parse::domain::function_body::{FunctionBody, WrapperFunctionBody};
use crate::transform_parse::domain::method_info::{Functions, MethodInfo, WrapperMethodInfo};

#[derive(Debug)]
pub struct WrapperMethod(pub(crate) hcl::Block);

struct TransientStructureMethod{
    method_info: Option<MethodInfo>,
    function_descr: Option<FunctionBody>,
    condition: Option<Condition>,
}
impl TransientStructureMethod {
    fn new() -> TransientStructureMethod {
        TransientStructureMethod{
            method_info: None,
            function_descr: None,
            condition: None,
        }
    }

    fn add_method_info(&mut self, method_info: MethodInfo) {
        self.method_info = Option::from(method_info);
    }
}

impl WrapperMethod {
    pub fn to_method(&self) -> Result<Method, ParseError>{
        let mut transient_structure = TransientStructureMethod::new();
        let method_info: MethodInfo = WrapperMethodInfo(self.0.labels.to_owned()).to_method_info()?;
        transient_structure.add_method_info(method_info);

        let blocks: Vec<&Block> = self.0.body.blocks().collect();
        if blocks.len() != 0 {
            return Err(ParseError::ValidationError(format!("this method '{:?}' shouldn't have any blocks: '{:?}'",self.0.labels, blocks)));
        }
        let attributes: Vec<&Attribute> = self.0.body.attributes().collect();
        for attribute in attributes{
            match attribute.key.as_str() {
                "function" => {
                    let function_body = WrapperFunctionBody(attribute.expr.to_owned()).to_function_body()?;
                    //methods.add_method(labels, &attribute.body, &attribute.identifier)?;
                }
                "condition" => {
                    println!("cond: {:?}", attribute);
                    let condition = WrapperCondition(attribute.expr.to_owned()).to_condition()?;
                    //methods.add_method(labels, &attribute.body, &attribute.identifier)?;
                }
                _ => {
                    return Err(ParseError::ValidationError(format!("this function does not exist: '{}'", attribute.key)));
                } }
        }
        Ok(Method{
            method_info:MethodInfo{ function: Functions::New, name: "".to_string() },
            function_descr: FunctionBody {},
            condition: None,
        })
    }
}



#[derive(Debug)]
pub struct Method {
    method_info: MethodInfo,
    function_descr: FunctionBody,
    condition: Option<Condition>,
}


impl Method {
    fn new (method_info: MethodInfo, function_descr: FunctionBody, condition: Option<Condition>) -> Method {
        return Method{
            method_info,
            function_descr,
            condition,
        }
    }

    pub(crate) fn add_method_info(&self, method_info: MethodInfo) {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::transform_parse::domain::method::WrapperMethod;

    #[test]
    fn test_transform_method() {
       let block = hcl::block!(method "new" "a"{
            // lower the b-variable
            function = "${a}_$lower({$b})"});
        let result = WrapperMethod(block).to_method();
        println!("{:?}", result);
        assert!(result.is_ok());
    }
}