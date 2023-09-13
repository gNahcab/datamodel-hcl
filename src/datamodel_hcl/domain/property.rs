use hcl::{Block};
use crate::domain::label::Label;
use crate::errors::DatamodelHCLError;

#[derive(Debug, PartialEq)]
pub struct Property {
    pub name: String,
    pub ontology: String,
    pub object: String,
    pub labels: Vec<Label>,
    pub gui_element: String,
}

impl TryFrom<&hcl::Block> for Property {
    type Error = DatamodelHCLError;

    fn try_from(value: &Block) -> Result<Self, Self::Error> {
        todo!()
    }
}
