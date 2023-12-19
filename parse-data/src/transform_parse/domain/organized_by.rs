use crate::errors::ParsingError;

#[derive(Debug, Copy, Clone)]
pub enum OrganizedBy {
    // two possibilities to structure a table by row or by column,
    // transforming can depend on structure of table
    // for example:
    //
    // by row:
    // ________________________________
    // | rowA1 | rowA2 | rowA3 ] rowA4 |
    // | rowB1 | rowB2 | rowB3 ] rowB4 |
    // |_______________________________|
    //
    // by col:
    // _________________
    // | rowA1 | rowA1 |
    // | rowA2 | rowB2 |
    // | rowA3 | rowB3 |
    // | rowA4 | rowB4 |
    // |_______________|
    //
    ROWOrganized,
    COLOrganized,
}

impl OrganizedBy {
    pub fn organized_by(organized_by_str: String) -> Result<OrganizedBy, ParsingError> {
        let organized_by = match organized_by_str.as_str() {
            "column" => {OrganizedBy::COLOrganized}
            "row" => {OrganizedBy::ROWOrganized}
            _ => {return Err(ParsingError::ValidationError(format!("couldn't parse '{:?}' to organized_by: only 'row' and 'column' allowed", organized_by_str)))}
        };
        Ok(organized_by)
    }
}
