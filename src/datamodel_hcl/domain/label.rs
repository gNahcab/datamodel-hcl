use hcl::{Attribute};
use crate::domain::ontology::Ontology;
use crate::errors::DatamodelHCLError;

#[derive(Debug, PartialEq)]
pub struct Label{
    pub(crate) language_abbr: String,
    pub(crate) text: String,
}

pub struct LabelBlockWrapper(pub(crate)  hcl::Block);
pub struct LabelWrapper(pub(crate) hcl::Attribute);

impl LabelWrapper {
    fn to_label(self) -> Result<Label, DatamodelHCLError> {
        let label = Label{language_abbr:String::from(self.0.key().to_string()), text:String::from(self.0.expr().to_string())};
        Ok(label)
    }
}
impl LabelBlockWrapper {
    pub fn to_labels(&self) -> Result<Vec<Label>, DatamodelHCLError> {
        let mut labels: Vec<Label> = vec![];
        let label_attributes: Vec<&hcl::Attribute> = self.0.body.attributes().collect();

        for label_attribute in label_attributes {
            let label_wrapper = LabelWrapper{ 0: label_attribute.to_owned() };
            let new_label:Label = label_wrapper.to_label()?;
            labels.push(new_label);
        }
        Ok(labels)
    }
}



#[cfg(test)]

mod test {
    use hcl::{attribute, block};
    use crate::domain::label::{Label, LabelWrapper, LabelBlockWrapper};
    use crate::errors::DatamodelHCLError;

    #[test]
    fn test_to_label() {
        let label_body = attribute!(
                en = "my label"
        );
        let label: Result<Label, DatamodelHCLError> = LabelWrapper{ 0: label_body }.to_label();
        assert!(label.is_ok())
    }

    #[test]
    fn test_to_labels() {
        let block = block!(
            property "hasTextMedium" {
                object = "StillImageRepresentation"
                ontology = "rosetta"
                labels {
                    en = "my text medium"
                    de = "mein Schriftmedium"
                    fr = "mon médium d'écriture"
                }
                gui_element = "todo!"
            }
        );
        let label = LabelBlockWrapper{ 0: block }.to_labels();
        println!("{:?}", label);

        assert!(label.is_ok())
    }
}
