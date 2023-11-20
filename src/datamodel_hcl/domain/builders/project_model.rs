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

fn are_propnames_consistent_with_properties(properties_names: &Vec<&str>, propnames: Vec<&str>, resource_name: &str) -> Result<(), DatamodelHCLError> {
    let result: Vec<_> = propnames.iter().filter(|propname| properties_names.contains(&propname)).collect();
    if result.len() != 0 {
        return Err(DatamodelHCLError::ValidationError(String::from(format!("propnames '{:?}' in resource '{:?}' don't exist in properties!", result, resource_name))));
    }
    Ok(())
}

fn do_mentioned_ontologies_exist() -> () {
    //ontology-names can be found in ResProp-structures
    unimplemented!()
}
fn project_model_is_correct(project_model_builder: &ProjectModelBuilder) -> Result<(), DatamodelHCLError> {
    // check that ontologies in file are declared as ontologies if mentioned in resource
    let properties_names:Vec<&str> = project_model_builder.properties.iter().map(|property| property.name.as_str()).collect();
    for resource in &project_model_builder.resources {
        let is_consistent_or_not =  are_propnames_consistent_with_properties(&properties_names,resource.res_props.iter().map(|prop| prop.name.as_str()).collect(),resource.name.as_str());
        if is_consistent_or_not.is_err() {
            return is_consistent_or_not;
        }
        println!("resource {:?}", resource.name)
    }

    //do_mentioned_ontologies_exist();
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
    if project_model_is_correct(&self).is_err() {
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
    use crate::domain::builders::project_model::{are_propnames_consistent_with_properties, ProjectModelBuilder};

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
        assert!(result_part.is_ok());

    }
    #[test]
    fn experiment() {
        let properties_names = vec!["property_name_1","property_name_2","property_name_3","property_name_4","property_name_5"];
        let  propnames_of_resource_part= vec!["property_name_1","property_name_2","property_name_7"];
        println!("{:?}", properties_names.contains(&"property_name_1"));
        println!("{:?}", properties_names.iter().filter(|propname| propname == &&"property_name_1"));
        let vec_int = [1,2,3];
        let string_vec = ["a".to_string(), "b".to_string(), "c".to_string()];
        let result: Vec<&String> = string_vec.iter().filter(|string|  &&"b".to_string() == string).collect();
        println!("{:?}", result);



    }


}
