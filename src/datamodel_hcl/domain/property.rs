use hcl::{Block};
use crate::errors::DatamodelHCLError;
use crate::domain::label::Label;


#[derive(Debug, PartialEq)]
pub struct Property {
    pub name: String,
    pub ontology: String,
    pub object: String,
    pub labels: Vec<Label>,
    pub gui_element: String,
}

impl TryFrom<&hcl::Block> for Property {
    type Error = DatamodelHCLError;

    fn try_from(block: &Block) -> Result<Self, Self::Error> {
        if block.labels.len() != 1 {
            return Err(DatamodelHCLError::ParseProjectModel(String::from(format!("propname should have one name and only one name but in '{:?}' found '{}'", block.labels(), block.labels.len()))));

        }
        let result = block.labels().get(0).ok_or(Err(DatamodelHCLError::ParseProjectModel(String::from(format!("couldn't parse propname '{:?}'", block.labels())))));
        let propname = match result {
            Ok(propname) => propname.as_str(),
            Err(prop_error) => return prop_error,
        };

        let attributes: Vec<&hcl::Attribute> = block.body.attributes().collect();

        let mut object: String = "".to_string();
        let mut ontology:String =  "".to_string();
        let mut gui_element:String =  "".to_string();


        for attribute in attributes {
            match attribute.key.as_str() {
                 "object" => object = attribute.expr.to_string(),
                 "ontology" => ontology = attribute.expr().to_string(),
                 "gui_element" => gui_element= attribute.expr().to_string(),
                 _ => return Err(DatamodelHCLError::ParseProjectModel(String::from(format!("found unknown attribute {} of property {}. Valid attributes are: object, ontology, gui_element", attribute.key.as_str(),  propname))))
            }
        }
        let blocks: Vec<&hcl::Block> = block.body.blocks().collect();
        if blocks.len() != 1 {
            return Err(DatamodelHCLError::ParseProjectModel(String::from(format!("found '{}' block(s) in '{}'. One block is necessary but not more blocks are allowed",blocks.len(), propname))))
        }
        let label_block = blocks.get(0).expect("expected to get first and only block");
        let mut labels: Vec<Label> = vec![];
        let label_attributes: Vec<&hcl::Attribute> = label_block.body.attributes().collect();

        for label_attribute in label_attributes {
            let new_label:Label = label_attribute.try_into()?;
            labels.push(new_label);
        }

        let property = Property{
            name: propname.to_string(),
            ontology: ontology.to_string(),
            object: object.to_string(),
            labels: vec![],
            gui_element: gui_element.to_string(),
        };
        println!("{:?}", property);
        Ok(property)
    }
}
#[cfg(test)]

mod test {
    use hcl::{block};
    use crate::domain::property::Property;
    use crate::errors::DatamodelHCLError;

    #[test]
    fn test_into_property() {
        let property_block = &block!(
            property "hasTextMedium" {
                object = "StillImageRepresentation"
                ontology = "rosetta"
                labels {
                    en = "my text medium"
                    de = "mein Schriftmedium"
                    fr = "mon médium d'écriture"
                }
                gui_element = "todo!"
            }
        );
        let property:Result<Property, DatamodelHCLError> = property_block.try_into();
        assert!(property.is_ok());

    }
}
