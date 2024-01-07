use std::collections::HashMap;
use parse_data::datamodel_parse::domain::project_model::ProjectModel;
use parse_data::datamodel_parse::domain::property::Property;
use parse_data::datamodel_parse::domain::resource::Resource;
use parse_data::errors::ParsingError;
use crate::manipulation::check_data::check_data;
use crate::manipulation::manipulated_data_sheet::ManipulatedDataSheet;

#[derive(Debug)]
pub struct ShapedData {
    pub resource: String,
    pub property_to_data: HashMap<String,Vec<String>>
}

impl ShapedData {
    fn new(transient: TransientShapedData) -> ShapedData {
        return ShapedData{ resource: transient.resource.unwrap().name, property_to_data: transient.property_to_data}
    }
}

struct TransientShapedData {
    id_:Option<String>,
    label: Option<String>,
    resource: Option<Resource>,
    property_to_data: HashMap<String, Vec<String>>,
    property_to_nr: HashMap<String, usize>,
}


impl TransientShapedData {
    fn new() -> TransientShapedData {
        TransientShapedData{ id_: None, label: None, resource: None, property_to_data: Default::default(), property_to_nr: Default::default() }
    }
    fn add_resource(&mut self, resources: Vec<&Resource>, name: &String) -> Result<(), ParsingError> {
        let resources: Vec<&&Resource> = resources.iter().filter(|resource|&resource.name == name).collect();
        let resource = resources.get(0);
        if resource.is_none() {
            return Err(ParsingError::CompareModelError(format!("a resource with the name '{:?}' doesn't exist in the datamodel.", name)))
        }
        let resource : Resource= resource.unwrap().to_owned().to_owned().to_owned();
        self.resource = Option::from(resource);
        Ok(())
    }
    fn add_properties(&mut self, assignments: &HashMap<String, usize>) -> Result<(), ParsingError> {
        let property_names_of_resource: Vec<String> = self.resource.as_ref().unwrap().res_props.iter().map(|prop|prop.name.to_owned()).collect();
        for (name, vec_nr) in assignments {
            if &&*name.to_lowercase() == &"id"  {
                if self.id_.is_some() {
                    return Err(ParsingError::ValidationError(format!("found multiple id-headers, but only one is allowed. The first was '{}', the second is: '{}'", self.id_.as_ref().unwrap(), name )))
                }
                self.property_to_nr.insert(name.to_owned(), vec_nr.to_owned());
            }
            if &&*name.to_lowercase() == &"label"  {
                if self.label.is_some() {
                    return Err(ParsingError::ValidationError(format!("found multiple label-headers, but only one is allowed. The first was '{}', the second is: '{}'", self.label.as_ref().unwrap(), name )))
                }
                self.property_to_nr.insert(name.to_owned(), vec_nr.to_owned());
            }
            if !property_names_of_resource.contains(&name) {
                continue
            }
            self.property_to_nr.insert(name.to_owned(), vec_nr.to_owned());
        }
        Ok(())
    }
    pub(crate) fn add_data(&mut self, data: &Vec<Vec<String>>) {
        for (value, nr) in self.property_to_nr.iter() {
            let vec_data :&Vec<String>= data.get(nr.to_owned()).unwrap();
            self.property_to_data.insert(value.to_owned(), vec_data.to_owned());
        }
    }
    pub(crate) fn check_data(&self, properties: &Vec<Property>) -> Result<(), ParsingError> {
        check_data(&self.property_to_data, properties)
    }
}
struct WrapperShapedData<'a>(ManipulatedDataSheet, &'a ProjectModel);

impl<'a> WrapperShapedData<'a>{
    fn to_shaped_data(&self) -> Result<ShapedData, ParsingError> {
       let mut transient_shaped_data = TransientShapedData::new();
       transient_shaped_data.add_resource(self.1.resources.iter().collect(), &self.0.resource)?;
       transient_shaped_data.add_properties(&self.0.assignments)?;
       transient_shaped_data.add_data(&self.0.data);
        transient_shaped_data.check_data(&self.1.properties)?;

        Ok(ShapedData::new(transient_shaped_data))
    }
}

pub(crate) fn shape_with_data_model(data: Vec<ManipulatedDataSheet>, model: ProjectModel) -> Result<Vec<ShapedData>, ParsingError> {
    //return shaped data eventually
    let mut all_shaped_data: Vec<ShapedData> = vec![];
    for manipulated in data {
        all_shaped_data.push(WrapperShapedData(manipulated, &model).to_shaped_data()?);
    }
    //return shaped data eventually
    Ok(all_shaped_data)
}


fn properties_exist_in_project_model(sheet: &ManipulatedDataSheet, names: Vec<&String>) -> Result<(), ParsingError> {
    // what are properties? all assignments should be properties, if they are not, they should be removed

    todo!()
}

fn resources_exist_in_project_model(sheet: &ManipulatedDataSheet, names: Vec<&String>) -> Result<(), ParsingError> {
    if !names.contains(&&sheet.resource) {
        return Err(ParsingError::ValidationError(format!("a transform-sheet contains the resource-name '{:?}'. But this resource doesn't exist in the data-model.All Resources in the data-model are: '{:?}'", sheet.resource, names)))
    }
    Ok(())
}