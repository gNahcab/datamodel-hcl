use std::todo;
use crate::errors::ParseError;
use crate::transform_parse::domain::read_transform_hcl::{TransformHCL, WorksheetInfo};

pub mod transform_hcl;

pub trait WorksheetBuilderTrait {
    type OutputType;
    fn new(/* ... */) -> Self;
    fn build(self) -> Result<WorksheetInfo, ParseError>;
}
pub trait TransformBuilderTrait {
    type OutputType;
    fn new(/* ... */) -> Self;
    fn build(self) -> Result<TransformHCL, ParseError>;
    fn add_sheets(&mut self, sheets: Vec<String>);
}
