pub(crate) mod project_model;

use crate::datamodel_parse::domain::ontology::Ontology;
use crate::datamodel_parse::domain::project_model::ProjectModel;
use crate::datamodel_parse::domain::property::Property;
use crate::datamodel_parse::domain::resource::Resource;
use crate::errors::ParseError;



pub trait Builder {
    type OutputType;
    fn new(/* ... */) -> Self;
    fn add_to_ontology(&mut self, ontology: Ontology);
    fn add_to_properties(&mut self, property:Property);
    fn add_to_resources(&mut self, resource:Resource);

    fn build(self) -> Result<ProjectModel, ParseError>;
}