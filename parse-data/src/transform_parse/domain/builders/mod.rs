use std::todo;
use crate::errors::ParseError;
use crate::transform_parse::domain::read_transform_hcl::TransformHCL;

pub mod transform_hcl;

pub trait Builder {
    type OutputType;
    fn new(/* ... */) -> Self;

    fn build(self) -> Result<TransformHCL, ParseError>;
    fn add_sheets(&mut self, p0: String);
}
