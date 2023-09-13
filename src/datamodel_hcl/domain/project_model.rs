use std::str::FromStr;
use hcl::{Attribute, attribute, Block, Body};

use crate::domain::property::Property;
use crate::domain::resource::Resource;
use crate::errors::DatamodelHCLError;



#[derive(Debug, PartialEq)]
pub struct ProjectModel {
    pub ontologies: Vec<std::string::String>,
    pub properties: Vec<Property>,
    pub resources: Vec<Resource>,
}


impl TryFrom<hcl::Body> for ProjectModel {

    type Error = DatamodelHCLError;

    fn try_from(body: Body) -> Result<Self, Self::Error> {
        let mut ontologies: Vec<std::string::String> = vec![];
        let mut properties: Vec<Property> = vec![];
        let mut resources: Vec<Resource> = vec![];

        let attributes: Vec<&hcl::Attribute> = body.attributes().collect();
        for attribute in attributes {
            match attribute.key() {
                "ontology" => ontologies.push(attribute.expr().to_string()),
                _ => (),
            }
        }

        let blocks: Vec<&Block> = body.blocks().collect();
        for block in blocks {
            match block.identifier() {
                "property" => {
                    let property = block.try_into();
                    properties.push(property.unwrap())
                },
                "Resource" => {
                    let resource = block.try_into();
                    resources.push(
                        resource.unwrap())
                },
                "StillImageRepresentation" =>{
                    let still_image = block.try_into();
                    resources.push(still_image.unwrap())
                } ,

                _ => (),
            }
        }
        let project_model = ProjectModel {
            ontologies,
            properties,
            resources,
        };

        Ok(project_model)

    }
}




#[cfg(test)]

mod test {
    use hcl::body;
    #[test]
    fn group_by_ontologies() {
    }
}

