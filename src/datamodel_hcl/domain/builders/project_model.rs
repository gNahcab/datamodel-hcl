use crate::domain::ontology::Ontology;
use crate::domain::project_model::ProjectModel;
use crate::domain::property::Property;
use crate::domain::res_props::ResProp;
use crate::domain::resource::Resource;
use crate::errors::DatamodelHCLError;

use super::Builder;

pub struct ProjectModelBuilder {
    pub ontologies: Vec<Ontology>,
    pub properties: Vec<Property>,
    pub resources: Vec<Resource>,
}

fn are_propnames_consistent_with_properties(property_names: &Vec<&str>, propnames: Vec<&str>, resource_name: &str) -> Result<(), DatamodelHCLError> {
    let result: Vec<_> = propnames.iter().filter(|propname| !property_names.contains(&propname)).collect();
    // result.len() == 0 means all values of Vec<&str> 'propnames' exist in Vec<&str> property_names'
    if result.len() != 0 {
        return Err(DatamodelHCLError::ValidationError(String::from(format!("propnames '{:?}' in resource '{:?}' don't exist in properties!", result, resource_name))));
    }
    Ok(())
}

fn do_ontology_names_of_propnames_exist(ontology_names: &Vec<&str>, ontology_names_res_props: Vec<&str>, resource_name: &str) -> Result<(), DatamodelHCLError> {
    // checks if ontologies that are mentioned down in propnames of resource exist in the datamodel
    let result_res_props: Vec<_> = ontology_names_res_props.iter().filter(|ontology_name_resource| !ontology_names.contains(&ontology_name_resource)).collect();
    // result.len() == 0 means all values of Vec<&str> 'ontology_names_res_props' exist in Vec<&str> ontology_names'

    println!("{:?}",ontology_names);
    println!("{:?}",ontology_names_res_props);
    println!("{:?}",result_res_props);
    if result_res_props.len() != 0 {
        return Err(DatamodelHCLError::ValidationError(String::from(format!("ontology-names of res-prop '{:?}' in resource '{:?}' don't exist in properties!", result_res_props, resource_name))));
    }
    Ok(())
}
fn project_model_is_correct(project_model_builder: &ProjectModelBuilder) -> Result<(), DatamodelHCLError> {
    // check that ontologies in file are declared as ontologies if mentioned in resource
    let property_names:Vec<&str> = project_model_builder.properties.iter().map(|property| property.name.as_str()).collect();
    let ontology_names:Vec<&str> = project_model_builder.ontologies.iter().map(|ontology| ontology.name.as_str()).collect();
    for resource in &project_model_builder.resources {
        println!("1");
         are_propnames_consistent_with_properties(&property_names, resource.res_props.iter().map(|prop| prop.name.as_str()).collect(), resource.name.as_str())?;
        println!("2");
         do_ontology_names_of_propnames_exist(&ontology_names, resource.res_props.iter().map(|prop| prop.ontology.as_str()).collect(), resource.name.as_str())?;
        println!("3");
         if !ontology_names.contains(&resource.ontology.as_str()) {
             println!("4");
             return Err(DatamodelHCLError::ValidationError(String::from(format!("'ontology name '{}' of resource '{}' not defined as ontology", resource.ontology.as_str(), resource.name.as_str()))));
         }
    }
    Ok(())
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


fn build(self) -> Result<ProjectModel, DatamodelHCLError> {
    project_model_is_correct(&self)?;

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
    use crate::domain::builders::project_model::{are_propnames_consistent_with_properties, do_ontology_names_of_propnames_exist, ProjectModelBuilder};

    #[test]
    fn test_project_model_is_correct() {
        let project_model_builder = ProjectModelBuilder::new();
        let result = project_model_builder.build();
        assert!(result.is_ok());
    }

    #[test]
    fn test_propnames_consistent_full() {
        let properties_names:Vec<&str> = vec!["property_name_1","property_name_2","property_name_3","property_name_4","property_name_5"];
        let  propnames_of_resource_full= vec!["property_name_1","property_name_2","property_name_3","property_name_4","property_name_5"];

        let result_full = are_propnames_consistent_with_properties(&properties_names, propnames_of_resource_full, "a_random_resource");
        assert!(result_full.is_ok());

    }
    #[test]
    fn test_propnames_consistent_part() {
        let properties_names = vec!["property_name_1","property_name_2","property_name_3","property_name_4","property_name_5"];
        let  propnames_of_resource_part= vec!["property_name_1","property_name_2","property_name_7"];

        let result_part = are_propnames_consistent_with_properties(&properties_names, propnames_of_resource_part, "a_random_resource");
        assert!(result_part.is_err());

    }
    #[test]
    fn test_mentioned_ontologies_exist() {
        let ontology_names = vec!["rosetta", "social_science_project", "new_project", "history_project"];
        let ontology_names_res_props = vec!["rosetta", "a_project"];
        let result = do_ontology_names_of_propnames_exist(&ontology_names, ontology_names_res_props, "a_random_resource");
        assert!(result.is_err());
    }


}
