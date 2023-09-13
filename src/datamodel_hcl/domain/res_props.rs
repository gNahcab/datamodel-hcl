use clap::builder::Str;
use hcl::{Block, Body};
use crate::errors::DatamodelHCLError;

#[derive(Debug, PartialEq)]
pub struct ResProp {
    name: String,
    cardinality: String,
    gui_order: u8,
}

impl TryFrom<&hcl::Block> for ResProp {
    type Error = DatamodelHCLError;

    fn try_from(value: &Block) -> Result<Self, Self::Error> {
        todo!()
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

