use hcl::{Attribute, Block};
use crate::errors::ParseError;
use crate::transform_parse::domain::methods_domain::combine_method::{CombineMethod, WrapperCombineMethod};
use crate::transform_parse::domain::methods_domain::lower_upper_method::{LowerMethod, UpperMethod, WrapperLowerUpperMethod};
use crate::transform_parse::domain::methods_domain::replace_method::{ReplaceMethod, WrapperReplaceMethod};
use crate::transform_parse::domain::methods_domain::to_date_method::{ToDateMethod, WrapperToDateMethod};

#[derive(Debug)]
pub struct TransformationsWrapper (pub(crate) Block);
#[derive(Debug)]
pub struct Transformations{
    lower_methods:Vec<LowerMethod>,
    upper_methods:Vec<UpperMethod>,
    combine_methods:Vec<CombineMethod>,
    replace_methods:Vec<ReplaceMethod>,
    to_date_methods:Vec<ToDateMethod>,
}

impl Transformations {
    fn new() -> Transformations {
        return Transformations {
            lower_methods: vec![],
            upper_methods: vec![],
            combine_methods: vec![],
            replace_methods: vec![],
            to_date_methods: vec![],
        }
    }
    pub(crate) fn add_lower_method(&mut self, lower_method: LowerMethod) {
        self.lower_methods.push(lower_method);
    }
    pub(crate) fn add_upper_method(&mut self, upper_method: UpperMethod) {
        self.upper_methods.push(upper_method);
    }
    pub(crate) fn add_combine_method(&mut self, combine_method: CombineMethod) {
        self.combine_methods.push(combine_method);
    }
    pub(crate) fn add_replace_method(&mut self, replace_method: ReplaceMethod) {
        self.replace_methods.push(replace_method);
    }
    pub(crate) fn add_to_date_method(&mut self, to_date_method: ToDateMethod) {
        self.to_date_methods.push(to_date_method);
    }

}
impl TransformationsWrapper {
    pub fn to_transformations(&self) -> Result<Transformations, ParseError> {
        let mut transformations: Transformations = Transformations::new();
        let attributes: Vec<&Attribute> = self.0.body.attributes().collect();
        if attributes.len() !=0 {
            return Err(ParseError::ValidationError(format!("found attributes in transformations, but only blocks allowed. Found attributes are: '{:?}'", attributes)));
        }
        let blocks: Vec<&Block> = self.0.body.blocks().collect();
        if blocks.len() == 0 {
            return Err(ParseError::ValidationError(format!("found zero blocks in transformations, but blocks should exist in: '{:?}'", self.0)));
        }
        for block in blocks {
             match block.identifier.as_str() {
                "lower" => {
                   let lower_method = WrapperLowerUpperMethod(block.to_owned()).to_lower_method()?;
                   transformations.add_lower_method(lower_method);
                }
                "upper" => {
                    let upper_method = WrapperLowerUpperMethod(block.to_owned()).to_upper_method()?;
                    transformations.add_upper_method(upper_method);
                }
                "combine"=> {
                    let combine_method = WrapperCombineMethod(block.to_owned()).to_combine_method()?;
                    transformations.add_combine_method(combine_method);
                }
                "replace"=> {
                    let replace_method = WrapperReplaceMethod(block.to_owned()).to_replace_method()?;
                    transformations.add_replace_method(replace_method);
                }
                "to_date"=> {
                    let to_date_method = WrapperToDateMethod(block.to_owned()).to_date_method()?;
                    transformations.add_to_date_method(to_date_method);
                }
                _ => {
                    return Err(ParseError::ValidationError(format!("unknown method found in transformations: can't find '{:?}'", block.identifier)));
                }
            };
        }
        Ok(transformations)
    }
}