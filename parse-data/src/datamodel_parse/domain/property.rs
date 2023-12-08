use hcl::{Attribute, Block, block, BlockLabel};
use crate::errors::ParseError;
use crate::domain::label::{Label, LabelBlockWrapper, LabelWrapper};
use crate::domain::remove_useless_quotation_marks;


#[derive(Debug, PartialEq)]
pub struct Property {
    pub name: String,
    pub ontology: String,
    pub object: String,
    pub labels: Vec<Label>,
    pub gui_element: String,
}
pub(crate) struct PropertyWrapper(pub(crate) hcl::Block);

struct TransientStructureProperty {
    propname: String,
    object: Option<String>,
    ontology: Option<String>,
    labels: Vec<Label>,
    gui_element: Option<String>,
}

impl TransientStructureProperty {
    fn new() -> TransientStructureProperty {
        TransientStructureProperty {
            propname: "".to_string(),
            object: None,
            ontology: None,
            labels: vec![],
            gui_element: None,
        }
    }
    pub(crate) fn add_propname(&mut self, labels: Vec<BlockLabel>) -> Result<(), ParseError> {
        if labels.len() > 1 {
           return Err(ParseError::ValidationError(String::from(format!("too many propnames in '{:?}'", labels))));
        }
        if labels.len()  == 0 {
            return Err(ParseError::ValidationError(String::from(format!("not enough propnames: '{:?}'", labels))));
        }
        let propname = labels.get(0).expect(&*format!("cannot parse propname {:?}", labels.get(0))).clone().into_inner();
        self.propname = propname;
        Ok(())
    }

    pub(crate) fn add_gui_element(&mut self, new_gui_element: String) -> Result<(), ParseError> {
        if !self.gui_element.is_none() {
            return Err(ParseError::ValidationError(String::from(format!("it was tried to add more than one gui_element: '{:?}'", new_gui_element))));
        }
        let gui_element = remove_useless_quotation_marks(new_gui_element);
        self.gui_element = Option::from(gui_element);
        Ok(())
    }
    pub(crate) fn add_ontology(&mut self, new_ontology: String) ->  Result<(), ParseError> {
        if !self.ontology.is_none() {
            return Err(ParseError::ValidationError(String::from(format!("it was tried to add more than one ontology: '{:?}'", new_ontology))));
        }
        let ontology = remove_useless_quotation_marks(new_ontology);
        self.ontology = Option::from(ontology);
        Ok(())
    }
    pub(crate) fn add_object(&mut self, new_object: String) ->  Result<(), ParseError> {
        if !self.object.is_none() {
            return Err(ParseError::ValidationError(String::from(format!("it was tried to add more than one object: '{:?}'", new_object))));
        }
        let object = remove_useless_quotation_marks(new_object);
        self.object = Option::from(object);
        Ok(())
    }
    pub(crate) fn add_labels(&mut self, blocks: Vec<&Block>) -> Result<(), ParseError> {
        if blocks.len() > 1 {
            return Err(ParseError::ParseProjectModel(String::from(
                format!("found '{}' block(s) in '{}'. One block is necessary but not more blocks are allowed",blocks.len(), self.propname))));
        }
        if blocks.len() == 0 {
            return Err(ParseError::ParseProjectModel(String::from(format!("found '{}' block(s) in '{}'. One and only one block is necessary", blocks.len(), self.propname))));
        }
        if blocks.get(0).as_ref().unwrap().identifier.as_str() != "labels" {
            return Err(ParseError::ParseProjectModel(String::from(
                format!("wrong identifier in property '{}' for labels, expected 'labels', found '{}'", self.propname, blocks.get(0).as_ref().unwrap().identifier.as_str()))));
        }
        let label_block = blocks.get(0).expect("expected to get first and only block");
        self.labels = LabelBlockWrapper(label_block.to_owned().to_owned()).to_labels()?;

        Ok(())
    }
    pub(crate) fn is_complete(&self) -> Result<(), ParseError> {
        if self.propname.is_empty() {
            return Err(ParseError::ValidationError(format!("propname doesn't exist or isn't provided correctly.")));
        }
        // one object
        if self.object.is_none() {
            return Err(ParseError::ValidationError(format!("object in '{:?}' doesn't exist or isn't provided correctly.", self.propname)));
        }
        // one ontology
        if self.ontology.is_none() {
            return Err(ParseError::ValidationError(format!("ontology in '{:?}' doesn't exist or isn't provided correctly.", self.propname)));
        }
        // one gui-element
        if self.gui_element.is_none() {
            return Err(ParseError::ValidationError(format!("gui_element in '{:?}' doesn't exist or isn't provided correctly.", self.propname)));
        }

        if self.labels.len() < 1 {
            return Err(ParseError::ValidationError(format!("labels in '{:?}' seem to be empty.", self.propname)));
        }
        Ok(())
    }
}


impl PropertyWrapper {
    pub fn to_property(self) -> Result<Property, ParseError> {
        // one propname
        let mut transienst_structure_property = TransientStructureProperty::new();
        transienst_structure_property.add_propname(self.0.labels)?;


        let attributes: Vec<&hcl::Attribute> = self.0.body.attributes().collect();

        for attribute in attributes {
            match attribute.key.as_str() {
                "object" => transienst_structure_property.add_object(attribute.expr.to_string())?,
                "ontology" => transienst_structure_property.add_ontology(attribute.expr.to_string())?,
                "gui_element" => transienst_structure_property.add_gui_element(attribute.expr.to_string())?,
                _ => return Err(ParseError::ParseProjectModel(String::from(format!("found unknown attribute {} of property {}. Valid attributes are: object, ontology, gui_element", attribute.key.as_str(), transienst_structure_property.propname))))
            }
        }
        let blocks: Vec<&hcl::Block> = self.0.body.blocks().collect();
        transienst_structure_property.add_labels(blocks)?;

        transienst_structure_property.is_complete()?;

        let property = Property{
            name: transienst_structure_property.propname,
            ontology: transienst_structure_property.ontology.unwrap(),
            object: transienst_structure_property.object.unwrap(),
            labels: transienst_structure_property.labels,
            gui_element: transienst_structure_property.gui_element.unwrap(),
        };
        Ok(property)
    }

}

#[cfg(test)]

mod test {
    use hcl::{block};
    use crate::domain::label::Label;
    use crate::domain::property::{Property, PropertyWrapper};
    use crate::errors::ParseError;

    #[test]
    fn test_into_property() {
        let property_block = block!(
            property "hasTextMedium" {
                object = "StillImageRepresentation"
                ontology = "rosetta"
                labels {
                    en = "my text medium"
                    de = "mein Schriftmedium"
                    fr = "mon médium d'écriture"
                }
                gui_element = "facultative"
            }
        );
        let property_wrapper = PropertyWrapper{ 0: property_block };
        let property:Result<Property, ParseError> = property_wrapper.to_property();
        println!("{:?}", property);
        assert!(property.as_ref().is_ok());
        assert_eq!(property.as_ref().unwrap().name, "hasTextMedium");
        assert_eq!(property.as_ref().unwrap().object, "StillImageRepresentation");
        assert_eq!(property.as_ref().unwrap().ontology, "rosetta");
        assert_eq!(property.as_ref().unwrap().gui_element, "facultative");
        assert_eq!(property.as_ref().unwrap().labels.get(0).unwrap().language_abbr,"en");
        assert_eq!(property.as_ref().unwrap().labels.get(0).unwrap().text,"my text medium");
        assert_eq!(property.as_ref().unwrap().labels.get(1).unwrap().language_abbr,"de");
        assert_eq!(property.as_ref().unwrap().labels.get(1).unwrap().text,"mein Schriftmedium");
        assert_eq!(property.as_ref().unwrap().labels.get(2).unwrap().language_abbr,"fr");
        assert_eq!(property.as_ref().unwrap().labels.get(2).unwrap().text,"mon médium d'écriture");

    }
}
