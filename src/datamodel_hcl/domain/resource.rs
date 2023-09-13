use hcl::{Block, Body};
use crate::domain::label::Label;
use crate::domain::res_props::ResProp;
use crate::errors::DatamodelHCLError;

#[derive(Debug, PartialEq)]
pub enum Types {
    Resource,
    StillImageRepresentation
}

#[derive(Debug, PartialEq)]
pub struct Resource{
    pub name: String,
    pub labels: Label,
    pub res_props: Vec<ResProp>,
    pub res_type: Types,
}

impl TryFrom<&hcl::Block> for Resource {
    type Error = DatamodelHCLError;

    fn try_from(value: &Block) -> Result<Self, Self::Error> {
        todo!()
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
      en = ""
      de = ""
      fr = ""
      it = ""
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

