use hcl::BlockLabel;
use crate::errors::ParseError;

#[derive(Debug)]
pub enum Function {
    New,
    Replace,
    Upper,
    Lower,
    ToDate,
}


fn get_function(maybe_function:&str) -> Option<Function> {
    //returns the function or None if non-existent
    match maybe_function {
        "new" => {Option::from(Function::New) }
       "replace" => {Option::from(Function::Replace)}
        "upper" => {Option::from(Function::Upper)}
        "lower" => {Option::from(Function::Lower)}
        "to_date" => {Option::from(Option::from(Function::ToDate))}
        _ => {
            None
        }
    }

    }


#[derive(Debug)]
pub struct MethodInfo {
    pub(crate) function: Function,
    pub(crate) name: String
}

#[derive(Debug)]
struct TransientStructureMethodInfo {
    function: Option<Function>,
    name: Option<String>,
}

impl TransientStructureMethodInfo {
    fn new() -> TransientStructureMethodInfo {
        TransientStructureMethodInfo{ function: None, name: None }
    }

    fn add_name(&mut self, name: String) -> Result<(), ParseError> {
        if self.name.is_some() {
                return Err(ParseError::ValidationError(format!("found two names for '{:?}'", self)));
        }
        self.name = Some(name);
        Ok(())
    }
     fn add_function(&mut self, function: Function) -> Result<(), ParseError> {
         if self.function.is_some() {
             return Err(ParseError::ValidationError(format!("found two functions for '{:?}'", self)));
         }
         self.function = Option::from(function);
         Ok(())
    }
    fn is_consistent(&self) -> Result<(), ParseError> {
        if self.name.is_none() {
            return Err(ParseError::ValidationError(format!("method name is missing for '{:?}'", &self)));
        }
        if self.function.is_none() {
            return Err(ParseError::ValidationError(format!("method with name '{:?}' doesn't have a function as label", self.name.to_owned().unwrap())));
        }
        Ok(())
    }
}
#[derive(Debug)]
pub struct WrapperMethodInfo (pub(crate) Vec<hcl::BlockLabel>);
impl WrapperMethodInfo {
    pub fn to_method_info(&self) -> Result<MethodInfo, ParseError> {
        let mut transient_structure = TransientStructureMethodInfo::new();
        if &self.0.len() == &0 {
            return Err(ParseError::ValidationError(format!("labels not found for 'method', every 'method' should have two labels: the specification and a name")));
        }
        if &self.0.len() == &1 {
            return Err(ParseError::ValidationError(format!("only one label '{:?}' for method, but need two", self)));
        }
        if &self.0.len() > &2 {
            return Err(ParseError::ValidationError(format!("only two labels for method allowed, but found more than two: '{:?}'", self)));
        }
        for label in &self.0 {
            let function: Option<Function> = get_function(label.as_str());
            match function {
                None => {
                    transient_structure.add_name(label.as_str().to_string())?;
                }
                Some(function) => {
                    transient_structure.add_function(function)?;
                }
            }

        }
        transient_structure.is_consistent()?;
        Ok(MethodInfo{function: transient_structure.function.unwrap(), name:transient_structure.name.unwrap()})
    }
}


#[cfg(test)]
mod test {
    use crate::errors::ParseError;
    use crate::transform_parse::domain::method_info::{MethodInfo, WrapperMethodInfo};

    #[test]
    fn test_read_simple_transform_hcl() {
        let body: Vec<hcl::BlockLabel> = vec![
            hcl::block_label!(
            "new"),
            hcl::block_label!(
            "a")
        ];

        let method_info: Result<MethodInfo, ParseError> = WrapperMethodInfo(body.to_owned()).to_method_info();
        assert!(method_info.is_ok());
    }
}

