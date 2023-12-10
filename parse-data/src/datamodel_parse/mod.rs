pub mod domain;
pub(crate) fn remove_useless_quotation_marks(string: String) -> String {
    return string.replace('"', "");
}
