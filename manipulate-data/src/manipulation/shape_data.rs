use parse_data::datamodel_parse::domain::project_model::ProjectModel;
use parse_data::errors::ParsingError;
use crate::manipulation::manipulated_data_sheet::ManipulatedDataSheet;

pub(crate) fn shape_with_data_model(data: Vec<ManipulatedDataSheet>, model: ProjectModel) -> Result<(), ParsingError> {
    //return shaped data eventually
    for sheet in data.iter() {
        let result = is_sheet_consistent_with_model(sheet, &model);
        if result.is_err() {
            return result
        }
    }
    //return shaped data eventually
    Ok(())
}

fn is_sheet_consistent_with_model(sheet: &ManipulatedDataSheet, model: &ProjectModel) -> Result<(), ParsingError> {
    resources_exist_in_project_model(sheet, model.resources.iter().map(|resource|&resource.name).collect::<Vec<&String>>())?;
    properties_exist_in_project_model(sheet, model.properties.iter().map(|property|&property.name).collect::<Vec<&String>>())?;
    todo!()
}

fn properties_exist_in_project_model(sheet: &ManipulatedDataSheet, names: Vec<&String>) -> Result<(), ParsingError> {
    // what are properties? all headers should be properties
    let missing_headers: Vec<&String> = sheet.headers.iter().filter(|header| !names.contains(header)).collect();

    todo!()
}

fn resources_exist_in_project_model(sheet: &ManipulatedDataSheet, names: Vec<&String>) -> Result<(), ParsingError> {
    if !names.contains(&&sheet.resource) {
        return Err(ParsingError::ValidationError(format!("a transform-sheet contains the resource-name '{:?}'. But this resource doesn't exist in the data-model.All Resources in the data-model are: '{:?}'", sheet.resource, names)))
    }
    Ok(())
}