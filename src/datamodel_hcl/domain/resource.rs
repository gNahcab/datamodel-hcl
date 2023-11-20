use hcl::{Attribute, Block, BlockLabel};
use crate::domain::label::{Label, LabelBlockWrapper};
use crate::domain::res_props::{ResProp, ResPropWrapper};
use crate::errors::DatamodelHCLError;


#[derive(Debug, PartialEq)]
pub struct Resource{
    pub name: String,
    pub labels: Vec<Label>,
    pub res_props: Vec<ResProp>,
    pub res_type: String,
}


impl Resource {
    pub fn new(name: String, labels: Vec<Label>, res_props: Vec<ResProp>, res_type: String ) -> Self {
        Self{
            name:String::from(name),
            labels,
            res_props,
            res_type,
        }
    }
}

#[derive(Debug)]
struct TransientStructureResource {
    name: Option<String>,
    labels: Vec<Label>,
    res_props: Vec<ResProp>,
    res_type: Option<String>,
    ontology: Option<String>,
}

impl TransientStructureResource {
}

impl TransientStructureResource {
}

impl TransientStructureResource {
    fn new() -> TransientStructureResource {
        TransientStructureResource{
            name: None,
            labels: vec![],
            res_props: vec![],
            res_type: None,
            ontology: None,
        }
    }
    pub(crate) fn add_name(&mut self, name_label: Result<&BlockLabel, DatamodelHCLError>) {
        self.name = Option::from(name_label.unwrap().as_str().to_string());
    }
    pub(crate) fn add_res_type(&mut self, identifier: String) {
        self.res_type = Option::from(identifier);

    }
    pub(crate) fn add_ontology(&mut self, onto_string: String) -> Result<(), DatamodelHCLError> {
        if !self.ontology.is_none() {
            return Err(DatamodelHCLError::ValidationError(format!("ontology should exist once, found more than once for '{:?}'", self)));
        }
        self.ontology = Option::from(onto_string);
        Ok(())
    }
    pub(crate) fn add_labels(&mut self, labels: Vec<Label>) -> Result<(), DatamodelHCLError> {
        if !self.labels.is_empty() {
            return Err(DatamodelHCLError::ValidationError(format!("labels should exist once, found more than once for '{:?}'", self)));
        }
        self.labels = labels;
        Ok(())
    }
    pub(crate) fn add_res_prop(&mut self, res_prop: ResProp) {
        self.res_props.push(res_prop);
    }
    pub(crate) fn is_consistent(&self) -> Result<(), DatamodelHCLError> {
        if self.name.is_none() {
            return Err(DatamodelHCLError::ValidationError(format!("couldn't find name for resource '{:?}'", self)));
        }
        if self.res_type.is_none() {
            return Err(DatamodelHCLError::ValidationError(format!("couldn't find res_type for resource '{:?}'", self)));
        }
        if self.ontology.is_none() {
            return Err(DatamodelHCLError::ValidationError(format!("couldn't find ontology for resource '{:?}'", self)));
        }
        if self.labels.len() == 0 {
            return Err(DatamodelHCLError::ValidationError(format!("couldn't find labels for resource '{:?}'", self)));
        }
        if self.res_props.len() == 0 {
            return Err(DatamodelHCLError::ValidationError(format!("couldn't find res_props for resource '{:?}'", self)));
        }
        Ok(())
    }
}
pub(crate) struct ResourceWrapper(pub(crate) hcl::Block);

impl ResourceWrapper {
    pub fn to_resource(self) -> Result<Resource, DatamodelHCLError> {
        let mut transient_structure_resource = TransientStructureResource::new();
        transient_structure_resource.add_res_type(self.0.identifier.to_string());
        // Resource name
        transient_structure_resource.add_name(self.0.labels().get(0).ok_or(DatamodelHCLError::ParseProjectModel(
            String::from(format!("couldn't parse name of resource: '{:?}'", self.0.labels())))));

        let labels: Vec<&Attribute> = self.0.body.attributes().collect();
        for label in &labels {
            match label.key.as_str() {
                "ontology" => {
                    transient_structure_resource.add_ontology(label.expr.to_string());
                }
                _ => {
                    return Err(DatamodelHCLError::ParseProjectModel(
            String::from(format!("only one ontology-attribute is allowed here but found: '{:?}'", labels))));
                }
            }
        }

        // prepare for inner block
        let blocks: Vec<&Block> = self.0.body.blocks().collect();
        for block in blocks {
            match block.identifier.as_str() {
                "labels" => transient_structure_resource.add_labels(  LabelBlockWrapper{0: block.to_owned().to_owned()}.to_labels()?)?,
                // it it is not labels, it must be a res_prop
                _ => {
                    let res_prop_wrapper = ResPropWrapper{ 0: block.to_owned() };
                    let res_prop: ResProp = res_prop_wrapper.to_res_prop()?;
                    transient_structure_resource.add_res_prop(res_prop);
                } }
        }
        transient_structure_resource.is_consistent()?;

        let resource = Resource::new(transient_structure_resource.name.unwrap(), transient_structure_resource.labels, transient_structure_resource.res_props, transient_structure_resource.res_type.unwrap());
        Ok(resource)
    }
}

#[cfg(test)]

mod test {
    use hcl::{block};
    use crate::domain::resource::{Resource, ResourceWrapper};
    use crate::errors::DatamodelHCLError;

    #[test]
    fn test_into_resource() {
        let resource_block = block!(
             StillImageRepresentation "Image2D" {
                ontology = "rosetta"
    labels {
      en = "add label"
      de = "hinzufügen"
      fr = "ajouter"
      it = "aggiungere"
    }
      hasTitle {
        cardinality = "1"
        gui_order = "0"
        ontology = "rosetta"

      }
  }
        );
        let resource:Result<Resource, DatamodelHCLError> = ResourceWrapper{0: resource_block}.to_resource();
        println!("{:?}", resource.as_ref().unwrap().labels.get(0).unwrap().text.to_string() == "add label");
        assert!(resource.is_ok())

    }
}

