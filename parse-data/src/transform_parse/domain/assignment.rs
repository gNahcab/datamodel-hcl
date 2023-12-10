use std::collections::HashMap;
use hcl::{Attribute, Expression, Number};
use crate::errors::ParseError;

#[derive(Debug)]
pub struct AssignmentsWrapper (pub(crate)hcl::Body);

impl AssignmentsWrapper {
    pub fn to_assignments(&self) -> Result<Assignments, ParseError> {
        let mut assignments = Assignments::new();
        let attributes: Vec<&Attribute> = self.0.attributes().collect();
        for attribute in attributes {
            match &attribute.expr {
                Expression::Number(value) => {
                    assignments.add_pair(attribute.key.as_str(), value.to_string())?;
                }
                Expression::String(value) => {
                    assignments.add_pair(attribute.key.as_str(), value.to_string());
                }
                _ => {
                    return Err(ParseError::ValidationError(format!("error in assignments: cannot handle type of value expression: '{:?}'", attribute)));
                }
            }
        }
        Ok(Assignments{ name_to_assignments: Default::default() })
    }
}


pub struct Assignments {
    pub(crate) name_to_assignments: HashMap<String, String>
}

impl Assignments {
}

impl Assignments {
    pub(crate) fn add_pair(&mut self, name_in_dm: &str, number: String) -> Result<(), ParseError> {
        if self.name_to_assignments.get(name_in_dm).is_some() {
            return Err(ParseError::ValidationError(format!("duplicate in assignment, this name already exists: '{}'", name_in_dm)));
        }
        self.name_to_assignments.insert(name_in_dm.to_string(), number.to_string());
        Ok(())
    }
}

impl Assignments {
    fn new() -> Assignments{
        Assignments{ name_to_assignments: Default::default() }
    }
}