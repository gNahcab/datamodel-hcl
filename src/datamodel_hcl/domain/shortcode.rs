pub struct Shortcode {
    pub shortcode: std::string::String
}


impl TryFrom<&hcl::Attribute> for Shortcode {
    type Error = crate::errors::DatamodelHCLError;
    fn try_from(attribute: &hcl::Attribute) -> Result<Self, Self::Error> {
        type Error = crate::errors::DatamodelHCLError;

        return if attribute.key() != "shortcode" {
            Err(Error::ParseShortcode(std::string::String::from("shortcode attribute is not provided.")))
        } else {
            let result = match attribute.expr() {
                hcl::Expression::String(value) => Ok(Shortcode { shortcode: value.to_string() }),
                _ => Err(Error::ParseShortname(std::string::String::from("shortcode needs to be a string"))),
            };
            result
        }
    }
}

#[cfg(test)]
mod test{
    use hcl::{attribute};
    use crate::domain::shortcode::Shortcode;

    #[test]
    fn test_read_shortcode() {
        let shortcode_attribute = attribute!(shortcode = "082E");
        let shortcode =  Shortcode::try_from(&shortcode_attribute).unwrap();


    }
}
