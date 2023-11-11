use std::unimplemented;
use crate::domain::builders::project_model::ProjectModelBuilder;
use crate::domain::ontology::Ontology;
use crate::domain::project_model::ProjectModel;
use crate::domain::property::Property;
use crate::domain::resource::Resource;
use crate::errors::DatamodelHCLError;

pub mod project_model;


pub trait Builder {
    type OutputType;
    fn new(/* ... */) -> Self;
    fn add_to_ontology(&mut self, ontology: Ontology);
    fn add_to_properties(&mut self, property:Property);
    fn add_to_resources(&mut self, resource:Resource);

    fn project_model_is_correct(&self) -> Result<(), DatamodelHCLError>;
    fn build(self) -> Result<ProjectModel, DatamodelHCLError>;
}