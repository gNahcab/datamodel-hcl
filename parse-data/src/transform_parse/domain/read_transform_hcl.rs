use std::collections::HashMap;
use std::path::Path;
use hcl::{Attribute, Block, Expression, Identifier};
use crate::errors::ParseError;
use crate::transform_parse::domain::builders::Builder;
use crate::transform_parse::domain::builders::transform_hcl::TransformHCLBuilder;

#[derive(Debug)]
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
#[derive(Debug)]
pub enum RowOrResourceName {
    RowNr,
    ResourceName,
}

#[derive(Debug)]
pub struct WorksheetInfo {
    organized_by: OrganizedBy,
    col_row_to_property:  HashMap<usize, String>,
    row_or_resource_name: RowOrResourceName,
    condition_to_code: HashMap<String, String>,

}
#[derive(Debug)]
pub struct TransformHCL {
    worksheets: Vec<WorksheetInfo>,
}

impl TransformHCL {
    pub(crate) fn new(worksheets: Vec<WorksheetInfo>) -> Self {
        TransformHCL{
            worksheets
        }
    }
}

impl TryFrom<hcl::Body> for TransformHCL {
    type Error = ParseError;
    fn try_from(body: hcl::Body) -> Result<Self, Self::Error> {
        let mut transform_builder = TransformHCLBuilder::new();

        let attributes: Vec<&hcl::Attribute> =  body.attributes().collect();
        for attribute in attributes {
            match attribute.key.as_str() {
                "sheets" => {
                transform_builder.add_sheets(attribute.expr().to_string());
                }
                _ => {
                    return Err(ParseError::ValidationError(format!("attribute '{}' with value '{}' not allowed", attribute.expr, attribute.key)));
                }
            }
        }

        let blocks: Vec<&Block> = body.blocks().collect();
        for block in blocks {
            println!("{:?}", block);
        }
        Ok(TransformHCL{ worksheets: vec![] })
        }
    }

#[cfg(test)]
mod test {
    use import_data::errors::DataImportError::PolarsError;
    use crate::errors::ParseError;
    use crate::transform_parse::domain::read_transform_hcl::TransformHCL;

    #[test]
    fn test_read_simple_transform_hcl() {
        let body = hcl::body!(
            sheets = ["all"]
            sheet "1" {
                structured_by = "row"
                resource = "Person"
                }

        );
        let result: Result<TransformHCL, ParseError> = body.try_into();
        println!("{:?}", result);
        assert!(result.is_ok())
    }
    #[test]
    fn test_read_transform_complex() {
        /*
        let transform_hcl = hcl::block!(
            transform "xlsx" {
               sheets = 1,3
                sheet "1" {

                }
                sheet "3" {

                }
            }
        );
*/

    }
}
