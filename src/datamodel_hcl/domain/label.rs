use hcl::{Block, Body};
use crate::errors::DatamodelHCLError;

#[derive(Debug, PartialEq)]
pub struct Label{
    language_abbr: String,
    text: String,
}

impl TryFrom<&hcl::Block> for Label {
    type Error = DatamodelHCLError;

    fn try_from(value: &Block) -> Result<Self, Self::Error> {
        todo!()
    }
}
