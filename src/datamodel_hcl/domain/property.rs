use hcl::{Block, BlockLabel};
use hcl::Structure::Attribute;
use crate::domain::label::Label;
use crate::errors::DatamodelHCLError;

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
        for attribute in attributes {
            match attribute.key {
                 &"object" => {},
                 &"ontology" => {},
                 &"gui_element" => {},
                 &_ => Err("not allowed element")
            }
        }
        println!("attr {:?}", attributes);
        let blocks: Vec<&hcl::Block> = block.body.blocks().collect();
        println!("block {:?}", blocks);


        let property = Property{
            name: propname.to_string(),
            ontology: "".to_string(),
            object: "".to_string(),
            labels: vec![],
            gui_element: "".to_string(),
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
                gui_element: "todo!"
            }
        );
        let property:Result<Property, DatamodelHCLError> = property_block.try_into();
        assert!(property.is_ok());
    }
}
