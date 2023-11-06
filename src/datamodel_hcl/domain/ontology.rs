use hcl::{Attribute, Block};
use crate::errors::DatamodelHCLError;

#[derive(Debug, PartialEq)]
pub struct Ontology {
    pub name: String,
    pub label: String
}

pub(crate) struct OntologyWrapper(pub(crate) hcl::Block);
impl OntologyWrapper {
    pub fn to_ontology(&self) -> Result<Ontology, DatamodelHCLError> {
        if &self.0.labels().len() != &1usize {
            return Err(DatamodelHCLError::ParseProjectModel(String::from(format!("the ontology '{:?}' should have one name and only one name but the ontology has '{}' name(s)", &self.0.labels(), &self.0.labels().len()))));
        }
        let result = &self.0.labels.get(0).ok_or(DatamodelHCLError::ParseProjectModel(String::from(format!("wasn't able to read the name from ontology '{:?}'", &self.0.labels()))));
        let name = match result {
            Ok(value) => value.as_str(),
            Err(parse_error) => return Err(DatamodelHCLError::ParseProjectModel(format!("{:?}", parse_error))),
        };
        let attributes: Vec<&Attribute> = self.0.body.attributes().collect();

        if attributes.len() != 1 {
            return Err(DatamodelHCLError::ParseProjectModel(
                String::from(format!(
                    "the ontology '{:?}' should have one labels and only one label but ontology '{:?}' has '{}' label(s)", name, attributes, attributes.len()))));
        }
        let result = attributes.get(0).ok_or(DatamodelHCLError::ParseProjectModel(String::from(format!("wasn't able to read the label from ontology '{:?}'", name))));
        let label = match result {
            Ok(value) => value.expr().to_string(),
            Err(parse_error) => return Err(parse_error),
        };
        let ontology = Ontology{name: name.to_string(), label:label.to_string()};
        Ok(ontology)
    }
}
#[cfg(test)]

mod test {
    use hcl::{block};
    use crate::domain::ontology::{OntologyWrapper, Ontology};
    use crate::errors::DatamodelHCLError;

    #[test]
    fn test_to_ontology() {
        let ontology_block = block!(
            ontology "rosetta" {
              label = "rosetta_label"
            }
        );
        let hcl_transformer: OntologyWrapper = OntologyWrapper(ontology_block);
        let ontology:Result<Ontology, DatamodelHCLError> = hcl_transformer.to_ontology();
        assert!(ontology.is_ok());
    }
}



