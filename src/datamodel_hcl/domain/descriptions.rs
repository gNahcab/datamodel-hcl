pub struct Descriptions {
    pub descriptions: Vec<Description>
}

pub struct Description {
    pub description: String,
    pub language: String
}


impl TryFrom<&hcl::Block> for Descriptions {
    type Error = crate::errors::DatamodelHCLError;
    fn try_from(block: &hcl::Block) -> Result<Self, Self::Error> {
        type Error = crate::errors::DatamodelHCLError;

        if block.identifier.as_str() != "descriptions" {
            return Err(Error::ParseDescriptions(std::string::String::from("descriptions attribute is not provided.")));
        }

        let descriptions: Vec<Description> = block.body.
            attributes().
            into_iter().
            map(|attribute| Description{
                description: attribute.expr().to_string() , language:attribute.key().to_string() })
            .collect();
       Ok(Descriptions{descriptions})
    }
}

#[cfg(test)]
mod test{
    use hcl::{block};
    use crate::domain::descriptions::Descriptions;

    #[test]
    fn test_read_descriptions() {
        let descriptions_attribute = block!(
            descriptions
            {
    de = "Rosetta ist das Beispielprojekt für die <em>DaSCH Service Platform</em>. Es soll einerseits die Möglichkeiten illustrieren, die die Plattform aktuell bietet, andererseits aber auch intern aufzeigen, wo noch Verbesserungsbedarf besteht."
    fr = "Rosetta est le projet exemplaire de la <em>DaSCH Service Platform</em>. D'une part, il vise à illustrer les possibilités actuellement offertes par la plate-forme, mais d'autre part, il montre également en interne les domaines où il est encore possible d'apporter des améliorations."
    en = "Rosetta is the sample project for the <em>DaSCH Service Platform</em>. On one hand, it is intended to illustrate the possibilities currently offered by the platform, but on the other hand, it also shows internally where there is still room for improvement."
        }
        );
        let descriptions =  Descriptions::try_from(&descriptions_attribute).unwrap();
    }
}
