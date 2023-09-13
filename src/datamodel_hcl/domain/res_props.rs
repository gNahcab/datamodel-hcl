use std::num::ParseIntError;
use clap::builder::Str;
use hcl::{Attribute, Block, Body, body, Error};
use crate::errors::DatamodelHCLError;

#[derive(Debug, PartialEq)]
pub struct ResProp {
    name: String,
    cardinality: String,
    gui_order: String,
}

impl TryFrom<&hcl::Block> for ResProp {
    type Error = DatamodelHCLError;

    fn try_from(block: &Block) -> Result<Self, Self::Error> {
        let propname = block.identifier.as_str();
        let attributes:Vec<&hcl::Attribute> = block.body.attributes().collect();
        let mut cardinality = std::string::String::from("");
        let mut gui_order =  std::string::String::from("");
        for attribute in attributes {
            match attribute.key() {
                "cardinality" => cardinality = attribute.expr().to_string(),
                "gui_order" => gui_order = attribute.expr().to_string(),
                _ => return Err(
                    DatamodelHCLError::ParseProjectModel(
                        String::from(
                            format!(
                                "invalid attribute:'{:?}'.\
                                 Only 'cardinality and 'gui_order' are valid.", attribute.key()))))}

        }



        let res_prop = ResProp{
            name: String::from(propname),
            cardinality: String::from(cardinality),
            gui_order: gui_order,
        };
        Ok(res_prop)

    }
}
#[cfg(test)]

mod test {
    use hcl::{block, body};
    use crate::domain::res_props::ResProp;
    use crate::errors::DatamodelHCLError;

    #[test]
    fn test_into_res_props() {
        let res_props_block = &block!(
              hasTitle {
                cardinality = "1"
                gui_order = "0"
            }
        );
        let res_props: Result<ResProp, DatamodelHCLError> = res_props_block.try_into();

        assert!(res_props.is_ok());
    }
}

