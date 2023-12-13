use hcl::Expression;
use crate::datamodel_parse::remove_useless_quotation_marks;
use crate::errors::ParseError;

pub struct WrapperFunctionBody(pub(crate) hcl::Expression);
#[derive(Debug)]
pub struct FunctionBody {
    pub(crate) array: Option<Vec<String>>,
    pub(crate) string: Option<String>,
}

#[derive(Debug)]
struct TransientStructureFunctionBody{
    array: Option<Vec<String>>,
    string: Option<String>,
}

impl TransientStructureFunctionBody {
    pub(crate) fn add_string(&mut self, string: String) {
        self.string = Option::from(string);
    }
    pub(crate) fn add_array(&mut self, array: Vec<Expression>) {
        let string_vec: Vec<String> =  array.iter().map(|expr| remove_useless_quotation_marks(expr.to_string())).collect();
        self.array = Option::from(string_vec);
    }
    pub(crate) fn is_consistent(&self) -> Result<(), ParseError> {
        //check that string is not empty, part of array could be empty though
        if self.string.is_some() && self.string.as_ref().unwrap().to_string() == "" {
                return Err(ParseError::ValidationError(format!("string of function is empty: {:?}", self)));
        }
        if self.array.is_some() {
            if self.array.as_ref().unwrap().iter().filter(|predicate| predicate.as_str() == "").collect::<Vec<&String>>().len() == self.array.as_ref().unwrap().len() {
                return Err(ParseError::ValidationError(format!("array of function has only empty members: {:?}", self)));
            }
        }
        return Ok(())
    }
}


impl TransientStructureFunctionBody {
    fn new() -> TransientStructureFunctionBody{
        TransientStructureFunctionBody{ array: None, string: None }
    }
}
impl WrapperFunctionBody {
    pub fn to_function_body(&self) -> Result<FunctionBody, ParseError>{
        let mut transient_structure = TransientStructureFunctionBody::new();
        match &self.0 {
            Expression::String(value) => {
                transient_structure.add_string(value.to_owned());
            }
            Expression::Array(value) => {
                transient_structure.add_array(value.to_owned());
            }
            _ => {return Err(ParseError::ValidationError(format!("only array and string are allowed for 'function', but found: {:?}", self.0)));}

        }
        transient_structure.is_consistent()?;
        Ok(FunctionBody{ array: transient_structure.array, string: transient_structure.string })
    }
}
#[cfg(test)]
mod test {
    use crate::transform_parse::domain::function_body::WrapperFunctionBody;

    #[test]
    fn test_function_body_string() {
        let function =  hcl::expression!("${a}_$lower({$b})");
        let result = WrapperFunctionBody(function).to_function_body();
        println!("result string: {:?}", result);
        assert!(result.is_ok());
    }
    #[test]
    fn test_function_body_array() {
       let function = hcl::expression!(["DICT", "DICTIONARY"]);
        let result = WrapperFunctionBody(function).to_function_body();
        println!("result array: {:?}", result);
        assert!(result.is_ok());
    }
}
