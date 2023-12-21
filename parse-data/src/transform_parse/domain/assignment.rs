use std::collections::HashMap;
use hcl::{Attribute, Expression};
use crate::errors::ParsingError;
use crate::transform_parse::domain::header_value::{HeaderMethods, HeaderValue};

#[derive(Debug)]
pub struct AssignmentsWrapper(pub(crate)hcl::Block);

impl AssignmentsWrapper {
    pub fn to_assignments(&self) -> Result<Assignments, ParsingError> {
        let mut assignments = Assignments::new();
        let attributes: Vec<&Attribute> = self.0.body().attributes().collect();
        for attribute in attributes {
            assignments.add_pair(attribute.key.as_str(), &attribute.expr)?;
        }
        Ok(assignments)
    }
}


#[derive(Debug, Clone)]
pub struct Assignments {
    pub assignments_to_header_value: HashMap<String, HeaderValue>
}


impl Assignments {
    fn new() -> Assignments{
        Assignments{ assignments_to_header_value: Default::default() }
    }
    pub(crate) fn add_pair(&mut self, name_in_dm: &str, identifier: &Expression) -> Result<(), ParsingError> {
        if self.assignments_to_header_value.get(name_in_dm).is_some() {
            return Err(ParsingError::ValidationError(format!("duplicate in assignment, this name already exists: '{}'", name_in_dm)));
        }
        self.assignments_to_header_value.insert(name_in_dm.to_string(), identifier.to_header_value()?);
        Ok(())
    }
}
#[cfg(test)]
mod test {
    use hcl::block;
    use crate::transform_parse::domain::assignment::AssignmentsWrapper;
    use crate::transform_parse::domain::methods_domain::combine_method::WrapperCombineMethod;

    #[test]
    fn test_assignments() {
        let block = block!( assignments {
                id = "ID" // String = Header, wenn vorhanden
                not_lowered = 1
                hasName = 2
                hasIdentifier = 3
                hasChildren = 4
                hasExternalLink = 5
            }
        );
        let result = AssignmentsWrapper(block).to_assignments();
        println!("{:?}", result);
        assert!(result.is_ok());
    }
}