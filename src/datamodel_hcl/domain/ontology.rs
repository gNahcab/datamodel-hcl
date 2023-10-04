use hcl::{Attribute, Block};
use crate::errors::DatamodelHCLError;

#[derive(Debug, PartialEq)]
pub struct Ontology {
    pub name: String,
    pub label: String
}


impl TryFrom<&hcl::Block> for Ontology {
    type Error = DatamodelHCLError;

    fn try_from(block: &Block) -> Result<Self, Self::Error> {
        if block.labels().len() != 1 {
           return Err(DatamodelHCLError::ParseProjectModel(String::from(format!("the ontology '{:?}' should have one name and only one name but the ontology has '{}' name(s)", block.labels(), block.labels().len()))));
        }
        let result = block.labels.get(0).ok_or(DatamodelHCLError::ParseProjectModel(String::from(format!("wasn't able to read the name from ontology '{:?}'", block.labels()))));
        let name = match result {
            Ok(value) => value.as_str(),
            Err(parse_error) => return Err(parse_error),
        };
        let attributes: Vec<&Attribute> = block.body.attributes().collect();
        if attributes.len() != 1 {
            return Err(DatamodelHCLError::ParseProjectModel(
                String::from(format!(
                    "the ontology '{:?}' should have one labels and only one labels but ontology '{:?}' has '{}' label(s)", name, attributes, attributes.len()))));
        }
        let result = attributes.get(0).ok_or(DatamodelHCLError::ParseProjectModel(String::from(format!("wasn't able to read the label from ontology '{:?}'", name))));
        let label = match result {
            Ok(value) => value.expr().to_string(),
            Err(parse_error) => return Err(parse_error),
        };
        let ontology = Ontology{name: name.to_string(), label};
        Ok(ontology)
    }
}

#[cfg(test)]

mod test {
    use hcl::{block};
    use crate::domain::ontology::Ontology;
    use crate::errors::DatamodelHCLError;

    #[test]
    fn test_into_ontology() {
        let ontology_block = &block!(
            ontology "rosetta" {
              label = "rosetta_label"
            }
        );
        let ontology:Result<Ontology, DatamodelHCLError> = ontology_block.try_into();
        assert!(ontology.is_ok());
    }
}



