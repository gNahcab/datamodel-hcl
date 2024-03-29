use crate::datamodel_parse::domain::ontology::Ontology;
use crate::datamodel_parse::domain::project_model::ProjectModel;
use crate::datamodel_parse::domain::property::Property;
use crate::datamodel_parse::domain::res_props::ResProp;
use crate::datamodel_parse::domain::resource::Resource;
use crate::errors::ParsingError;

use super::Builder;

pub struct ProjectModelBuilder {
    pub ontologies: Vec<Ontology>,
    pub properties: Vec<Property>,
    pub resources: Vec<Resource>,
}

fn are_propnames_consistent_with_properties(property_names: &Vec<&str>, propnames: Vec<&str>, resource_name: &str) -> Result<(), ParsingError> {
    let result: Vec<_> = propnames.iter().filter(|propname| !property_names.contains(&propname)).collect();
    // result.len() == 0 means all values of Vec<&str> 'propnames' exist in Vec<&str> property_names'
    if result.len() != 0 {
        return Err(ParsingError::ValidationError(String::from(format!("propnames '{:?}' in resource '{:?}' don't exist in properties!", result, resource_name))));
    }
    Ok(())
}

fn do_ontology_names_of_propnames_exist(ontology_names: &Vec<&str>, ontology_names_res_props: Vec<&str>, resource_name: &str) -> Result<(), ParsingError> {
    // checks if ontologies that are mentioned down in propnames of resource exist in the datamodel
    let result_res_props: Vec<_> = ontology_names_res_props.iter().filter(|ontology_name_resource| !ontology_names.contains(&ontology_name_resource)).collect();
    // result.len() == 0 means all values of Vec<&str> 'ontology_names_res_props' exist in Vec<&str> ontology_names'
    if result_res_props.len() != 0 {
        return Err(ParsingError::ValidationError(String::from(format!("ontology-names of res-prop '{:?}' in resource '{:?}' don't exist in properties!", result_res_props, resource_name))));
    }
    Ok(())
}
fn project_model_is_correct(project_model_builder: &ProjectModelBuilder) -> Result<(), ParsingError> {
    let resource_names:Vec<&str> = project_model_builder.resources.iter().map(|resource| resource.name.as_str()).collect();
    are_properties_correct(project_model_builder.properties.iter().collect(),resource_names)?;
    let property_names:Vec<&str> = project_model_builder.properties.iter().map(|property| property.name.as_str()).collect();
    let ontology_names:Vec<&str> = project_model_builder.ontologies.iter().map(|ontology| ontology.name.as_str()).collect();
    for resource in &project_model_builder.resources {
        is_resource_correct(&resource)?;
         are_propnames_consistent_with_properties(&property_names, resource.res_props.iter().map(|prop| prop.name.as_str()).collect(), resource.name.as_str())?;
        // check if ontologies in res-props exist
         do_ontology_names_of_propnames_exist(&ontology_names, resource.res_props.iter().map(|prop| prop.ontology.as_str()).collect(), resource.name.as_str())?;
         if !ontology_names.contains(&resource.ontology.as_str()) {
             // check if ontology of resource exists
             return Err(ParsingError::ValidationError(String::from(format!("'ontology name '{}' of resource '{}' not defined as ontology", resource.ontology.as_str(), resource.name.as_str()))));
         }
    }
    Ok(())
}

fn are_properties_correct(properties: Vec<&Property>, resource_names: Vec<&str>) -> Result<(), ParsingError> {
    // todo: add all knora_objects
    let knora_objects = [ "BooleanValue","ColorValue", "DateValue","DecimalValue","GeonameValue","IntValue","IntervalValue","ListValue","TextValue","TimeValue","UriValue","AudioRepresentation", "MovingImageRepresentation", "StillImageRepresentation", "Region", "Representation", "Resource"];
    for property in properties {
       let object: &String = &property.object;
        if knora_objects.contains(&object.as_str()) {
            continue;
        }
         if contains_prefix_other_datamodel(object) {
            // don't check for datamodel-prefixes
            continue;
        } else {
            // check if object exists in resources
            let reduced = object.as_str().split_at(1);
            // remove ":", to check if it exists in resources
            if !resource_names.contains(&&reduced.1) {
                return Err(ParsingError::ValidationError(format!("resource '{}' is referenced in property '{}' but it doesn't exist in resources", object, property.name)));
            }
        }
        // gui-element
        let gui_element_objects = ["DecimalValue", "IntValue", "TextValue"];
        if gui_element_objects.contains(&&**object) && property.gui_element.is_none() {
            return Err(ParsingError::ValidationError(format!("property '{:?}' has 'object' '{:?}' but gui_element doesn't exist.", property.name, object)));
        }
        if !gui_element_objects.contains(&&**object) && property.gui_element.is_some() {
            return Err(ParsingError::ValidationError(format!("property '{:?}' has 'object' '{:?}' but gui_element exists, gui_element is only used with these objects: '{:?}'", property.name, object, gui_element_objects)));
        }
        let text_value_gui_elements = ["Richtext", "Textarea", "Simpletext"];
        if object == "TextValue" && !text_value_gui_elements.contains(&&**property.gui_element.as_ref().unwrap()) {
            return Err(ParsingError::ValidationError(format!("property '{:?}' has 'object' '{:?}' and gui_element is '{:?}' but it should be one of these: '{:?}'", property.name, object, property.gui_element.as_ref().unwrap(), text_value_gui_elements)));
        }
        let int_value_gui_elements = ["Spinbox", "Simpletext"];
        if object == "DecimalValue" || object == "IntValue" && !int_value_gui_elements.contains(&&**property.gui_element.as_ref().unwrap()) {
            return Err(ParsingError::ValidationError(format!("property '{:?}' has 'object' '{:?}' and gui_element is '{:?}' but it should be one of these: '{:?}'", property.name, object, property.gui_element.as_ref().unwrap(), text_value_gui_elements)));
        }
    }
    Ok(())
}

fn contains_prefix_other_datamodel(object_string: &String) -> bool {
    let pos = object_string.find(":");
    if pos.is_none() {
        return false;
    }
    if pos.unwrap() == 0 {
        return false;
    }
    return true;
}

fn is_resource_correct(resource: &&Resource) -> Result<(), ParsingError> {
    //check formal correctness of resource
    // are all res-props valid?
    for res_prop in resource.res_props.iter().collect::<Vec<&ResProp>>() {
        check_res_prop(res_prop)?;
    }
    // is res_type valid?
    is_res_type_valid(&resource)?;
    Ok(())
}

fn is_res_type_valid(resource: &&&Resource) -> Result<(), ParsingError> {
    //check if res_type is valid (i.e. is it part of dsp-base-resources?), other cases are rare and could be implemented later
    let dsp_base_resources = ["Resource",
        "ArchiveRepresentation","AudioRepresentation","DDDRepresentation",
        "DocumentRepresentation", "MovingImageRepresentation","StillImageRepresentation",
        "TextRepresentation"];
    if dsp_base_resources.contains(&resource.res_type.as_str()) {
        return Ok(());
    }
    //todo: handle other case than dsp-resource-base cases (rare)
    Err(ParsingError::ValidationError(format!("res_type '{}' of res_prop '{}'", resource.res_type, resource.name)))
}

fn check_res_prop(res_prop: &ResProp) -> Result<(), ParsingError> {
    // check if res_prop of a resource is formally correct
    let valid_cardinalities = ["0-1", "0-n","1","1-n"];
    //check cardinalities
    if  !valid_cardinalities.contains(&&*res_prop.cardinality) {
        return Err(ParsingError::ValidationError(format!("cardinality '{}' of res_prop '{}' is invalid.", res_prop.cardinality, res_prop.name)));
    }
    if res_prop.gui_order <= 0 {
        return Err(ParsingError::ValidationError(format!("gui_order '{}' of res_prop '{}' is negative or 0, but must be 1 or greater.", res_prop.gui_order, res_prop.name)));
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
        self.ontologies.push(ontology);
    }

    fn add_to_properties(&mut self, property: Property) {
        self.properties.push(property);
    }

    fn add_to_resources(&mut self, resource: Resource) {
        self.resources.push(resource);
    }


fn build(self) -> Result<ProjectModel, ParsingError> {
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
    use std::result;
    use crate::datamodel_parse::domain::builders::Builder;
    use crate::datamodel_parse::domain::builders::project_model::{are_properties_correct, are_propnames_consistent_with_properties, contains_prefix_other_datamodel, do_ontology_names_of_propnames_exist, is_resource_correct, ProjectModelBuilder};
    use crate::datamodel_parse::domain::label::Label;
    use crate::datamodel_parse::domain::property::Property;
    use crate::datamodel_parse::domain::res_props::ResProp;
    use crate::datamodel_parse::domain::resource::Resource;

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

    #[test]
    fn test_are_properties_correct() {
        let properties = &vec![Property{
            name: "hasProperty1".to_string(),
            ontology: "rosetta".to_string(),
            object: "TextValue".to_string(),
            labels: vec![
                Label{ language_abbr: "en".to_string(), text: "a text property".to_string() }
            ],
            gui_element: Option::from("0-n".to_string()),
            },
            Property{
            name: "hasProperty2".to_string(),
            ontology: "rosetta".to_string(),
            object: "other_ontology:hasUriProperty".to_string(),
            labels: vec![
                Label{ language_abbr: "en".to_string(), text: "a uri property from another resource".to_string() }
            ],
            gui_element: Option::from("1-n".to_string()),
            },
            Property{
            name: "hasLinkToResource".to_string(),
            ontology: "rosetta".to_string(),
            object: ":OtherResource".to_string(),
            labels: vec![
                Label{ language_abbr: "en".to_string(), text: "a text property".to_string() }
            ],
            gui_element: Option::from("0-1".to_string()),
        }];
        let resources = vec![
            Resource{
                name: "Text".to_string(),
                labels: vec![],
                res_props: vec![],
                res_type: "".to_string(),
                ontology: "".to_string(),
            },
            Resource{
                name: "OtherResource".to_string(),
                labels: vec![],
                res_props: vec![],
                res_type: "".to_string(),
                ontology: "".to_string(),
            },
            Resource{
                name: "Image".to_string(),
                labels: vec![],
                res_props: vec![],
                res_type: "".to_string(),
                ontology: "".to_string(),
            }
        ];
        let properties_list = properties.iter().collect();
        let resource_names: Vec<&str> = resources.iter().map(|resource| resource.name.as_str()).collect();

       let result = are_properties_correct(properties_list,resource_names);
        println!("{:?}", result);
        assert!(result.is_ok());
    }

    #[test]
    fn test_contains_prefix_other_datamodel() {
        let object_other_datamodel = "otherDataModel:TextResource";
        let result = contains_prefix_other_datamodel(&object_other_datamodel.to_string());
        assert_eq!(result, true);
    }
    #[test]
    fn test_is_resource_correct() {
        let result = is_resource_correct(&&Resource {
            name: "Text".to_string(),
            labels: vec![Label{ language_abbr: "en".to_string(), text: "a resource".to_string() },
                         Label{ language_abbr: "de".to_string(), text: "eine Ressource".to_string()},
                         Label{ language_abbr: "fr".to_string(), text: "une ressource".to_string()},
            ],
            res_props: vec![
                ResProp{
                name: "hasText".to_string(),
                cardinality: "0-1".to_string(),
                gui_order: 1,
                ontology: "rosetta".to_string(),
            }
            ],
            res_type: "Resource".to_string(),
            ontology: "rosetta".to_string(),
        });
        assert!(result.is_ok())
    }

}
