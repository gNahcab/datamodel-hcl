use crate::errors::DatamodelHCLError;

#[derive(Debug, PartialEq)]
pub struct ResProp {
    pub(crate) name: String,
    cardinality: String,
    gui_order: String,
    ontology: String,
}

pub struct ResPropWrapper (pub(crate) hcl::Block);
#[derive(Debug)]
struct TransientStructureResProp {
    propname: String,
    cardinality: Option<String>,
    gui_order: Option<String>,
    ontology: Option<String>,
}

impl TransientStructureResProp {
    fn new() -> TransientStructureResProp {
        TransientStructureResProp {
            propname: "".to_string(),
            cardinality: None,
            gui_order: None,
            ontology: None,
        }
    }

    pub(crate) fn add_propname(&mut self, new_propname: &str) {
        self.propname = new_propname.to_string();
    }
    pub(crate) fn add_ontology(&mut self, new_ontology: String) -> Result<(), DatamodelHCLError> {
        if !self.ontology.is_none() {
            return Err(DatamodelHCLError::ValidationError(String::from("multiple ontologies were provided to res_prop")));
        }
        self.ontology = Option::from(new_ontology);
        Ok(())
    }
    pub(crate) fn add_gui_order(&mut self, new_gui_order: String) -> Result<(), DatamodelHCLError> {
        if !self.gui_order.is_none() {
            return Err(DatamodelHCLError::ValidationError(String::from("multiple gui_orders were provided to res_prop")));
        }
        self.gui_order = Option::from(new_gui_order);
        Ok(())
    }
    pub(crate) fn add_cardinality(&mut self, new_cardinality: String) -> Result<(), DatamodelHCLError> {
        if !self.cardinality.is_none() {
            return Err(DatamodelHCLError::ValidationError(String::from("multiple cardinalities was provided to res_prop")));
        }
        self.cardinality = Option::from(new_cardinality);
        Ok(())
    }

    pub(crate) fn is_complete(&self) -> Result<(), DatamodelHCLError> {
        // check if the TransientStructure can be converted into a ResProp-Structure
        if self.propname.is_empty() {
            return Err(DatamodelHCLError::ValidationError(String::from(format!("propname doesn't exist or isn't provided correctly in '{:?}'", self))));
        }
        if self.ontology.is_none() {
            return Err(DatamodelHCLError::ValidationError(String::from(format!("ontology doesn't exist or isn't provided correctly in '{:?}'", self))));
        }

        if self.cardinality.is_none() {
            return Err(DatamodelHCLError::ValidationError(String::from(format!("cardinality name doesn't exist or isn't provided correctly in '{:?}'", self))));
        }

        if self.gui_order.is_none() {
            return Err(DatamodelHCLError::ValidationError(String::from(format!("gui_order doesn't exist or isn't provided correctly in '{:?}'", self))));
        }

        Ok(())
    }

}
impl ResPropWrapper {
    pub fn to_res_prop(&self) -> Result<ResProp, DatamodelHCLError> {
        let attributes:Vec<&hcl::Attribute> = self.0.body.attributes().collect();
        let mut transient_structure = TransientStructureResProp::new();
        transient_structure.add_propname(self.0.identifier.as_str());


        for attribute in attributes {
        match attribute.key() {
                "cardinality" => transient_structure.add_cardinality(attribute.expr.to_string())? ,
                "gui_order" => transient_structure.add_gui_order(attribute.expr.to_string())? ,
                "ontology" => transient_structure.add_ontology(attribute.expr.to_string())? ,
                _ => return Err(
                    DatamodelHCLError::ParseProjectModel(
                        String::from(
                            format!(
                                "invalid attribute:'{:?}'.\
                                 Only 'cardinality', 'gui_order' and 'ontology' are valid.", attribute.key()))))};
        }

        transient_structure.is_complete()?;

       let res_prop = ResProp{
            name:  transient_structure.propname.to_owned(),
            cardinality:transient_structure.cardinality.unwrap().to_owned(),
            gui_order:transient_structure.gui_order.as_ref().unwrap().to_owned(),
           ontology:transient_structure.ontology.as_ref().unwrap().to_owned(),
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

