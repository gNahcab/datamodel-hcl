use hcl::{Attribute};
use hcl::format::{Format, Formatter};
use crate::domain::ontology::Ontology;
use crate::domain::remove_useless_quotation_marks;
use crate::errors::DatamodelHCLError;

#[derive(Debug, PartialEq)]
pub struct Label{
    pub(crate) language_abbr: String,
    pub(crate) text: String,
}

pub struct LabelBlockWrapper(pub(crate)  hcl::Block);
pub struct LabelWrapper(pub(crate) hcl::Attribute);
struct TransientStructureLabels {
    labels: Vec<Label>,
}


impl TransientStructureLabels {
    fn new() -> TransientStructureLabels {
        TransientStructureLabels {
            labels: vec![],
        }
    }
    pub(crate) fn add_label(&mut self, label: Label) {
        self.labels.push(label);
    }
    pub(crate) fn is_complete(&self) -> Result<(), DatamodelHCLError> {
        if self.labels.is_empty() {
            return Err(DatamodelHCLError::ValidationError(format!("block 'labels' doesn't contain any labels")));
        }
        Ok(())
    }
}


impl LabelWrapper {
    fn to_label(self) -> Result<Label, DatamodelHCLError> {
        let mut text = self.0.expr().to_string();
        text = remove_useless_quotation_marks(text);

        let label = Label{language_abbr:self.0.key().to_string(),text:text.to_string()};
        Ok(label)
    }
}
impl LabelBlockWrapper {
    pub fn to_labels(&self) -> Result<Vec<Label>, DatamodelHCLError> {
        let mut transient_structure_label = TransientStructureLabels::new();
        let label_attributes: Vec<&hcl::Attribute> = self.0.body.attributes().collect();

        for label_attribute in label_attributes {
            let label_wrapper = LabelWrapper{ 0: label_attribute.to_owned() };
            let new_label:Label = label_wrapper.to_label()?;
            transient_structure_label.add_label(new_label);
        }
        transient_structure_label.is_complete()?;
        Ok(transient_structure_label.labels)
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
        assert!(label.as_ref().is_ok());
        assert_eq!(label.as_ref().unwrap().text, "my label");
        assert_eq!(label.as_ref().unwrap().language_abbr, "en");
    }

    #[test]
    fn test_to_labels() {
        let block = block!(
                labels {
                    en = "my text medium"
                    de = "mein Schriftmedium"
                    fr = "mon médium d'écriture"
                }
        );
        let label = LabelBlockWrapper{ 0: block }.to_labels();
        assert!(label.as_ref().is_ok());
        assert_eq!(label.as_ref().unwrap().get(0).unwrap().language_abbr,"en");
        assert_eq!(label.as_ref().unwrap().get(0).unwrap().text,"my text medium");
        assert_eq!(label.as_ref().unwrap().get(1).unwrap().language_abbr,"de");
        assert_eq!(label.as_ref().unwrap().get(1).unwrap().text,"mein Schriftmedium");
        assert_eq!(label.as_ref().unwrap().get(2).unwrap().language_abbr,"fr");
        assert_eq!(label.as_ref().unwrap().get(2).unwrap().text,"mon médium d'écriture");
    }
}
