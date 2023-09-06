pub struct Keywords {
    pub keywords: Vec<Keyword>
}

pub struct Keyword {
    pub keyword: String
}


impl TryFrom<&hcl::Attribute> for Keywords {
    type Error = crate::errors::DatamodelHCLError;
    fn try_from(attribute: &hcl::Attribute) -> Result<Self, Self::Error> {
        type Error = crate::errors::DatamodelHCLError;

        return if attribute.key() != "keywords" {
            Err(Error::ParseKeywords(std::string::String::from("keywords attribute is not provided.")))
        } else {
            let result = match attribute.expr() {
                hcl::Expression::Array(value)
                => Ok(Keywords{keywords: value.
                    iter().
                    map(|x| Keyword{keyword:x.to_string() } ).
                    collect()}),
                _ => Err(Error::ParseKeywords(std::string::String::from("keywords needs to be an array"))),
            };
            result
        }
    }
}

#[cfg(test)]
mod test{
    use hcl::{attribute, block};
    use crate::domain::keywords::Keywords;

    #[test]
    fn test_read_keywords() {
        let keywords_attribute = attribute!(keywords = ["study", "university", "field studies", "humanities", "digital"]);
        let keywords =  Keywords::try_from(&keywords_attribute).unwrap();
    }
}