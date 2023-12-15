use hcl::{Attribute, Block};
use crate::errors::ParseError;
pub trait Wrapper {
    fn get_output(&self) -> Result<String, ParseError>;
    fn no_blocks(&self) -> Result<(), ParseError> ;
    fn blocks(&self) -> Vec<&Block>;
    fn attributes(&self) -> Vec<&Attribute>;
}
impl Wrapper for Block  {
    fn get_output(&self) -> Result<String, ParseError> {
        // returns the output variable of the method, this is the variable with which we can call the column/row of data
        // error if no label or more than one label was found
        if self.labels.len() == 0 {
            return Err(ParseError::ValidationError(format!("no label found for method: '{:?}'", self)));
        }
        if self.labels.len() > 1 {
            return Err(ParseError::ValidationError(format!("this method should have one label but has more than one: '{:?}'", self.labels)));
        }
        return Ok(self.labels.get(0).unwrap().as_str().to_string());
    }

    fn no_blocks(&self) -> Result<(), ParseError> {
        // check that no block exists within this method-block
        let blocks: Vec<&Block> = self.blocks();
        if blocks.len() != 0 {
            return Err(ParseError::ValidationError(format!("found those blocks '{:?}' in method '{:?}', but blocks are not allowed.", blocks, self)));
        }
        Ok(())
    }
    fn blocks(&self) -> Vec<&Block> {
        return self.body.blocks().collect();
    }
    fn attributes(&self) -> Vec<&Attribute> {
        return self.body.attributes().collect();
    }
}
