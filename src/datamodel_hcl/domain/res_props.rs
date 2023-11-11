use hcl::{Block};
use crate::errors::DatamodelHCLError;

#[derive(Debug, PartialEq)]
pub struct ResProp {
    pub(crate) name: String,
    cardinality: String,
    gui_order: String,
}
pub struct ResPropWrapper (pub(crate) hcl::Block);

impl ResPropWrapper {
    pub fn to_res_prop(&self) -> Result<ResProp, DatamodelHCLError> {
        let propname = self.0.identifier.as_str();
        let attributes:Vec<&hcl::Attribute> = self.0.body.attributes().collect();
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
            name: propname.to_owned(),
            cardinality: cardinality.to_owned(),
            gui_order: gui_order.to_owned(),
        };

        Ok(res_prop)
    }

}

#[cfg(test)]

mod test {
    use hcl::{block};
    use crate::domain::res_props::{ResProp, ResPropWrapper};
    use crate::errors::DatamodelHCLError;

    #[test]
    fn test_into_res_props() {
        let res_props_block = block!(
              hasTitle {
                cardinality = "1"
                gui_order = "0"
            }
        );
        let res_props: Result<ResProp, DatamodelHCLError> = ResPropWrapper{0: res_props_block}.to_res_prop();

        assert!(res_props.is_ok());
    }
}

