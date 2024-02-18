use std::collections::{HashMap, HashSet};
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
        assignments.no_duplicates_in_values()?;
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
        // adds a pair of name(string) and header(string or number)
        if self.assignments_to_header_value.get(name_in_dm).is_some() {
            return Err(ParsingError::ValidationError(format!("duplicate in assignment, this name already exists: '{}'", name_in_dm)));
        }
        self.assignments_to_header_value.insert(name_in_dm.to_string(), identifier.to_header_value()?);
        Ok(())
    }
    fn no_duplicates_in_values(&self) -> Result<(), ParsingError> {
        // checks that no duplicates among the strings or the numbers exist in assignments
        // e.g. "hasNumber" = 3 and "hasOtherNumber" = 3 would result in an Error
        let mut numbers :HashSet<&u8>= HashSet::new();
        let mut names :HashSet<&String>= HashSet::new();
        for header in self.assignments_to_header_value.values() {
            //no name more than once used
            match header {
                HeaderValue::Name(name) => {
                    if names.insert(name) == false {
                        return Err(ParsingError::ValidationError(format!("found duplicated header in values assigned in assignments: '{:?}'. Every header shouldn't be assigned more than once.", name)));
                    }
                }
                HeaderValue::Number(number) => {
                    if numbers.insert(number) == false {
                        return Err(ParsingError::ValidationError(format!("found duplicated column/row number in values assigned in assignments: '{:?}'. Every number shouldn't be assigned more than once.", number)));
                    }
                }
            }
        }
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