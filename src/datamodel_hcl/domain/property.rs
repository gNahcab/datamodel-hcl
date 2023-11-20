use hcl::{Attribute, Block, block, BlockLabel};
use crate::errors::DatamodelHCLError;
use crate::domain::label::{Label, LabelBlockWrapper, LabelWrapper};


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
    pub(crate) fn add_propname(&mut self, labels: Vec<BlockLabel>) -> Result<(), DatamodelHCLError> {
        if labels.len() > 1 {
           return Err(DatamodelHCLError::ValidationError(String::from(format!("too many propnames in '{:?}'", labels))));
        }
        if labels.len()  == 0 {
            return Err(DatamodelHCLError::ValidationError(String::from(format!("not enough propnames: '{:?}'", labels))));
        }
        let propname = labels.get(0).expect(&*format!("cannot parse propname {:?}", labels.get(0))).clone().into_inner();
        self.propname = propname;
        Ok(())
    }

    pub(crate) fn add_gui_element(&mut self, new_gui_element: String) -> Result<(), DatamodelHCLError> {
        if !self.gui_element.is_none() {
            return Err(DatamodelHCLError::ValidationError(String::from(format!("it was tried to add more than one gui_element: '{:?}'", new_gui_element))));
        }
        self.gui_element = Option::from(new_gui_element);
        Ok(())
    }
    pub(crate) fn add_ontology(&mut self, new_ontology: String) ->  Result<(), DatamodelHCLError> {
        if !self.ontology.is_none() {
            return Err(DatamodelHCLError::ValidationError(String::from(format!("it was tried to add more than one ontology: '{:?}'", new_ontology))));
        }
        self.ontology = Option::from(new_ontology);
        Ok(())
    }
    pub(crate) fn add_object(&mut self, new_object: String) ->  Result<(), DatamodelHCLError> {
        if !self.object.is_none() {
            return Err(DatamodelHCLError::ValidationError(String::from(format!("it was tried to add more than one object: '{:?}'", new_object))));
        }
        self.object = Option::from(new_object);
        Ok(())
    }
    pub(crate) fn add_labels(&mut self, blocks: Vec<&Block>) -> Result<(), DatamodelHCLError> {
        if blocks.len() > 1 {
            return Err(DatamodelHCLError::ParseProjectModel(String::from(
                format!("found '{}' block(s) in '{}'. One block is necessary but not more blocks are allowed",blocks.len(), self.propname))));
        }
        if blocks.len() == 0 {
            return Err(DatamodelHCLError::ParseProjectModel(String::from(format!("found '{}' block(s) in '{}'. One and only one block is necessary",blocks.len(), self.propname))));
        }
        let label_block = blocks.get(0).expect("expected to get first and only block");
        self.labels = LabelBlockWrapper(label_block.to_owned().to_owned()).to_labels()?;

        Ok(())
    }
    pub(crate) fn is_complete(&self) -> Result<(), DatamodelHCLError> {
        if self.propname.is_empty() {
            return Err(DatamodelHCLError::ValidationError(String::from("propname doesn't exist or isn't provided correctly.")));
        }
        // one object
        if self.object.is_none() {
            return Err(DatamodelHCLError::ValidationError(String::from("object doesn't exist or isn't provided correctly.")));
        }
        // one ontology
        if self.ontology.is_none() {
            return Err(DatamodelHCLError::ValidationError(String::from("ontology doesn't exist or isn't provided correctly.")));
        }
        // one gui-element
        if self.gui_element.is_none() {
            return Err(DatamodelHCLError::ValidationError(String::from("gui_element doesn't exist or isn't provided correctly.")));
        }

        if self.labels.len() < 1 {
            return Err(DatamodelHCLError::ValidationError(String::from("labels seem to be empty.")));
        }
        Ok(())
    }
}


impl PropertyWrapper {
    pub fn to_property(self) -> Result<Property, DatamodelHCLError> {
        // one propname
        let mut transienst_structure_property = TransientStructureProperty::new();
        transienst_structure_property.add_propname(self.0.labels)?;


        let attributes: Vec<&hcl::Attribute> = self.0.body.attributes().collect();

        for attribute in attributes {
            match attribute.key.as_str() {
                "object" => transienst_structure_property.add_object(attribute.expr.to_string())?,
                "ontology" => transienst_structure_property.add_ontology(attribute.expr.to_string())?,
                "gui_element" => transienst_structure_property.add_gui_element(attribute.expr.to_string())?,
                _ => return Err(DatamodelHCLError::ParseProjectModel(String::from(format!("found unknown attribute {} of property {}. Valid attributes are: object, ontology, gui_element", attribute.key.as_str(), transienst_structure_property.propname))))
            }
        }
        let blocks: Vec<&hcl::Block> = self.0.body.blocks().collect();
        transienst_structure_property.add_labels(blocks);

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
    use crate::errors::DatamodelHCLError;

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
        let property:Result<Property, DatamodelHCLError> = property_wrapper.to_property();
        assert!(property.as_ref().is_ok());
        assert_eq!(property.as_ref().unwrap().name, "hasTextMedium");
        assert_eq!(property.as_ref().unwrap().object, "\"StillImageRepresentation\"");
        assert_eq!(property.as_ref().unwrap().ontology, "\"rosetta\"");
        assert_eq!(property.as_ref().unwrap().labels.get(0).unwrap().language_abbr,"en");
        assert_eq!(property.as_ref().unwrap().labels.get(0).unwrap().text,"\"my text medium\"");
        assert_eq!(property.as_ref().unwrap().labels.get(1).unwrap().language_abbr,"de");
        assert_eq!(property.as_ref().unwrap().labels.get(1).unwrap().text,"\"mein Schriftmedium\"");
        assert_eq!(property.as_ref().unwrap().labels.get(2).unwrap().language_abbr,"fr");
        assert_eq!(property.as_ref().unwrap().labels.get(2).unwrap().text,"\"mon médium d'écriture\"");
        assert_eq!(property.as_ref().unwrap().gui_element, "\"facultative\"");

    }
}
