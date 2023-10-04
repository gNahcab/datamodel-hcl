use hcl::{Attribute, Block};
use crate::domain::label::Label;
use crate::domain::res_props::ResProp;
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

impl TryFrom<&hcl::Block> for Resource {
    type Error = DatamodelHCLError;

    fn try_from(block: &Block) -> Result<Self, Self::Error> {
        let resource_type = block.identifier.to_string();
        // Resource name
        let name  =
            block.labels().get(0).ok_or(DatamodelHCLError::ParseProjectModel(
                String::from(format!("couldn't parse name of resource: '{:?}'", block.labels()))));
        let name = name.unwrap().as_str();
        // prepare for inner block
        let mut res_labels:Vec<Label> = vec![];
        let blocks: Vec<&Block> = block.body.blocks().collect();

        // read label
        let label_block = blocks.get(0).ok_or(
            DatamodelHCLError::ParseProjectModel(String::from(format!("couldn't read label of resource '{:?}'. Does Label exist?", block.labels()))));

        let attributes: Vec<&Attribute> = label_block?.body.attributes().collect();
        for attribute in attributes {
            let label: Label = attribute.try_into()?;
            res_labels.push(label);
        }

        // read resource-properties
        let mut res_props = vec![];
        let mut counter = 1;
        while counter < blocks.len() {
            let block = blocks.get(counter).ok_or(DatamodelHCLError::ParseProjectModel(String::from("couldn't read from block resource")));
            let block = *block?;
            counter  += 1;
            let res_prop: ResProp = block.try_into()?;
            res_props.push(res_prop);
        }

        let resource = Resource::new(name, res_labels, res_props, resource_type);
        Ok(resource)
    }
}

#[cfg(test)]

mod test {
    use hcl::{block};
    use crate::domain::resource::Resource;
    use crate::errors::DatamodelHCLError;

    #[test]
    fn test_into_resource() {
        let resource_block = &block!(
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

        let resource:Result<Resource, DatamodelHCLError> = resource_block.try_into();

        assert!(resource.is_ok())

    }
}

