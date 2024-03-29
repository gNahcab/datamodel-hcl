use hcl::Attribute;
use crate::errors::ParsingError;
use crate::expression_trait::ExpressionTransform;

#[derive(Debug, PartialEq)]
pub struct Ontology {
    pub name: String,
    pub label: String
}
#[derive(Debug)]
struct TransientStructureOntology {
    name: Option<String>,
    label: Option<String>,
}

impl TransientStructureOntology {
    pub(crate) fn add_name(&mut self, name_string: String) -> Result<(), ParsingError> {
        if !self.name.is_none() {
            return Err(ParsingError::ParseProjectModel(String::from(format!("the ontology '{:?}' should have one name and only one name but the ontology has a second '{}' name", self.label, name_string))));
        }
        self.name = Option::from(name_string);
        Ok(())
        }
    pub(crate) fn add_label(&mut self, label: String) -> Result<(), ParsingError> {
        if !self.label.is_none() {
            return Err(ParsingError::ParseProjectModel(String::from(format!("the ontology '{:?}' should have one label and only one label but the ontology has a second '{}' label", self.label, label))));

        }
        self.label = Option::from(label);
        Ok(())
    }
    pub(crate) fn is_complete(&self) -> Result<(), ParsingError> {
        if self.name.is_none() {
            return Err(ParsingError::ParseProjectModel(String::from(format!("the ontology with label '{:?}' doesn't have a name", self.label))));
        }
        if self.label.is_none() {
            return Err(ParsingError::ParseProjectModel(String::from(format!("the ontology '{:?}' doesn't have a label", self.name))));
        }
        Ok(())
    }
}

impl TransientStructureOntology {
    fn new() -> TransientStructureOntology {
        TransientStructureOntology{
            name: None,
            label: None,
        }
    }
}

pub(crate) struct OntologyWrapper(pub(crate) hcl::Block);
impl OntologyWrapper {
    pub fn to_ontology(&self) -> Result<Ontology, ParsingError> {
        let mut transient_structure_ontology = TransientStructureOntology::new();
        let labels = self.0.labels().to_owned();
        for label in labels {
            transient_structure_ontology.add_name(label.as_str().to_string())?;
        }
        let attributes: Vec<&Attribute> = self.0.body.attributes().collect();
        for attribute in attributes {
            match attribute.key.as_str() {
                "label" => {
                    transient_structure_ontology.add_label(attribute.expr.to_string_2()?)?;
                }
                _ => {
                    return Err(ParsingError::ValidationError(String::from(format!(
                        "only 'label' allowed for ontology but found '{:?}' in ontology '{:?}'",attribute, transient_structure_ontology.name))));
                }
            }
        }
        transient_structure_ontology.is_complete()?;

        let ontology = Ontology{name:transient_structure_ontology.name.unwrap(), label:transient_structure_ontology.label.unwrap()};
        Ok(ontology)
    }
}
#[cfg(test)]

mod test {
    use hcl::{block};
    use crate::datamodel_parse::domain::ontology::{OntologyWrapper, Ontology};
    use crate::errors::ParsingError;

    #[test]
    fn test_to_ontology() {
        let ontology_block = block!(
            ontology "rosetta" {
              label = "rosetta_label"
            }
        );
        let hcl_transformer: OntologyWrapper = OntologyWrapper(ontology_block);
        let ontology:Result<Ontology, ParsingError> = hcl_transformer.to_ontology();
        assert!(ontology.is_ok());
        assert_eq!(ontology.as_ref().unwrap().name,"rosetta");
        assert_eq!(ontology.as_ref().unwrap().label,"rosetta_label");
    }
}



