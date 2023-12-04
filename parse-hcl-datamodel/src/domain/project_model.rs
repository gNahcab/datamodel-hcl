use std::str::FromStr;
use hcl::{Block, Body};
use crate::domain::builders::Builder;
use crate::domain::ontology::{OntologyWrapper, Ontology};

use crate::domain::property::{PropertyWrapper, Property};
use crate::domain::resource::{ResourceWrapper, Resource};
use crate::errors::DatamodelHCLError;
use crate::domain::builders::project_model::ProjectModelBuilder;



#[derive(Debug, PartialEq)]
pub struct ProjectModel {
    pub ontologies: Vec<Ontology>,
    pub properties: Vec<Property>,
    pub resources: Vec<Resource>,

}
impl ProjectModel {
    pub(crate) fn new(ontologies:  Vec<Ontology>,
                      properties:  Vec<Property>,
                      resources:  Vec<Resource>) -> Self {
        ProjectModel{
            ontologies,
            properties,
            resources,
        }
    }
}

impl TryFrom<hcl::Body> for ProjectModel {
    type Error = DatamodelHCLError;
    fn try_from(body: Body) -> Result<Self, Self::Error> {
        // transform a hcl::Body to a ProjectModel
        let mut project_model_builder: ProjectModelBuilder = ProjectModelBuilder::new();

        let attributes: Vec<&hcl::Attribute> = body.attributes().collect();
        for attribute in attributes {
            match attribute.key() {
                _ => return Err(DatamodelHCLError::ParseProjectModel(String::from(format!("found top attribute-name: '{}'. no attributes are allowed on top-level", attribute.key())))),
            }
        }
        let blocks: Vec<&hcl::Block> = body.blocks().collect();
        for block in blocks{
            match block.identifier() {
                "ontology" => {
                    let ontology:Ontology = OntologyWrapper { 0:  block.to_owned()}.to_ontology()?;
                    &project_model_builder.add_to_ontology(ontology);
                }
                "property" => {
                    let property = PropertyWrapper{0: block.to_owned()}.to_property()?;
                    &project_model_builder.add_to_properties(property);
                },
                //todo: add more Resource types, at the moment only 'Resource' and 'StillImageRepresentation' are allowed
                "Resource" | "StillImageRepresentation" => {
                    let resource = ResourceWrapper{0: block.to_owned()}.to_resource()?;
                    &project_model_builder.add_to_resources(resource);
                },
                _ => return Err(DatamodelHCLError::ParseProjectModel(
                    String::from(format!("found invalid block-name: '{}'. Only 'property', 'Resource', 'StillImageRepresentation' allowed", block.identifier())))),
            }
        }
        project_model_builder.build()
    }
}




