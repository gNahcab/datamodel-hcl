use hcl::{Block};
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

    fn try_from(value: &Block) -> Result<Self, Self::Error> {
        todo!()
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
                labels {
                    en = "my text medium"
                    de = "mein Schriftmedium"
                    fr = "mon médium d'écriture"
                }
            }
        );
        let property:Result<Property, DatamodelHCLError> = property_block.try_into();
        assert!(property.is_ok());
    }
}
