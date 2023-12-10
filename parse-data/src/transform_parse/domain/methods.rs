use std::collections::HashMap;
use hcl::{Attribute, Block};
use crate::errors::ParseError;

pub struct WrapperMethods (pub(crate) hcl::Body);

impl WrapperMethods {
    pub fn to_methods(&self) -> Result<Methods, ParseError>{
        let methods: HashMap<String, String> = HashMap::new();
        let attributes: Vec<&Attribute> = self.0.attributes().collect();
        let blocks: Vec<&Block> = self.0.blocks().collect();
        for attribute in attributes{
            match attribute.key.as_str() {

                "add" => {

                }
                _ => {
                    return Err(ParseError::ValidationError(format!("this function does not exist: '{}'", attribute.expr)));
                } }
        }

        Ok(Methods{ methods })

    }
}

pub struct Methods {
    methods: HashMap<String, String>,
}