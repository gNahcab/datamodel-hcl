use crate::domain::ontology::Ontology;
use crate::domain::project_model::ProjectModel;
use crate::domain::property::Property;
use crate::domain::resource::Resource;
use crate::errors::DatamodelHCLError;

use super::Builder;

pub struct ProjectModelBuilder {
    pub ontologies: Vec<Ontology>,
    pub properties: Vec<Property>,
    pub resources: Vec<Resource>,
}

// ProjectModelBuilder declares steps for assembling a ProjectModel
impl Builder for ProjectModelBuilder {
    type OutputType = ProjectModel;

     fn new(/* ... */) -> Self {
        // Set the minimally required fields of ProjectModelBuilder.
        ProjectModelBuilder{
            ontologies: vec![],
            properties: vec![],
            resources: vec![],
        }
    }

    fn add_to_ontology(&mut self, ontology: Ontology) {
        &self.ontologies.push(ontology);
    }

    fn add_to_properties(&mut self, property: Property) {
        self.properties.push(property);
    }

    fn add_to_resources(&mut self, resource: Resource) {
        self.resources.push(resource);
    }

    fn project_model_is_correct(&self) -> bool {
        false
    }

fn build(self) -> Result<ProjectModel, DatamodelHCLError> {
    if !self.project_model_is_correct() {
          return Err(DatamodelHCLError::ValidationError(String::from("data model not consistent")))
    }
       Ok(ProjectModel::new(
           self.ontologies,
           self.properties,
           self.resources
       ))
   }
}
#[cfg(test)]

mod test {
    use crate::domain::builders::Builder;
    use crate::domain::builders::project_model::ProjectModelBuilder;

    #[test]
    fn test_project_model_is_correct() {
        let project_model_builder = ProjectModelBuilder::new();
        let result = project_model_builder.build();
        assert!(result.is_ok());
    }
}
