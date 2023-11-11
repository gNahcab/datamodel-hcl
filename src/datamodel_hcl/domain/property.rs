use hcl::{Attribute, Block, block};
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

impl PropertyWrapper {
    pub fn to_property(self) -> Result<Property, DatamodelHCLError> {
        // one propname
        if self.0.labels.len() != 1 {
            return Err(DatamodelHCLError::ParseProjectModel(String::from(format!("propname should have one name and only one name but in '{:?}' found '{}'", self.0.labels(), self.0.labels.len()))));

        }
        // one object

        // one ontology

        // one gui-element

        let result = self.0.labels().get(0).ok_or(Err(DatamodelHCLError::ParseProjectModel(String::from(format!("couldn't parse propname '{:?}'", self.0.labels())))));
        let propname = match result {
            Ok(propname) => propname.as_str(),
            Err(prop_error) => return prop_error,
        };

        let attributes: Vec<&hcl::Attribute> = self.0.body.attributes().collect();

        let mut object: String = "".to_string();
        let mut ontology:String = "".to_string();
        let mut gui_element:String =  "".to_string();


        for attribute in attributes {
            match attribute.key.as_str() {
                "object" => object = attribute.expr.to_string(),
                "ontology" => ontology = attribute.expr().to_string(),
                "gui_element" => gui_element= attribute.expr().to_string(),
                _ => return Err(DatamodelHCLError::ParseProjectModel(String::from(format!("found unknown attribute {} of property {}. Valid attributes are: object, ontology, gui_element", attribute.key.as_str(),  propname))))
            }
        }
        let blocks: Vec<&hcl::Block> = self.0.body.blocks().collect();
        if blocks.len() != 1 {
            return Err(DatamodelHCLError::ParseProjectModel(String::from(format!("found '{}' block(s) in '{}'. One block is necessary but not more blocks are allowed",blocks.len(), propname))))
        }
        let label_block = blocks.get(0).expect("expected to get first and only block");
        let labels = LabelBlockWrapper(label_block.to_owned().to_owned()).to_labels()?;

        let property = Property{
            name: propname.to_string(),
            ontology: ontology.to_string(),
            object: object.to_string(),
            labels,
            gui_element: gui_element.to_string(),
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
