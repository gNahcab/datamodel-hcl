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

