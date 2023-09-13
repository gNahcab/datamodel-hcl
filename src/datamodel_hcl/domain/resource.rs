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


