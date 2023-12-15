use hcl::Block;
use crate::errors::ParseError;
pub trait Wrapper {
    fn get_output(&self) -> Result<String, ParseError>;
    fn no_blocks(&self) -> Result<(), ParseError> ;

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
        let blocks: Vec<&Block> = self.body.blocks().collect();
        if blocks.len() != 0 {
            return Err(ParseError::ValidationError(format!("found those selfs '{:?}' in method '{:?}', but selfs are not allowed.", blocks, self)));
        }
        Ok(())
    }
}
