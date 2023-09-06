pub struct Shortname {
    pub shortname: std::string::String
}


impl TryFrom<&hcl::Attribute> for Shortname {
    type Error = crate::errors::DatamodelHCLError;
    fn try_from(attribute: &hcl::Attribute) -> Result<Self, Self::Error> {
        type Error = crate::errors::DatamodelHCLError;

        return if attribute.key() != "shortname" {
            Err(Error::ParseShortname(std::string::String::from("shortname attribute is not provided.")))
        } else {
            let result = match attribute.expr() {
                hcl::Expression::String(value) => Ok(Shortname { shortname: value.to_string() }),
                _ => Err(Error::ParseShortname(std::string::String::from("shortname needs to be a string"))),
            };
            result
        }
    }
}

#[cfg(test)]
mod test{
    use hcl::{attribute, block};
    use crate::domain::shortname::Shortname;

    #[test]
    fn test_read_shortname() {
        let shortname_attribute = attribute!(shortname = "rosetta");
        let shortname =  Shortname::try_from(&shortname_attribute).unwrap();


    }
}
