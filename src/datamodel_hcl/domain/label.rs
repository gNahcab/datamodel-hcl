use hcl::{Block, Body};
use crate::errors::DatamodelHCLError;

#[derive(Debug, PartialEq)]
pub struct Label{
    language_abbr: String,
    text: String,
}

impl TryFrom<&hcl::Block> for Label {
    type Error = DatamodelHCLError;

    fn try_from(value: &Block) -> Result<Self, Self::Error> {
        todo!()
    }
}
#[cfg(test)]

mod test {
    use hcl::{block, body};
    use crate::domain::label::Label;
    use crate::errors::DatamodelHCLError;

    #[test]
    fn test_into_label() {
        let label_block = &block!(
            labels {
                en = "my label"
                de = "mein Aufkleber"
                fr = "ma vignette"
                la = "meus titulus"
                it = "la mia etichetta"
                ru = "мой лейбл"
       }
        );
        let label: Result<Label, DatamodelHCLError> = label_block.try_into();
        assert!(label.is_ok())
    }
}
