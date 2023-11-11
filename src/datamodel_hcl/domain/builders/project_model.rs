use std::any::type_name;
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

    fn project_model_is_correct(&self) -> Result<(), DatamodelHCLError> {
        // check if propnames of resources consistent with properties
        //todo get a vec of all properties names and check if every prop-name of a resource is part of properties
        let mut properties_names = vec![];
        for _property in &self.properties {
            properties_names.push(_property.name.to_string());
        }
        for _resource in &self.resources {
            for _res_prop in &_resource.res_props {
                if !properties_names.contains(&_res_prop.name) {
                    return Err(DatamodelHCLError::ValidationError(
                        String::from(format!("resource-prop '{:?}' of resource {:?}",
                                             _res_prop, _resource.name))));
                }
            }
        }
        // check that ontologies in file are declared as ontologies if mentioned in resource

        Ok(())
    }

fn build(self) -> Result<ProjectModel, DatamodelHCLError> {
    if !self.project_model_is_correct().is_ok() {
          return Err(DatamodelHCLError::ValidationError(String::from("cannot build: data model not consistent")))
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
