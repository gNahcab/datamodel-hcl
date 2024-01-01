use parse_data::datamodel_parse::domain::project_model::ProjectModel;
use parse_data::errors::ParsingError;
use crate::manipulation::manipulated_data_sheet::ManipulatedDataSheet;

pub(crate) fn shape_with_data_model(data: Vec<ManipulatedDataSheet>, model: ProjectModel) -> Result<(), ParsingError> {
    //return shaped data eventually
    todo!()
}