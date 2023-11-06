use hcl::{Block};
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
    pub fn new(name: &str, labels: Vec<Label>, res_props: Vec<ResProp>, res_type: String ) -> Self {
        Self{
            name:String::from(name),
            labels,
            res_props,
            res_type,
        }
    }
}

pub(crate) struct ResourceWrapper(pub(crate) hcl::Block);

impl ResourceWrapper {
    pub fn to_resource(self) -> Result<Resource, DatamodelHCLError> {
        let resource_type = self.0.identifier.to_string();
        // Resource name
        let name  =
            self.0.labels().get(0).ok_or(DatamodelHCLError::ParseProjectModel(
                String::from(format!("couldn't parse name of resource: '{:?}'", self.0.labels()))));
        let name = name.unwrap().as_str();
        // prepare for inner block
        let blocks: Vec<&Block> = self.0.body.blocks().collect();

        // read label
        let label_block = blocks.get(0).ok_or(
            DatamodelHCLError::ParseProjectModel(String::from(format!("couldn't read label of resource '{:?}'. Does Label exist?", self.0.labels()))));

        let labelBlockWrapper = LabelBlockWrapper{ 0: label_block.unwrap().to_owned().to_owned()};
        let labels = labelBlockWrapper.to_labels()?;

        // read resource-properties
        let mut res_props = vec![];
        let mut counter = 1;
        while counter < blocks.len() {
            let block = blocks.get(counter).ok_or(DatamodelHCLError::ParseProjectModel(String::from("couldn't read from block resource")));
            let block = *block?;
            counter  += 1;
            let res_prop_wrapper = ResPropWrapper{ 0: block.to_owned() };
            let res_prop: ResProp = res_prop_wrapper.to_res_prop()?;
            res_props.push(res_prop);
        }

        let resource = Resource::new(name, labels, res_props, resource_type);
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
    labels {
      en = "add label"
      de = "hinzufügen"
      fr = "ajouter"
      it = "aggiungere"
    }
      hasTitle {
        cardinality = "1"
        gui_order = "0"
      }
  }
        );
        let resource:Result<Resource, DatamodelHCLError> = ResourceWrapper{0: resource_block}.to_resource();
        assert!(resource.is_ok())

    }
}

