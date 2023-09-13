use hcl::{Attribute, Block, Body, body, Expression};
use crate::errors::DatamodelHCLError;

#[derive(Debug, PartialEq)]
pub struct Label{
    pub(crate) language_abbr: String,
    pub(crate) text: String,
}

impl TryFrom<&hcl::Attribute> for Label {
    type Error = DatamodelHCLError;

    fn try_from(attribute: &Attribute) -> Result<Self, Self::Error> {
        let label = Label{language_abbr:String::from(attribute.key().to_string()), text:String::from(attribute.expr().to_string())};
        Ok(label)
    }
}


#[cfg(test)]

mod test {
    use hcl::{attribute};
    use crate::domain::label::Label;
    use crate::errors::DatamodelHCLError;

    #[test]
    fn test_into_label() {
        let label_body = &attribute!(
                en = "my label"
        );
        let label: Result<Label, DatamodelHCLError> = label_body.try_into();
        assert!(label.is_ok())
    }
}
