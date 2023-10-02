use std::str::FromStr;
use hcl::{Attribute, attribute, Block, Body};
use crate::domain::ontology::Ontology;

use crate::domain::property::Property;
use crate::domain::resource::Resource;
use crate::errors::DatamodelHCLError;



#[derive(Debug, PartialEq)]
pub struct ProjectModel {
    pub ontologies: Vec<Ontology>,
    pub properties: Vec<Property>,
    pub resources: Vec<Resource>,
}


impl TryFrom<hcl::Body> for ProjectModel {

    type Error = DatamodelHCLError;

    fn try_from(body: Body) -> Result<Self, Self::Error> {
        let mut ontologies: Vec<Ontology> = vec![];
        let mut properties: Vec<Property> = vec![];
        let mut resources: Vec<Resource> = vec![];

        let attributes: Vec<&hcl::Attribute> = body.attributes().collect();
        for attribute in attributes {
            match attribute.key() {
                _ => return Err(DatamodelHCLError::ParseProjectModel(String::from(format!("found invalid attribute-name: '{}'. no attributes are allowed on top-level", attribute.key())))),
            }
        }

        let blocks: Vec<&Block> = body.blocks().collect();
        for block in blocks {
            match block.identifier() {
                "ontology" => {
                   let ontology:Result<Ontology, DatamodelHCLError> = block.try_into();
                    ontologies.push(ontology.unwrap());
                }
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

                _ => return Err(DatamodelHCLError::ParseProjectModel(
                    String::from(format!("found invalid block-name: '{}'. Only 'property', 'Resource', 'StillImageRepresentation' allowed", block.identifier())))),
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
    use hcl::{block, body};
    use crate::errors::DatamodelHCLError;

    #[test]
    fn test_into_project_model() {
        todo!()
    }


}

