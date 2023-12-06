pub mod project_model;

pub(crate) mod ontology;
pub(crate) mod property;
pub(crate) mod resource;
mod res_props;

mod builders;
mod label;



fn remove_useless_quotation_marks(string: String) -> String {
    return string.replace('"', "");
}