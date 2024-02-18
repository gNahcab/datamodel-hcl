use crate::errors::ParsingError;
use crate::expression_trait::ExpressionTransform;

#[derive(Debug, Clone, PartialEq)]
pub struct ResProp {
    pub name: String,
    pub(crate) cardinality: String,
    pub(crate) gui_order: usize,
    pub(crate) ontology: String,
}

pub struct ResPropWrapper (pub(crate) hcl::Block);
#[derive(Debug)]
struct TransientStructureResProp {
    propname: String,
    cardinality: Option<String>,
    gui_order: Option<usize>,
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
    pub(crate) fn add_ontology(&mut self, ontology: String) -> Result<(), ParsingError> {
        if !self.ontology.is_none() {
            return Err(ParsingError::ValidationError(String::from("multiple ontologies were provided to res_prop")));
        }
        self.ontology = Option::from(ontology);
        Ok(())
    }
    pub(crate) fn add_gui_order(&mut self, gui_order: String) -> Result<(), ParsingError> {
        if !self.gui_order.is_none() {
            return Err(ParsingError::ValidationError(String::from("multiple gui_orders were provided to res_prop")));
        }
        let gui_order_maybe = gui_order.parse::<usize>();
        let gui_order = match gui_order_maybe {
            Ok(value) => {value}
            Err(_) => {
                return Err(ParsingError::ValidationError(String::from(format!("cannot parse this gui_order-expression '{:?}' to usize. Is it a number?", gui_order))));
            }
        };
        self.gui_order = Option::from(gui_order);
        Ok(())
    }
    pub(crate) fn add_cardinality(&mut self, cardinality: String) -> Result<(), ParsingError> {
        if !self.cardinality.is_none() {
            return Err(ParsingError::ValidationError(String::from("multiple cardinalities was provided to res_prop")));
        }
        self.cardinality = Option::from(cardinality);
        Ok(())
    }

    pub(crate) fn is_complete(&self) -> Result<(), ParsingError> {
        // check if the TransientStructure can be converted into a ResProp-Structure
        if self.propname.is_empty() {
            return Err(ParsingError::ValidationError(String::from(format!("propname doesn't exist or isn't provided correctly in '{:?}'", self))));
        }
        if self.ontology.is_none() {
            return Err(ParsingError::ValidationError(String::from(format!("ontology doesn't exist or isn't provided correctly in '{:?}'", self))));
        }

        if self.cardinality.is_none() {
            return Err(ParsingError::ValidationError(String::from(format!("cardinality name doesn't exist or isn't provided correctly in '{:?}'", self))));
        }

        if self.gui_order.is_none() {
            return Err(ParsingError::ValidationError(String::from(format!("gui_order doesn't exist or isn't provided correctly in '{:?}'", self))));
        }

        Ok(())
    }
}
impl ResPropWrapper {
    pub fn to_res_prop(&self) -> Result<ResProp, ParsingError> {
        let attributes:Vec<&hcl::Attribute> = self.0.body.attributes().collect();
        let mut transient_structure = TransientStructureResProp::new();
        transient_structure.add_propname(self.0.identifier.as_str());


        for attribute in attributes {
        match attribute.key() {
                "cardinality" => transient_structure.add_cardinality(attribute.expr.to_string_2()?)? ,
                "gui_order" => transient_structure.add_gui_order(attribute.expr.to_string_2()?)? ,
                "ontology" => transient_structure.add_ontology(attribute.expr.to_string_2()?)? ,
                _ => return Err(
                    ParsingError::ParseProjectModel(
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
    use crate::datamodel_parse::domain::res_props::{ResProp, ResPropWrapper};
    use crate::errors::ParsingError;

    #[test]
    fn test_into_res_props() {
        let res_props_block = block!(
              hasTitle {
                ontology = "rosetta"
                cardinality = "1"
                gui_order = "0"
            }
        );
        let res_props: Result<ResProp, ParsingError> = ResPropWrapper{0: res_props_block}.to_res_prop();
        assert!(res_props.is_ok());
        assert!(res_props.as_ref().is_ok());
        assert_eq!(res_props.as_ref().unwrap().name, "hasTitle");
        assert_eq!(res_props.as_ref().unwrap().ontology, "rosetta");
        assert_eq!(res_props.as_ref().unwrap().cardinality, "1");
        assert_eq!(res_props.as_ref().unwrap().gui_order, 0);
    }
}

